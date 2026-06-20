# Native Source Map

The checked-in project is now organized around the native port, not generated
intermediate sources.

## Runtime

- `src/ported/`: ported C systems that still expose the original C ABI.
- `src/native/`: C++ coroutine scripts and native replacements for blocking
  frame/input routines.
- `src/ppu.c`, `src/apu.c`: host-side PPU/APU shims used by tests and the SDL
  frontend.
- `test/play_sdl.cc`: playable SDL frontend.

## Verification

- `test/*_test.c` and `test/native_coroutine_test.cc`: focused host tests.
- `tools/re/check_scheduler_contract.py`: guardrails for scheduler/runtime
  structure.
- `tools/re/replay_regression.py` plus `tools/fceux_capture.lua`: replay-driven
  visual/audio reference checks.
- `tools/re/lockstep.py` and `tools/re/lockstep_fceux.lua`: RAM trace comparison
  while the legacy host drivers are being retired.

## Migration Status

Frame-spanning gameplay should move from `src/ported/` into `src/native/`.
The scheduler contract keeps the allowlist small so each migration removes one
more blocking frame wait or input spin from the active runtime.
