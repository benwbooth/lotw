# Verification: PRG banks 0-12 contain NO executable code

Date: 2026-05-28

## Claim under test
PRG banks 0-12 are pure DATA (room maps, metatiles, music, title nametable);
all executable code lives in banks 13/14/15.

## ROM layout (verified)
iNES header `4e45531a 0808...`: 8x16KB PRG = 128KB = 16 banks of 8KB (banks 0-15).
- bank N file offset = `0x10 + N*0x2000`.
- banks 0-12 -> file 0x00010 .. 0x19FFF
- bank 13 (swappable, runs at $A000) -> file 0x1A010 (matches charter)
- fixed banks 14/15 -> file 0x1C010 (matches charter)

MMC3 maps the swappable banks into the $8000 and $A000 CPU windows. Therefore if
banks 0-12 were ever executed, code/the disassembly would show a JSR/JMP/branch
target inside $8000-$9FFF (or $A000-$BFFF when one of these banks is swapped in).

## Evidence 1 - reference (xref) analysis
Grepping disasm/bank13.s + disasm/bankfix.s for control-flow targets in the
$8000-$9FFF window:

    JSR/JMP into $8000-$9FFF : NONE
    branch into $8000-$9FFF  : NONE

The ONLY references into that window are indexed table reads (DATA access):

    bank13.s:2570  LDA $9B9F,X
    bank13.s:2599  LDA $9EC9,X
    bank13.s:2605  LDA $9FC9,X
    bankfix.s:2582 LDA $9B9F,X
    bankfix.s:2588 LDA $9C9E,X
    bankfix.s:2600 LDA $9D3E,X
    bankfix.s:2606 LDA $9DC9,X
    bankfix.s:7587 LDA $8014,X
    bankfix.s:7589 LDA $8015,X

All are `LDA tbl,X` -> the data banks are read as tables, never called/jumped to.

## Evidence 2 - automated code-signature scan
A 6502 linear decoder scanned every byte offset of each bank 0-12 looking for a
genuine subroutine signature: >=8 consecutive legal opcodes, containing at least
one JSR, all branch targets in-range, terminating in RTS/RTI/JMP within 40 insns.

Result per bank (candidate coherent JSR+TERM runs):

    bank 0:0  1:0  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:1  10:0  11:0  12:0

Exactly ONE candidate, in bank 9 at file 0x128AD (CPU $A89D if mapped). It is a
FALSE POSITIVE:

    A89D: JSR $2021     <- target is RAM/zero-page area, not a valid code address
    A8A0: PHA / CLI / EOR $59
    A8A4: DEC $CECE   x8 (a run of 0xCE data bytes)
    ...   ORA $01 / AND $10 ...   RTS

The surrounding bytes are clearly structured graphics/nametable data with regular
nibble-paired patterns:

    12880: 0192bc93 8292bd01 2a3a2b3b 2a3a493b
    12890: 483a4959 2a3a4959 483a493b 2a3a5a59
    128a0: 483a4a3b 4c5c4dc2 62726378 21202120
    128b0: 48584959 cececece cececece cececece   <- 'DEC $CECE' run = data
    128d0: 31103110 cececece 8c616010 619c1070
    128f0: cececece 4c5c4d5d 66766777 8c9c6070

Patterns like 2a3a2b3b / 4c5c4d5d / 8c9c8d9d / 667667778c9c are tile-arrangement
(metatile / nametable) data, not instructions. The JSR $2021 + DEC $CECE x8
coincidence is what tripped the heuristic.

## Evidence 3 - byte composition
Decode-sampling each bank shows the characteristic DATA fingerprints throughout:
long runs of a single repeated byte (b6b6b6.., bcbcbc.., 7c7c7c.., 7d7d7d..,
fcfcfc.., 000000..) = metatile/attribute fills, plus per-bank pointer-table-like
structures (e.g. the `..1e01 5d02..` / `..2301 6d02..` patterns near offset +0x333
in banks 0-7, consistent with room-layout headers). None decode as coherent code.

## Conclusion
Banks 0-12 are PURE DATA. No executable code found.
- No control-flow (JSR/JMP/branch) target lands in the $8000-$9FFF window.
- The sole code-shaped run (bank 9 @0x128AD) is a coincidental decode inside
  graphics data (JSR to RAM + a run of 0xCE bytes).
- All cross-references into these banks are indexed table reads.

code_entries: EMPTY (no missed code in banks 0-12).
