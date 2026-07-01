"""Policy-guided action proposals for the composer's sample() search.

Wraps a trained PPO checkpoint (train_ppo.Agent) so a Composer rollout can draw
actions from the policy's distribution instead of uniform random. The policy
chains coherent navigation (ladder climbs, room crossings) that uniform random
almost never strings together, which massively densifies goal-hitting rollouts.
Epsilon-mixing keeps tail exploration for the moves the policy never learned.
"""

from __future__ import annotations

import collections

import numpy as np
import torch

from lotw_gym import ACTIONS
from train_ppo import Agent


class PolicyProposer:
    """Callable: observe the composer's env, return a controller byte."""

    def __init__(self, checkpoint: str, epsilon: float = 0.15,
                 temperature: float = 1.0, seed: int = 0):
        ck = torch.load(checkpoint, map_location="cpu", weights_only=False)
        self.agent = Agent(len(ACTIONS))
        self.agent.load_state_dict(ck["model"])
        self.agent.eval()
        self.epsilon = epsilon
        self.temperature = temperature
        self.rng = np.random.default_rng(seed)
        self.frames: collections.deque = collections.deque(maxlen=4)

    def reset(self):
        self.frames.clear()

    def _obs(self, composer) -> np.ndarray:
        # RGB frame -> grayscale -> 84x84, matching the training pipeline.
        raw = np.frombuffer(composer.env.render(), np.uint8).reshape(240, 256, 3)
        gray = raw.mean(axis=2).astype(np.float32)
        # cheap 84x84 resize via strided sampling (bilinear not needed for proposals)
        ys = np.linspace(0, 239, 84).astype(int)
        xs = np.linspace(0, 255, 84).astype(int)
        return gray[np.ix_(ys, xs)]

    def __call__(self, composer) -> int:
        self.frames.append(self._obs(composer))
        while len(self.frames) < 4:
            self.frames.append(self.frames[-1])
        if self.rng.random() < self.epsilon:
            return ACTIONS[int(self.rng.integers(0, len(ACTIONS)))]
        x = torch.tensor(np.stack(self.frames), dtype=torch.float32).unsqueeze(0)
        with torch.no_grad():
            logits = self.agent.actor(self.agent.features(x)) / self.temperature
        a = torch.distributions.Categorical(logits=logits).sample().item()
        return ACTIONS[a]
