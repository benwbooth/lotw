#!/usr/bin/env python3
"""Differential test: prove a ported C routine behaves identically to the
original 6502. The m6502 interpreter runs the ORIGINAL bytes (oracle); the
compiled C port runs the same injected states; RAM images are compared.

The C port is an INDEPENDENT reimplementation (written from the asm's meaning),
so agreement across thousands of random states is strong evidence both are
correct. Usage (inside `nix develop` or with gcc on PATH):
    python3 tools/re/difftest.py
"""
import os
import struct
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from m6502 import CPU, ROM

ROOT = Path(__file__).resolve().parents[2]
HARNESS_SRC = [ROOT / "test" / "host_harness.c", ROOT / "src" / "rng.c"]
HARNESS_BIN = ROOT / "build" / "host_harness"

# routine_id -> (name, cpu_addr, bank-map, build initial state)
ROUTINES = {
    0: dict(name="rng_update", pc=0xCC64, region="fixed"),
}


def lcg(seed):
    s = seed & 0xFFFFFFFF
    while True:
        s = (s * 1103515245 + 12345) & 0xFFFFFFFF
        yield (s >> 16) & 0xFF


def build_harness():
    HARNESS_BIN.parent.mkdir(exist_ok=True)
    cc = os.environ.get("CC", "gcc")
    cmd = [cc, "-O2", "-DLOTW_HOST", "-o", str(HARNESS_BIN), *map(str, HARNESS_SRC)]
    subprocess.run(cmd, check=True)


def oracle(rom, rid, a, ram):
    c = CPU()
    c.map_fixed(rom)
    c.mem[0x0000:0x0800] = ram
    info = ROUTINES[rid]
    # Entry flags model the caller's `LDA a` just before the JSR (Z/N reflect A),
    # which routines like rng_update rely on (STA doesn't set flags).
    from m6502 import Z, N, U, I
    p = (U | I) | (Z if a == 0 else 0) | (a & N)
    c.run_routine(info["pc"], a=a, p=p, max_steps=20000)
    return c.a, bytes(c.mem[0x0000:0x0800])


# Stack page ($0100-$01FF) is dirtied by the oracle's JSR/RTS call mechanism
# (the C port uses the host stack), so it is excluded from the RAM comparison.
def ram_eq(x, y):
    return x[:0x100] == y[:0x100] and x[0x200:] == y[0x200:]


def main():
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 20000
    rom = ROM.read_bytes()
    build_harness()

    fails = 0
    for rid, info in ROUTINES.items():
        rng = lcg(0xC0FFEE ^ rid)
        records = bytearray()
        states = []
        for k in range(n):
            # count: mostly $80-$FF (one loop iteration, always terminates since
            # the masked result is <=$7F < count), plus some 0 (early return).
            r = next(rng)
            a = 0 if (k % 20 == 0) else (0x80 | (r & 0x7F))
            ram = bytes(next(rng) for _ in range(0x800))
            states.append((a, ram))
            records += bytes([rid, a]) + ram
        # run compiled C port over all states
        proc = subprocess.run([str(HARNESS_BIN)], input=bytes(records),
                              stdout=subprocess.PIPE, check=True)
        rec = 1 + 0x800
        assert len(proc.stdout) == n * rec, f"harness output size {len(proc.stdout)}"
        bad = 0
        for i, (a, ram) in enumerate(states):
            exp_a, exp_ram = oracle(rom, rid, a, ram)
            got = proc.stdout[i * rec:(i + 1) * rec]
            got_a, got_ram = got[0], got[1:]
            if not ram_eq(exp_ram, got_ram):
                bad += 1
                if bad <= 3:
                    diffs = [f"${j:04X}:orig={exp_ram[j]:02X},port={got_ram[j]:02X}"
                             for j in range(0x800) if exp_ram[j] != got_ram[j]]
                    print(f"  [{info['name']}] MISMATCH state#{i} a={a:02X}: {diffs[:6]}")
        status = "PASS" if bad == 0 else f"FAIL ({bad}/{n})"
        print(f"{info['name']:14} ({n} random states): {status}")
        fails += bad
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
