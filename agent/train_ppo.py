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
import threading
import time

import gymnasium as gym
import numpy as np
import torch
import torch.nn as nn
from torch.distributions import Categorical

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lotw_gym import LotwEnv, load_replay  # noqa: E402


class GoalPlanes(gym.Wrapper):
    """Append 2 constant planes encoding the goal direction (dx, dy in [-1,1],
    scaled to 0..255) to the (4,84,84) framestack -> (6,84,84). This is how the
    goal-conditioned policy SEES its instruction."""

    def __init__(self, env):
        super().__init__(env)
        import numpy as _np
        self.observation_space = gym.spaces.Box(0, 255, (6, 84, 84), _np.uint8)

    def _aug(self, obs, info):
        dx, dy = info["goal_delta"]
        p = np.empty((2, 84, 84), np.uint8)
        p[0] = int((dx + 1) / 2 * 255)
        p[1] = int((dy + 1) / 2 * 255)
        return np.concatenate([np.asarray(obs), p], axis=0)

    def reset(self, **kw):
        obs, info = self.env.reset(**kw)
        return self._aug(obs, info), info

    def step(self, a):
        obs, r, term, trunc, info = self.env.step(a)
        return self._aug(obs, info), r, term, trunc, info


def make_env(checkpoint, max_steps, reward_mode):
    def thunk():
        env = LotwEnv(checkpoint=checkpoint, frame_skip=4, max_steps=max_steps,
                      reward_mode=reward_mode)
        env = gym.wrappers.GrayscaleObservation(env, keep_dim=False)
        env = gym.wrappers.ResizeObservation(env, (84, 84))
        env = gym.wrappers.FrameStackObservation(env, 4)
        if reward_mode == "goto":
            env = GoalPlanes(env)
        return env

    return thunk


def layer_init(layer, std=np.sqrt(2), bias=0.0):
    nn.init.orthogonal_(layer.weight, std)
    nn.init.constant_(layer.bias, bias)
    return layer


class Agent(nn.Module):
    def __init__(self, n_actions, in_ch=4):
        super().__init__()
        self.net = nn.Sequential(
            layer_init(nn.Conv2d(in_ch, 32, 8, stride=4)), nn.ReLU(),
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
    p.add_argument("--reward-mode", default="explore",
                   choices=["explore", "explore_lab", "goto", "motion"])
    p.add_argument("--vec", default="async", choices=["async", "sync"],
                   help="async = one env per subprocess (REQUIRED for >1 env; see below)")
    p.add_argument("--save-path", default="agent/runs/ppo.pt")
    p.add_argument("--save-every", type=int, default=10, help="save every N updates")
    p.add_argument("--stall-timeout", type=int, default=180,
                   help="exit if no update completes in this many seconds (env-hang guard)")
    p.add_argument("--init-from", default=None,
                   help="warm-start: load model weights from a prior checkpoint .pt")
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
    # AsyncVectorEnv (subprocess per env) is REQUIRED for >1 env: the game runs as
    # a stackful coroutine, and running several coroutines in ONE process
    # miscompiles under the release optimizer when a coroutine is dropped on reset
    # while a sibling is live (the single-`&mut Engine`-across-suspends aliasing
    # hazard documented in src/frame.rs). One coroutine per process is the
    # proven-good path AND gives true parallelism (no GIL). SyncVectorEnv is kept
    # only for single-env debugging.
    if args.vec == "sync":
        envs = gym.vector.SyncVectorEnv(
            [make_env(cp, args.max_steps, args.reward_mode) for _ in range(args.num_envs)]
        )
    else:
        envs = gym.vector.AsyncVectorEnv(
            [make_env(cp, args.max_steps, args.reward_mode) for _ in range(args.num_envs)]
        )
    envs = gym.wrappers.vector.RecordEpisodeStatistics(envs)
    n_actions = int(envs.single_action_space.n)

    agent = Agent(n_actions, in_ch=envs.single_observation_space.shape[0]).to(device)
    if args.init_from:
        ck = torch.load(args.init_from, map_location=device, weights_only=False)
        agent.load_state_dict(ck["model"])
        print(f"warm-started from {args.init_from} (step {ck.get('global_step','?')})")
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

    # Stall detector: if no update completes for a while, an env has hung (a game
    # loop spinning with no frame yield blocks that subprocess, and AsyncVectorEnv
    # waits on it forever). The engine's input-poll watchdog should prevent the
    # known class, but if a new one appears, fail LOUD instead of silently
    # freezing for hours — periodic checkpoints (--save-every) are the recovery.
    heartbeat = [time.time()]

    def _stall_watch():
        while True:
            time.sleep(15)
            if time.time() - heartbeat[0] > args.stall_timeout:
                print(f"\n!!! STALL: no update in {args.stall_timeout}s — an env "
                      f"likely hung. Exiting; resume from {args.save_path}.", flush=True)
                os._exit(3)

    threading.Thread(target=_stall_watch, daemon=True).start()

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

        heartbeat[0] = time.time()  # an update completed; reset the stall guard
        sps = int(global_step / (time.time() - start))
        recent = returns_hist[-20:]
        mean_ret = np.mean(recent) if recent else float("nan")
        # With reward_mode=explore, episodic return ≈ 5·(new rooms)+0.3·(new cells),
        # so a rising mean_return IS rising coverage — the learning signal to watch.
        print(f"update {update}/{num_updates}  step {global_step}  SPS {sps}  "
              f"mean_return({len(recent)}ep) {mean_ret:.1f}  v_loss {v_loss.item():.2f}")

        if update % args.save_every == 0 or update == num_updates:
            os.makedirs(os.path.dirname(args.save_path) or ".", exist_ok=True)
            torch.save({"model": agent.state_dict(), "args": vars(args),
                        "global_step": global_step}, args.save_path)

    envs.close()
    if len(returns_hist) >= 10:
        first = np.mean(returns_hist[: len(returns_hist) // 3])
        last = np.mean(returns_hist[-len(returns_hist) // 3 :])
        print(f"\nreturn trend: first-third {first:.1f} -> last-third {last:.1f}  "
              f"({'up' if last > first else 'flat/down'})")


if __name__ == "__main__":
    main()
