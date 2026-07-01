"""CleanRL-style PPO for LotwEnv — a compact, self-contained baseline trainer.

Observation pipeline: RGB frame -> grayscale -> 84x84 -> stack 4 (the Atari recipe).
Policy: NatureCNN actor-critic. Device-agnostic: uses cuda (== ROCm on AMD) if
available, else CPU. This is the "does it learn" loop — verify it runs and the mean
episodic return trends up on a short run here (CPU), then run it long on the GPU
from the rocm/pytorch container.

Run (in the uv venv, extension built release for speed):
    uv sync --group train
    maturin develop --release --manifest-path lotw-env/Cargo.toml
    python agent/train_ppo.py --total-timesteps 25000
"""

from __future__ import annotations

import argparse
import os
import sys
import time

import gymnasium as gym
import numpy as np
import torch
import torch.nn as nn
from torch.distributions import Categorical

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lotw_gym import LotwEnv, load_replay  # noqa: E402


def make_env(checkpoint, max_steps):
    def thunk():
        env = LotwEnv(checkpoint=checkpoint, frame_skip=4, max_steps=max_steps)
        env = gym.wrappers.GrayscaleObservation(env, keep_dim=False)
        env = gym.wrappers.ResizeObservation(env, (84, 84))
        env = gym.wrappers.FrameStackObservation(env, 4)
        return env

    return thunk


def layer_init(layer, std=np.sqrt(2), bias=0.0):
    nn.init.orthogonal_(layer.weight, std)
    nn.init.constant_(layer.bias, bias)
    return layer


class Agent(nn.Module):
    def __init__(self, n_actions):
        super().__init__()
        self.net = nn.Sequential(
            layer_init(nn.Conv2d(4, 32, 8, stride=4)), nn.ReLU(),
            layer_init(nn.Conv2d(32, 64, 4, stride=2)), nn.ReLU(),
            layer_init(nn.Conv2d(64, 64, 3, stride=1)), nn.ReLU(),
            nn.Flatten(),
            layer_init(nn.Linear(64 * 7 * 7, 512)), nn.ReLU(),
        )
        self.actor = layer_init(nn.Linear(512, n_actions), std=0.01)
        self.critic = layer_init(nn.Linear(512, 1), std=1.0)

    def features(self, x):
        return self.net(x / 255.0)

    def get_value(self, x):
        return self.critic(self.features(x))

    def get_action_and_value(self, x, action=None):
        h = self.features(x)
        logits = self.actor(h)
        dist = Categorical(logits=logits)
        if action is None:
            action = dist.sample()
        return action, dist.log_prob(action), dist.entropy(), self.critic(h)


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--total-timesteps", type=int, default=25000)
    p.add_argument("--num-envs", type=int, default=8)
    p.add_argument("--num-steps", type=int, default=128)
    p.add_argument("--max-steps", type=int, default=256)
    p.add_argument("--lr", type=float, default=2.5e-4)
    p.add_argument("--gamma", type=float, default=0.99)
    p.add_argument("--gae-lambda", type=float, default=0.95)
    p.add_argument("--minibatches", type=int, default=4)
    p.add_argument("--update-epochs", type=int, default=4)
    p.add_argument("--clip-coef", type=float, default=0.1)
    p.add_argument("--ent-coef", type=float, default=0.01)
    p.add_argument("--vf-coef", type=float, default=0.5)
    p.add_argument("--checkpoint", default="fixtures/reference/outside_walk.replay")
    args = p.parse_args()

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    # On a many-core box, torch's default (one intra-op thread per core) is a huge
    # oversubscription penalty for these small batches — cap it. Likewise cv2.
    torch.set_num_threads(int(os.environ.get("TORCH_THREADS", "8")))
    try:
        import cv2

        cv2.setNumThreads(1)
    except Exception:
        pass
    torch.manual_seed(0)
    np.random.seed(0)
    print(f"device={device}  torch={torch.__version__}  threads={torch.get_num_threads()}")

    cp = load_replay(args.checkpoint)
    envs = gym.vector.SyncVectorEnv([make_env(cp, args.max_steps) for _ in range(args.num_envs)])
    envs = gym.wrappers.vector.RecordEpisodeStatistics(envs)
    n_actions = int(envs.single_action_space.n)

    agent = Agent(n_actions).to(device)
    opt = torch.optim.Adam(agent.parameters(), lr=args.lr, eps=1e-5)

    batch = args.num_envs * args.num_steps
    mb_size = batch // args.minibatches
    obs_shape = envs.single_observation_space.shape

    obs = torch.zeros((args.num_steps, args.num_envs, *obs_shape), device=device)
    actions = torch.zeros((args.num_steps, args.num_envs), device=device, dtype=torch.long)
    logprobs = torch.zeros((args.num_steps, args.num_envs), device=device)
    rewards = torch.zeros((args.num_steps, args.num_envs), device=device)
    dones = torch.zeros((args.num_steps, args.num_envs), device=device)
    values = torch.zeros((args.num_steps, args.num_envs), device=device)

    global_step = 0
    start = time.time()
    next_obs, _ = envs.reset(seed=0)
    next_obs = torch.tensor(np.asarray(next_obs), device=device, dtype=torch.float32)
    next_done = torch.zeros(args.num_envs, device=device)
    returns_hist: list[float] = []

    num_updates = args.total_timesteps // batch
    for update in range(1, num_updates + 1):
        for step in range(args.num_steps):
            global_step += args.num_envs
            obs[step] = next_obs
            dones[step] = next_done
            with torch.no_grad():
                action, logprob, _, value = agent.get_action_and_value(next_obs)
            actions[step], logprobs[step], values[step] = action, logprob, value.flatten()

            no, r, term, trunc, infos = envs.step(action.cpu().numpy())
            next_done = torch.tensor(np.logical_or(term, trunc), device=device, dtype=torch.float32)
            rewards[step] = torch.tensor(r, device=device, dtype=torch.float32)
            next_obs = torch.tensor(np.asarray(no), device=device, dtype=torch.float32)
            if "episode" in infos:
                m = infos.get("_episode", np.ones_like(infos["episode"]["r"], dtype=bool))
                returns_hist.extend(float(x) for x in np.asarray(infos["episode"]["r"])[m])

        # GAE
        with torch.no_grad():
            next_value = agent.get_value(next_obs).flatten()
            advantages = torch.zeros_like(rewards)
            lastgae = 0
            for t in reversed(range(args.num_steps)):
                nextnonterminal = 1.0 - (next_done if t == args.num_steps - 1 else dones[t + 1])
                nextvalues = next_value if t == args.num_steps - 1 else values[t + 1]
                delta = rewards[t] + args.gamma * nextvalues * nextnonterminal - values[t]
                advantages[t] = lastgae = delta + args.gamma * args.gae_lambda * nextnonterminal * lastgae
            returns = advantages + values

        b_obs = obs.reshape((-1, *obs_shape))
        b_actions = actions.reshape(-1)
        b_logprobs = logprobs.reshape(-1)
        b_adv = advantages.reshape(-1)
        b_ret = returns.reshape(-1)
        b_val = values.reshape(-1)

        idx = np.arange(batch)
        for _ in range(args.update_epochs):
            np.random.shuffle(idx)
            for s in range(0, batch, mb_size):
                mb = idx[s : s + mb_size]
                _, newlp, ent, newval = agent.get_action_and_value(b_obs[mb], b_actions[mb])
                ratio = (newlp - b_logprobs[mb]).exp()
                adv = b_adv[mb]
                adv = (adv - adv.mean()) / (adv.std() + 1e-8)
                pg = torch.max(-adv * ratio, -adv * torch.clamp(ratio, 1 - args.clip_coef, 1 + args.clip_coef)).mean()
                v_loss = 0.5 * ((newval.flatten() - b_ret[mb]) ** 2).mean()
                loss = pg - args.ent_coef * ent.mean() + args.vf_coef * v_loss
                opt.zero_grad()
                loss.backward()
                nn.utils.clip_grad_norm_(agent.parameters(), 0.5)
                opt.step()

        sps = int(global_step / (time.time() - start))
        recent = returns_hist[-20:]
        mean_ret = np.mean(recent) if recent else float("nan")
        print(f"update {update}/{num_updates}  step {global_step}  SPS {sps}  "
              f"mean_return({len(recent)}ep) {mean_ret:.1f}  v_loss {v_loss.item():.2f}")

    envs.close()
    if len(returns_hist) >= 10:
        first = np.mean(returns_hist[: len(returns_hist) // 3])
        last = np.mean(returns_hist[-len(returns_hist) // 3 :])
        print(f"\nreturn trend: first-third {first:.1f} -> last-third {last:.1f}  "
              f"({'up' if last > first else 'flat/down'})")


if __name__ == "__main__":
    main()
