# LotW Music — VSCode extension

Live-edit playback for Legacy of the Wizard songs written in the `lotw_music`
DSL. Above each `pub fn name() -> Song` it shows **▶ Play / ⏹ Stop / 🔁 Loop**
CodeLens buttons; pressing Play compiles and plays the song through the real
ported sound engine, and highlights the section that's currently sounding. While
a song plays, your edits are debounced, recompiled (the `music-server`'s JIT
recompiles the `music-jit` cdylib in ~130–200 ms), and reloaded **in place** —
playback keeps its position.

## How it works

```
extension.js ──(src <tmpfile> <idx> / play / stop, on stdin)──▶ music-server (Rust)
   ▲  pos <tick> <s0> <s1> <s2> <s3>  ◀───────────────────────  ├─ compile music-jit, dlopen, read song bytes
   │  highlight the section() being played                       ├─ patch into ROM PRG, run sound_tick + APU
                                                                 └─ SDL3 audio out; report section per channel
```

The extension never builds the piano roll itself (that needs running Rust); it
streams the buffer text to the server, which compiles and plays it.

## Setup (development install)

This is an unpackaged extension. Install its dependencies (the tree-sitter
parser used to read the source — no regex), then symlink it into your VSCode
extensions dir and reload:

```sh
( cd editor && npm install )
ln -s "$PWD/editor" ~/.vscode/extensions/lotw-music
```

Open the repo as the workspace folder (the server runs `cargo run --bin
music-server --features server` from the workspace root, so it needs the
`Cargo.toml` and the ROM at `lotwMusic.rom`, default `rom/lotw.nes`).

## Settings

- `lotwMusic.rom` — ROM path (relative to the workspace), default `rom/lotw.nes`.
- `lotwMusic.debounceMs` — delay after you stop typing before reloading, default `300`.

## Limitations (v1)

- Highlighting is **section-granular** (the `section(...)` block playing), not
  per-note — reliable across `env!` expansion. Per-note needs compile-time span
  instrumentation.
- The edited song's channel streams are patched over the ROM song's slot, so a
  song that grows much longer than the original may not fit yet.
- Position is preserved by re-ticking to the same tick on reload; a structural
  diff to remap across big edits is future work.
