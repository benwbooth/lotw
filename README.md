# Legacy of the Wizard — Decompilation

A from-scratch reverse-engineering effort on the NES game *Legacy of the Wizard*
(USA), in two stages:

1. **Matching disassembly** — a complete, labeled, commented **6502 assembly**
   source (ca65/ld65) that re-assembles to a **byte-identical ROM**. This is
   the ground truth: every byte is classified as code or data and accounted for.
2. **Readable C port** — game systems hand-rewritten in **C**, verified by
   *differential testing* against a reference emulator (same inputs ⇒ same
   RAM/PPU/APU state), plus assets extracted to open formats: tiles/rooms →
   **PNG**, music/SFX → **MIDI** and a **PLAY-like DSL**.

This is *attempt 2*. The earlier proof-ledger approach is archived at git tag
`attempt-1` (branch `archive/attempt-1`).

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
- `disasm/`     — Stage 1 matching 6502 disassembly (ca65 `.s` + `ld65` config).
- `src/`        — Stage 2 C port.
- `tools/re/`   — RE tooling (disassembler, tracer, asset extractors).
- `tools/*.lua` — FCEUX scripts for replay-driven reference capture.
- `assets/`     — extracted assets (PNG / MIDI / DSL).
- `fixtures/`   — replay fixtures (gameplay coverage for tracing & diff tests).
- `docs/`       — the growing knowledge base: bank map, memory map, system docs.

## Toolchain

`nix develop` provides: `cc65` (ca65/ld65/da65), `fceux`, `gcc`/`make`/`cmake`,
`SDL2`, `cargo`/`rustc`, `python3`.

## Verification loops

- **Stage 1:** `disasm/` assembles → output sha256 must equal the ROM's.
- **Stage 2:** each C system runs against the emulator on the replay fixtures;
  RAM/PPU/APU state must match frame-for-frame.
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
- **Scheduler/NMI contract:** translated gameplay code must not add ad hoc
  frame yields to hide fast-host-CPU hangs. Explicit NMI waits call
  `nes_frame_wait()`, and tight controller polling is interrupted through the
  central CPU-cycle checkpoint in `read_controllers()`. `ctest` runs
  `tools/re/check_scheduler_contract.py` to reject direct `nes_vblank_wait()`
  calls or manual input-yield calls in `src/ported/`.

See `docs/PLAN.md` for the full strategy and phase breakdown.
