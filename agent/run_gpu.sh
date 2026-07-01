#!/usr/bin/env bash
# First real PPO training run on the AMD GPU (RX 7900 XT/XTX, gfx1100) via the
# rocm/pytorch container. gfx1100 is officially ROCm-supported, so no
# HSA_OVERRIDE_GFX_VERSION is needed. Run from the repo root ON THE GPU HOST:
#
#     bash agent/run_gpu.sh                 # defaults below
#     TIMESTEPS=10000000 NUM_ENVS=24 bash agent/run_gpu.sh
#
# The container has torch+ROCm but not our env, so it builds lotw_env (needs a
# rust toolchain, fetched once) and installs the Python deps, then trains.
#
# Watch `mean_return`: with reward_mode=explore it ≈ 5·(new rooms)+0.2·(new cells),
# so a rising mean_return IS the agent exploring further through the labyrinth —
# the learning signal. Checkpoints land in agent/runs/ (gitignored).
set -euo pipefail

IMAGE="${IMAGE:-rocm/pytorch:latest}"
TIMESTEPS="${TIMESTEPS:-2000000}"
NUM_ENVS="${NUM_ENVS:-16}"
NUM_STEPS="${NUM_STEPS:-128}"
MAX_STEPS="${MAX_STEPS:-1024}"
CHECKPOINT="${CHECKPOINT:-fixtures/reference/outside_walk.replay}"
SAVE_PATH="${SAVE_PATH:-agent/runs/ppo_gpu.pt}"

# Use an interactive TTY only when we actually have one (so the script works both
# from a terminal and when launched/monitored non-interactively).
TTY=; [ -t 1 ] && TTY=-it

exec docker run --rm $TTY \
  --device=/dev/kfd --device=/dev/dri --group-add video \
  --security-opt seccomp=unconfined \
  --shm-size 8G \
  -v "$PWD":/lotw -w /lotw \
  -e TIMESTEPS="$TIMESTEPS" -e NUM_ENVS="$NUM_ENVS" -e NUM_STEPS="$NUM_STEPS" \
  -e MAX_STEPS="$MAX_STEPS" -e CHECKPOINT="$CHECKPOINT" -e SAVE_PATH="$SAVE_PATH" \
  "$IMAGE" bash -lc '
    set -euo pipefail
    # Rust toolchain (only to build the lotw_env PyO3 extension; pure-rust, no SDL).
    if ! command -v cargo >/dev/null 2>&1; then
      curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
      . "$HOME/.cargo/env"
    fi
    pip install -q --no-input maturin gymnasium numpy opencv-python-headless
    echo "=== building lotw_env (release) ==="
    # Build a wheel against the CONTAINER python and pip-install it. (Do NOT use
    # `maturin develop`: it would grab the mounted host .venv, whose interpreter
    # is the nix python that does not exist in the container.) Isolate the cargo
    # target dir so the container build never clobbers the host nix target/.
    export CARGO_TARGET_DIR=/tmp/lotw-target
    maturin build --release --interpreter python3 -m lotw-env/Cargo.toml
    pip install -q --force-reinstall --no-deps "$CARGO_TARGET_DIR"/wheels/lotw_env-*.whl
    python -c "import torch; print(\"torch\", torch.__version__, \"cuda(=ROCm):\", torch.cuda.is_available(), torch.cuda.get_device_name(0) if torch.cuda.is_available() else \"\")"
    echo "=== training ==="
    exec python -u agent/train_ppo.py \
      --reward-mode explore --vec async \
      --total-timesteps "$TIMESTEPS" --num-envs "$NUM_ENVS" --num-steps "$NUM_STEPS" \
      --max-steps "$MAX_STEPS" --checkpoint "$CHECKPOINT" --save-path "$SAVE_PATH"
  '
