#!/usr/bin/env python3
"""Differential test: prove ported C routines behave identically to the original
6502. The m6502 interpreter runs the ORIGINAL bytes (oracle); the compiled C
port runs the same injected states (RAM + A/X/Y); RAM, registers, and the carry
flag are compared per the routine's declared outputs.

The C ports are INDEPENDENT reimplementations, so agreement across thousands of
random states is strong evidence of correctness. Run inside `nix develop`:
    python3 tools/re/difftest.py [n_states]
"""
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from m6502 import CPU, ROM, C as FLAG_C, N as FLAG_N, Z as FLAG_Z, U, I

ROOT = Path(__file__).resolve().parents[2]
HARNESS_SRC = [ROOT / "test" / "host_harness.c", ROOT / "src" / "rng.c", ROOT / "src" / "leaves.c"]
HARNESS_BIN = ROOT / "build" / "host_harness"
REC_IN = 4 + 0x800
REC_OUT = 4 + 0x800


def rng_count_shaper(r):
    # rng_update: count in $80-$FF (one iteration, always terminates) or 0.
    return 0 if (r & 0x1F) == 0 else (0x80 | (r & 0x7F))


# id -> spec. compare: subset of {ram, a, x, y, c}
ROUTINES = {
    0: dict(name="rng_update", pc=0xCC64, compare=["ram"], a_shaper=rng_count_shaper),
    1: dict(name="sub_E41E",   pc=0xE41E, compare=["x"]),
    2: dict(name="sub_F233",   pc=0xF233, compare=["c"]),
    3: dict(name="inc16_95",   pc=0xFD6B, compare=["ram", "x"]),
}


def lcg(seed):
    s = seed & 0xFFFFFFFF
    while True:
        s = (s * 1103515245 + 12345) & 0xFFFFFFFF
        yield (s >> 16) & 0xFF


def build_harness():
    HARNESS_BIN.parent.mkdir(exist_ok=True)
    cc = os.environ.get("CC", "gcc")
    subprocess.run([cc, "-O2", "-DLOTW_HOST", "-o", str(HARNESS_BIN), *map(str, HARNESS_SRC)],
                   check=True)


def oracle(rom, info, a, x, y, ram):
    c = CPU()
    c.map_fixed(rom)
    c.map_bank(rom, 13, 0xA000)            # bank 13 also resident
    c.mem[0x0000:0x0800] = ram
    p = (U | I) | (FLAG_Z if a == 0 else 0) | (a & FLAG_N)   # entry flags ~ caller's LDA a
    c.run_routine(info["pc"], a=a, x=x, y=y, p=p, max_steps=20000)
    return c.a, c.x, c.y, 1 if (c.p & FLAG_C) else 0, bytes(c.mem[0x0000:0x0800])


def ram_eq(x, y):  # stack page is the oracle's call-mechanism scratch
    return x[:0x100] == y[:0x100] and x[0x200:] == y[0x200:]


def main():
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 20000
    rom = ROM.read_bytes()
    build_harness()
    total_fail = 0
    for rid, info in ROUTINES.items():
        rng = lcg(0xC0FFEE ^ rid)
        states, records = [], bytearray()
        for _ in range(n):
            a = info.get("a_shaper", lambda r: r)(next(rng))
            x, y = next(rng), next(rng)
            ram = bytearray(next(rng) for _ in range(0x800))
            # The oracle pushes a sentinel return address ($0FFE) at $01FC/$01FD;
            # seed those bytes so pointer reads into the stack page agree.
            ram[0x1FD], ram[0x1FC] = 0x0F, 0xFE
            ram = bytes(ram)
            states.append((a, x, y, ram))
            records += bytes([rid, a, x, y]) + ram
        proc = subprocess.run([str(HARNESS_BIN), str(ROM)], input=bytes(records),
                              stdout=subprocess.PIPE, check=True)
        bad = 0
        for i, (a, x, y, ram) in enumerate(states):
            oa, ox, oy, oc, oram = oracle(rom, info, a, x, y, ram)
            g = proc.stdout[i * REC_OUT:(i + 1) * REC_OUT]
            ga, gx, gy, gc, gram = g[0], g[1], g[2], g[3], g[4:]
            ok = True
            for what in info["compare"]:
                if what == "ram" and not ram_eq(oram, gram): ok = False
                if what == "a" and oa != ga: ok = False
                if what == "x" and ox != gx: ok = False
                if what == "y" and oy != gy: ok = False
                if what == "c" and oc != gc: ok = False
            if not ok:
                bad += 1
                if bad <= 2:
                    print(f"  [{info['name']}] MISMATCH #{i} a={a:02X} x={x:02X} y={y:02X}: "
                          f"orig(a={oa:02X},x={ox:02X},y={oy:02X},c={oc}) "
                          f"port(a={ga:02X},x={gx:02X},y={gy:02X},c={gc})")
        print(f"{info['name']:14} {n} states, cmp={info['compare']}: "
              f"{'PASS' if bad == 0 else f'FAIL ({bad})'}")
        total_fail += bad
    sys.exit(1 if total_fail else 0)


if __name__ == "__main__":
    main()
