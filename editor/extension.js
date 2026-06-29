// LotW Music — VSCode extension.
//
// Adds Play/Stop/Loop CodeLens buttons above each `pub fn name() -> Song`,
// spawns the Rust `music-server`, and highlights the section that's currently
// playing. While a song plays, edits are debounced, recompiled (via the
// server's JIT), and reloaded in place — playback keeps its position.

const vscode = require("vscode");
const cp = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");

const SONG_FN = /pub\s+fn\s+(\w+)\s*\(\s*\)\s*->\s*Song/g;

let server = null; // { proc, ready }
let playing = null; // { doc, name, index }
let debounce = null;
const highlight = vscode.window.createTextEditorDecorationType({
  backgroundColor: "rgba(120,200,80,0.18)",
  isWholeLine: false,
  overviewRulerColor: "rgba(120,200,80,0.8)",
  overviewRulerLane: vscode.OverviewRulerLane.Center,
});

function workspaceDir(doc) {
  const f = vscode.workspace.getWorkspaceFolder(doc.uri);
  return f ? f.uri.fsPath : path.dirname(doc.uri.fsPath);
}

// --- server process ---

function ensureServer(doc) {
  if (server) return server;
  const cfg = vscode.workspace.getConfiguration("lotwMusic");
  const dir = workspaceDir(doc);
  const proc = cp.spawn(
    "cargo",
    ["run", "--quiet", "--bin", "music-server", "--features", "server", "--", cfg.get("rom", "rom/lotw.nes")],
    { cwd: dir, env: { ...process.env, NIX_LDFLAGS: "" } }
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
    const p = line.split(/\s+/);
    const secs = p.slice(2).map(Number).filter((n) => n >= 0);
    if (secs.length) applyHighlight(Math.max(...secs));
    return;
  }
  out.appendLine(line); // loaded / err / etc.
  if (line.startsWith("err ")) vscode.window.showWarningMessage("LotW Music: " + line.slice(4));
}

// --- source parsing ---

// Index of a song function = its number in the get() dispatch (`N => name(),`),
// else its position among song fns.
function songIndex(text, name) {
  const m = text.match(new RegExp("(\\d+)\\s*=>\\s*" + name + "\\s*\\(\\s*\\)", ""));
  if (m) return Number(m[1]);
  let i = 0, mm;
  const re = new RegExp(SONG_FN.source, "g");
  while ((mm = re.exec(text))) { if (mm[1] === name) return i; i++; }
  return 0;
}

// The function body's text range and the document ranges of each `section(`.
function sectionRanges(doc, name) {
  const text = doc.getText();
  const start = text.search(new RegExp("pub\\s+fn\\s+" + name + "\\s*\\("));
  if (start < 0) return [];
  // Find the function's closing brace by counting braces from the first `{`.
  let i = text.indexOf("{", start), depth = 0, end = text.length;
  for (let j = i; j < text.length; j++) {
    if (text[j] === "{") depth++;
    else if (text[j] === "}" && --depth === 0) { end = j; break; }
  }
  const body = text.slice(start, end);
  const ranges = [];
  const re = /\bsection\s*\(/g;
  let m;
  while ((m = re.exec(body))) {
    const a = start + m.index;
    ranges.push(new vscode.Range(doc.positionAt(a), doc.positionAt(a + m[0].length)));
  }
  return ranges;
}

function applyHighlight(sectionIdx) {
  const ed = vscode.window.activeTextEditor;
  if (!ed || !playing || ed.document !== playing.doc) return;
  const ranges = sectionRanges(ed.document, playing.name);
  if (sectionIdx < ranges.length) ed.setDecorations(highlight, [ranges[sectionIdx]]);
}

// --- transport ---

function functionAt(doc, line) {
  const text = doc.getText();
  let m, best = null;
  const re = new RegExp(SONG_FN.source, "g");
  while ((m = re.exec(text))) {
    const fnLine = doc.positionAt(m.index).line;
    if (fnLine <= line) best = { name: m[1], line: fnLine };
  }
  return best;
}

function play(doc, name) {
  ensureServer(doc);
  const idx = songIndex(doc.getText(), name);
  const tmp = path.join(os.tmpdir(), `lotw_music_${process.pid}.rs`);
  fs.writeFileSync(tmp, doc.getText());
  playing = { doc, name, index: idx };
  send(`src ${tmp} ${idx}`);
}

function reloadIfPlaying() {
  if (!playing) return;
  const tmp = path.join(os.tmpdir(), `lotw_music_${process.pid}.rs`);
  fs.writeFileSync(tmp, playing.doc.getText());
  send(`src ${tmp} ${playing.index}`);
}

// --- CodeLens ---

class Lenses {
  provideCodeLenses(doc) {
    const lenses = [];
    let m;
    const re = new RegExp(SONG_FN.source, "g");
    while ((m = re.exec(doc.getText()))) {
      const range = new vscode.Range(doc.positionAt(m.index), doc.positionAt(m.index));
      const name = m[1];
      lenses.push(new vscode.CodeLens(range, { title: "▶ Play", command: "lotwMusic.play", arguments: [doc, name] }));
      lenses.push(new vscode.CodeLens(range, { title: "⏹ Stop", command: "lotwMusic.stop" }));
      lenses.push(new vscode.CodeLens(range, { title: "🔁 Loop", command: "lotwMusic.toggleLoop" }));
    }
    return lenses;
  }
}

function activate(ctx) {
  ctx.subscriptions.push(vscode.languages.registerCodeLensProvider({ language: "rust" }, new Lenses()));

  ctx.subscriptions.push(
    vscode.commands.registerCommand("lotwMusic.play", (doc, name) => {
      if (!doc) {
        const ed = vscode.window.activeTextEditor;
        if (!ed) return;
        const fn = functionAt(ed.document, ed.selection.active.line);
        if (!fn) return vscode.window.showInformationMessage("No song() under the cursor.");
        doc = ed.document;
        name = fn.name;
      }
      play(doc, name);
    }),
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
