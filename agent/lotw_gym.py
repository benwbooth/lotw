"""Gymnasium environment wrapping the `lotw_env` PyO3 extension.

- **Observation**: the RGB frame `(H, W, 3) uint8` — the agent's ONLY input.
- **Action**: `Discrete` over a curated set of NES button combos (`ACTIONS`).
- **Reward**: a pluggable `reward_fn(state, prev_state, ram) -> float`; default is
  total motion (a robust "did it move" signal — good for the first objective).
- **Checkpoint**: an input prefix (a deterministic save-state). `reset()` reboots
  and fast-forwards there (cheap), so episodes start mid-game where the character
  is actually controllable.

The policy sees only the frame; `state`/`ram` (privileged) are for the reward only.
For RL throughput build the extension in release (`maturin develop --release`).
"""

from __future__ import annotations

import gymnasium as gym
import numpy as np
from gymnasium import spaces

import lotw_env

# Curated discrete action set (hardware controller bytes). Small + meaningful keeps
# exploration tractable vs the full 256-value byte space.
ACTIONS: list[int] = [
    0,                              # 0  NOOP
    lotw_env.RIGHT,                 # 1  walk right
    lotw_env.LEFT,                  # 2  walk left
    lotw_env.RIGHT | lotw_env.A,    # 3  run-jump right
    lotw_env.LEFT | lotw_env.A,     # 4  run-jump left
    lotw_env.A,                     # 5  jump
    lotw_env.B,                     # 6  attack (magic)
    lotw_env.UP,                    # 7  up (ladders / doors / portraits)
    lotw_env.DOWN,                  # 8  down (ladders)
    lotw_env.RIGHT | lotw_env.B,    # 9  walk right + attack
]


def load_replay(path: str) -> bytes:
    """Expand a `frame <count> <buttons...>` replay fixture into one byte/frame."""
    bit = {"A": 1, "B": 2, "select": 4, "start": 8, "up": 16, "down": 32, "left": 64, "right": 128}
    out = bytearray()
    for line in open(path):
        toks = line.split("#", 1)[0].split()
        if len(toks) < 2 or toks[0] != "frame":
            continue
        b = 0
        for name in toks[2:]:
            b |= bit.get(name, 0)
        out += bytes([b]) * int(toks[1])
    return bytes(out)


def motion_reward(state: dict, prev: dict, ram: bytes) -> float:
    """Total movement this step (room/scroll/tile/y). Robust default objective."""
    return float(
        abs(state["map_screen_x"] - prev["map_screen_x"]) * 256
        + abs(state["scroll_pixel_x"] - prev["scroll_pixel_x"]) * 4
        + abs(state["player_x_tile"] - prev["player_x_tile"])
        + abs(state["player_y"] - prev["player_y"])
    )


class LotwEnv(gym.Env):
    metadata = {"render_modes": ["rgb_array"], "render_fps": 60}

    def __init__(
        self,
        rom: str = "rom/lotw.nes",
        checkpoint: bytes = b"",
        reward_fn=motion_reward,
        frame_skip: int = 4,
        max_steps: int = 1000,
        render_mode: str | None = "rgb_array",
    ):
        super().__init__()
        self._env = lotw_env.Lotw(rom)
        self.checkpoint = checkpoint
        self.reward_fn = reward_fn
        self.frame_skip = frame_skip
        self.max_steps = max_steps
        self.render_mode = render_mode
        self.action_space = spaces.Discrete(len(ACTIONS))
        self.observation_space = spaces.Box(0, 255, (lotw_env.FRAME_H, lotw_env.FRAME_W, 3), np.uint8)
        self._steps = 0
        self._prev: dict = {}

    def _obs(self) -> np.ndarray:
        return np.frombuffer(self._env.render(), np.uint8).reshape(
            lotw_env.FRAME_H, lotw_env.FRAME_W, 3
        ).copy()

    def reset(self, *, seed=None, options=None):
        super().reset(seed=seed)
        cp = (options or {}).get("checkpoint", self.checkpoint)
        self._env.reset_replay(cp)
        self._steps = 0
        self._prev = self._env.state()
        return self._obs(), {"state": self._prev}

    def step(self, action):
        a = ACTIONS[int(action)]
        done = False
        for _ in range(self.frame_skip):  # act every `frame_skip` frames
            done = self._env.advance(a)
            if done:
                break
        state = self._env.state()
        reward = self.reward_fn(state, self._prev, self._env.ram())
        self._prev = state
        self._steps += 1
        truncated = self._steps >= self.max_steps
        return self._obs(), reward, bool(done), truncated, {"state": state}

    def render(self):
        if self.render_mode == "rgb_array":
            return self._obs()
        return None
