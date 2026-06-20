# Porting And Coroutine Migration

The active port is native C/C++. New work should make behavior clearer and more
testable, not add compatibility shims around blocking port code.

## Migrating A Frame-Spanning Routine

1. Find the blocking wait or input loop in `src/ported/`.
2. Move the routine to `src/native/<name>.cc` behind the same `extern "C"` ABI
   when callers still expect that symbol.
3. Express frame boundaries with `FrameTask`/`co_yield` when the routine is
   naturally a script. Compatibility wrappers should use
   `src/native/frame_wait_helpers.hpp`; direct raw frame-hook calls are limited
   to that helper and the PPU hook. Native frame/input/prompt state should go
   through `GameState`, not raw `RAM8()` bytes.
4. Preserve register and RAM side effects at the C ABI boundary. If callers read
   `Regs::a` or a RAM scratch byte after the call, set it explicitly.
5. Add or update a focused test when the behavior is observable: item pickup,
   rendered sprites, inventory state, music/APU writes, or replay assertions.

## Rules

- Do not add fake CPU-cycle advancement.
- Do not add new raw vblank waits in ported C.
- Do not add direct raw frame-hook calls outside the native frame-wait helper.
- Do not add direct controller polling or raw frame/input/prompt state access in
  `src/native/`; use `frame_wait_helpers.hpp` and `GameState`.
- Do not add direct `sub_C135()` frame-commit calls in `src/native/`; only the
  compatibility wrapper implementation owns that ABI. Move frame-spanning loops
  to coroutine scripts with explicit `co_yield` waits.
- Do not add `sub_C135()` frame-commit calls in `src/ported/`; the ported
  allowlist is empty and must stay empty.
- Do not add checked-in ROM-derived source listings or generated code artifacts.
- Do not paper over host-speed bugs with ad hoc yields. Fix the routine so its
  state machine has an explicit frame/input boundary.
- Prefer named RAM helpers in `src/ram.h` when a byte's role is understood.

## Verification

```sh
python3 tools/re/check_scheduler_contract.py
nix develop --command cmake --build build -j
nix develop --command ctest --test-dir build --output-on-failure
env SDL_VIDEODRIVER=dummy SDL_AUDIODRIVER=dummy ./build/play rom/lotw.nes 240 auto
```
