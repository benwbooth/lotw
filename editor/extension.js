// LotW Music — VSCode extension.
//
// Adds Play/Stop/Loop CodeLens above each `pub fn name() -> Song` and before
// each `section(...)`, spawns the Rust `music-server`, and highlights the
// section that's currently playing. While a song plays, edits are debounced,
// recompiled (via the server's JIT), and reloaded in place — keeping position.
//
// The source is parsed with tree-sitter (no regex): correct through comments,
// strings and nesting. A function's position among the song fns is its song
// index (the generator emits them in get()-index order).

const vscode = require("vscode");
const { Parser, Language, Query } = require("web-tree-sitter");
const cp = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");

const QUERY = `
(function_item
  name: (identifier) @name
  return_type: (type_identifier) @ret
  body: (block) @body
  (#eq? @ret "Song"))
(function_item
  name: (identifier) @sfxname
  return_type: (generic_type) @sfxret
  (#eq? @sfxret "Vec<Tok>"))
(call_expression
  function: (identifier) @sec
  (#eq? @sec "section"))
`;

let ts = null; // Promise<{ parser, query }>
let server = null; // { proc }
let out = null; // shared output channel
let playing = null; // { doc, name, index, section, channelElements, paused }
let looping = true; // loop toggle (🔁), mirrored to the server
let debounce = null;
let previewDebounce = null;
let lensChanged = new vscode.EventEmitter();
const cache = new Map(); // doc uri -> { version, fns }

const highlight = vscode.window.createTextEditorDecorationType({
  backgroundColor: "rgba(120,220,90,0.40)",
  border: "1px solid rgba(120,220,90,0.95)",
  borderRadius: "2px",
  overviewRulerColor: "rgba(120,220,90,1.0)",
  overviewRulerLane: vscode.OverviewRulerLane.Full,
});

// --- tree-sitter parsing (cached per document version) ---

function initTreeSitter(ctx) {
  ts = (async () => {
    await Parser.init();
    const parser = new Parser();
    const rust = await Language.load(path.join(ctx.extensionPath, "node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm"));
    parser.setLanguage(rust);
    return { parser, query: new Query(rust, QUERY) };
  })();
}

// Song functions (with their section() offsets) and SFX functions (fn -> Vec<Tok>),
// both in document order, so a function's position is its ROM index.
async function structure(doc) {
  const key = doc.uri.toString();
  const hit = cache.get(key);
  if (hit && hit.version === doc.version) return hit.val;

  const { parser, query } = await ts;
  const tree = parser.parse(doc.getText());
  const fns = [];
  const sfx = [];
  const sections = [];
  for (const m of query.matches(tree.rootNode)) {
    const cap = {};
    for (const c of m.captures) cap[c.name] = c.node;
    if (cap.name && cap.body) {
      fns.push({ name: cap.name.text, nameAt: cap.name.startIndex, bodyA: cap.body.startIndex, bodyB: cap.body.endIndex, sections: [] });
    }
    if (cap.sfxname) sfx.push({ name: cap.sfxname.text, nameAt: cap.sfxname.startIndex });
    if (cap.sec) sections.push(cap.sec.startIndex);
  }
  fns.sort((a, b) => a.nameAt - b.nameAt);
  sfx.sort((a, b) => a.nameAt - b.nameAt);
  sfx.forEach((s, i) => (s.index = i)); // contiguous in the generated dispatch
  for (const s of sections) {
    const f = fns.find((f) => s >= f.bodyA && s < f.bodyB);
    if (f) f.sections.push(s);
  }
  for (const f of fns) f.sections.sort((a, b) => a - b);
  const val = { fns, sfx };
  cache.set(key, { version: doc.version, val });
  return val;
}

async function songIndex(doc, name) {
  const i = (await structure(doc)).fns.findIndex((f) => f.name === name);
  return i < 0 ? 0 : i;
}

// Per channel [0..3], the source elements grouped by section of song `name` across all
// sections, each with its byte span and how many Tok it assembles to (1, except
// env! which expands to one cmd + its carrier notes per segment). Lets us map
// the server's per-channel token index back to the exact note being played.
const elemCache = new Map();
async function channelElements(doc, name) {
  const key = `${doc.uri}#${name}#${doc.version}`;
  if (elemCache.has(key)) return elemCache.get(key);
  const { parser } = await ts;
  const tree = parser.parse(doc.getText());
  const fn = tree.rootNode.descendantsOfType("function_item").find((f) => {
    const n = f.childForFieldName("name");
    return n && n.text === name;
  });
  const chans = [[], [], [], []];
  const songCall = fn && fn.descendantsOfType("call_expression").find((c) => {
    const g = c.childForFieldName("function");
    return g && g.text === "song";
  });
  if (songCall) {
    const args = songCall.childForFieldName("arguments");
    const sectionsArr = args && arrayOf(args.namedChildren[1]);
    for (const sec of sectionsArr ? sectionsArr.namedChildren : []) {
      if (sec.type !== "call_expression") continue;
      const cargs = sec.childForFieldName("arguments");
      // One element group per section, per channel (so a section play can map
      // its section-relative token index into just that section's notes).
      for (let c = 0; c < 4; c++) {
        const arr = arrayOf(cargs.namedChildren[c]);
        const group = [];
        if (arr) for (const el of arr.namedChildren) group.push(...elemsOf(el));
        chans[c].push(group);
      }
    }
  }
  elemCache.set(key, chans);
  return chans;
}

function arrayOf(node) {
  if (!node) return null;
  if (node.type === "array_expression") return node;
  return node.descendantsOfType("array_expression")[0] || null;
}

// A complete note token: a pitched note (c4e, as3q3), a noise hit (hite), or a
// rest (rq). Used for type-to-play.
const VAL = "(?:hdd|hd|qdd|qd|edd|ed|id|td|h3|q3|e3|i3|t3|w|h|q|e|i|t|x)";
const NOTE_RE = new RegExp(`^(?:[a-g]s?\\d+|hit)${VAL}$`);

// Which channel (0..3 = pulse1/pulse2/tri/noise) the byte `offset` is inside,
// i.e. which of a section()'s four array args contains it (null if none).
async function channelAt(doc, offset) {
  const { parser } = await ts;
  const tree = parser.parse(doc.getText());
  for (const call of tree.rootNode.descendantsOfType("call_expression")) {
    const fn = call.childForFieldName("function");
    if (!fn || fn.text !== "section") continue;
    const refs = call.childForFieldName("arguments").namedChildren;
    for (let c = 0; c < Math.min(4, refs.length); c++) {
      if (offset >= refs[c].startIndex && offset <= refs[c].endIndex) return c;
    }
  }
  return null;
}

// Type-to-play: if the token just typed at the cursor is a complete note, hear it.
async function previewAtCursor(doc) {
  const ed = vscode.window.visibleTextEditors.find((e) => e.document.uri.toString() === doc.uri.toString());
  if (!ed) return;
  const pos = ed.selection.active;
  const m = doc.lineAt(pos.line).text.slice(0, pos.character).match(/[a-z0-9]+$/);
  if (!m || !NOTE_RE.test(m[0])) return;
  const chan = await channelAt(doc, doc.offsetAt(pos));
  if (chan == null) return;
  ensureServer(doc); // start the server on first note so type-to-play works before Play
  send(`preview ${chan} ${m[0]}`);
}

const PARAM_MACROS = ["duty", "volume", "flags", "pitch", "sweep"];

// A source element -> the list of {a, b, toks:1} it assembles to. Most elements
// are one token; an envelope macro (env!/duty!/volume!/…) expands to one element
// per emitted token — the command (spanning its value) and each carrier note —
// so the playhead highlights the individual value/note inside it, not the block.
function elemsOf(el) {
  if (el.type === "macro_invocation") {
    const name = (el.childForFieldName("macro") || {}).text;
    if (name === "env") return envElements(el, 1); // env!(param, segs…): skip the param group
    if (PARAM_MACROS.includes(name)) return envElements(el, 0); // volume!(segs…): param is the name
  }
  return [{ a: el.startIndex, b: el.endIndex, toks: 1 }];
}

// Each segment `<value> <note>…` -> a command element (spanning the value) then
// one element per carrier note. `skip` drops the leading parameter group.
function envElements(el, skip) {
  const tt = el.descendantsOfType("token_tree")[0];
  if (!tt) return [{ a: el.startIndex, b: el.endIndex, toks: 1 }];
  const toks = tt.children.filter((c) => c.type !== "(" && c.type !== ")");
  const segs = [[]];
  for (const c of toks) (c.type === "," ? segs.push([]) : segs[segs.length - 1].push(c));
  const out = [];
  for (const seg of segs.slice(skip)) {
    if (!seg.length) continue;
    const val = seg.filter((c) => c.type !== "identifier"); // the number / +N value
    const v0 = val[0] || seg[0];
    const v1 = val[val.length - 1] || seg[0];
    out.push({ a: v0.startIndex, b: v1.endIndex, toks: 1 }); // the param command
    for (const n of seg.filter((c) => c.type === "identifier")) out.push({ a: n.startIndex, b: n.endIndex, toks: 1 });
  }
  return out.length ? out : [{ a: el.startIndex, b: el.endIndex, toks: 1 }];
}

// --- server process ---

function workspaceDir(doc) {
  const f = vscode.workspace.getWorkspaceFolder(doc.uri);
  return f ? f.uri.fsPath : path.dirname(doc.uri.fsPath);
}

function ensureServer(doc) {
  if (server) return server;
  const cfg = vscode.workspace.getConfiguration("lotwMusic");
  const proc = cp.spawn(
    "cargo",
    ["run", "--quiet", "--bin", "music-server", "--features", "server", "--", cfg.get("rom", "rom/lotw.nes")],
    { cwd: workspaceDir(doc), env: { ...process.env, NIX_LDFLAGS: "" } }
  );
  proc.stdout.setEncoding("utf8");
  let buf = "";
  proc.stdout.on("data", (d) => {
    buf += d;
    let nl;
    while ((nl = buf.indexOf("\n")) >= 0) {
      handleEvent(buf.slice(0, nl).trim());
      buf = buf.slice(nl + 1);
    }
  });
  proc.stderr.on("data", (d) => out.append(String(d)));
  proc.on("exit", (c) => { out.appendLine(`server exited (${c})`); server = null; });
  server = { proc };
  return server;
}

function send(cmd) {
  if (server && server.proc.stdin.writable) server.proc.stdin.write(cmd + "\n");
}

let posLogged = false;
function handleEvent(line) {
  if (!line) return;
  if (line.startsWith("pos ")) {
    if (!posLogged) { out.appendLine("first pos event: " + line); posLogged = true; }
    applyHighlight(line.split(/\s+/).slice(2).map(Number)); // [t0,t1,t2,t3] token indices
    return;
  }
  out.appendLine(line);
  if (line.startsWith("err ")) vscode.window.showWarningMessage("LotW Music: " + line.slice(4));
}

// The visible editor showing a document (not necessarily the active one).
function editorFor(doc) {
  const uri = doc.uri.toString();
  return vscode.window.visibleTextEditors.find((e) => e.document.uri.toString() === uri);
}

// Highlight the source element each channel is on (up to 4 notes at once).
let hlLog = [-1, -1, -1, -1]; // last logged element index per channel
const CH = ["pulse1", "pulse2", "tri", "noise"];
function applyHighlight(tokens) {
  if (!playing) return;
  const ed = editorFor(playing.doc);
  const chans = playing.channelElements;
  if (!ed || !chans) return;
  const ranges = [];
  for (let c = 0; c < 4; c++) {
    const t = tokens[c];
    if (t == null || t < 0) continue;
    // A section play maps its section-relative index into that section's notes;
    // a full-song play maps the absolute index across all sections.
    const els = playing.section != null ? chans[c][playing.section] || [] : chans[c].flat();
    let cum = 0;
    for (let ei = 0; ei < els.length; ei++) {
      const el = els[ei];
      if (t < cum + el.toks) {
        ranges.push(new vscode.Range(playing.doc.positionAt(el.a), playing.doc.positionAt(el.b)));
        // Log when the highlighted element changes (or wraps) for this channel.
        if (ei !== hlLog[c]) {
          const wrapped = ei < hlLog[c] ? " (wrap)" : "";
          out.appendLine(`hl ${CH[c]}: tok ${t} -> elem ${ei}/${els.length}${wrapped} "${playing.doc.getText(ranges[ranges.length - 1])}"`);
          hlLog[c] = ei;
        }
        break;
      }
      cum += el.toks;
    }
  }
  ed.setDecorations(highlight, ranges);
}

// --- transport ---

function isCurrent(doc, name, section) {
  return (
    playing &&
    playing.name === name &&
    playing.doc.uri.toString() === doc.uri.toString() &&
    (section == null ? playing.section == null : playing.section === section)
  );
}

// The Play/§N button: a play/pause toggle for the song/section/SFX it's on.
// `sfxIndex` non-null means this is a sound effect (played on the pulse2 channel).
async function playToggle(doc, name, section, sfxIndex) {
  if (isCurrent(doc, name, section)) {
    playing.paused = !playing.paused;
    send(playing.paused ? "stop" : "play");
    lensChanged.fire();
  } else {
    await play(doc, name, section, sfxIndex);
  }
}

async function play(doc, name, section, sfxIndex) {
  ensureServer(doc);
  const sfx = sfxIndex != null;
  const idx = sfx ? sfxIndex : await songIndex(doc, name);
  const els = sfx ? await sfxElements(doc, name) : await channelElements(doc, name);
  posLogged = false;
  hlLog = [-1, -1, -1, -1];
  out.appendLine(`▶ ${name} (${sfx ? "sfx" : "song"} ${idx})${section != null ? ` §${section + 1}` : ""}: elements/channel = [${els.map((c) => c.length).join(", ")}]`);
  playing = { doc, name, index: idx, section, channelElements: els, paused: false, sfx };
  send(`loop ${looping ? "on" : "off"}`);
  writeAndSend();
  lensChanged.fire();
}

function writeAndSend() {
  if (!playing) return;
  const tmp = path.join(os.tmpdir(), `lotw_music_${process.pid}.rs`);
  fs.writeFileSync(tmp, playing.doc.getText());
  if (playing.sfx) send(`sfxsrc ${tmp} ${playing.index}`);
  else send(`src ${tmp} ${playing.index}${playing.section != null ? " " + playing.section : ""}`);
}

async function reloadIfPlaying() {
  if (!playing) return;
  playing.channelElements = playing.sfx ? await sfxElements(playing.doc, playing.name) : await channelElements(playing.doc, playing.name);
  writeAndSend();
}

// A sound-effect function (`line(tempo, &[…])`) as channelElements: its elements
// on channel 1 (pulse2), the others empty — so the song highlight path applies.
async function sfxElements(doc, name) {
  const { parser } = await ts;
  const tree = parser.parse(doc.getText());
  const fn = tree.rootNode.descendantsOfType("function_item").find((f) => (f.childForFieldName("name") || {}).text === name);
  const chans = [[], [], [], []];
  const lineCall = fn && fn.descendantsOfType("call_expression").find((c) => (c.childForFieldName("function") || {}).text === "line");
  if (lineCall) {
    const arr = arrayOf(lineCall.childForFieldName("arguments").namedChildren[1]);
    const els = [];
    if (arr) for (const el of arr.namedChildren) els.push(...elemsOf(el));
    chans[1] = [els]; // pulse2, single section
  }
  return chans;
}

async function functionAt(doc, line) {
  const { fns } = await structure(doc);
  let best = null;
  for (const f of fns) if (doc.positionAt(f.nameAt).line <= line) best = f;
  return best;
}

// --- CodeLens ---

class Lenses {
  get onDidChangeCodeLenses() { return lensChanged.event; }
  async provideCodeLenses(doc) {
    const { fns, sfx } = await structure(doc);
    const lenses = [];
    const at = (off) => { const p = doc.positionAt(off); return new vscode.Range(p, p); };
    const playIcon = (cur) => (cur && !playing.paused ? "⏸ Pause" : "▶ Play");
    const loopTitle = (short) => (short ? (looping ? "🔁" : "🔁̶") : `🔁 Loop ${looping ? "on" : "off"}`);
    for (const fn of fns) {
      const r = at(fn.nameAt);
      const cur = isCurrent(doc, fn.name, null);
      lenses.push(new vscode.CodeLens(r, { title: playIcon(cur), command: "lotwMusic.play", arguments: [doc, fn.name] }));
      lenses.push(new vscode.CodeLens(r, { title: "⏹ Stop", command: "lotwMusic.stop" }));
      lenses.push(new vscode.CodeLens(r, { title: loopTitle(false), command: "lotwMusic.toggleLoop" }));
      fn.sections.forEach((off, k) => {
        const sr = at(off);
        const scur = isCurrent(doc, fn.name, k);
        lenses.push(new vscode.CodeLens(sr, { title: scur && !playing.paused ? `⏸ §${k + 1}` : `▶ §${k + 1}`, command: "lotwMusic.playSection", arguments: [doc, fn.name, k] }));
        lenses.push(new vscode.CodeLens(sr, { title: "⏹", command: "lotwMusic.stop" }));
        lenses.push(new vscode.CodeLens(sr, { title: loopTitle(true), command: "lotwMusic.toggleLoop" }));
      });
    }
    // Sound effects get the same transport as songs (no sections).
    for (const s of sfx) {
      const r = at(s.nameAt);
      const cur = isCurrent(doc, s.name, null);
      lenses.push(new vscode.CodeLens(r, { title: playIcon(cur), command: "lotwMusic.playSfx", arguments: [doc, s.name, s.index] }));
      lenses.push(new vscode.CodeLens(r, { title: "⏹ Stop", command: "lotwMusic.stop" }));
      lenses.push(new vscode.CodeLens(r, { title: loopTitle(false), command: "lotwMusic.toggleLoop" }));
    }
    return lenses;
  }
}

function activate(ctx) {
  out = vscode.window.createOutputChannel("LotW Music");
  out.appendLine("LotW Music activated");
  initTreeSitter(ctx);
  ctx.subscriptions.push(vscode.languages.registerCodeLensProvider({ language: "rust" }, new Lenses()));

  ctx.subscriptions.push(
    vscode.commands.registerCommand("lotwMusic.play", async (doc, name) => {
      if (!doc) {
        const ed = vscode.window.activeTextEditor;
        if (!ed) return;
        const fn = await functionAt(ed.document, ed.selection.active.line);
        if (!fn) return vscode.window.showInformationMessage("No song() under the cursor.");
        doc = ed.document;
        name = fn.name;
      }
      playToggle(doc, name);
    }),
    vscode.commands.registerCommand("lotwMusic.playSection", (doc, name, section) => playToggle(doc, name, section)),
    vscode.commands.registerCommand("lotwMusic.playSfx", (doc, name, index) => playToggle(doc, name, undefined, index)),
    vscode.commands.registerCommand("lotwMusic.stop", () => {
      send("reset"); // stop + return to the start of the song/section
      const ed = playing ? editorFor(playing.doc) : vscode.window.activeTextEditor;
      if (ed) ed.setDecorations(highlight, []);
      playing = null;
      lensChanged.fire();
    }),
    vscode.commands.registerCommand("lotwMusic.toggleLoop", () => {
      looping = !looping;
      send(`loop ${looping ? "on" : "off"}`);
      lensChanged.fire();
    })
  );

  ctx.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.languageId !== "rust") return;
      lensChanged.fire(); // refresh CodeLens (structure may have changed)
      // Type-to-play: preview the note token at the cursor (separate voice).
      clearTimeout(previewDebounce);
      previewDebounce = setTimeout(() => previewAtCursor(e.document), 120);
      if (!playing || e.document !== playing.doc) return;
      clearTimeout(debounce);
      const ms = vscode.workspace.getConfiguration("lotwMusic").get("debounceMs", 300);
      debounce = setTimeout(reloadIfPlaying, ms);
    })
  );
}

function deactivate() {
  if (server) server.proc.kill();
}

module.exports = { activate, deactivate };
