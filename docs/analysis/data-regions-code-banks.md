# Data regions embedded in the code banks (13 / 14 / 15)

Goal: catalogue the genuine DATA bytes that live inside the disassembled code banks so
they are not mistaken for code, and flag the `.byte` runs that are actually MISSED CODE
(jump-table targets that recursive descent never followed).

Method: reproduced the disassembler's recursive-descent `starts` set
(`tools/re/disasm6502.py`, same entries/fixpoint as `gen_disasm.py`) for the fixed unit
($C000-$FFFF, file `0x1C010..0x20010`) and bank 13 ($A000-$BFFF, file `0x1A010..0x1C010`),
then took the complement (= every `.byte` byte) and classified each gap by (a) decoding it
as 6502 and (b) grepping the disasm + ROM for who references its start address.

CPU<->file: fixed unit `file = 0x1C010 + (cpu-0xC000)`; bank 13 `file = 0x1A010 + (cpu-0xA000)`.

---

## Confirmed DATA regions

### Fixed unit (banks 14/15, $C000-$FFFF)

| CPU | file | kind | name | evidence |
|---|---|---|---|---|
| `$D244-$D251` | `0x1D254` | jump_table | `nmi_vram_dispatch_table` | 7 LE words ($D351 $D252 $D25F $D290 $D2E5 $D334 $D344); read by `LDA nmi_vram_dispatch_table,X` (bankfix.s:2634). $D252 is the first code after it. |
| `$DB06-$DB25` | `0x1D716` | jump_table | sfx/sound-cue dispatch | 16 LE words; dispatched by `LDA $DB06,X / LDA $DB07,X / JMP ($000C)` (bankfix.s:3851-3855). |
| `$EAAD-$EABE` | `0x1E6BD` | jump_table | object/boss state dispatch | 9 LE words ($EAFD $EB69 $EB90 $EBD8 $EC76 $ECA8 $ED2A $ED6F $ED9F); `LDA $EAAD,X / LDA $EAAE,X / JMP ($000E)` (bankfix.s:5910-5914). |
| `$EEB3-$EEBA` | `0x1EAC3` | other | small byte lookup `01 05 04 06 02 0A 08 09` | read by `LDA $EEB3,X` at $EE9F and $EEAC (inside the EE53 code block). |
| `$EFE7-$EFEF` | `0x1EFF7` | other | `drop_item_table` (9 bytes `03 03 03 03 04 04 05 06 07`) | `LDX drop_item_table,Y` (bankfix.s:6182); item-drop index/probability table. |
| `$F033-$F03A` | `0x1F043` | jump_table | 4 LE words ($F03B $F04B $F071 $F0B9) | preceded by `LDA $F033,X / LDA $F034,X / JMP ($000E)` at $F026 (inside F01E code). |
| `$FBBB-$FBC4` | `0x1FBCB` | jump_table | audio command jump table | 5 LE words ($FBC5 $FBE2 $FBFF $FC02 $FC05); the `$FF cc aa` sound-command escape (recon 06). |
| `$FDB1-$FFDF` | `0x1FDC1` | sound | sound-engine data block | period table + envelope/instrument records + pitch macros + descriptors (see breakdown below). |
| `$FFEF-$FFF9` | `0x1FFFF` | other | padding (zeros) between reset code and vectors | reset code ends `JMP $C000` at $FFEC. |
| `$FFFA-$FFFF` | `0x2000A` | vectors | CPU NMI/RESET/IRQ vectors | NMI=$D1FE, RESET=$FFE0, IRQ=$D1FE. |

Sound-engine data block `$FDB1-$FFDF` breakdown (all DATA; matches docs/recon/06-audio-engine.md):
- `$FDB1-$FDCA` (12 LE words) = `note_period_table` (chromatic octave, base C2; ratio 1.0595).
- `$FDCB-$FE8A` = 12-byte envelope/instrument records (`$FDCB`/`$FDD2` per recon).
- `$FE8B-$FF67` = pitch/vibrato macro sequences (`FB e?`, `FD FC FC`, `FE FC FC` runs).
- `$FF6F-$FFA6` = 4-byte sound descriptor records (`9B 0D 00 64`, `AB 43 40 44`, `BB 63 40 64`...).
- `$FFA7-$FFC3` = 4-byte parameter records (`12 03 01 10`, `14 02 02 18`...) + a few pointer-ish words.
- `$FFC4-$FFD7` = 4-byte `0F`-led records (`0F 0F 2A 36`, `0F 0C 25 36`...; palette/macro-like).
- `$FFD8-$FFDF` = zeros (pad).

### Bank 13 ($A000-$BFFF)

| CPU | file | kind | name | evidence |
|---|---|---|---|---|
| `$A000-$A2E8` | `0x1A010` | sprite | PPU upload graphics/nametable data | streamed `LDA $A0C9,X / STA PPUDATA`, `LDA $A1C9,X / ...`, `LDA $A2C9,X / ...` (bank13.s:2611,2617,2675). 256-byte pages copied to PPUDATA. |
| `$A2E9-$A2EA` | `0x1A2F9` | other | MMC3 R0/R1 bank values | `LDA $A2E9 / STA mmc3_r0_shadow`, `LDA $A2EA / STA mmc3_r1_shadow` (bank13.s:2621-2624). |
| `$AAFC-$ABBB` | `0x1AB0C` | sprite | metasprite / OAM record tables | 4-byte records (`58 51 03 A0`, `64 61 03 A8`...); read by `LDA $AAFC,X`, `LDA $AB3C,X`, `LDA $AB7C,X` (bank13.s:743,751,760). |
| `$B0AC-$B0B0` | `0x1B0BC` | other | small byte table `03 04 05 02 08` | in a data island between routines. |
| `$B0FE-$B101` | `0x1B10E` | other | small byte table `81 84 82 00` | in a data island between routines. |
| `$B4AF-$B4C4` | `0x1B4BF` | text | HUD/menu text (tile = ASCII+$A0) | decodes to `GAME OVER` / `RETRY` / `CONTINUE`. |
| `$B6FC-$B73B` | `0x1B70C` | sprite | metasprite / OAM records | 4-byte records (`80 C1 00 60`, `6E 01 00 68`...). |
| `$B73C-$B79B` | `0x1B74C` | other | `$F8` uniform fill (96 bytes) | blank-tile / off-screen sprite fill. |
| `$B79C-$BD88` | `0x1B7AC` | text | ending / credits text (raw ASCII) | `CREDITS`/`CAST`/`STAFF`, character + monster names, `(C)1987 Falcom`, `(C)1988 Broderbund Software, Inc.`; `0D`=newline, `00`=terminator. NOTE: raw ASCII here, NOT +$A0. |
| `$BD89-$BFA3` | `0x1BD99` | other | zero padding | (matches recon 04's 539-byte zero run at file 0x1BD99). |
| `$BFA4-$BFD3` | `0x1BFB4` | other | two small data records | `44 51 45 50 44 60 47 4C 4C 46 48 49 45 C0 C1 C0` and `61 61 1A 46 45 45 47 4B 45 74 74 45 C0 C1 C0 C0`. |
| `$BFD4-$BFFF` | `0x1BFE4` | other | zero padding (bank end) | |

---

## Missed CODE found incidentally (NOT data)

These `.byte` runs decode as coherent 6502 (sensible opcodes, in-range branches, end at
RTS/JMP) and are reached only through jump tables / cross-bank calls that static descent
did not follow. They should be disassembled, not left as data.

| CPU | file | reached via | notes |
|---|---|---|---|
| `$C035-$C040` | `0x1C045` | dead code after `JMP L_C041` ($C031) | LDA #$07/STA $25/STA $8000/LDA #$0D/STA $8001 (bank-setup fragment). 1st byte $80 at $C034 is a stray operand tail. |
| `$C4B4-$C51F` | `0x1C4C4` | computed/indirect | clean routine, copies via `($77),Y` to $00A0, JSR $C520/$C135, ends RTS at $C51F. |
| `$D252-$D25E` | `0x1D262` | `nmi_vram_dispatch_table` target | `LDX $1A / LDA $18 / STA $2007 / DEX / BNE / JMP $D351`. |
| `$D5F2` | `0x1D602` | fallthrough/target | lone `RTS`. |
| `$DB26-$DBDC` | `0x1D736` | `$DB06` jump table targets | sfx-trigger handlers (set $8F, `JSR $E8xx`, RTS). |
| `$E800-$E841`, `$E852-$E86E` | `0x1E810` | `JSR $E800` at $DB2D etc. | add-with-clamp routines for health/magic/gold/keys ($58-$5B). |
| `$EAFD-$EE18` | `0x1EB0D` | `$EAAD` jump table targets | large object/boss state machine. |
| `$EE53-$EEB2` | `0x1EE63` | jump-table / fallthrough | continuation of the state machine. |
| `$F01E-$F0E0` (minus `$F033-$F03A`) | `0x1F02E` | indirect-jump dispatcher | code with the embedded `$F033` jump table. |
| `$F11B-$F135` | `0x1F12B` | jump-table / call target | clean routine ending RTS. |
| `$F841-$F845` | `0x1F851` | fallthrough into `$F896` | `INC $F3 / JMP $F896`. |
| `$FBFF-$FC01` | `0x1FC0F` | `$FBBB` audio cmd table ($FBFF) | `STA $99,X / RTS` (sound command 2 handler). |

(`$FBC5/$FBE2/$FC02/$FC05` from the audio command table are already decoded as code.)
