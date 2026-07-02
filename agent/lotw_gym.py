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


def screen_of(state: dict) -> tuple:
    """The labyrinth screen (room) the player is on."""
    return (state["map_screen_x"], state["map_screen_y"])


def cell_of(state: dict) -> tuple:
    """A position cell: screen + quantized within-screen (2-tile column, 16px row).
    New cells = genuinely new ground, so it can't be farmed by jittering, but the
    grid is fine enough (~8 cols × ~16 rows per screen) to give a dense, climbable
    "cover the room / find the exit" signal."""
    return (
        state["map_screen_x"],
        state["map_screen_y"],
        state["player_x_tile"] >> 1,
        state["player_y"] >> 4,
    )


class LotwEnv(gym.Env):
    metadata = {"render_modes": ["rgb_array"], "render_fps": 60}

    def __init__(
        self,
        rom: str = "rom/lotw.nes",
        checkpoint: bytes = b"",
        reward_fn=motion_reward,
        reward_mode: str = "motion",
        frame_skip: int = 4,
        max_steps: int = 1000,
        render_mode: str | None = "rgb_array",
    ):
        super().__init__()
        self._env = lotw_env.Lotw(rom)
        self.checkpoint = checkpoint
        self.reward_fn = reward_fn
        # "motion": stateless reward_fn (a movement smoke signal).
        # "explore": directed-exploration — reward reaching new ground (new
        #   labyrinth screens, and new coarse position cells within a screen). This
        #   is the first real objective: it maps directly onto route progress
        #   (P0 = "traverse right/down through the labyrinth") and can't be farmed
        #   by standing still or jittering the way total-motion can.
        self.reward_mode = reward_mode
        self.frame_skip = frame_skip
        self.max_steps = max_steps
        self.render_mode = render_mode
        self.action_space = spaces.Discrete(len(ACTIONS))
        self.observation_space = spaces.Box(0, 255, (lotw_env.FRAME_H, lotw_env.FRAME_W, 3), np.uint8)
        self._steps = 0
        self._prev: dict = {}
        self._seen_screens: set = set()
        self._seen_cells: set = set()

    def _obs(self) -> np.ndarray:
        return np.frombuffer(self._env.render(), np.uint8).reshape(
            lotw_env.FRAME_H, lotw_env.FRAME_W, 3
        ).copy()

    def _pos(self, st) -> tuple:
        """World position in pixels (room-relative x, y)."""
        return (st["player_x_tile"] * 16 + st["player_x_fine"], st["player_y"])

    def reset(self, *, seed=None, options=None):
        super().reset(seed=seed)
        cp = (options or {}).get("checkpoint", self.checkpoint)
        self._env.reset_replay(cp)
        self._steps = 0
        self._prev = self._env.state()
        # Seed coverage with the start location so it earns no reward.
        self._seen_screens = {screen_of(self._prev)}
        self._seen_cells = {cell_of(self._prev)}
        if self.reward_mode == "goto":
            # Scout a goal that is reachable BY CONSTRUCTION: run a random
            # rollout (movement-biased, actions held in bursts so it actually
            # travels), record where it ends, deterministically reset back.
            # Resample until the goal is a real journey (>= 48 px away).
            for _attempt in range(6):
                n = int(self.np_random.integers(40, 200))
                a = 0
                for i in range(n):
                    if i % 6 == 0:  # hold each choice ~6 steps: coherent travel
                        a = ACTIONS[int(self.np_random.integers(1, len(ACTIONS)))]
                    for _ in range(self.frame_skip):
                        self._env.advance(a)
                    st = self._env.state()
                    if st["character_index"] > 4 or st["map_screen_y"] == 16:
                        break
                gs = self._env.state()
                ok = gs["character_index"] <= 4 and gs["map_screen_y"] != 16
                self.goal = (self._pos(gs), (gs["map_screen_x"], gs["map_screen_y"]))
                self._env.reset_replay(cp)
                self._prev = self._env.state()
                self._gdist = self._goal_dist(self._prev)
                if ok and self._gdist >= 48:
                    break
            return self._obs(), {"state": self._prev,
                                 "goal_delta": self.goal_delta(self._prev)}
        return self._obs(), {"state": self._prev}

    def _goal_dist(self, st) -> float:
        (gx, gy), (grx, gry) = self.goal
        px, py = self._pos(st)
        # rooms are 1024x192 px; add room offsets for a global distance
        dx = (st["map_screen_x"] - grx) * 1024 + (px - gx)
        dy = (st["map_screen_y"] - gry) * 192 + (py - gy)
        return abs(dx) + abs(dy)

    def step(self, action):
        a = ACTIONS[int(action)]
        done = False
        for _ in range(self.frame_skip):  # act every `frame_skip` frames
            done = self._env.advance(a)
            if done:
                break
        state = self._env.state()
        self._steps += 1
        # character_index 0..4 = a playable family member; anything else means the
        # character died / returned to the title-select screen. Treat that as
        # terminal — correct RL semantics, and it keeps the agent out of the
        # character-select screen, where holding A on a non-selectable tile spins
        # the game loop with no frame yield (a faithful reproduction of an original
        # freeze that can't be interrupted once entered).
        left_gameplay = state["character_index"] > 4

        # Surfacing check (labyrinth modes): map_screen_y == 16 is the hub row
        # (overworld strip / menus), i.e. the agent LEFT the labyrinth.
        surfaced = state["map_screen_y"] == 16

        if self.reward_mode == "explore":
            screen, cell = screen_of(state), cell_of(state)
            if screen not in self._seen_screens:
                reward = 5.0            # a whole new room reached (the real goal)
                self._seen_screens.add(screen)
            elif cell not in self._seen_cells:
                reward = 0.2            # new ground within a known room
            else:
                reward = -0.005         # mild efficiency pressure; easily overcome
            self._seen_cells.add(cell)  #   by exploring, so noop < move < explore
            if left_gameplay:
                reward = -1.0           # dying is a setback, not progress
        elif self.reward_mode == "goto":
            # Goal-conditioned navigation: dense progress toward the scouted
            # goal, big bonus on arrival. This trains the "go to (x,y)" skill
            # the composer needs as its executor.
            d = self._goal_dist(state)
            reward = (self._gdist - d) / 16.0       # progress, in tiles
            self._gdist = d
            if d < 16:
                reward = 10.0
                done = True                          # goal reached: episode ends
            if left_gameplay or surfaced:
                reward = -2.0
            left_gameplay = left_gameplay or surfaced
        elif self.reward_mode == "explore_lab":
            # Labyrinth-only exploration. The plain "explore" policy discovered a
            # loophole: climb back UP the entry ladder and farm the big, safe
            # overworld strip for cell bonuses. Here only underground rooms
            # (map_screen_y != 16) earn anything, and surfacing ends the episode —
            # the objective is "explore the DUNGEON", so leaving it is done, not
            # a farm.
            screen, cell = screen_of(state), cell_of(state)
            if surfaced:
                reward = -1.0
            elif screen not in self._seen_screens:
                reward = 5.0
                self._seen_screens.add(screen)
            elif cell not in self._seen_cells:
                reward = 0.2
                self._seen_cells.add(cell)
            else:
                reward = -0.005
            self._seen_cells.add(cell)
            if left_gameplay:
                reward = -1.0
            left_gameplay = left_gameplay or surfaced
        else:
            reward = self.reward_fn(state, self._prev, self._env.ram())

        self._prev = state
        terminated = bool(done) or left_gameplay
        truncated = self._steps >= self.max_steps
        info = {"state": state, "screens": len(self._seen_screens), "cells": len(self._seen_cells)}
        if self.reward_mode == "goto":
            info["goal_delta"] = self.goal_delta(state)
        return self._obs(), reward, terminated, truncated, info

    def goal_delta(self, st) -> tuple:
        """Goal direction as (dx, dy) normalized to [-1, 1] (clipped)."""
        (gx, gy), (grx, gry) = self.goal
        px, py = self._pos(st)
        dx = (grx - st["map_screen_x"]) * 1024 + (gx - px)
        dy = (gry - st["map_screen_y"]) * 192 + (gy - py)
        return (max(-1.0, min(1.0, dx / 512)), max(-1.0, min(1.0, dy / 192)))

    def render(self):
        if self.render_mode == "rgb_array":
            return self._obs()
        return None
