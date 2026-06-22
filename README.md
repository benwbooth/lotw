# Legacy of the Wizard — Rust Port

A Rust playable port of *Legacy of the Wizard* (USA). The runtime is native
Rust: game systems, software PPU/APU shims, explicit frame-spanning tasks,
a threaded frame runner for NMI-style interruption points, and SDL playback.
The source ROM is a runtime input only.

## ROM

| | |
|---|---|
| File | `Legacy of the Wizard (USA).nes` (canonical copy at `rom/lotw.nes`, gitignored) |
| Size | 196,624 bytes (16-byte iNES header + 192 KiB) |
| Mapper | 4 (MMC3) |
| PRG-ROM | 128 KiB (8 × 16 KiB; MMC3 swaps 8 KiB windows) |
| CHR-ROM | 64 KiB (4096 tiles; MMC3 swaps 1–2 KiB windows) |
| Mirroring | horizontal · no battery |
| sha256 | `079f648d669966357fe4414a986573eacd7ecadf5c4f289c288427b8c5f491f1` |

ROMs are never committed; verify local copies against the checksum above.

## Layout

- `rom/`      — the source ROM (gitignored).
- `src/`      — Rust runtime, generated game routines, shims, bins, and player.
- `tests/`    — Rust contract and gameplay checks.
- `fixtures/` — replay fixtures for smoke and gameplay checks.
- `docs/`     — reverse-engineering notes and routine catalog.

## Toolchain

`nix develop` provides Rust, Cargo, SDL3, and small ROM/replay utilities.

## Verification loops

- `cargo fmt --check`
- `cargo check --all-targets`
- `cargo check --features sdl --bin play`
- `cargo test`
- `env SDL_VIDEODRIVER=dummy SDL_AUDIODRIVER=dummy cargo run --features sdl --bin play -- rom/lotw.nes 240 auto`

`cargo test` includes a repository contract check that rejects any return of
`CMakeLists.txt` or C/C++ source/header files. This keeps the port Rust-only
instead of allowing the old runtime to creep back in as dormant reference code.
