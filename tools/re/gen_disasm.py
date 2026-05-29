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
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ROM = ROOT / "rom" / "lotw.nes"
OUT = ROOT / "disasm"

HEADER_LEN = 0x10
PRG_BASE = 0x10
PRG_LEN = 0x20000        # 128 KiB
BANK_LEN = 0x2000        # 8 KiB
N_BANKS = 16
CHR_BASE = PRG_BASE + PRG_LEN
CHR_LEN = 0x10000        # 64 KiB

# Runtime CPU origin per PRG bank. Banks 0-13 are swappable; under the captured
# coverage the code banks page in at $8000, and data banks' origin is irrelevant
# to the byte output. Banks 14/15 are fixed at $C000/$E000.
BANK_ORIGIN = {n: 0x8000 for n in range(14)}
BANK_ORIGIN[14] = 0xC000
BANK_ORIGIN[15] = 0xE000


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

    # --- header.s ---
    hdr = rom[0:HEADER_LEN]
    (OUT / "header.s").write_text(
        '; iNES header (mapper 4 / MMC3, 128K PRG, 64K CHR)\n'
        '.segment "HEADER"\n' + emit_bytes(hdr) + "\n"
    )

    # --- bankNN.s (all .byte for now) ---
    for n in range(N_BANKS):
        start = PRG_BASE + n * BANK_LEN
        data = rom[start:start + BANK_LEN]
        origin = BANK_ORIGIN[n]
        kind = "FIXED" if n >= 14 else "swappable"
        (OUT / f"bank{n:02d}.s").write_text(
            f"; PRG bank {n} ({kind}) — file 0x{start:05X}..0x{start+BANK_LEN:05X}, "
            f"CPU origin ${origin:04X}\n"
            f'.segment "CODE{n:02d}"\n' + emit_bytes(data) + "\n"
        )

    # --- chr.s ---
    chr_data = rom[CHR_BASE:CHR_BASE + CHR_LEN]
    (OUT / "chr.s").write_text(
        '; CHR-ROM — 64 KiB, 4096 tiles, 64 x 1 KiB MMC3 banks\n'
        '.segment "CHRROM"\n' + emit_bytes(chr_data) + "\n"
    )

    # --- lotw.cfg ---
    mem = ['    HDR:   start=$0000, size=$0010, file=%O, fill=yes;']
    for n in range(N_BANKS):
        mem.append(f'    PRG{n:02d}: start=${BANK_ORIGIN[n]:04X}, size=$2000, file=%O, fill=yes;')
    mem.append('    CHR:   start=$0000, size=$10000, file=%O, fill=yes;')
    seg = ['    HEADER: load=HDR,   type=ro;']
    for n in range(N_BANKS):
        seg.append(f'    CODE{n:02d}: load=PRG{n:02d}, type=ro;')
    seg.append('    CHRROM: load=CHR,   type=ro;')
    (OUT / "lotw.cfg").write_text(
        "# ld65 config for the Legacy of the Wizard matching disassembly.\n"
        "# MEMORY regions are written to %O in declaration order, reproducing the\n"
        "# iNES file layout: header + PRG banks 0..15 + CHR.\n"
        "MEMORY {\n" + "\n".join(mem) + "\n}\n\n"
        "SEGMENTS {\n" + "\n".join(seg) + "\n}\n"
    )

    # --- Makefile ---
    objs = "header.o " + " ".join(f"bank{n:02d}.o" for n in range(N_BANKS)) + " chr.o"
    (OUT / "Makefile").write_text(
        "# Matching-disassembly build. Run inside `nix develop`.\n"
        "AS = ca65\nLD = ld65\nCFG = lotw.cfg\n"
        f"OBJS = {objs}\n"
        "ROM = ../rom/lotw.nes\nOUT = build/lotw.nes\n\n"
        ".PHONY: all verify clean\n\n"
        "all: $(OUT)\n\n"
        "build:\n\tmkdir -p build\n\n"
        "%.o: %.s\n\t$(AS) -o $@ $<\n\n"
        "$(OUT): $(OBJS) $(CFG) | build\n\t$(LD) -C $(CFG) -o $(OUT) $(OBJS)\n\n"
        "verify: $(OUT)\n"
        "\t@a=$$(sha256sum $(OUT) | cut -d' ' -f1); \\\n"
        "\t b=$$(sha256sum $(ROM) | cut -d' ' -f1); \\\n"
        "\t if [ \"$$a\" = \"$$b\" ]; then echo \"MATCH: $$a\"; \\\n"
        "\t else echo \"MISMATCH\"; echo \" built: $$a\"; echo \" rom:   $$b\"; \\\n"
        "\t   cmp $(OUT) $(ROM) || true; exit 1; fi\n\n"
        "clean:\n\trm -f *.o $(OUT)\n"
    )
    print(f"generated {OUT}/ : header.s, bank00..15.s, chr.s, lotw.cfg, Makefile")


if __name__ == "__main__":
    main()
