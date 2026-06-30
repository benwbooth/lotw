# lotw-env — Python bindings for the agent environment

PyO3/maturin wrapper over `lotw::env::Env`: drive the real game one frame at a time
from Python, observe RGB frames, read privileged RAM/state, and load deterministic
replay-checkpoint save-states.

## Build / install

Deps are managed by **uv** from the repo-root `pyproject.toml`; direnv auto-runs
`uv sync` and activates `.venv` on entry (see `.envrc`). Manually:

```sh
nix develop          # rust toolchain, python, uv, maturin
uv sync              # builds .venv: numpy + gymnasium + this extension (editable)
```

After changing the Rust here, rebuild the extension:
`uv sync --reinstall-package lotw-env` (or `maturin develop` inside the venv).
For a fast env (RL throughput) build release: `maturin develop --release`.

## Use

```python
import numpy as np, lotw_env

env = lotw_env.Lotw("rom/lotw.nes")            # boot

# A checkpoint is an input prefix (one hardware controller byte per frame):
#   bit0=A bit1=B bit2=Select bit3=Start bit4=Up bit5=Down bit6=Left bit7=Right
# (constants lotw_env.A/B/SELECT/START/UP/DOWN/LEFT/RIGHT). reset_replay reboots
# and fast-forwards without rendering — the cheap "load save-state k".
env.reset_replay(bytes([0])*300 + bytes([lotw_env.START])*12 + bytes([0])*108)

frame, ram, done = env.step(lotw_env.RIGHT)     # frame/ram are bytes
img = np.frombuffer(frame, np.uint8).reshape(lotw_env.FRAME_H, lotw_env.FRAME_W, 3)

# Policy observes ONLY `img` (pixels). ram()/state() are the privileged training
# signal (reward, success checks) — keep them out of the observation.
print(env.state())   # {'player_x_tile':..., 'map_screen_x':..., ...}
```

API: `reset()`, `reset_replay(inputs: bytes)`, `advance(action) -> done` (no render),
`step(action) -> (frame_bytes, ram_bytes, done)`, `render() -> bytes`, `ram() -> bytes`,
`state() -> dict`, properties `frame_count` / `done`. Module consts `FRAME_W/H/BYTES`
and the button bits.

Notes: the `Lotw` class is `unsendable` (one env per thread); parallel RL should use
separate **processes** (e.g. Gymnasium `SubprocVecEnv`), each with its own env. Torch
(ROCm) runs from the `rocm/pytorch` container, not this venv.
