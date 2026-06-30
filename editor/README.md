# LotW Music — VSCode extension

Live-edit playback for Legacy of the Wizard songs written in the `lotw_music`
DSL. Above each `pub fn name() -> Song` (and each `section(...)`) it shows
CodeLens transport buttons:

- **▶ Play / ⏸ Pause** — a toggle; plays the song/section through the real ported
  sound engine and highlights the **note currently playing in each channel**.
- **⏹ Stop** — stop and return to the start.
- **🔁 Loop on/off** — toggle looping.

While a song plays, your edits are debounced, recompiled (the `music-server`'s
JIT recompiles the `music-jit` cdylib in ~130–200 ms), and reloaded **in place** —
playback keeps its position.

Each sound-effect function (`fn … -> Vec<Tok>`) gets the **same transport**
(Play/Pause, Stop, Loop) and per-note highlighting as songs — it's played as a
one-channel (pulse2) song, so the whole SFX library is auditionable and editable
the same way.

**Type-to-play**: as you type a note (`c4e`, `hite`, …), it's played immediately
on a separate preview voice so you hear what you're writing — even without
pressing Play, and without disturbing any ongoing playback.

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

## Limitations

- The edited song's channel streams are patched over the ROM song's slot, so a
  song that grows much longer than the original may not fit yet.
- Position is preserved by re-ticking to the same tick on reload; a structural
  diff to remap across big edits is future work.
- The type-to-play preview uses a fixed default timbre (duty/volume), not the
  channel's surrounding commands.
