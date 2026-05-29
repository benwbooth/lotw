# MMC3 Bank-Switching & IRQ — Recon

ROM: `rom/lotw.nes` (Legacy of the Wizard / Dragon Slayer IV: Drasle Family).
iNES mapper 4 (MMC3), 16x 8 KiB PRG banks, 64x 1 KiB CHR banks. flags6 = `$40`.

File-offset model used throughout:
- iNES header `[0,16)`, PRG-ROM `[16, 131088)` (= 0x10 .. 0x20010), CHR-ROM after.
- PRG offset (prgoff) = file offset − 0x10.
- The last 8 KiB bank (bank 0x0F) is fixed at CPU `$E000-$FFFF` = prgoff `0x1E000-0x1FFFF`.
- The `$C000-$DFFF` window during boot/runtime holds PRG bank 0x0E (prgoff `0x1C000-0x1DFFF`);
  all the bank-management code, NMI/IRQ handler, and the dispatcher live there.

## 1. Summary of register usage (exhaustive PRG scan)

Scanned all 3-byte store opcodes (`8D` STA, `8E` STX, `8C` STY abs; `9D`/`99` indexed) for
operands in the MMC3 register range. Results:

| Register | # writes | Notes |
|----------|----------|-------|
| `$8000` bank-select | 26 | select values only 0,1,4,5,6,7 (see §4) |
| `$8001` bank-data   | 24 | |
| `$A000` mirroring   | 1  | writes `$00` (vertical) at reset, prgoff `0x1C02B` |
| `$A001` PRG-RAM prot| 1  | writes `$00` at reset, prgoff `0x1FFE6` |
| `$C000` IRQ latch   | **0** | never written |
| `$C001` IRQ reload  | **0** | never written |
| `$E000` IRQ disable | 1  | writes `$00` at reset, prgoff `0x1FFE9` |
| `$E001` IRQ enable  | **0** | never written |

There is also an indirect write path: `stx $8000` / `sty $8001` (the per-frame loop in §3),
already counted above.

## 2. Reset / power-on init  (CPU `$FFE0`, prgoff `0x1FFE0`)

RESET vector = `$FFE0`. NMI = IRQ = `$D1FE`.

```
FFE0  78        SEI
FFE1  A9 00     LDA #$00
FFE3  8D 00 80  STA $8000     ; bank-select = 0 (mode bits clear: PRG mode 0, CHR mode 0)
FFE6  8D 01 A0  STA $A001     ; PRG-RAM protect = 0
FFE9  8D 00 E0  STA $E000     ; IRQ disable  <-- scanline IRQ turned OFF and never re-enabled
FFEC  4C 00 C0  JMP $C000     ; enter boot code in the C000 window (bank 0x0E)
```

Boot continues at CPU `$C000` (prgoff `0x1C000`):
- `LDA #$00 / STA $A000` → mirroring = vertical (prgoff `0x1C02B`).
- `JSR $CD08` (LCD08) → seeds the PRG R6/R7 windows (see §3).
- A path writes `LDA #$07 / STA $25 / STA $8000 / LDA #$0D / STA $8001` → R7 = bank `$0D`.

## 3. The shadow-register array and the per-frame programmer (CRITICAL)

The game keeps a **contiguous 8-byte zero-page shadow of MMC3 registers R0..R7 at `$2A..$31`**:

| zp | MMC3 reg | window (mode 0) |
|----|----------|-----------------|
| `$2A` | R0 | CHR 2 KiB @ PPU `$0000` |
| `$2B` | R1 | CHR 2 KiB @ PPU `$0800` |
| `$2C` | R2 | CHR 1 KiB @ PPU `$1000` |
| `$2D` | R3 | CHR 1 KiB @ PPU `$1400` |
| `$2E` | R4 | CHR 1 KiB @ PPU `$1800` |
| `$2F` | R5 | CHR 1 KiB @ PPU `$1C00` |
| `$30` | R6 | PRG 8 KiB @ CPU `$8000` |
| `$31` | R7 | PRG 8 KiB @ CPU `$A000` |

A second shadow `$25` holds the last value written to `$8000` (the select register), so the
NMI can restore the select latch on exit.

**`LD41D` — the canonical "commit all 8 registers" routine** (CPU `$D41D`, prgoff `0x1D41D`):
```
D41D  LDX #$07
D41F  LDA $2A,X      ; shadow[reg]
      STX $8000      ; select register X
      STA $8001      ; write bank-data
      DEX
      BPL $D41F
      RTS
```
This is called from the NMI tail (`LD351 → JSR LD41D`), i.e. the entire MMC3 register file is
re-asserted from RAM **every frame** during vblank. Consequence for the disassembler:
**the live bank in any window is determined by the zp shadows `$2A-$31`, which are the single
source of truth.** Track those, not individual `$8001` stores.

## 4. PRG / CHR mode bits

Distinct `$8000` (select) immediates used anywhere: `{0,1,4,5,6,7}`. **None set bit6 (PRG mode)
or bit7 (CHR mode).** Therefore:
- **PRG mode = 0**: R6 → `$8000-$9FFF` (swappable), `$C000-$DFFF` = fixed second-to-last bank
  (0x0E), `$E000-$FFFF` = fixed last bank (0x0F), R7 → `$A000-$BFFF`.
- **CHR mode = 0**: R0/R1 are the two 2 KiB banks at PPU `$0000`/`$0800`; R2..R5 are the four
  1 KiB banks at PPU `$1000`/`$1400`/`$1800`/`$1C00`.

## 5. Bank-switched far-call dispatcher  (`LCC9C`, CPU `$CC9C`, prgoff `0x1CC9C`)

This is the main mechanism for calling code that lives in a non-resident PRG bank. ~13 JSR
call sites.

```
LCC9C: lda $30 / sta $32     ; save current R6 shadow
       lda $31 / sta $33     ; save current R7 shadow
       lda #$CC / pha
       lda #$C7 / pha        ; push return addr $CCC7 (the restore stub)
       ldy #$06 / sty $25 / sty $8000 / lda #$0C / sta $30 / sta $8001  ; R6 = bank $0C
       iny   / sty $25 / sty $8000 / lda #$0D / sta $31 / sta $8001     ; R7 = bank $0D
       jmp ($000E)           ; call target whose address the caller put in zp $0E/$0F
; on RTS the callee returns to $CCC7:
$CCC7: ldy #$07 / sty $25 / sty $8000 / lda $33 / sta $31 / sta $8001   ; restore R7
       dey     / sty $25 / sty $8000 / lda $32 / sta $30 / sta $8001    ; restore R6
       rts
```
Caller protocol: stash a 16-bit target address into `$0E/$0F`, then `JSR LCC9C`. The dispatcher
maps banks `$0C`(R6)/`$0D`(R7) into `$8000`/`$A000`, calls through, and restores the previous
banks afterward. A variant immediately after (`pushes $CD07`) does the same with the
restore-stub embedded. `LC833` (6 call sites) is a similar save/swap-to-bank-`$09`/restore
wrapper for R7 specifically.

`LCD08` (CPU `$CD08`) is the simpler "set R6=$0C, R7=$0D and remember prior in $32/$33" seeder
used during boot.

## 6. Scanline IRQ — NOT USED. Split screen is via sprite-0 hit + CHR swap

There are **zero** writes to `$C000`/`$C001` (IRQ latch/reload) and `$E001` (IRQ enable). The
only `$E000` write is the reset-time IRQ disable. The MMC3 scanline counter is therefore never
armed; NMI and IRQ share the same vector `$D1FE`.

The status-bar / split effect is instead done **in the NMI handler** by polling the PPU
sprite-0 hit flag and reprogramming CHR banks mid-frame. In `LD36E` (called from NMI tail
`LD351`), gated by zp flag `$29`:

1. Before the split, program R1/R4/R5 with fixed banks:
   `#$01→$8000, #$16→$8001` (R1=$16); `#$04→$8000, #$3E→$8001` (R4=$3E);
   `#$05→$8000, #$3F→$8001` (R5=$3F).
2. Busy-wait on sprite-0 hit:
   ```
   LD3C6: bit $2002 / bvs LD3C6      ; wait for sprite-0 flag to clear
   LD3CB: bit $2002 / bvs LD3D5 / bit $2002 / bvc LD3CB   ; wait for hit (bit6)
   LD3D5: ldx #$12 / dex / bne ...   ; small delay into hblank
   ```
3. After the split line, reprogram the same CHR regs from the shadows:
   `#$01→$8000, $2B→$8001` (R1); `#$04→$8000, $2E→$8001` (R4);
   `#$05→$8000, $2F→$8001` (R5).

So the top region (status/HUD) uses one set of CHR tiles and the play area uses another,
switched at the sprite-0 raster position — a software split, no mapper IRQ involved.

## 7. NMI handler entry  (CPU `$D1FE`, prgoff `0x1D1FE`)

```
D1FE  PHA / TXA / PHA / TYA / PHA
      LDA $2002 / STA $26
      LDA #$00 / STA $2003 / LDA #$02 / STA $4014   ; OAM DMA from $0200
      LDA $28 ...                                   ; dispatch via jump table @ $D244
      ... -> LD351: JSR LD41D (commit R0..R7) ; JSR LD36E (sprite-0 split CHR swap)
      LD360: JSR LD408 ; LDA $25 / STA $8000   ; restore select latch
             PLA/TAY PLA/TAX PLA / RTI
```

## 8. Implications for the disassembler / bank tracking

- The authoritative "current bank" state lives in zp `$2A..$31` (R0..R7) plus select-shadow
  `$25`. A static tracker should model these 8 bytes.
- `$C000-$DFFF` and `$E000-$FFFF` are **fixed** (banks 0x0E and 0x0F). Code/vectors there are
  always resident. NMI/IRQ/dispatcher all live in bank 0x0E at `$C000`.
- At a `JSR LCC9C` (or `LC833`) call site, R6/R7 are forced to `$0C`/`$0D` for the duration of
  the call and the indirect target is in `$0E/$0F`; afterward R6/R7 are restored to whatever
  they were.
- No MMC3 IRQ to model; only NMI. Mid-frame CHR changes (R1,R4,R5) happen inside NMI via
  sprite-0 polling — relevant only for CHR/PPU emulation, not PRG bank tracking.

## Key offsets (quick reference)

| Symbol | CPU | prgoff | Role |
|--------|-----|--------|------|
| RESET  | `$FFE0` | `0x1FFE0` | SEI; init $8000/$A001/$E000; JMP $C000 |
| NMI=IRQ| `$D1FE` | `0x1D1FE` | vblank handler (also does split) |
| LCD08  | `$CD08` | `0x1CD08` | seed R6=$0C/R7=$0D |
| LCC9C  | `$CC9C` | `0x1CC9C` | bank-switched far-call dispatcher |
| LC833  | `$C833` | `0x1C833` | R7→bank $09 wrapper |
| LD41D  | `$D41D` | `0x1D41D` | commit shadow R0..R7 → MMC3 (per frame) |
| LD36E  | `$D36E` | `0x1D36E` | sprite-0 split CHR swap |
| shadows| zp `$2A-$31` | — | R0..R7 mirror; `$25` = $8000 select shadow |
