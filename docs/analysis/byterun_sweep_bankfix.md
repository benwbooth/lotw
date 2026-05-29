# Full `.byte` run sweep — bankfix.s (fixed banks 14+15, CPU $C000-$FFFF)

Charter: classify EVERY `.byte` run in `disasm/bankfix.s` as CODE (missed by static descent) or DATA.

## Method
- Re-derived the exact CPU address of every `.byte` run by assembling the file (resolving
  zero-page symbols from `lotw.inc` so operand lengths are exact). Validated: all 876 `L_XXXX`
  labels land on their named address and the segment ends exactly at $10000 (zero drift).
- Decoded every run with a hand-written 6502 disassembler.
- For each candidate entry, searched the ROM (`rom/lotw.nes`, fixed PRG base 0x1C010) for a
  `JSR`/`JMP`/branch to it and for a little-endian pointer to it (jump/pointer tables), and
  grepped both `.s` files. ROM map: CPU $C000 -> file 0x1C010 + (addr-$C000).

There are 18 contiguous `.byte` blocks. Summary table (CPU / PRG / verdict):

| Block            | PRG range          | Verdict |
|------------------|--------------------|---------|
| $C034-$C040      | 0x1C044-0x1C050    | DATA / dead fragment (unreferenced) |
| $C4B4-$C51F      | 0x1C4C4-0x1C52F    | CODE (2 unreferenced fade routines) |
| $D244-$D251      | 0x1D254-0x1D261    | DATA — nmi_vram_dispatch_table |
| $D252-$D25E      | 0x1D262-0x1D26E    | CODE — VRAM job handler (table target) |
| $D5F2            | 0x1D602            | DATA / stray RTS (unreferenced) |
| $DB06-$DB25      | 0x1DB16-0x1DB35    | DATA — 2 overlapping jump tables |
| $DB26-$DBDC      | 0x1DB36-0x1DBEC    | CODE — item/action handlers (table targets) |
| $E800-$E841      | 0x1E810-0x1E851    | CODE — sound add/clamp routines |
| $E852-$E86E      | 0x1E862-0x1E87E    | CODE — sound add/clamp routines |
| $EAAD-$EABE      | 0x1EABD-0x1EACE    | DATA — boss-state jump table |
| $EAFD-$EE18      | 0x1EB0D-0x1EE28    | CODE — boss/music state machine (huge) |
| $EE53-$EEB2      | 0x1EE63-0x1EEC2    | CODE — sound helpers |
| $EEB3-$EEBA      | 0x1EEC3-0x1EECA    | DATA — 8-byte lookup (LDA $EEB3,X) |
| $EFE7-$EFEF      | 0x1EFF7-0x1EFFF    | DATA — 9-byte lookup (LDX $EFE7,Y) |
| $F01E-$F032      | 0x1F02E-0x1F042    | CODE — phase dispatch + inner table |
| $F033-$F03A      | 0x1F043-0x1F04A    | DATA — 4-entry jump table (JMP ($0E)) |
| $F03B-$F0E0      | 0x1F04B-0x1F0F0    | CODE — phase handlers (table targets) |
| $F11B-$F135      | 0x1F12B-0x1F145    | CODE — sound subroutine |
| $F841-$F845      | 0x1F851-0x1F855    | CODE (unreferenced; `INC $F3; JMP $F896`) |
| $FBBB-$FBC4      | 0x1FBCB-0x1FBD4    | DATA — sound-command jump table |
| $FBFF-$FC01      | 0x1FC0F-0x1FC11    | CODE — sound handler (table target $FBFF) |
| $FDB1-$FFDF      | 0x1FDC1-0x1FFEF    | DATA — note_period_table + HUD text + OAM |
| $FFEF-$FFF9      | 0x1FFFF-0x20009    | DATA — zero padding |
| $FFFA-$FFFF      | 0x2000A-0x2000F    | DATA — CPU vectors |

## Why the missed code was missed
The bulk of it is the **sound engine and the boss/effect state machines**. These bodies are
reachable ONLY through indexed jump tables (`LDA tbl,X / STA $zp / JMP ($zp)`) and through cross
calls *from inside other missed bodies*; recursive descent that does not follow indirect jumps
emits the whole interconnected region as `.byte`. Once the table targets (block heads) below are
seeded, ordinary descent recovers every internal label.

## Genuine DATA (NOT code)

- **$D244-$D251** `nmi_vram_dispatch_table` — 7 LE pointers, indexed at bankfix.s:2634-2646
  (`LDA $D244,X / LDA $D245,X / JMP ($0006)`). Already named in lotw.inc.
- **$DB06-$DB25** two overlapping jump tables ($DB06 read at L_DAF2:3848-3855, $DB16 read at
  L_DA97:3795-3804). 16 LE pointers; see jump_tables.md sections 2-3.
- **$EAAD-$EABE** boss-state jump table — 9 LE pointers, indexed at L_EA94:5901-5914
  (`LDA $EAAD,X / JMP ($000E)`). Targets EAFD,EB69,EB90,EBD8,EC76,ECA8,ED2A,ED6F,ED9F.
- **$EEB3-$EEBA** = `01 05 04 06 02 0A 08 09` — read by `LDA $EEB3,X` at $EEA0 and $EEAD
  (inside the $EE53 code body). Octave/duty lookup.
- **$EFE7-$EFEF** = `03 03 03 03 04 04 05 06 07` — read by `LDX $EFE7,Y` (opcode $BE) at
  PRG 0x1EFA6 = CPU $EF96. Lookup table.
- **$F033-$F03A** = LE pointers $F03B,$F04B,$F071,$F0B9 — inner jump table indexed at
  $F01E-$F032 (`LDA $F033,X / LDA $F034,X / JMP ($000E)`).
- **$FBBB-$FBC4** sound-command jump table — 5 LE pointers (FBC5,FBE2,FBFF,FC02,FC05), indexed at
  L_FBA8:7741-7750. See jump_tables.md section 5.
- **$FDB1-$FFDF** sound/graphics data: `note_period_table` at $FDB1 (read `LDA $FDB1,X` /
  `LDA $FDB2,X` at CPU $FC7D, PRG 0x1FC8D); sound-instrument descriptor records; HUD text
  ASCII+$A0 at ~$FEE1-$FF5x ("LIFE","MAGIC","KEY","GOLD","ITEM"); OAM/sprite metasprite records
  (4-byte Y,tile,attr,X) at ~$FF61-$FFDF. None decode as coherent code; all read by index.
- **$FFEF-$FFF9** eleven `$00` padding bytes after `reset:` (`JMP main_init`).
- **$FFFA-$FFFF** CPU vectors: NMI=$D1FE, RESET=$FFE0, IRQ=$D1FE.
- **$C034-$C040** dead fragment after `JMP L_C041`: byte `$80` (illegal) + an MMC3 bank-write
  snippet (`LDA #$07; STA $25; STA $8000; LDA #$0D; STA $8001`) with NO entry point anywhere in
  ROM. Leftover/unused; treat as data.
- **$D5F2** single `$60` (RTS) between `JMP L_D866` and `L_D5F3`; no JSR/JMP/branch reaches it.
  Stray byte.

## MISSED CODE — entry points to seed
Each is reached as shown. Internal labels are recovered by descent from these heads.

- **$D252** — handler reached via `nmi_vram_dispatch_table` (entry index for $D252). Decodes
  `LDX $1A; LDA $18; STA PPUDATA; DEX; BNE; JMP $D351` (RTS-equivalent via JMP). The other table
  targets (D25F=vram_upload_palette, D290, D2E5, D334, D344, D351) are already decoded; $D252 is
  the only one left inside the `.byte` run.
- **$DB26,$DB31,$DB3C,$DB47,$DB52,$DB5D,$DB66,$DB71,$DB7B,$DB85,$DB9B,$DBB7** — the 12 distinct
  handler entries of the two jump tables at $DB06/$DB16. All decode as short
  `LDA #imm; STA $8F; ... ; (JSR|RTS)` item/action handlers; e.g. $DB2D `JSR $E800`,
  $DB38 `JSR $E816`. None were previously code-starts.
- **$E800,$E816,$E82C** — three near-identical "add-to-stat-with-clamp" routines (health/magic/
  gold). Called by the $DBxx handlers (`JSR $E800` @ $DB2D, `JSR $E816` @ $DB38,
  `JSR $E82C` @ $DB43/$DB4E). End in RTS.
- **$E852,$E859** — key add/increment routines, called `JSR $E852` @ $DB62, `JSR $E859` @ $DB6D.
- **$EAFD** — head of the boss/music state machine; target #0 of the $EAAD boss-state table.
  ~796 bytes of coherent code ($EAFD-$EE18) cross-calling rng_update ($CC64), $CD70, etc. The
  other table targets EB69,EB90,EBD8,EC76,ECA8,ED2A,ED6F,ED9F sit inside this same run and are
  the remaining boss states.
- **$EE53** — sub reached `JSR $EC82` (inside the $EAFD body). Decodes a coordinate-delta calc
  ending in RTS.
- **$EE8D,$EE9A,$EEA6** — sound helpers; `JSR $EE8D` @ $EB96, `JSR $EE9A` @ $EB6F,
  `JSR $EEA6` @ $EB11.
- **$F01E** — phase-dispatch routine; `JSR $F01E` @ $EB60/$EB8A/$EBD2/$EC70. Reads inner table
  $F033 and `JMP ($000E)` to the four handlers below.
- **$F03B,$F04B,$F071,$F0B9** — the four $F033-table handlers (envelope/pitch phases), all inside
  $F03B-$F0E0; each ends in RTS.
- **$F11B** — sound subroutine; `JSR $F11B` @ $EB7C. Decodes `JSR $EFF1; JSR $CE7C; ...; RTS`.
- **$FBFF** — sound-command handler `STA $99,X; RTS`; target #2 of the $FBBB command table
  (LE pointer at PRG 0x1FBBF).
- **$C4B4, $C4E0** — two complete RTS-terminated screen-fade/scroll routines in the same family
  as L_C492/L_C496 (which IS called by `JSR $C492` from ~12 sites). They call L_C520, L_C135,
  L_C569 and loop coherently. NOTE: neither has any JSR/JMP/branch or pointer reference anywhere
  in the ROM, so they are most likely **dead/unused** variants — flagged medium/low confidence.
- **$F841** — `INC $F3; JMP $F896` between `JMP L_F896` and `L_F846`. Coherent but with NO
  reference in ROM; probable dead code. Low confidence.

## Cross-reference
This sweep is consistent with `docs/analysis/jump_tables.md` (the 6 fixed-bank jump tables) and
extends it with the non-table missed code: the $E8xx/$EE/$F0/$F1 sound-engine subroutines that are
called only from inside other missed bodies.
