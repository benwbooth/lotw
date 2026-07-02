# AI strategy — an agent that beats Legacy of the Wizard

## The goal

**Input:** the game (our deterministic Rust port) + the objective DAG.
**Output:** a single **input log** (`.replay`, one controller byte per frame) that
plays the game from power-on through every objective to the Keela kill,
reasonably efficiently. Replay it in the `play` binary and watch it win.

This is TAS construction, not real-time play. That framing is load-bearing:

- A live agent must never fail; a **composer** only needs to *eventually find*
  a working segment — it can retry from a savestate thousands of times and
  keep the best attempt.
- The port is deterministic, so a found segment replays **identically,
  guaranteed**. No robustness tax at playback.
- Stretch goal: construction fast enough to stream — the env steps ~3000 fps
  headless vs 60 fps playback, a ~50× compute budget for staying ahead of the
  playhead.

## Core assets and constraints

- **Savestates are input prefixes.** The game body runs in a coroutine that
  can't be snapshotted, so a checkpoint = replay-from-power-on. What looked
  like a limitation is the product: every checkpoint we mint *is* the master
  log up to that point. `fixtures/reference/*.replay` are both test fixtures
  and route milestones.
- **Pixels-only is a policy constraint, not a search constraint.** The
  deployed/learned policy sees frames (+ its instruction). The *composer* may
  read privileged state freely — RAM predicates, the object table, decoded
  maps — exactly as a human TASer uses RAM watch. The final log is pure
  button bytes either way.
- **The engine is the physics model.** We never re-implement movement rules
  (jump arcs, ladder-mount windows, drop-through platforms, enemies) outside
  the game. Anything that needs dynamics is answered by *rolling out in the
  real engine*, never by simulating on a map. (We tried inferring passability
  from tile IDs; it stalled us for hours. Lesson learned.)

## The layered architecture

```
KNOWLEDGE (static truth + intent)
  ROM-decoded world map      assettool render: all 64 rooms + world.png
  room topology              openings per room edge (metatile 64=ladder, 192=shaft)
  live object table          composer.objects(): chests/doors/enemies w/ exact positions
  aligned walkthrough        video_align.py: UCBVG frames paired with spoken captions
  objective DAG              agent/objectives.json (P0..P9) + agent/route.md

PLANNER (frontier model, zero-shot)
  reads the knowledge layer, writes COMMAND LISTS per segment:
      goto (room, x, y) → descend → touch → verify has_item(...)
  diagnoses failures from renders + state; updates the route

COMPOSER (agent/composer.py — the machine that grows the master log)
  do / macro / walk_to_px    scripted legs
  goto(x, y)                 distance-guided beam search over REAL rollouts
  sample(goal, ...)          predicate search (keeps the shortest winning suffix)
  objects() / RAM predicates verification: keys, coins, inventory, room, hp
  mark / rewind / save       segment retry + stitching, .replay export

EXECUTOR (how goto legs get done — three generations)
  1. random rollouts          works, wasteful
  2. policy-proposed rollouts PolicyProposer: trained net as the action distribution
  3. goal-conditioned policy  "go to (x,y)" as a NETWORK INPUT (goal planes in
                              the observation) — one-shots what search brute-forces
```

**Division of labor:** the frontier model zero-shots *perception and planning*
(reading maps, frames, captions, code; deciding what to do). The local model
and search handle *motor control* (frame-level inputs). Zero-shot reasoning
found the hidden dungeon entrance; no amount of it can time a pixel-perfect
ladder mount at 60 fps — and vice versa.

## Why not classical pathfinding (A*)?

Tile-level A* requires a faithful movement model — jump physics per character,
±5 px ladder-mount windows, moving enemies. Building that model means
re-implementing the game, badly. Instead:

- The decoded map provides **coarse room-graph routing only** ("to reach
  Pochi's section: (0,2) → (0,3) near x≈20 → (0,4)") — direction, not paths.
- The gap between waypoints is closed by **rollout search in the engine**
  (dynamics correct by construction) or by the goal-conditioned policy.

## The training strategy (local model)

Curriculum so far, each stage feeding the next:

1. **explore** — coverage reward. Taught survival + locomotion from pixels
   (0% deaths vs 92% random). Plateaued by design once its area was mastered.
2. **explore_lab** — dungeon-only coverage, surfacing terminates. Caught and
   closed a real specification-gaming exploit (the policy climbed back out and
   farmed the safe overworld). Reached consistent multi-room navigation.
3. **goto** — goal-conditioned: reach (x, y). Goals are **reachable by
   construction**: a scout rollout picks the goal, deterministic replay resets
   back. Dense progress reward + arrival bonus; goal direction enters the CNN
   as two constant observation planes. Success metric: % of held-out scouted
   goals reached within the step budget.

**The bootstrap loop (planned):** every segment the composer solves by search
is a demonstration for the policy ("the buttons that achieved goto X"); with
hindsight relabeling, even failed rollouts are demos for wherever they ended
up. Search bootstraps the policy → the policy accelerates search → the
composer speeds up as it works. No hand-labeled data anywhere.

## Verification discipline

- Every objective has a **RAM predicate** (item count, crown count, room
  reached, boss HP). A segment is stitched only when its predicate holds.
- Eval measures **behavior, not training return** — the explore-reward model
  looked great by return while it was secretly farming the overworld strip.
  Return is a proxy; rooms-reached / success-rate / items-obtained are truth.
- Faithfulness of the port is non-negotiable: engine changes must keep the
  byte-exact diff tests green (the input-poll safety valve is the template:
  fixes the harness hang while remaining inert in normal play).

## Roadmap

1. **Now:** goal-conditioned `goto` model (in training). Bar: solid success
   rate on scouted goals; then it becomes the composer's default executor.
2. **P0 by command list:** descend to Pochi's section ((0,4)-ish tan maze, per
   the decoded map + captioned video), collect money → Scroll → Gloves →
   Crystal, verified by inventory predicates; stitch fixtures per milestone.
3. **Route data flywheel:** waypoint the whole DAG from the aligned video
   sheets (`video_align.py` output) — every phase gets (room, landmark,
   action) waypoints.
4. **P1..P9:** same loop per phase. Bosses and glove puzzles will lean on
   heavier search (and possibly per-segment RL fine-tunes with the boss-HP
   predicate as reward).
5. **Efficiency pass:** re-solve fat segments for shorter suffixes; the log is
   "reasonably efficient", not frame-perfect.
6. **Stretch:** streaming construction; a generalist instruction-conditioned
   policy across games (the interface — command lists — is already
   game-agnostic).

## Hard-won lessons (do not relearn)

- **Reward hacking is real:** "explore" was satisfied by leaving the dungeon.
  Terminal-on-exit + domain-restricted rewards fixed it. Always eval behavior.
- **Don't infer physics from data tables** — ask the engine (rollouts) or the
  code (collision routines), never guess from tile/palette IDs.
- **Search needs targets, not just predicates.** Blind predicate search burned
  ~900 rollouts failing a two-tile approach; `goto` with a coordinate target
  solved the same leg in one round. Spatial knowledge (object table, maps)
  turns search from diffusion into navigation.
- **Narration ≠ routing.** The walkthrough transcript alone mislocated P0;
  frames + captions together are frame-accurate instructions
  (`video_align.py`). Distill routes from *aligned* video, not audio text.
- **Menus are a minefield:** character-select sub-menus contain hang states
  the input-poll valve can't catch (they spin without polling input). Search
  around menus with per-attempt subprocess timeouts.
- **NES trivia that cost us hours:** DOWN snaps x to the tile grid (mount
  assist); mounted-on-ladder ignores LEFT/RIGHT (looks frozen — press
  UP/DOWN); walking is ~1.1 px/frame, so "walls" may just be short walks;
  chests are locked-door objects (open on touch, consume a key).

## Tooling & setup

### The environment stack (game → Python)

| layer | what | where |
|---|---|---|
| Rust port | the faithful game engine; deterministic, headless, ~3000 fps | `src/` (`lotw::env::Env` in `src/env.rs` is the RL gate) |
| PyO3 bridge | `import lotw_env`: `Lotw(rom)`, `reset_replay(bytes)`, `advance/step`, `render()` (256×240×3), `ram()` (2 KiB), `state()` | `lotw-env/` |
| Gym env | `LotwEnv`: Discrete(10) button combos, pixel obs, reward modes `explore` / `explore_lab` / `goto` / `motion`, checkpoint = input prefix | `agent/lotw_gym.py` |
| Trainer | CleanRL-style PPO, NatureCNN (channel-count adapts to goal planes), GAE | `agent/train_ppo.py` |
| Composer | segment solver + stitcher (see architecture) | `agent/composer.py` |

The game body runs in a `corosensei` stackful coroutine (`src/frame.rs`) that
cannot be snapshotted — hence replay-based savestates. Two hard rules that
came from painful debugging:

- **One env per process.** Multiple game coroutines in one process miscompile
  in release when one is dropped mid-life (`&mut Engine`-across-suspends
  aliasing hazard; see `frame.rs` docs). `train_ppo.py --vec async`
  (AsyncVectorEnv, subprocess per env) is the default and required for >1 env.
- **The input-poll safety valve** (`game::read_controllers` →
  `frame::input_poll_watchdog`) converts input-wait spin loops (a faithful
  original freeze) into yieldable frames. Engine changes must keep
  `cargo run --release --bin env_smoke` reporting `deterministic: true`.

### Python: uv + nix

The flake (`flake.nix`) provides interpreters and native deps only
(`python3`, `uv`, `maturin`, plus `stdenv.cc.cc.lib`/`zlib` on
`LD_LIBRARY_PATH` so manylinux wheels load on NixOS). uv owns the Python
dependencies via the root `pyproject.toml`; `.envrc` runs `uv sync` and
activates `.venv`.

```bash
nix develop                       # toolchain shell (direnv does this + uv sync)
uv sync --group train             # torch (CPU wheel via pytorch-cpu index) + opencv
maturin develop --release --manifest-path lotw-env/Cargo.toml   # (re)build the extension
```

Gotchas: `uv sync` **without** `--group train` silently drops torch; do not
set `UV_PYTHON` in the flake (it would push maturin onto the read-only nix
python — only `UV_PYTHON_DOWNLOADS=never` is set); after editing lotw-env
Rust, re-run `maturin develop` (or `uv sync --reinstall-package lotw-env`).

### GPU training: rocm/pytorch container

Hardware: AMD RX 7900 XTX (gfx1100 — officially ROCm-supported, no
`HSA_OVERRIDE`). Torch-on-ROCm comes from the `rocm/pytorch` docker image,
never from nix. One command:

```bash
bash agent/run_gpu.sh                          # defaults: 2M steps, 16 envs, explore
REWARD_MODE=goto CHECKPOINT=fixtures/reference/labyrinth_room.replay \
  SAVE_PATH=agent/runs/ppo_goto.pt TIMESTEPS=3000000 MAX_STEPS=256 \
  bash agent/run_gpu.sh                        # env-var overridable
INIT_FROM=agent/runs/prev.pt bash agent/run_gpu.sh   # warm-start / resume
```

The script mounts the repo, installs a minimal rust toolchain in-container,
builds the wheel with `maturin build --interpreter python3` (NOT `develop`,
which would grab the mounted host venv), isolates `CARGO_TARGET_DIR=/tmp`,
and trains with a stall detector (`--stall-timeout`, exits loud instead of
hanging silently). Checkpoints land in `agent/runs/` (gitignored, root-owned
because docker). Monitor with `docker logs <cid>` / the redirected log file.

Throughput reference: ~1550 SPS on 16 async envs (env-bound — the CNN barely
taxes the GPU). CPU quirk: `torch.set_num_threads(8)` (`TORCH_THREADS` env) —
the default one-thread-per-core is ~14× slower on small batches.

### Evaluation

```bash
python agent/eval.py --checkpoint-model agent/runs/ppo_lab3.pt \
  --replay fixtures/reference/labyrinth_room.replay --episodes 15 [--greedy|--random]
```

Reports behavior: distinct rooms reached, cells covered, death rate. For the
goto model: success-rate on held-out scouted goals (see scratch eval).

### ROM asset tools (the knowledge layer)

```bash
# lossless extraction (already committed under assets/):
cargo run --release --features assets --bin assettool -- extract rom/lotw.nes assets
# render every room + the full 4×16 world map as PNGs:
cargo run --release --features assets --bin assettool -- render rom/lotw.nes assets OUT_DIR
```

`assets/rooms/room-YY-X.csv` are 64×12 metatile grids (learned ids: 64=ladder,
192=vertical shaft, 182=floor-ish, 111=air); `manifest.json` carries mapx/mapy,
headers (CHR banks at bytes +5/+6), actor spawns, palettes. Read the rendered
rooms (downscale via ffmpeg first — they're 1024×192 each).

### Video → routing tools

```bash
# subtitles (auto-subs) once:
nix run nixpkgs#yt-dlp -- --skip-download --write-auto-subs --sub-lang en \
  --convert-subs srt -o subs "https://www.youtube.com/watch?v=V9R8SGS5-LM"
# clip a chapter:
nix run nixpkgs#yt-dlp -- --download-sections "*11:50-16:30" -f "b[height<=480]" \
  -o clip.mp4 "https://www.youtube.com/watch?v=V9R8SGS5-LM"
# pair frames with spoken captions -> contact sheets (the routing document):
python agent/video_align.py clip.mp4 subs.en.srt outdir 710 275   # base_s, clip_len_s
```

Plain uncaptioned sheets when needed:
`ffmpeg -i clip.mp4 -vf "fps=1/5,scale=320:-1,tile=3x3" sheet%02d.png`.

### Checkpoints / fixtures (`.replay` format)

Text run-length format, one line per held input: `frame <count> [buttons…]`
(`A B select start up down left right`); `lotw_gym.load_replay()` expands it
to one byte/frame (hardware bit order: A=1 B=2 sel=4 start=8 U=16 D=32 L=64
R=128). Milestones committed under `fixtures/reference/`:
`labyrinth_entry` (Lyll at world (0,0)), `labyrinth_room` (training spawn),
`pochi_lab` (Pochi at (0,0)), `pochi_p0` (Pochi at (0,2) — master-log head).
`Composer.save()` writes this format; every stitched milestone should become
a fixture + commit (stage files explicitly — never `git add -A`).
