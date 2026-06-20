#!/usr/bin/env python3
"""Replay-driven user-visible regression checks.

This is the layer the RAM lockstep harness intentionally does not cover:

* render the C port at deterministic replay frames and compare screenshots with
  FCEUX captures, catching palette/scroll/sprite/layering mistakes;
* optionally compare APU register write streams, catching wrong song/bank/audio
  driver behavior before it becomes a listening test;
* optionally assert RAM outcomes at named frames for gameplay contracts such as
  item pickup or inventory changes.

Examples:
  nix develop --command python3 tools/re/replay_regression.py outside_walk
  nix develop --command python3 tools/re/replay_regression.py outside_walk --refresh-reference
  nix develop --command python3 tools/re/replay_regression.py room_transition --check-apu --refresh-reference
  nix develop --command python3 tools/re/replay_regression.py outside_walk \\
    --assert-ram outside_walk:1830:0047=03
"""
from __future__ import annotations

import argparse
import os
import re
import shutil
import signal
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ROM = ROOT / "rom" / "lotw.nes"
FIXDIR = ROOT / "fixtures" / "reference"
REF_SCRIPT = ROOT / "tools" / "fceux_capture.lua"
OUTROOT = ROOT / "build" / "replay_regression"
PORT_CAPTURE = ROOT / "build" / "replay_capture_port"


@dataclass(frozen=True)
class Case:
    name: str
    fixture: Path
    reference: Path
    frames: tuple[int, ...]
    max_mapped_diff: int = 512
    max_mean_abs: float = 30.0
    apu_window: tuple[int, int] | None = None


DEFAULT_CASES: dict[str, Case] = {
    "title_idle": Case("title_idle", FIXDIR / "title_idle.replay", ROOT / "build/reference/title_idle", (180,), 8192, 30.0),
    "start_game": Case("start_game", FIXDIR / "start_game.replay", ROOT / "build/reference/start_game", (420,), 2500, 30.0),
    "gameplay_walk": Case("gameplay_walk", FIXDIR / "gameplay_walk.replay", ROOT / "build/reference/gameplay_walk", (732,), 2500, 30.0),
    "gameplay_climb": Case("gameplay_climb", FIXDIR / "gameplay_climb.replay", ROOT / "build/reference/gameplay_climb", (840,), 2500, 30.0),
    "room_transition": Case(
        "room_transition",
        FIXDIR / "room_transition.replay",
        ROOT / "build/reference/ctest_replay_smoke/room_transition",
        (924, 1044),
        512,
        30.0,
        (924, 1044),
    ),
    # These older references were generated from temporary replays that match the
    # checked-in fixtures' button streams. Keep the mapping explicit so the harness
    # can reuse them but future refreshes can move them under build/replay_regression.
    "outside_walk": Case(
        "outside_walk",
        FIXDIR / "outside_walk.replay",
        ROOT / "build/reference/tmp_outside_walk",
        (1044, 1261, 1440, 1620, 1830),
    ),
    "door_return": Case(
        "door_return",
        FIXDIR / "door_return.replay",
        ROOT / "build/reference/tmp_outside_door",
        (1044, 1261, 1380, 1500, 1710),
        2500,
        30.0,
    ),
}


def run(cmd: list[str], *, env: dict[str, str] | None = None, timeout: float | None = None) -> None:
    print("+ " + " ".join(str(x) for x in cmd))
    subprocess.run(cmd, cwd=ROOT, env=env, timeout=timeout, check=True)


def build_port_capture() -> None:
    if not (ROOT / "build/CMakeCache.txt").exists():
        run(["cmake", "-S", ".", "-B", "build"])
    run(["cmake", "--build", "build", "--target", "replay_capture_port", "-j"])


def frame_numbers(refdir: Path) -> list[int]:
    frames = []
    for path in refdir.glob("frame_*.ppm"):
        m = re.fullmatch(r"frame_(\d+)\.ppm", path.name)
        if m:
            frames.append(int(m.group(1)))
    return sorted(frames)


def parse_case(text: str) -> Case:
    if text in DEFAULT_CASES:
        return DEFAULT_CASES[text]
    parts = text.split(":")
    if len(parts) != 3:
        known = ", ".join(sorted(DEFAULT_CASES))
        raise SystemExit(f"unknown case {text!r}; use one of {known} or name:fixture:reference-dir")
    name, fixture, reference = parts
    return Case(name, (ROOT / fixture).resolve(), (ROOT / reference).resolve(), ())


def run_reference_capture(case: Case, frames: list[int], refdir: Path, check_apu: bool, timeout: float) -> Path | None:
    if refdir.exists():
        shutil.rmtree(refdir)
    refdir.mkdir(parents=True)
    done = refdir / "DONE"
    apu_trace = refdir / "apu_writes.tsv" if check_apu else None
    env = dict(os.environ)
    env["SDL_AUDIODRIVER"] = "dummy"
    env.update(
        LOTW_REFERENCE_OUT_DIR=str(refdir),
        LOTW_REFERENCE_REPLAY=str(case.fixture),
        LOTW_REFERENCE_FRAMES=",".join(str(f) for f in frames),
        LOTW_REFERENCE_DONE=str(done),
    )
    if apu_trace:
        env["LOTW_REFERENCE_APU_TRACE"] = str(apu_trace)

    cmd = ["xvfb-run", "-a", "fceux", "--sound", "0", "--loadlua", str(REF_SCRIPT), str(ROM)]
    print(f"[{case.name}] refresh FCEUX reference -> {refdir}")
    proc = subprocess.Popen(cmd, cwd=ROOT, env=env, stdout=subprocess.DEVNULL,
                            stderr=subprocess.STDOUT, preexec_fn=os.setsid)
    t0 = time.time()
    try:
        while not done.exists() and proc.poll() is None:
            if time.time() - t0 > timeout:
                raise TimeoutError(f"FCEUX reference capture timed out after {timeout}s")
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
            proc.wait()

    if not done.exists():
        raise RuntimeError(f"FCEUX reference capture did not complete for {case.name}")
    return apu_trace


def run_port_capture(case: Case, frames: list[int], outdir: Path, check_apu: bool, timeout: float) -> Path | None:
    if outdir.exists():
        shutil.rmtree(outdir)
    outdir.mkdir(parents=True)
    apu_trace = outdir / "apu_writes.tsv" if check_apu else None
    env = dict(os.environ)
    if apu_trace:
        env["LOTW_PORT_APU_TRACE"] = str(apu_trace)
    run([str(PORT_CAPTURE), str(ROM), str(case.fixture), str(outdir), ",".join(str(f) for f in frames)],
        env=env, timeout=timeout)
    return apu_trace


def read_ppm(path: Path) -> tuple[int, int, bytes]:
    data = path.read_bytes()
    pos = 0

    def token() -> bytes:
        nonlocal pos
        while pos < len(data) and data[pos] in b" \t\r\n":
            pos += 1
        if pos < len(data) and data[pos] == ord("#"):
            while pos < len(data) and data[pos] not in b"\r\n":
                pos += 1
            return token()
        start = pos
        while pos < len(data) and data[pos] not in b" \t\r\n":
            pos += 1
        return data[start:pos]

    magic = token()
    if magic != b"P6":
        raise ValueError(f"{path}: expected P6 PPM, got {magic!r}")
    w = int(token())
    h = int(token())
    maxv = int(token())
    if maxv != 255:
        raise ValueError(f"{path}: unsupported max value {maxv}")
    while pos < len(data) and data[pos] in b" \t\r\n":
        pos += 1
    pixels = data[pos:]
    if len(pixels) != w * h * 3:
        raise ValueError(f"{path}: got {len(pixels)} pixel bytes, expected {w*h*3}")
    return w, h, pixels


def nearest_color_map(src: bytes, ref: bytes) -> dict[bytes, bytes]:
    src_colors = {src[i:i + 3] for i in range(0, len(src), 3)}
    ref_colors = list({ref[i:i + 3] for i in range(0, len(ref), 3)})
    out: dict[bytes, bytes] = {}
    for color in src_colors:
        cr, cg, cb = color
        out[color] = min(
            ref_colors,
            key=lambda r: (cr - r[0]) * (cr - r[0]) + (cg - r[1]) * (cg - r[1]) + (cb - r[2]) * (cb - r[2]),
        )
    return out


def write_diff(path: Path, ref: bytes, port: bytes, mapped: dict[bytes, bytes]) -> None:
    out = bytearray()
    for i in range(0, len(ref), 3):
        rc = ref[i:i + 3]
        pc = mapped[port[i:i + 3]]
        if pc == rc:
            # dim reference pixels so failure blobs stand out
            out.extend(bytes((ref[i] // 3, ref[i + 1] // 3, ref[i + 2] // 3)))
        else:
            out.extend(b"\xff\x00\x00")
    path.write_bytes(b"P6\n256 240\n255\n" + bytes(out))


def image_metrics(ref_path: Path, port_path: Path, diff_path: Path) -> dict[str, float]:
    rw, rh, ref = read_ppm(ref_path)
    pw, ph, port = read_ppm(port_path)
    if (rw, rh) != (pw, ph):
        raise ValueError(f"size mismatch: {ref_path} {(rw, rh)} vs {port_path} {(pw, ph)}")

    raw_diff = 0
    total_abs = 0
    max_abs = 0
    for i in range(0, len(ref), 3):
        if ref[i:i + 3] != port[i:i + 3]:
            raw_diff += 1
        for c in range(3):
            d = abs(ref[i + c] - port[i + c])
            total_abs += d
            if d > max_abs:
                max_abs = d

    mapped = nearest_color_map(port, ref)
    mapped_diff = 0
    for i in range(0, len(ref), 3):
        if mapped[port[i:i + 3]] != ref[i:i + 3]:
            mapped_diff += 1
    write_diff(diff_path, ref, port, mapped)
    return {
        "raw_diff_pixels": raw_diff,
        "mapped_diff_pixels": mapped_diff,
        "mean_abs_channel": total_abs / len(ref),
        "max_abs_channel": max_abs,
        "ref_colors": len({ref[i:i + 3] for i in range(0, len(ref), 3)}),
        "port_colors": len({port[i:i + 3] for i in range(0, len(port), 3)}),
    }


def load_apu_trace(path: Path, start_frame: int | None = None, end_frame: int | None = None) -> list[tuple[int, int, int]]:
    rows = []
    for line in path.read_text().splitlines():
        if not line or line.startswith("frame"):
            continue
        frame, addr, value = line.split("\t")[:3]
        f = int(frame)
        if start_frame is not None and f < start_frame:
            continue
        if end_frame is not None and f > end_frame:
            continue
        rows.append((f, int(addr, 16), int(value, 16)))
    return rows


def compare_apu(case: str, ref_trace: Path, port_trace: Path, start_frame: int, end_frame: int) -> list[str]:
    ref = load_apu_trace(ref_trace, start_frame, end_frame)
    port = load_apu_trace(port_trace, start_frame, end_frame)
    errors = []
    if len(ref) != len(port):
        errors.append(
            f"[{case}] APU write count differs in frames {start_frame}..{end_frame}: "
            f"ref={len(ref)} port={len(port)}"
        )
    for i, (r, p) in enumerate(zip(ref, port)):
        if (r[1], r[2]) != (p[1], p[2]):
            errors.append(
                f"[{case}] first APU mismatch in frames {start_frame}..{end_frame} at write {i}: "
                f"ref f{r[0]} ${r[1]:04X}={r[2]:02X}, port f{p[0]} ${p[1]:04X}={p[2]:02X}"
            )
            break
    return errors


def parse_assertions(values: list[str]) -> dict[tuple[str, int, int], int]:
    out = {}
    for value in values:
        try:
            lhs, rhs = value.split("=", 1)
            case, frame, addr = lhs.split(":", 2)
        except ValueError as exc:
            raise SystemExit(f"invalid --assert-ram {value!r}; expected case:frame:addr=value") from exc
        out[(case, int(frame), int(addr, 16))] = int(rhs, 16)
    return out


def check_ram_assertions(case: str, outdir: Path, assertions: dict[tuple[str, int, int], int]) -> list[str]:
    errors = []
    for (name, frame, addr), expected in assertions.items():
        if name != case:
            continue
        path = outdir / f"ram_{frame:06d}.bin"
        if not path.exists():
            errors.append(f"[{case}] RAM assertion frame missing: {path}")
            continue
        data = path.read_bytes()
        actual = data[addr]
        if actual != expected:
            errors.append(f"[{case}] RAM ${addr:04X} at frame {frame}: got {actual:02X}, expected {expected:02X}")
    return errors


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("cases", nargs="*", default=["outside_walk"],
                    help="case names, 'all', or name:fixture:reference-dir")
    ap.add_argument("--refresh-reference", action="store_true",
                    help="generate fresh FCEUX references under build/replay_regression/reference")
    ap.add_argument("--check-apu", action="store_true",
                    help="compare APU register writes too; implies a fresh FCEUX trace unless an apu_writes.tsv exists")
    ap.add_argument("--timeout", type=float, default=120.0)
    ap.add_argument("--max-mapped-diff-pixels", type=int, default=None,
                    help="override the per-case palette-normalized pixel mismatch limit")
    ap.add_argument("--max-mean-abs-channel", type=float, default=None,
                    help="override the per-case raw RGB sanity limit")
    ap.add_argument("--assert-ram", action="append", default=[],
                    help="assert a port RAM byte after capture, format case:frame:addr=value, hex addr/value")
    args = ap.parse_args()

    if not ROM.exists():
        raise SystemExit(f"ROM missing: {ROM}")

    build_port_capture()
    assertions = parse_assertions(args.assert_ram)

    requested = list(DEFAULT_CASES) if args.cases == ["all"] else args.cases
    failures: list[str] = []
    for text in requested:
        case = parse_case(text)
        refdir = case.reference
        frames = list(case.frames) or frame_numbers(refdir)
        if args.refresh_reference:
            if not frames:
                # For a new reference directory, capture the end of the replay.
                frames = [sum(int(line.split()[1]) for line in case.fixture.read_text().splitlines()
                              if line.split("#", 1)[0].split()[:1] == ["frame"])]
            refdir = OUTROOT / "reference" / case.name
            ref_apu = run_reference_capture(case, frames, refdir, args.check_apu, args.timeout)
        else:
            ref_apu = refdir / "apu_writes.tsv" if args.check_apu and (refdir / "apu_writes.tsv").exists() else None

        frames = frame_numbers(refdir)
        if not frames:
            failures.append(f"[{case.name}] no reference frame_*.ppm files in {refdir}")
            continue

        outdir = OUTROOT / "port" / case.name
        port_apu = run_port_capture(case, frames, outdir, args.check_apu, args.timeout)

        print(f"[{case.name}] compare {len(frames)} frame(s)")
        for frame in frames:
            ref_frame = refdir / f"frame_{frame:06d}.ppm"
            port_frame = outdir / f"frame_{frame:06d}.ppm"
            diff_frame = outdir / f"diff_{frame:06d}.ppm"
            metrics = image_metrics(ref_frame, port_frame, diff_frame)
            print(
                f"  f{frame}: mapped_diff={metrics['mapped_diff_pixels']} "
                f"raw_diff={metrics['raw_diff_pixels']} mean_abs={metrics['mean_abs_channel']:.2f} "
                f"colors ref/port={int(metrics['ref_colors'])}/{int(metrics['port_colors'])}"
            )
            max_mapped = args.max_mapped_diff_pixels if args.max_mapped_diff_pixels is not None else case.max_mapped_diff
            max_mean = args.max_mean_abs_channel if args.max_mean_abs_channel is not None else case.max_mean_abs
            if metrics["mapped_diff_pixels"] > max_mapped:
                failures.append(
                    f"[{case.name}] frame {frame} mapped pixel diff {metrics['mapped_diff_pixels']} "
                    f"> {max_mapped}; diff={diff_frame}"
                )
            if metrics["mean_abs_channel"] > max_mean:
                failures.append(
                    f"[{case.name}] frame {frame} mean channel abs {metrics['mean_abs_channel']:.2f} "
                    f"> {max_mean}"
                )

        failures.extend(check_ram_assertions(case.name, outdir, assertions))

        if args.check_apu:
            if ref_apu is None:
                failures.append(f"[{case.name}] no reference APU trace; rerun with --refresh-reference")
            elif port_apu is None:
                failures.append(f"[{case.name}] no port APU trace")
            elif case.apu_window is None:
                print(f"[{case.name}] skip APU compare: no stable APU window configured")
            else:
                failures.extend(compare_apu(case.name, ref_apu, port_apu, case.apu_window[0], case.apu_window[1]))

    if failures:
        print("\nFAIL")
        for failure in failures:
            print("  " + failure)
        return 1

    print("\nPASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
