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
let server = null; // { proc, out }
let playing = null; // { doc, name, index, section, sectionRanges }
let debounce = null;
let lensChanged = new vscode.EventEmitter();
const cache = new Map(); // doc uri -> { version, fns }

const highlight = vscode.window.createTextEditorDecorationType({
  backgroundColor: "rgba(120,200,80,0.18)",
  overviewRulerColor: "rgba(120,200,80,0.8)",
  overviewRulerLane: vscode.OverviewRulerLane.Center,
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
  const out = vscode.window.createOutputChannel("LotW Music");
  proc.stdout.setEncoding("utf8");
  let buf = "";
  proc.stdout.on("data", (d) => {
    buf += d;
    let nl;
    while ((nl = buf.indexOf("\n")) >= 0) {
      handleEvent(buf.slice(0, nl).trim(), out);
      buf = buf.slice(nl + 1);
    }
  });
  proc.stderr.on("data", (d) => out.append(String(d)));
  proc.on("exit", (c) => { out.appendLine(`server exited (${c})`); server = null; });
  server = { proc, out };
  return server;
}

function send(cmd) {
  if (server && server.proc.stdin.writable) server.proc.stdin.write(cmd + "\n");
}

function handleEvent(line, out) {
  if (!line) return;
  if (line.startsWith("pos ")) {
    const secs = line.split(/\s+/).slice(2).map(Number).filter((n) => n >= 0);
    if (secs.length) applyHighlight(Math.max(...secs));
    return;
  }
  out.appendLine(line);
  if (line.startsWith("err ")) vscode.window.showWarningMessage("LotW Music: " + line.slice(4));
}

function applyHighlight(sectionIdx) {
  const ed = vscode.window.activeTextEditor;
  if (!ed || !playing || ed.document !== playing.doc) return;
  const ranges = playing.sectionRanges || [];
  ed.setDecorations(highlight, sectionIdx < ranges.length ? [ranges[sectionIdx]] : []);
}

// --- transport ---

async function play(doc, name, section) {
  ensureServer(doc);
  const idx = await songIndex(doc, name);
  playing = { doc, name, index: idx, section, sectionRanges: await sectionRanges(doc, name) };
  writeAndSend();
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
