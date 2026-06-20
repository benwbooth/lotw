# Native Port Plan

## Target

*Legacy of the Wizard* (USA), mapper 4 (MMC3), 128 KiB PRG + 64 KiB CHR.
The ROM is a gitignored input artifact pinned by sha256. The checked-in project
is a native C/C++ playable port with tests and replay-based visual/audio checks.

## Current Direction

- Keep gameplay logic native: ported C systems, C++20 coroutine scripts for
  frame-spanning routines, and no checked-in ROM-derived source listings.
- Treat fast-host-CPU failures as game bugs. Do not hide them by sprinkling
  scheduler yields through ported C.
- Treat vblank as a frame boundary. Code that waits across frames should be a
  coroutine script or one of the temporary explicit legacy frame waits called
  out by the scheduler contract.
- Keep the ROM/reference emulator path as an oracle for replay captures only;
  runtime behavior in `play` and host tests must come from the native port.

## Verification

- `ctest` runs focused host tests plus `scheduler_contract`, which rejects old
  scheduler APIs, fake CPU-cycle advancement, checked-in ROM-derived source
  listings, ported spin-loop regressions, raw native frame/input state access,
  and new untracked source files that bypass the intended helpers.
- `tools/re/replay_regression.py` compares rendered frames, RAM assertions, and
  optional APU register streams against captured references.
- `./build/play rom/lotw.nes <frames> auto` is the smoke path for the playable
  SDL frontend in dummy video/audio mode.

## Near-Term Work

- Move remaining frame-blocking ported routines into `src/native/` coroutine
  scripts.
- Convert the remaining native C-ABI compatibility wrappers into coroutine-owned
  scripts/state machines instead of RAM/Regs-heavy adapters.
- Broaden replay assertions for outdoor rendering, sprite Y placement, item
  pickup, inventory state, and inventory music.
- Keep docs and build metadata aligned with the native port only.
