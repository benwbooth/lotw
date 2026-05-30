#!/usr/bin/env python3
"""Rank every engine subroutine for the C port: size, callees, hardware I/O,
leaf-ness. Helps decide port order (leaves first) and what the diff-test harness
must model (hardware, sub-calls).

Builds a memory image with both resident code windows mapped simultaneously
(bank 13 -> $A000-$BFFF, fixed banks 14+15 -> $C000-$FFFF), recursively decodes
from the known entries, collects all JSR targets, then traces each routine's
body (following branches, NOT recursing into callees)."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from disasm6502 import OPS, MODE_LEN, ABS, ABX, ABY, REL, TERMINATORS, BankDisasm
import symbols

ROOT = Path(__file__).resolve().parents[2]
ROM = (ROOT / "rom" / "lotw.nes").read_bytes()
COV = ROOT / "build" / "coverage" / "merged_coverage.tsv"
ENTRIES = ROOT / "disasm" / "entries.txt"
PRG = 0x10
BANK = 0x2000
FIX_ANCHORS = {0xC000, 0xD1FE, 0xD41D, 0xD36E, 0xCC9C, 0xCD08, 0xC833, 0xCCE4,
               0xFFE0, 0xF89A, 0xFC08}


def build_mem():
    mem = bytearray(0x10000)
    mem[0xC000:0x10000] = ROM[PRG + 14 * BANK: PRG + 16 * BANK]   # fixed 14+15
    mem[0xA000:0xC000] = ROM[PRG + 13 * BANK: PRG + 14 * BANK]    # bank 13
    return mem


def gather_entries():
    fix, b13 = set(FIX_ANCHORS), set()
    for a in symbols.ROUTINES:
        (fix if a >= 0xC000 else b13 if 0xA000 <= a < 0xC000 else set()).add(a)
    if COV.exists():
        for ln in COV.read_text().splitlines()[1:]:
            c = ln.split("\t"); off, cpu = int(c[0], 16), int(c[2], 16)
            b = off // BANK
            if b == 13 and 0xA000 <= cpu < 0xC000:
                b13.add(cpu)
            elif b >= 14:
                fix.add(cpu)
    if ENTRIES.exists():
        for ln in ENTRIES.read_text().splitlines():
            ln = ln.split("#", 1)[0].split()
            if ln:
                a = int(ln[0], 16)
                (fix if a >= 0xC000 else b13).add(a)
    return fix, b13


def all_jsr_targets(mem, fix, b13):
    """Recursively decode from entries; return every JSR target address."""
    fb = BankDisasm(mem[0xC000:0x10000], 0xC000, "FIX",
                    dispatchers={0xCC9C: "0C0D", 0xCD08: "0C0D"})
    fb.trace(fix)
    bb = BankDisasm(mem[0xA000:0xC000], 0xA000, "13",
                    dispatchers={0xCC9C: "0C0D", 0xCD08: "0C0D"})
    bb.trace(b13)
    targets = set()
    for bd, base in ((fb, 0xC000), (bb, 0xA000)):
        for pc in bd.starts:
            if mem[pc] == 0x20:  # JSR abs
                targets.add(mem[pc + 1] | (mem[pc + 2] << 8))
    return targets


def analyze(mem, entry, routine_entries=frozenset()):
    """Trace one routine body: follow branches/local JMPs + fallthrough, record
    (don't recurse into) JSR callees and tail-JMPs to other routines.
    Returns size/instr/callees/hw/has_indirect."""
    seen, work, callees = set(), [entry], set()
    hw = has_indirect = False
    while work:
        pc = work.pop()
        while True:
            if pc in seen or not (0xA000 <= pc < 0x10000):
                break
            op = mem[pc]
            info = OPS.get(op)
            if info is None:
                break
            mn, md = info
            ln = MODE_LEN[md]
            seen.add(pc)
            if md in (ABS, ABX, ABY):
                a = mem[pc + 1] | (mem[pc + 2] << 8)
                if 0x2000 <= a < 0x4020:
                    hw = True
            if md == REL:
                work.append((pc + 2 + ((mem[pc + 1] ^ 0x80) - 0x80)) & 0xFFFF)
            elif op == 0x20:                       # JSR
                callees.add(mem[pc + 1] | (mem[pc + 2] << 8))
            elif op == 0x4C:                       # JMP abs
                t = mem[pc + 1] | (mem[pc + 2] << 8)
                if t in routine_entries:           # tail-call into another routine
                    callees.add(t)
                else:                              # local jump (loop / in-routine)
                    work.append(t)
                break
            elif op == 0x6C:                       # JMP (indirect) — control transfer
                has_indirect = True
                break
            if op in TERMINATORS:
                break
            pc += ln
    size = sum(MODE_LEN[OPS[mem[p]][1]] for p in seen)
    return dict(size=size, ninstr=len(seen), callees=callees, hw=hw, has_indirect=has_indirect)


def main():
    mem = build_mem()
    fix, b13 = gather_entries()
    targets = {t for t in all_jsr_targets(mem, fix, b13) if 0xA000 <= t < 0x10000}
    names = symbols.ROUTINES
    rows = []
    for t in sorted(targets):
        r = analyze(mem, t, targets)
        # callees that are themselves real routines (in $A000-$FFFF code)
        deps = {c for c in r["callees"] if 0xA000 <= c < 0x10000}
        rows.append((t, names.get(t, ""), r["size"], r["ninstr"],
                     len(deps), r["hw"], not deps and not r["hw"]))
    rows.sort(key=lambda x: (not x[6], x[2]))  # easy (leaf+no-hw) first, then size

    out = ROOT / "build" / "port_worklist.tsv"
    out.parent.mkdir(exist_ok=True)
    with out.open("w") as f:
        f.write("addr\tname\tsize\tinstr\tdeps\thw\teasy_leaf\n")
        for t, nm, sz, ni, nd, hw, easy in rows:
            f.write(f"{t:04X}\t{nm}\t{sz}\t{ni}\t{nd}\t{int(hw)}\t{int(easy)}\n")

    n = len(rows)
    easy = [r for r in rows if r[6]]
    hw = [r for r in rows if r[5]]
    leaves = [r for r in rows if r[4] == 0]
    tiny = [r for r in rows if r[2] <= 32]
    print(f"total called routines analysed: {n}  -> {out}")
    print(f"  easy (leaf, no hardware):    {len(easy)}  (avg {sum(r[2] for r in easy)//max(len(easy),1)}B)")
    print(f"  pure leaves (no sub-calls):  {len(leaves)}")
    print(f"  touch hardware (PPU/APU/JOY):{len(hw)}")
    print(f"  tiny (<=32 bytes):           {len(tiny)}")
    print(f"  size buckets: <=16B={sum(1 for r in rows if r[2]<=16)}, "
          f"17-64B={sum(1 for r in rows if 16<r[2]<=64)}, "
          f"65-256B={sum(1 for r in rows if 64<r[2]<=256)}, "
          f">256B={sum(1 for r in rows if r[2]>256)}")
    print("\n  10 easiest unported leaves:")
    for t, nm, sz, ni, nd, hwf, easyf in [r for r in rows if r[6]][:10]:
        print(f"    ${t:04X} {nm or '(unnamed)':24} {sz}B {ni} instr")


if __name__ == "__main__":
    main()
