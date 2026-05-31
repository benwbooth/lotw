#!/usr/bin/env python3
"""Bulk differential-test runner for the spec-driven C port.

Each routine is a self-contained src/ported/<name>.c (uniform `void name(Regs*)`)
plus port/specs/<name>.json. This runner generates the dispatch table, compiles
the selected routines with the generic harness, runs each against the m6502
oracle on thousands of random states, and reports PASS/FAIL.

Usage (inside `nix develop`):
    python3 tools/re/bulkdiff.py                 # test every spec
    python3 tools/re/bulkdiff.py --only NAME     # test one routine (agent self-check)
    python3 tools/re/bulkdiff.py -n 30000        # states per routine

A routine is conflict-free for parallel agents: an agent writes only its own
src/ported/<name>.c + port/specs/<name>.json, then runs --only <name>.
"""
import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from m6502 import CPU, ROM, C as FC, Z as FZ, N as FN, V as FV, U, I

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / "src"
PORTED = SRC / "ported"
SPECS = ROOT / "port" / "specs"
BUILD = ROOT / "build" / "port"
HARNESS = ROOT / "test" / "bulk_harness.c"
REC = 8 + 0x800


def lcg(seed):
    s = seed & 0xFFFFFFFF
    while True:
        s = (s * 1103515245 + 12345) & 0xFFFFFFFF
        yield (s >> 16) & 0xFF


SHAPERS = {
    # rng_update: modulus $80-$FF (one iteration, always terminates) or 0.
    "rng_count": lambda r: 0 if (r & 0x1F) == 0 else (0x80 | (r & 0x7F)),
}


def load_specs(only):
    specs = []
    for p in sorted(SPECS.glob("*.json")):
        s = json.loads(p.read_text())
        if only and s["name"] != only:
            continue
        if not (PORTED / f"{s['name']}.c").exists():
            print(f"!! {s['name']}: spec present but src/ported/{s['name']}.c missing", file=sys.stderr)
            continue
        specs.append(s)
    return specs


def build(dispatch_names, link_names, tag):
    """dispatch_names: routines callable by id (the ones under test).
    link_names: all .c to compile/link (under-test + their ported callees)."""
    bdir = BUILD / tag                      # isolated per-invocation build dir
    bdir.mkdir(parents=True, exist_ok=True)
    disp = ['#include "regs.h"']
    disp += [f"void {n}(Regs*);" for n in dispatch_names]
    disp.append("PortFn PORT_FNS[] = {" + ", ".join(dispatch_names) + "};")
    disp.append(f"int PORT_N = {len(dispatch_names)};")
    (bdir / "dispatch.c").write_text("\n".join(disp) + "\n")
    cc = os.environ.get("CC", "gcc")
    srcs = [str(HARNESS), str(bdir / "dispatch.c")] + [str(PORTED / f"{n}.c") for n in link_names]
    out = bdir / "harness"
    subprocess.run([cc, "-O2", "-DLOTW_HOST", f"-I{SRC}", "-o", str(out), *srcs], check=True)
    return out


def entry_flags(spec, a, x, y, rbits):
    e = spec.get("entry", "lda_a")
    if e == "lda_a":
        v = a
    elif e == "lda_x":
        v = x
    elif e == "lda_y":
        v = y
    elif e == "flags_input":
        return (rbits & 1, (rbits >> 1) & 1, (rbits >> 2) & 1, (rbits >> 3) & 1)
    else:  # "none"
        return (0, 0, 0, 0)
    return (0, 1 if v == 0 else 0, (v >> 7) & 1, 0)   # c,z,n,v from a LDA


def oracle(rom, spec, a, x, y, c, z, n, v, ram):
    cpu = CPU()
    cpu.map_fixed(rom)
    cpu.map_bank(rom, 13, 0xA000)
    cpu.mem[0x0000:0x0800] = ram
    # opt-in oracle hooks for NMI/PPU-synced routines (default: flat memory)
    oh = spec.get("oracle", {})
    if "ppu_status" in oh:
        cpu.ppu_status = int(str(oh["ppu_status"]), 16)
    p = (U | I) | (FC if c else 0) | (FZ if z else 0) | (FN if n else 0) | (FV if v else 0)
    sc = [int(x, 16) for x in oh["sync_clear"]] if "sync_clear" in oh else None
    cpu.run_routine(int(spec["addr"], 16), a=a, x=x, y=y, p=p, max_steps=200000,
                    vram_sync=bool(oh.get("vram_sync")), sync_clear=sc)
    return (cpu.a, cpu.x, cpu.y,
            1 if cpu.p & FC else 0, 1 if cpu.p & FZ else 0,
            1 if cpu.p & FN else 0, 1 if cpu.p & FV else 0,
            bytes(cpu.mem[0x0000:0x0800]))


def ram_eq(p, q):
    return p[:0x100] == q[:0x100] and p[0x200:] == q[0x200:]  # exclude stack page


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only")
    ap.add_argument("-n", type=int, default=20000)
    args = ap.parse_args()

    specs = load_specs(args.only)
    if not specs:
        print("no specs to test"); sys.exit(1)
    rom = ROM.read_bytes()
    dispatch_names = [s["name"] for s in specs]
    # Transitive closure of 'calls' across ALL specs, so a spec need only declare
    # its DIRECT callees (the harness pulls in callees-of-callees automatically).
    all_calls = {}
    for p in SPECS.glob("*.json"):
        cs = json.loads(p.read_text())
        all_calls[cs["name"]] = cs.get("calls", [])
    link, frontier = set(dispatch_names), list(dispatch_names)
    while frontier:
        n = frontier.pop()
        for c in all_calls.get(n, []):
            if c not in link:
                link.add(c); frontier.append(c)
    link = [n for n in link if (PORTED / f"{n}.c").exists()]
    harness = build(dispatch_names, link, args.only or "_all")

    idx = {s["name"]: i for i, s in enumerate(specs)}
    total_fail = 0
    for s in specs:
        rid = idx[s["name"]]
        rng = lcg(0xC0FFEE ^ (rid + 1) ^ (hash(s["name"]) & 0xFFFF))
        shaper = SHAPERS.get(s.get("shaper"))
        states, recs = [], bytearray()
        for _ in range(args.n):
            a = next(rng); a = shaper(a) if shaper else a
            x, y, fb = next(rng), next(rng), next(rng)
            ram = bytearray(next(rng) for _ in range(0x800))
            ram[0x1FD], ram[0x1FC] = 0x0F, 0xFE   # oracle sentinel ($0FFE)
            for k, v in s.get("setup", {}).items():  # pin bytes (e.g. a valid pointer)
                ram[int(k, 16) & 0x7FF] = int(v, 16)
            ram = bytes(ram)
            c, z, n, v = entry_flags(s, a, x, y, fb)
            states.append((a, x, y, c, z, n, v, ram))
            recs += bytes([rid, a, x, y, c, z, n, v]) + ram
        proc = subprocess.run([str(harness), str(ROM)], input=bytes(recs),
                              stdout=subprocess.PIPE, check=True)
        bad = skipped = 0
        cmp = s.get("compare", ["ram"])
        for i, st in enumerate(states):
            try:
                o = oracle(rom, s, *st)
            except (RuntimeError, ValueError):
                # The ORIGINAL crashes/hangs on this (unrealistic) random input —
                # e.g. a pointer-write routine whose random pointer hits the stack.
                # Not the port's fault; skip the state.
                skipped += 1
                continue
            g = proc.stdout[i * REC:(i + 1) * REC]
            ga = dict(a=g[1], x=g[2], y=g[3], c=g[4], z=g[5], n=g[6], v=g[7])
            oa = dict(a=o[0], x=o[1], y=o[2], c=o[3], z=o[4], n=o[5], v=o[6])
            ok = all((ram_eq(o[7], g[8:]) if w == "ram" else oa[w] == ga[w]) for w in cmp)
            if not ok:
                bad += 1
                if bad <= 2:
                    print(f"  [{s['name']}] MISMATCH #{i}: cmp={cmp} "
                          f"orig={oa} port={ga} ram_eq={ram_eq(o[7], g[8:])}")
        tested = args.n - skipped
        tag = f" [{skipped} states skipped: original crashed on random input]" if skipped else ""
        verdict = "PASS" if (bad == 0 and tested > 0) else f"FAIL ({bad})"
        print(f"{s['name']:16} {tested}/{args.n} states cmp={cmp}: {verdict}{tag}")
        total_fail += bad
    sys.exit(1 if total_fail else 0)


if __name__ == "__main__":
    main()
