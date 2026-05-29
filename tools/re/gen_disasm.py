#!/usr/bin/env python3
"""Generate the ca65/ld65 matching-disassembly skeleton for lotw.nes.

Stage-1 keystone: lay out the real bank-segmented structure (iNES header + 16
PRG banks at their runtime CPU origins + CHR) but emit every byte as `.byte`
data. This is the trivially-matching baseline whose `assemble -> link` output is
byte-identical to the ROM. Code regions get progressively disassembled into real
instructions later, re-checking the sha256 gate at every step.

Usage (inside `nix develop`):
    python3 tools/re/gen_disasm.py          # (re)generate disasm/ sources + cfg
    make -C disasm verify                   # assemble, link, compare sha256

Outputs into disasm/:
    lotw.cfg            ld65 linker config
    header.s            iNES header segment
    bank00.s .. bank15.s  one segment per 8 KiB PRG bank
    chr.s               CHR-ROM data segment
    Makefile
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from disasm6502 import disassemble_bank
import symbols

ROOT = Path(__file__).resolve().parents[2]
ROM = ROOT / "rom" / "lotw.nes"
OUT = ROOT / "disasm"
COVERAGE = ROOT / "build" / "coverage" / "merged_coverage.tsv"
ANALYSIS_SYMS = ROOT / "build" / "analysis_symbols.json"  # optional, from agents
INCLUDE = '.include "lotw.inc"\n'

HEADER_LEN = 0x10
PRG_BASE = 0x10
PRG_LEN = 0x20000        # 128 KiB
BANK_LEN = 0x2000        # 8 KiB
N_BANKS = 16
CHR_BASE = PRG_BASE + PRG_LEN
CHR_LEN = 0x10000        # 64 KiB

# Swappable banks 0-13 page into $8000 (the window the code banks use; data
# banks' origin is irrelevant to byte output). Banks 14+15 are BOTH fixed and
# adjacent ($C000-$DFFF and $E000-$FFFF), so they form one contiguous 16 KiB
# code window $C000-$FFFF — disassembled as a single unit so branches/labels
# that cross the $E000 boundary resolve in-range.
SWAP_ORIGIN = 0x8000
N_SWAP = 14
FIX_ORIGIN = 0xC000
FIX_BANKS = (14, 15)

# High-confidence code entry anchors (CPU addresses), from docs/recon/SUMMARY.md.
FIX_ANCHORS = {0xC000, 0xD1FE, 0xD41D, 0xD36E, 0xCC9C, 0xCD08, 0xC833, 0xCCE4,
               0xFFE0, 0xF89A, 0xFC08}


def emit_bytes(data: bytes, indent: str = "    ") -> str:
    lines = []
    for i in range(0, len(data), 16):
        chunk = data[i:i + 16]
        lines.append(indent + ".byte " + ",".join(f"${b:02X}" for b in chunk))
    return "\n".join(lines)


def main():
    rom = ROM.read_bytes()
    assert len(rom) == CHR_BASE + CHR_LEN, f"unexpected ROM size {len(rom)}"
    OUT.mkdir(exist_ok=True)

    symbols.load_extra(ANALYSIS_SYMS)
    names = symbols.REGS_RAM
    routines = symbols.ROUTINES

    # --- lotw.inc (address constants; including it changes no bytes) ---
    inc = ["; Symbol address constants for the matching disassembly.",
           "; Generated from tools/re/symbols.py — including this changes no bytes."]
    for addr, name in sorted(names.items()):
        inc.append(f"{name} = ${addr:04X}")
    (OUT / "lotw.inc").write_text("\n".join(inc) + "\n")

    # --- header.s ---
    hdr = rom[0:HEADER_LEN]
    (OUT / "header.s").write_text(
        '; iNES header (mapper 4 / MMC3, 128K PRG, 64K CHR)\n'
        '.segment "HEADER"\n' + emit_bytes(hdr) + "\n"
    )

    # --- coverage-derived code entries (use observed cpu_addr for true window) ---
    obs: dict[int, list[int]] = {n: [] for n in range(N_SWAP)}  # bank -> cpu_addrs
    fix_entries: set[int] = set(FIX_ANCHORS)
    if COVERAGE.exists():
        for line in COVERAGE.read_text().splitlines()[1:]:
            cols = line.split("\t")
            off, cpu = int(cols[0], 16), int(cols[2], 16)
            bank = off // BANK_LEN
            if bank < N_SWAP:
                obs[bank].append(cpu)
            else:
                fix_entries.add(cpu)  # cpu is already $C000-$FFFF
    # Pick each swappable code bank's window from the majority observed cpu addr.
    swap_origin: dict[int, int] = {}
    swap_entries: dict[int, set[int]] = {}
    for n in range(N_SWAP):
        cpus = obs[n]
        hi = sum(1 for c in cpus if c >= 0xA000)
        origin = 0xA000 if hi * 2 > len(cpus) else 0x8000
        swap_origin[n] = origin
        swap_entries[n] = {c for c in cpus if origin <= c < origin + BANK_LEN}

    # Bank-switch far-call dispatchers: resolve cross-bank code entries (this is
    # how bank 12 code, never hit by coverage, is reached). Scheme 0C0D forces
    # R6=$0C ($8000=bank12) / R7=$0D ($A000=bank13).
    DISPATCHERS = {0xCC9C: "0C0D", 0xCD08: "0C0D"}
    fstart = PRG_BASE + FIX_BANKS[0] * BANK_LEN
    fdata = rom[fstart:fstart + len(FIX_BANKS) * BANK_LEN]

    def disasm_fix():
        return disassemble_bank(fdata, FIX_ORIGIN, "FIX", fix_entries,
                                force_labels=FIX_ANCHORS | set(routines),
                                names=names, label_names=routines, dispatchers=DISPATCHERS)

    def disasm_swap(n):
        data = rom[PRG_BASE + n * BANK_LEN: PRG_BASE + (n + 1) * BANK_LEN]
        return disassemble_bank(data, swap_origin[n], f"{n:02d}", swap_entries[n],
                                force_labels=set(routines),
                                names=names, label_names=routines, dispatchers=DISPATCHERS)

    # --- fixpoint: disassemble, harvest far-call targets, feed back as entries ---
    rf = None
    swap_results: dict[int, dict] = {}
    ambiguous_8000: set[int] = set()
    for _ in range(16):
        rf = disasm_fix()
        swap_results = {n: disasm_swap(n) for n in range(N_SWAP) if swap_entries[n]}
        added = False
        # (1) dispatcher far-call targets -> bank 12/13
        targets = list(rf["farcall_targets"])
        for r in swap_results.values():
            targets += r["farcall_targets"]
        for bank, cpu in targets:
            origin = 0x8000 if bank == 12 else 0xA000  # 0C0D scheme windows
            swap_origin[bank] = origin
            if cpu not in swap_entries[bank]:
                swap_entries[bank].add(cpu)
                added = True
        # (2) direct cross-window JSR/JMP targets between resident banks:
        #     $C000-$FFFF -> always-mapped fixed unit; $A000-$BFFF -> code bank 13.
        xrefs = set(rf["xrefs"])
        for r in swap_results.values():
            xrefs |= r["xrefs"]
        for t in xrefs:
            if 0xC000 <= t < 0x10000:
                if t not in fix_entries:
                    fix_entries.add(t)
                    added = True
            elif 0xA000 <= t < 0xC000:
                swap_origin[13] = 0xA000
                if t not in swap_entries[13]:
                    swap_entries[13].add(t)
                    added = True
            else:  # $8000-$9FFF: ambiguous bank (R6 runtime-dependent) — record only
                ambiguous_8000.add(t)
        if not added:
            break

    stats = []

    # --- write swappable bank .s (disassembled if it has entries, else data) ---
    for n in range(N_SWAP):
        start = PRG_BASE + n * BANK_LEN
        data = rom[start:start + BANK_LEN]
        if n in swap_results:
            r = swap_results[n]
            origin = swap_origin[n]
            (OUT / f"bank{n:02d}.s").write_text(
                INCLUDE +
                f"; PRG bank {n} (swappable, runs at ${origin:04X}) — "
                f"file 0x{start:05X}..0x{start+BANK_LEN:05X}\n"
                f"; {r['instructions']} instructions, {r['code_bytes']}/{BANK_LEN} code bytes, "
                f"{r['labels']} labels\n" + r["text"])
            stats.append((f"{n:02d}", r["instructions"], r["code_bytes"], BANK_LEN))
        else:
            (OUT / f"bank{n:02d}.s").write_text(
                f"; PRG bank {n} (swappable) — file 0x{start:05X}..0x{start+BANK_LEN:05X} "
                f"(data: no code reached)\n"
                f'.segment "CODE{n:02d}"\n' + emit_bytes(data) + "\n")

    # --- fixed banks 14+15 as one $C000-$FFFF code unit ---
    (OUT / "bankfix.s").write_text(
        INCLUDE +
        f"; PRG banks 14+15 (FIXED, contiguous $C000-$FFFF) — file 0x{fstart:05X}..0x{fstart+len(fdata):05X}\n"
        f"; {rf['instructions']} instructions, {rf['code_bytes']}/{len(fdata)} code bytes, "
        f"{rf['labels']} labels\n" + rf["text"])
    stats.append(("FIX", rf["instructions"], rf["code_bytes"], len(fdata)))

    # --- chr.s ---
    chr_data = rom[CHR_BASE:CHR_BASE + CHR_LEN]
    (OUT / "chr.s").write_text(
        '; CHR-ROM — 64 KiB, 4096 tiles, 64 x 1 KiB MMC3 banks\n'
        '.segment "CHRROM"\n' + emit_bytes(chr_data) + "\n"
    )

    # --- lotw.cfg ---
    mem = ['    HDR:   start=$0000, size=$0010, file=%O, fill=yes;']
    for n in range(N_SWAP):
        mem.append(f'    PRG{n:02d}: start=${swap_origin[n]:04X}, size=$2000, file=%O, fill=yes;')
    mem.append(f'    PRGF:  start=${FIX_ORIGIN:04X}, size=$4000, file=%O, fill=yes;')
    mem.append('    CHR:   start=$0000, size=$10000, file=%O, fill=yes;')
    seg = ['    HEADER: load=HDR,   type=ro;']
    for n in range(N_SWAP):
        seg.append(f'    CODE{n:02d}: load=PRG{n:02d}, type=ro;')
    seg.append('    CODEFIX: load=PRGF, type=ro;')
    seg.append('    CHRROM: load=CHR,   type=ro;')
    (OUT / "lotw.cfg").write_text(
        "# ld65 config for the Legacy of the Wizard matching disassembly.\n"
        "# MEMORY regions are written to %O in declaration order, reproducing the\n"
        "# iNES file layout: header + PRG banks 0..13 + fixed 14/15 + CHR.\n"
        "MEMORY {\n" + "\n".join(mem) + "\n}\n\n"
        "SEGMENTS {\n" + "\n".join(seg) + "\n}\n"
    )

    # --- Makefile ---
    objs = ("header.o " + " ".join(f"bank{n:02d}.o" for n in range(N_SWAP))
            + " bankfix.o chr.o")
    (OUT / "Makefile").write_text(
        "# Matching-disassembly build. Run inside `nix develop`.\n"
        "AS = ca65\nLD = ld65\nCFG = lotw.cfg\n"
        f"OBJS = {objs}\n"
        "ROM = ../rom/lotw.nes\nOUT = build/lotw.nes\n\n"
        ".PHONY: all verify clean\n\n"
        "all: $(OUT)\n\n"
        "build:\n\tmkdir -p build\n\n"
        "%.o: %.s lotw.inc\n\t$(AS) -o $@ $<\n\n"
        "$(OUT): $(OBJS) $(CFG) | build\n\t$(LD) -C $(CFG) -o $(OUT) $(OBJS)\n\n"
        "verify: $(OUT)\n"
        "\t@a=$$(sha256sum $(OUT) | cut -d' ' -f1); \\\n"
        "\t b=$$(sha256sum $(ROM) | cut -d' ' -f1); \\\n"
        "\t if [ \"$$a\" = \"$$b\" ]; then echo \"MATCH: $$a\"; \\\n"
        "\t else echo \"MISMATCH\"; echo \" built: $$a\"; echo \" rom:   $$b\"; \\\n"
        "\t   cmp $(OUT) $(ROM) || true; exit 1; fi\n\n"
        "clean:\n\trm -f *.o $(OUT)\n"
    )
    print(f"generated {OUT}/ : header.s, bank00..13.s, bankfix.s, chr.s, lotw.cfg, Makefile")
    if stats:
        print("disassembled code units (unit: instructions, code bytes):")
        for name, ins, cb, total in stats:
            print(f"  {name}: {ins} instr, {cb}/{total} code bytes ({100*cb/total:.1f}%)")
    if ambiguous_8000:
        print(f"ambiguous $8000-window call targets (runtime bank unknown): "
              f"{len(ambiguous_8000)} -> {sorted(hex(a) for a in ambiguous_8000)[:8]}")


if __name__ == "__main__":
    main()
