# Legacy of the Wizard — Native Port

A C/C++ playable port of *Legacy of the Wizard* (USA). The runtime is native
code: ported C systems, a software PPU/APU shim, SDL playback, and a growing
C++20 coroutine layer for frame-spanning gameplay. The source ROM is used as an
input artifact only; ROM-derived source listings are not part of the active
codebase.

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
- `src/`        — native C/C++ runtime: ported systems, shims, and coroutines.
- `tools/re/`   — analysis, tracer, replay, and asset-extraction tooling.
- `tools/*.lua` — FCEUX scripts for replay-driven reference capture.
- `assets/`     — extracted assets (PNG / MIDI / DSL).
- `fixtures/`   — replay fixtures (gameplay coverage for tracing & diff tests).
- `docs/`       — the growing knowledge base: bank map, memory map, system docs.

## Toolchain

`nix develop` provides: `fceux`, `gcc`/`make`/`cmake`, `SDL3`,
`cargo`/`rustc`, `python3`.

## Verification loops

- **Playable-port regression:** `tools/re/replay_regression.py` runs replay
  fixtures through the C port and compares rendered frames against FCEUX
  captures. Use this for user-visible checks that RAM lockstep cannot prove:
  palettes/scrolling/sprite position, APU register streams, and explicit RAM
  outcome assertions such as item pickup/inventory changes.
  Example:
  `nix develop --command python3 tools/re/replay_regression.py outside_walk`
  Use `--refresh-reference` to regenerate FCEUX captures from the ROM instead
  of reusing local `build/reference` artifacts, and `--check-apu` to compare APU
  register writes for music/SFX regressions.
- **Ported compatibility scheduler:** ported gameplay code must not add
  ad hoc frame yields to hide fast-host-CPU hangs. Frame-spanning gameplay must
  move to `src/native/` coroutine/native scripts instead of expanding the
  compatibility scheduler.
  `ctest` runs `tools/re/check_scheduler_contract.py` to reject direct
  frame waits, `nes_vblank_wait()`, fake CPU-cycle advancement, old `nmi_*`
  entry points, manual input-yield calls, or checked-in ROM-derived source
  listings. The check walks the active source tree, so newly added native files
  are covered before they are staged.
- **Native C++ coroutine layer:** `src/native/` is the migration path away from
  blocking frame/input loops. C++20 `FrameTask` scripts use `co_yield
  Wait::...` for explicit frame/input waits, while ported C routines remain
  available behind a C ABI during the transition. Raw frame waits are
  centralized in `src/native/frame_wait_helpers.hpp`; frame/input/prompt state
  goes through `src/native/game_state.*`.
- **Frame runner:** the SDL, replay, and lockstep hosts use the shared native
  frame runner to park the game thread at `nes_frame_wait()` and let the host
  perform `vblank_commit()` before resuming it. Legacy stackful context
  switching is forbidden by the scheduler contract.

See `docs/PLAN.md` for the full strategy and phase breakdown.
