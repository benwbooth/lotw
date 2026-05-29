#!/usr/bin/env python3
"""Drive FCEUX over replay fixtures with tools/fceux_coverage.lua and merge the
per-fixture PRG coverage into a single code/data coverage map.

Run inside `nix develop` (needs fceux + xvfb-run). Examples:
    python3 tools/re/run_coverage.py title_idle            # one fixture
    python3 tools/re/run_coverage.py all                   # every fixture
    python3 tools/re/run_coverage.py all --extra-frames 300

Outputs (gitignored) under build/coverage/:
    <fixture>/coverage.tsv + write logs
    merged_coverage.tsv   union of PRG offsets, summed exec counts
    merged_summary.txt
"""
import argparse
import os
import shutil
import signal
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ROM = ROOT / "rom" / "lotw.nes"
SCRIPT = ROOT / "tools" / "fceux_coverage.lua"
FIXDIR = ROOT / "fixtures" / "reference"
OUTROOT = ROOT / "build" / "coverage"


def fixture_path(name: str) -> Path:
    p = FIXDIR / f"{name}.replay"
    if not p.exists():
        sys.exit(f"no such fixture: {p}")
    return p


def replay_frames(path: Path) -> int:
    total = 0
    for line in path.read_text().splitlines():
        line = line.split("#", 1)[0].split()
        if line and line[0] == "frame":
            total += int(line[1])
    return total


def run_one(name: str, extra_frames: int, timeout: float) -> dict:
    replay = fixture_path(name)
    frames = replay_frames(replay) + extra_frames
    out = OUTROOT / name
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    done = out / "DONE"

    env = dict(os.environ)
    env["SDL_AUDIODRIVER"] = "dummy"  # mute: no real audio device (max-speed = noise)
    env.update(
        LOTW_COV_OUT_DIR=str(out),
        LOTW_COV_REPLAY=str(replay),
        LOTW_COV_FRAMES=str(frames),
        LOTW_COV_DONE=str(done),
    )
    cmd = [
        "xvfb-run", "-a",
        "fceux", "--sound", "0", "--loadlua", str(SCRIPT), str(ROM),
    ]
    print(f"[{name}] {frames} frames -> {out}")
    proc = subprocess.Popen(cmd, env=env, stdout=subprocess.DEVNULL,
                            stderr=subprocess.STDOUT, preexec_fn=os.setsid)
    t0 = time.time()
    try:
        while True:
            if done.exists():
                break
            if proc.poll() is not None:
                break
            if time.time() - t0 > timeout:
                print(f"[{name}] TIMEOUT after {timeout}s", file=sys.stderr)
                break
            time.sleep(0.25)
    finally:
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
        except ProcessLookupError:
            pass
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)

    cov = out / "coverage.tsv"
    n = sum(1 for _ in cov.open()) - 1 if cov.exists() else 0
    print(f"[{name}] covered PRG offsets: {n}  (done={done.exists()})")
    return {"name": name, "out": out, "covered": n, "done": done.exists()}


def run_explore(seed: int, frames: int, timeout: float, prefix: str = "start_game") -> dict:
    """Run a deterministic exploration session: a replay prefix to reach gameplay,
    then LCG-driven pseudo-random input for `frames` frames to wander code."""
    replay = fixture_path(prefix)
    total = replay_frames(replay) + frames
    name = f"explore_{seed}"
    out = OUTROOT / name
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    done = out / "DONE"
    env = dict(os.environ)
    env["SDL_AUDIODRIVER"] = "dummy"  # mute: no real audio device (max-speed = noise)
    env.update(
        LOTW_COV_OUT_DIR=str(out),
        LOTW_COV_REPLAY=str(replay),
        LOTW_COV_FRAMES=str(total),
        LOTW_COV_EXPLORE=str(seed),
        LOTW_COV_DONE=str(done),
    )
    cmd = ["xvfb-run", "-a", "fceux", "--sound", "0", "--loadlua", str(SCRIPT), str(ROM)]
    print(f"[{name}] {total} frames (prefix {prefix} + {frames} explore) -> {out}")
    proc = subprocess.Popen(cmd, env=env, stdout=subprocess.DEVNULL,
                            stderr=subprocess.STDOUT, preexec_fn=os.setsid)
    t0 = time.time()
    try:
        while not done.exists() and proc.poll() is None and time.time() - t0 < timeout:
            time.sleep(0.25)
    finally:
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
            proc.wait(timeout=5)
        except (ProcessLookupError, subprocess.TimeoutExpired):
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
            except ProcessLookupError:
                pass
    cov = out / "coverage.tsv"
    n = sum(1 for _ in cov.open()) - 1 if cov.exists() else 0
    print(f"[{name}] covered PRG offsets: {n}  (done={done.exists()})")
    return {"name": name, "out": out, "covered": n, "done": done.exists()}


def merge(results: list[dict]):
    merged: dict[int, list] = {}  # off -> [count, first_frame_in_run, cpu_addr, len]
    for r in results:
        cov = r["out"] / "coverage.tsv"
        if not cov.exists():
            continue
        lines = cov.read_text().splitlines()[1:]
        for ln in lines:
            off_s, cnt, ff, addr, ilen = ln.split("\t")
            off = int(off_s, 16)
            cnt = int(cnt)
            if off in merged:
                merged[off][0] += cnt
            else:
                merged[off] = [cnt, int(ff), addr, int(ilen)]
    OUTROOT.mkdir(parents=True, exist_ok=True)
    out = OUTROOT / "merged_coverage.tsv"
    with out.open("w") as f:
        f.write("prg_offset\texec_count\tcpu_addr\tinstr_len\n")
        for off in sorted(merged):
            c, _ff, addr, ilen = merged[off]
            f.write(f"{off:05X}\t{c}\t{addr}\t{ilen}\n")
    # covered bytes (union of instruction spans)
    covered_bytes = set()
    for off, (_c, _ff, _addr, ilen) in merged.items():
        for b in range(off, off + ilen):
            covered_bytes.add(b)
    PRG = 128 * 1024
    summary = OUTROOT / "merged_summary.txt"
    summary.write_text(
        f"fixtures={len(results)}\n"
        f"covered_instruction_starts={len(merged)}\n"
        f"covered_bytes={len(covered_bytes)}\n"
        f"prg_bytes={PRG}\n"
        f"prg_code_coverage_pct={100.0*len(covered_bytes)/PRG:.2f}\n"
    )
    print(f"\nmerged: {len(merged)} instruction starts, "
          f"{len(covered_bytes)} bytes ({100.0*len(covered_bytes)/PRG:.2f}% of PRG) -> {out}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("target", help="fixture name (no .replay), 'all', or 'explore'")
    ap.add_argument("--extra-frames", type=int, default=120,
                    help="frames to run past the replay end (idle exploration)")
    ap.add_argument("--explore-seeds", type=int, default=0,
                    help="number of pseudo-random exploration sessions to add")
    ap.add_argument("--explore-frames", type=int, default=12000,
                    help="explore frames per session (after the start_game prefix)")
    ap.add_argument("--timeout", type=float, default=600.0)
    args = ap.parse_args()

    if not ROM.exists():
        sys.exit(f"ROM missing: {ROM}")

    results = []
    if args.target in ("all", "explore"):
        if args.target == "all":
            for n in sorted(p.stem for p in FIXDIR.glob("*.replay")):
                results.append(run_one(n, args.extra_frames, args.timeout))
        seeds = args.explore_seeds or (6 if args.target == "explore" else 0)
        for s in range(1, seeds + 1):
            results.append(run_explore(s * 2654435761 % 2147483647,
                                       args.explore_frames, args.timeout))
    else:
        results.append(run_one(args.target, args.extra_frames, args.timeout))
    merge(results)


if __name__ == "__main__":
    main()
