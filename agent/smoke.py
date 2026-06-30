"""Validate the Gymnasium env: run the official env checker, then a random rollout
reporting reward — confirms the obs/action spaces, the reset/step API, and that the
reward signal moves.

Run from the repo root (in the uv venv): `python agent/smoke.py`
"""

import os
import sys

import numpy as np
from gymnasium.utils.env_checker import check_env

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lotw_gym import LotwEnv, load_replay  # noqa: E402

# Start from a checkpoint where the character is actually controllable.
cp = load_replay("fixtures/reference/outside_walk.replay")

# check_env makes its own env interactions, so give it a fresh one.
check_env(LotwEnv(checkpoint=cp, frame_skip=4, max_steps=50), skip_render_check=False)
print("check_env: OK (spaces, reset/step API, render all conform)")

env = LotwEnv(checkpoint=cp, frame_skip=4, max_steps=200)
obs, info = env.reset()
lit = int(obs.reshape(-1, 3).any(1).sum())
print(f"reset: obs {obs.shape} {obs.dtype}, lit_px {lit}, start {info['state']}")

rng = np.random.default_rng(0)
total = 0.0
t = 0
for t in range(200):
    obs, r, term, trunc, info = env.step(int(rng.integers(env.action_space.n)))
    total += r
    if term or trunc:
        break
print(f"random rollout: {t + 1} steps, total_reward={total:.0f}, end {info['state']}")
assert total > 0, "no reward accrued — env/reward wiring is wrong"
print("OK")
