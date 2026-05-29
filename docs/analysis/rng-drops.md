# RNG and Enemy-Drop Logic — Legacy of the Wizard

Status: **CONFIRMED against ROM bytes.** Both the RNG update routine and the enemy-drop
chooser were located in the FIXED banks (disassembled in `disasm/bankfix.s`), byte-verified,
and matched against the Data Crystal behavioral description (recon `08-external-research.md`,
section 4). This both names the routines/data and preps the C port.

Offset convention: `file = PRG + 0x10`; fixed banks `$C000-$FFFF` map to PRG `0x1C000-0x1FFFF`
(file `0x1C010-0x1FFFF`). So CPU `$XXXX` (>= `$C000`) -> file `0x1C010 + ($XXXX - $C000)`.

---

## 1. RNG update routine — `rng_update` @ `$CC64`

- CPU `$CC64`  /  PRG `0x1CC64`  /  file `0x1CC74`  /  bank **14** (`$C000-$DFFF`, FIXED).
- `disasm/bankfix.s:1556` (label `L_CC64`).
- Verified bytes at file `0x1CC74`:
  `85 38 F0 26 A6 3B A4 3A 84 39 98 0A A8 8A 2A AA C8 D0 01 E8 18 98 65 3A A8 8A 65 3B 18 65 39 29 7F AA 86 3B 84 3A C5 38 B0 DE 60`

### Calling convention
Caller does `LDA #count` then `JSR $CC64`. On entry `A` = the loop/modulus count which is
stored to `rng_count` ($38). The routine iterates a 16-bit LFSR-style step until the running
7-bit value (re-derived each iteration and `AND #$7F`) is **>= count**; it returns the final
value in `A` (= `rng_s2` = `$3B`). Net effect: returns a pseudo-random value in `0..count-1`
when the byte-exact loop terminates (e.g. `count=$14` -> roll `0..19`; `count=$09` -> `0..8`).

### Decoded
```
$CC64  rng_update:
  85 38       STA  rng_count        ; $38  upper bound / iteration count
  F0 26       BEQ  $CC8E            ; count==0 -> return immediately
  A6 3B       LDX  rng_s2           ; $3B  state high
  A4 3A       LDY  rng_s1           ; $3A  state mid
$CC6C loop:
  84 39       STY  rng_s0           ; $39  state low = old mid
  98          TYA
  0A          ASL  A                ; <<1
  A8          TAY
  8A          TXA
  2A          ROL  A                ; carry from ASL into high
  AA          TAX
  C8          INY
  D0 01       BNE  +
  E8          INX
+ 18          CLC
  98          TYA
  65 3A       ADC  rng_s1           ; + mid
  A8          TAY
  8A          TXA
  65 3B       ADC  rng_s2           ; + high
  18          CLC
  65 39       ADC  rng_s0           ; + low
  29 7F       AND  #$7F             ; <- result AND #$7F (matches Data Crystal)
  AA          TAX
  86 3B       STX  rng_s2           ; -> $3B  (result stored here)
  84 3A       STY  rng_s1           ; -> $3A
  C5 38       CMP  rng_count        ; loop while value >= count
  B0 DE       BCS  $CC6C
$CC8E:
  60          RTS                   ; A = rng_s2 = $3B = roll in [0,count)
```

This is exactly the Data Crystal description: "loop of ASL/ROL/ADC/AND over `$0039-$003B`,
count driven by `$0038`, result `AND #$7F` stored to `$003B`."

### RNG state (zero page) — already symbolized in the disassembly
| Addr | Symbol      | Meaning |
|------|-------------|---------|
| `$38`| `rng_count` | iteration count / exclusive upper bound (set per call) |
| `$39`| `rng_s0`    | RNG state byte 0 (low) |
| `$3A`| `rng_s1`    | RNG state byte 1 (mid) |
| `$3B`| `rng_s2`    | RNG state byte 2 (high) == returned result |

### Usage
`rng_update` is called from ~23 sites: 11 in swappable bank 13 (spawn placement + the
password scramble at PRG `0x1B55B`/`0x1B592`) and 12 in the fixed banks (enemy spawn/movement
type selectors and the drop chooser). The only `count=$14` (roll 0-19) caller is the drop
chooser at `$EF9E` (below).

---

## 2. Enemy-drop chooser — `enemy_drop_choose` @ `$EF85`

- CPU `$EF85`  /  PRG `0x1EF75`  /  file `0x1EF85`  /  bank **15** (`$E000-$FFFF`, FIXED).
- `disasm/bankfix.s:5881` (label `L_EF85`).
- Verified bytes (`$EF85..$EFC4` chooser body, then the 9-byte table at `$EFE7`):
  `A2 00 A5 58 C9 14 90 37 E8 A5 59 C9 1E 90 30 A2 04 A5 5B C9 02 90 28 A9 14 20 64 CC C9 09 B0 07 A8 BE E7 EF 4C C4 EF ...`

The chooser produces an **item-type index in X** (`0..7`), then `$EFC4` (`item_spawn_setup`)
converts that index into a sprite/object descriptor (`$ED/$EE/$EF`) and spawns it via
`JSR $F179`.

### Guaranteed drops first, then the roll (matches Data Crystal exactly)
```
$EF85  enemy_drop_choose:
  A2 00      LDX #$00            ; X = item-type 0 (BREAD)
  A5 58      LDA health          ; $58
  C9 14      CMP #$14            ; health < 20?
  90 37      BCC $EFC4           ;   yes -> drop BREAD (X=0)   [guaranteed]
  E8         INX                 ; X = 1 (POTION)
  A5 59      LDA magic           ; $59
  C9 1E      CMP #$1E            ; magic < 30?
  90 30      BCC $EFC4           ;   yes -> drop POTION (X=1)  [guaranteed]
  A2 04      LDX #$04            ; X = 4 (KEY)
  A5 5B      LDA keys            ; $5B
  C9 02      CMP #$02            ; keys < 2?
  90 28      BCC $EFC4           ;   yes -> drop KEY (X=4)     [guaranteed]
  A9 14      LDA #$14            ; modulus = 20
  20 64 CC   JSR rng_update      ; roll = rand[0..19]
  C9 09      CMP #$09            ; roll < 9?
  B0 07      BCS $EFAC           ;   roll >= 9 -> bread/potion/money chooser
  A8         TAY
  BE E7 EF   LDX drop_item_table,Y  ; $EFE7[roll]  -> item-type index
  4C C4 EF   JMP $EFC4
$EFAC  drop_money_chooser:      ; rolls 9..19: pick by which stat is lowest
  A2 00      LDX #$00            ; X = 0 (BREAD)
  A5 58      LDA health
  C5 59      CMP magic
  90 0A      BCC $EFBE          ;  health < magic -> compare gold
  E8         INX                ; X = 1 (POTION)
  A5 59      LDA magic
  C5 5A      CMP gold
  90 09      BCC $EFC4          ;  magic < gold -> drop POTION
  4C C2 EF   JMP $EFC2          ;  else -> MONEY
$EFBE:
  C5 5A      CMP gold           ; (health) < gold?
  90 02      BCC $EFC4          ;  yes -> drop BREAD (X=0)
$EFC2:
  A2 02      LDX #$02           ; X = 2 (MONEY)
$EFC4  item_spawn_setup:       ; X = item-type index -> spawn descriptor
  TXA / CLC / ADC #$02 / STA $EE
  TXA / ASL / ASL / ORA #$81 / STA $ED
  LDA #$01 / STA $EF ...
  JSR $F179 / RTS
```

### Drop item-type table — `drop_item_table` @ `$EFE7` (9 bytes)
- CPU `$EFE7` / PRG `0x1EFD7` / file `0x1EFE7` / bank 15.
- `disasm/bankfix.s:5937` (the `.byte $03,$03,$03,$03,$04,$04,$05,$06,$07`).
- Verified bytes: `03 03 03 03 04 04 05 06 07`.

Indexed by the `0..8` slice of the 0-19 roll (`roll < 9`); the value is the **item-type
index** consumed by `item_spawn_setup` at `$EFC4`.

| Roll (0-19) | table value (X) | Item    |
|-------------|-----------------|---------|
| 0,1,2,3     | `$03`           | poison  |
| 4,5         | `$04`           | key     |
| 6           | `$05`           | ring    |
| 7           | `$06`           | cross   |
| 8           | `$07`           | scroll  |
| 9..19       | (via $EFAC)     | bread / potion / money (by life vs magic vs gold) |

This reproduces the Data Crystal drop map exactly:
`$00-$03 poison, $04-$05 key, $06 ring, $07 cross, $08 scroll, $09-$13 bread/potion/money`.

### Item-type index -> item (full mapping)
Derived from the guaranteed-drop branches and the table. The index in X means:
| X | Item   | How reached |
|---|--------|-------------|
| 0 | bread  | health<20 guard; roll>=9 + health lowest |
| 1 | potion | magic<30 guard; roll>=9 + magic lowest |
| 2 | money  | roll>=9 + gold lowest |
| 3 | poison | roll 0-3 |
| 4 | key    | keys<2 guard; roll 4-5 |
| 5 | ring   | roll 6 |
| 6 | cross  | roll 7 |
| 7 | scroll | roll 8 |

(`item_spawn_setup` at `$EFC4` maps X -> object descriptor: `$EE = X+2`, `$ED = (X<<2)|$81`.)

---

## 3. Relevant RAM (Data Crystal RAM map, byte-confirmed in code)
| Addr | Symbol  | Meaning | Drop guard |
|------|---------|---------|------------|
| `$58`| `health`| player health | `< $14` (20) -> bread |
| `$59`| `magic` | player magic  | `< $1E` (30) -> potion |
| `$5A`| `gold`  | player gold   | tiebreak for money |
| `$5B`| `keys`  | player keys   | `< $02` -> key |

---

## 4. Symbols to apply (summary)
| Addr | Name | Kind | Notes |
|------|------|------|-------|
| `$CC64` | `rng_update` | routine | RNG step; A=count in, A=roll out (already `L_CC64`) |
| `$38` | `rng_count` | ram (1) | RNG count/upper bound (already symbolized) |
| `$39` | `rng_s0` | ram (1) | RNG state low (already symbolized) |
| `$3A` | `rng_s1` | ram (1) | RNG state mid (already symbolized) |
| `$3B` | `rng_s2` | ram (1) | RNG state high == result (already symbolized) |
| `$EF85` | `enemy_drop_choose` | routine | guaranteed drops + 0-19 roll dispatch |
| `$EFAC` | `drop_money_chooser` | routine | rolls 9-19: bread/potion/money by stat |
| `$EFC4` | `item_spawn_setup` | routine | X(item-type) -> object descriptor, spawn |
| `$EFE7` | `drop_item_table` | data (9) | roll[0..8] -> item-type index |

NOTE: the disassembly is byte-exact and must not be edited; these names are for the recon/RAM
symbol set and the upcoming C port, not for in-place edits to `disasm/`.
