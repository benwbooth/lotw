# Legacy of the Wizard — Native Port

A C/C++ playable port of *Legacy of the Wizard* (USA). The runtime is native
code: game C systems, a software PPU/APU shim, SDL playback, and C++20
coroutines for frame-spanning gameplay. The source ROM is a runtime input only.

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

Pinned in `config/rom.sha256`. ROMs are never committed.

## Layout

- `rom/`        — the source ROM (gitignored, sha-pinned in `config/`).
- `src/`        — native C/C++ runtime: game systems, shims, and coroutines.
- `tools/verify/` — repository contract checks.
- `fixtures/`    — replay fixtures for smoke and gameplay checks.

## Toolchain

`nix develop` provides: `gcc`/`make`/`cmake`, `SDL3`, `cargo`/`rustc`, and
`python3`.

## Verification loops

- `nix develop --command cmake -S . -B build`
- `nix develop --command cmake --build build -j`
- `nix develop --command ctest --test-dir build --output-on-failure`
- `env SDL_VIDEODRIVER=dummy SDL_AUDIODRIVER=dummy ./build/play rom/lotw.nes 240 auto`

`ctest` runs `tools/verify/check_port_contract.py`, which rejects stale runtime
terms, generated source listings, direct frame waits from game routines, and
native code that bypasses the coroutine frame helpers.
