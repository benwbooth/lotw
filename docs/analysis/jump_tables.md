# Jump tables / indirect-jump dispatch (banks 13/14/15)

Charter: enumerate every jump table / indirect-jump dispatch in the CODE banks and list all
target addresses (each target is a CODE entry).

## Method

- Grepped both `.s` files for `JMP ($..)` indirect jumps. There are exactly 7 sites, all in
  `bankfix.s`; bank13 has none.
- For each indirect jump, traced the zero-page pointer back to the `LDA tbl,X / STA $zp` /
  `LDA tbl+1,X / STA $zp+1` load idiom to find the table base.
- Cross-checked by scanning every `LDA <addr>,X -> STA $06/$0C/$0E` pair (the ZP pointers used
  by the indirect jumps). All indexed loads feeding a jump pointer map to one of the tables below.
- Scanned for RTS-trampoline dispatch (`LDA tbl,X; PHA; LDA tbl,X; PHA; RTS`): none found.
- Scanned all `.byte` runs for little-endian address pairs into $A000-$FFFF: the only true
  pointer tables are the 5 jump tables; the other matches are ASCII+$A0 text, $F8 sprite fill,
  and 4-byte OAM records (Y,tile,attr,X) in bank13 — all DATA.

ROM mapping: fixed PRG base = 0x1C010, CPU $C000 -> file = 0x1C010 + (addr-$C000).

## The 2 indirect JMPs that are NOT table dispatch

- `JMP ($000E)` at $C7E5 (bankfix.s:1805) and the twin at the seed variant (1836): these are the
  far-call bank-switch trampolines (`farcall_bank_0C0D`). The pointer $0E/$0F is supplied by the
  caller as an immediate far-call target, not loaded from a table. All far-call sites use
  `LDA #lo; STA $0E; LDA #hi; STA $0F` (no indexed table feeds $0E here), so there is no hidden
  pointer table — those targets are already covered by far-call xref extraction.

Note: `LDA $8000,X / STA $0E` at bankfix.s:7819 (CPU ~$FC37) is a *data* copy loop (reads 8-byte
records via ($0E),Y into a buffer), not a jump; $8000 is a data table in the swappable bank.

## The 6 jump tables (DATA) and their targets (CODE)

### 1. NMI VRAM-job dispatch — `nmi_vram_dispatch_table`
- Table: CPU $D244..$D251 (14 bytes, 7 LE entries). PRG 0x1D254..0x1D261.
- Index site: bankfix.s:2634-2646. `LDA nmi_vram_req` (0..6) `ASL; TAX`; `LDA $D244,X / LDA $D245,X`
  -> $06/$07; `JMP ($0006)`.
- Targets: D351, D252, D25F, D290, D2E5, D334, D344.
  (D25F = vram_upload_palette, D290 = vram_upload_hud; D252/D334/D344/D351/D2E5 are still inside
  the `.byte` run at bankfix.s:2647-2648 = genuinely missed code.)

### 2. Item/action dispatch A — table at $DB06
- Table: CPU $DB06..$DB15 (16 bytes, 8 LE entries). PRG 0x1DB16..0x1DB25.
- Index site: bankfix.s:3848-3855 (L_DAF2). value (0..7) `ASL; TAX`; `LDA $DB06,X / $DB07,X` -> $0C/$0D;
  `JMP ($000C)`.
- Targets: DB26, DB31, DB3C, DB52, DB5D, DB71, DBB7, DB85.

### 3. Item/action dispatch B — table at $DB16
- Table: CPU $DB16..$DB25 (16 bytes, 8 LE entries). PRG 0x1DB26..0x1DB35.
- Index site: bankfix.s:3795-3804 (L_DA97). value (0..7, values >=8 fall through to a different
  path) `ASL; TAX`; `LDA $DB16,X / $DB17,X` -> $0C/$0D; `JMP ($000C)`.
- Targets: D16A, D199, DB47, DB52, DB66, DB7B, DBB7, DB9B.
  (D16A/D199 already decoded as L_D16A/L_D199; the DBxx targets are handler code following the
  tables, inside the `.byte` run bankfix.s:3860-3873 = missed code.)

### 4. Boss state machine — table at $EAAD
- Table: CPU $EAAD..$EABE (18 bytes, 9 LE entries). PRG 0x1EABD..0x1EACE.
- Index site: bankfix.s:5901-5914 (L_EA94). `LDY #$08; LDA ($E7),Y` (state, clamped <9 else 0)
  `ASL; TAX`; `LDA $EAAD,X / $EAAE,X` -> $0E/$0F; `JMP ($000E)`.
- Targets: EAFD, EB69, EB90, EBD8, EC76, ECA8, ED2A, ED6F, ED9F.

### 5. Sound command dispatch — table at $FBBB
- Table: CPU $FBBB..$FBC4 (10 bytes, 5 LE entries). PRG 0x1FBCB..0x1FBD4.
- Index site: bankfix.s:7741-7750 (L_FBA8). command (0..4, `CMP #$05 BCC`) `ASL; TAX`;
  `LDA $FBBB,X / $FBBC,X` -> $06/$07; `JMP ($0006)`.
- Targets: FBC5, FBE2, FBFF, FC02, FC05.

## Data confirmed NOT code (incidental address-pair matches)

- bankfix.s:8062-8070 ($FCxx region): HUD/menu text (ASCII+$A0) and $FC/$FD/$FE/$F8 sprite-tile fill.
- bank13.s:1120-1131: 4-byte OAM metasprite records (Y, tile, attr, X), e.g. $58,$51,$03,$A0.
- bank13.s:2361-2362: ASCII+$A0 text.
- bank13.s:2684-2689: $F8 sprite/blank fill.
