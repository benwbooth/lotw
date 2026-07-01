"""Evaluate a trained policy: roll it out and report what it actually DOES in the
game — distinct labyrinth screens reached (real route progress), ground covered,
episode length, death rate — not just the training return. Runs on CPU; loads a
checkpoint saved by train_ppo.py.

    python agent/eval.py --checkpoint-model agent/runs/ppo_gpu.pt --episodes 20
"""

from __future__ import annotations

import argparse
import os
import sys

import gymnasium as gym  # noqa: F401  (import parity with train_ppo wrappers)
import numpy as np
import torch

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lotw_gym import load_replay  # noqa: E402
from train_ppo import Agent, make_env  # noqa: E402


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--checkpoint-model", default="agent/runs/ppo_gpu.pt")
    p.add_argument("--replay", default="fixtures/reference/outside_walk.replay")
    p.add_argument("--episodes", type=int, default=20)
    p.add_argument("--max-steps", type=int, default=1024)
    p.add_argument("--greedy", action="store_true", help="argmax instead of sampling")
    p.add_argument("--random", action="store_true", help="ignore the model; random baseline")
    args = p.parse_args()

    torch.set_num_threads(int(os.environ.get("TORCH_THREADS", "8")))
    cp = load_replay(args.replay)
    env = make_env(cp, args.max_steps, "explore")()
    n_actions = int(env.action_space.n)

    agent = Agent(n_actions)
    if not args.random:
        ck = torch.load(args.checkpoint_model, map_location="cpu", weights_only=False)
        agent.load_state_dict(ck["model"])
        agent.eval()
        print(f"loaded {args.checkpoint_model} (trained to step {ck.get('global_step','?')})")

    start = None
    all_screens: dict = {}
    ep_screens, ep_cells, ep_len = [], [], []
    deaths = 0
    for ep in range(args.episodes):
        obs, info = env.reset(seed=ep)
        start = (info["state"]["map_screen_x"], info["state"]["map_screen_y"])
        screens = set()
        term = trunc = False
        t = 0
        while not (term or trunc):
            if args.random:
                a = np.random.randint(n_actions)
            else:
                o = torch.tensor(np.asarray(obs), dtype=torch.float32).unsqueeze(0)
                with torch.no_grad():
                    logits = agent.actor(agent.features(o))
                a = (logits.argmax(1).item() if args.greedy
                     else torch.distributions.Categorical(logits=logits).sample().item())
            obs, r, term, trunc, info = env.step(a)
            st = info["state"]
            screens.add((st["map_screen_x"], st["map_screen_y"]))
            t += 1
        ep_screens.append(info["screens"])
        ep_cells.append(info["cells"])
        ep_len.append(t)
        if term and not trunc:
            deaths += 1
        for s in screens:
            all_screens[s] = all_screens.get(s, 0) + 1

    mode = "random" if args.random else ("greedy" if args.greedy else "sampled")
    print(f"\n=== eval: {args.episodes} episodes, {mode}, start screen {start} ===")
    print(f"distinct screens / ep : mean {np.mean(ep_screens):.1f}  max {max(ep_screens)}")
    print(f"coarse cells / ep     : mean {np.mean(ep_cells):.1f}  max {max(ep_cells)}")
    print(f"episode length        : mean {np.mean(ep_len):.0f}/{args.max_steps}  death rate {deaths/args.episodes:.0%}")
    left = {s: n for s, n in all_screens.items() if s != start}
    print(f"screens OTHER than start reached ({len(left)} distinct): "
          f"{sorted(left.items(), key=lambda kv: -kv[1])}")


if __name__ == "__main__":
    main()
