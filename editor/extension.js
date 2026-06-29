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
(call_expression
  function: (identifier) @sec
  (#eq? @sec "section"))
`;

let ts = null; // Promise<{ parser, query }>
let server = null; // { proc }
let out = null; // shared output channel
let playing = null; // { doc, name, index, section, sectionRanges, channelElements }
let debounce = null;
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

// Song functions in document order, each with its section() byte offsets.
async function structure(doc) {
  const key = doc.uri.toString();
  const hit = cache.get(key);
  if (hit && hit.version === doc.version) return hit.fns;

  const { parser, query } = await ts;
  const tree = parser.parse(doc.getText());
  const fns = [];
  const sections = [];
  for (const m of query.matches(tree.rootNode)) {
    const cap = {};
    for (const c of m.captures) cap[c.name] = c.node;
    if (cap.name && cap.body) {
      fns.push({ name: cap.name.text, nameAt: cap.name.startIndex, bodyA: cap.body.startIndex, bodyB: cap.body.endIndex, sections: [] });
    }
    if (cap.sec) sections.push(cap.sec.startIndex);
  }
  fns.sort((a, b) => a.nameAt - b.nameAt);
  for (const s of sections) {
    const f = fns.find((f) => s >= f.bodyA && s < f.bodyB);
    if (f) f.sections.push(s);
  }
  for (const f of fns) f.sections.sort((a, b) => a - b);
  cache.set(key, { version: doc.version, fns });
  return fns;
}

async function songIndex(doc, name) {
  const fns = await structure(doc);
  const i = fns.findIndex((f) => f.name === name);
  return i < 0 ? 0 : i;
}

async function sectionRanges(doc, name) {
  const fns = await structure(doc);
  const fn = fns.find((f) => f.name === name);
  if (!fn) return [];
  return fn.sections.map((off) => new vscode.Range(doc.positionAt(off), doc.positionAt(off + 7)));
}

// Per channel [0..3], the ordered source elements of song `name` across all
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
      cargs.namedChildren.forEach((ref, c) => {
        if (c > 3) return;
        const arr = arrayOf(ref);
        if (!arr) return;
        for (const el of arr.namedChildren) chans[c].push({ a: el.startIndex, b: el.endIndex, toks: tokCount(el) });
      });
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

const PARAM_MACROS = ["duty", "volume", "flags", "pitch", "sweep"];
function tokCount(el) {
  if (el.type === "macro_invocation") {
    const name = (el.childForFieldName("macro") || {}).text;
    if (name === "env") return envTokCount(el, 1); // env!(param, segs…): skip the param group
    if (PARAM_MACROS.includes(name)) return envTokCount(el, 0); // volume!(segs…): param is the name
  }
  return 1; // notes, rests, duty()/raw()/... each assemble to one Tok
}

// Envelope tokens = sum over segments of 1 (the command) + the carrier notes.
// `skip` drops the leading parameter group (1 for env!, 0 for volume!/…).
function envTokCount(el, skip) {
  const tt = el.descendantsOfType("token_tree")[0];
  if (!tt) return 1;
  const toks = tt.children.filter((c) => c.type !== "(" && c.type !== ")");
  const segs = [[]];
  for (const c of toks) (c.type === "," ? segs.push([]) : segs[segs.length - 1].push(c));
  let n = 0;
  for (const seg of segs.slice(skip)) n += 1 + seg.filter((c) => c.type === "identifier").length;
  return n || 1;
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
function applyHighlight(tokens) {
  if (!playing || playing.section != null) return; // isolated section keeps its static highlight
  const ed = editorFor(playing.doc);
  const chans = playing.channelElements;
  if (!ed || !chans) return;
  const ranges = [];
  for (let c = 0; c < 4; c++) {
    const t = tokens[c];
    if (t == null || t < 0) continue;
    let cum = 0;
    for (const el of chans[c]) {
      if (t < cum + el.toks) {
        ranges.push(new vscode.Range(playing.doc.positionAt(el.a), playing.doc.positionAt(el.b)));
        break;
      }
      cum += el.toks;
    }
  }
  ed.setDecorations(highlight, ranges);
}

// --- transport ---

async function play(doc, name, section) {
  ensureServer(doc);
  const idx = await songIndex(doc, name);
  const els = await channelElements(doc, name);
  posLogged = false;
  out.appendLine(`▶ ${name} (song ${idx})${section != null ? ` §${section + 1}` : ""}: elements/channel = [${els.map((c) => c.length).join(", ")}]`);
  playing = { doc, name, index: idx, section, sectionRanges: await sectionRanges(doc, name), channelElements: els };
  writeAndSend();
  // Playing one section in isolation: highlight it statically (it loops, so the
  // server doesn't stream section changes to drive the highlight).
  if (section != null) {
    const ed = vscode.window.activeTextEditor;
    if (ed && ed.document === doc && playing.sectionRanges[section]) {
      ed.setDecorations(highlight, [playing.sectionRanges[section]]);
    }
  }
}

function writeAndSend() {
  if (!playing) return;
  const tmp = path.join(os.tmpdir(), `lotw_music_${process.pid}.rs`);
  fs.writeFileSync(tmp, playing.doc.getText());
  send(`src ${tmp} ${playing.index}${playing.section != null ? " " + playing.section : ""}`);
}

async function reloadIfPlaying() {
  if (!playing) return;
  playing.sectionRanges = await sectionRanges(playing.doc, playing.name);
  playing.channelElements = await channelElements(playing.doc, playing.name);
  writeAndSend();
}

async function functionAt(doc, line) {
  const fns = await structure(doc);
  let best = null;
  for (const f of fns) if (doc.positionAt(f.nameAt).line <= line) best = f;
  return best;
}

// --- CodeLens ---

class Lenses {
  get onDidChangeCodeLenses() { return lensChanged.event; }
  async provideCodeLenses(doc) {
    const fns = await structure(doc);
    const lenses = [];
    const at = (off) => { const p = doc.positionAt(off); return new vscode.Range(p, p); };
    for (const fn of fns) {
      const r = at(fn.nameAt);
      lenses.push(new vscode.CodeLens(r, { title: "▶ Play", command: "lotwMusic.play", arguments: [doc, fn.name] }));
      lenses.push(new vscode.CodeLens(r, { title: "⏹ Stop", command: "lotwMusic.stop" }));
      lenses.push(new vscode.CodeLens(r, { title: "🔁 Loop", command: "lotwMusic.toggleLoop" }));
      fn.sections.forEach((off, k) => {
        const sr = at(off);
        lenses.push(new vscode.CodeLens(sr, { title: `▶ §${k + 1}`, command: "lotwMusic.playSection", arguments: [doc, fn.name, k] }));
        lenses.push(new vscode.CodeLens(sr, { title: "⏹", command: "lotwMusic.stop" }));
        lenses.push(new vscode.CodeLens(sr, { title: "🔁", command: "lotwMusic.toggleLoop" }));
      });
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
      play(doc, name);
    }),
    vscode.commands.registerCommand("lotwMusic.playSection", (doc, name, section) => play(doc, name, section)),
    vscode.commands.registerCommand("lotwMusic.stop", () => {
      send("stop");
      playing = null;
      const ed = vscode.window.activeTextEditor;
      if (ed) ed.setDecorations(highlight, []);
    }),
    vscode.commands.registerCommand("lotwMusic.toggleLoop", () => send("loop on"))
  );

  ctx.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.languageId !== "rust") return;
      lensChanged.fire(); // refresh CodeLens (structure may have changed)
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
