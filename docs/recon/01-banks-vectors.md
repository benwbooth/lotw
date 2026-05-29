# Recon 01 — Bank/Segment Model and CPU Vectors

ROM: `rom/lotw.nes` — *Legacy of the Wizard* (US; JP original *Dragon Slayer IV: Drasle Family*, Falcom / Broderbund).

## 1. iNES Header (file offset `0x00..0x10`)

Raw 16 bytes: `4E 45 53 1A 08 08 40 08 00 00 00 00 00 00 00 01`

| Off | Byte | Meaning |
|-----|------|---------|
| `0x00-0x03` | `4E 45 53 1A` | Magic `"NES\x1A"` |
| `0x04` | `08` | PRG-ROM = 8 x 16 KiB = **131072 bytes (128 KiB)** |
| `0x05` | `08` | CHR-ROM = 8 x 8 KiB = **65536 bytes (64 KiB)** |
| `0x06` | `40` | flags6: mapper low nibble = `4`; mirroring bit0=0 -> **horizontal**; no battery, no trainer, no 4-screen |
| `0x07` | `08` | flags7: mapper high nibble = `0`; bits2-3 = `0b10` -> **NES 2.0** identifier |
| `0x08-0x0F` | `00 00 00 00 00 00 00 01` | NES 2.0 extension bytes (mapper bits, sizes, region). Effective mapper = `0<<4 | 4` = **4 (MMC3)** |

Confirmed: **iNES mapper 4 (MMC3), horizontal mirroring, 128 KiB PRG, 64 KiB CHR, no PRG-RAM battery.**

### File offset map
- Header: `0x00000 .. 0x00010` (16 bytes)
- PRG-ROM: `0x00010 .. 0x20010` (131072 bytes)
- CHR-ROM: `0x20010 .. 0x30010` (65536 bytes)

## 2. CPU Vectors (file offset `0x1FFEA..0x1FFF0`, CPU `$FFFA..$FFFF`)

Vectors live in the always-fixed last 8 KiB PRG bank (bank 15) at `$E000-$FFFF`.

Raw 6 bytes: `FE D1 E0 FF FE D1`

| Vector | CPU addr | Target | Notes |
|--------|----------|--------|-------|
| NMI | `$FFFA/B` | **`$D1FE`** | Handler in fixed bank 14 (`$C000-$DFFF`). |
| RESET | `$FFFC/D` | **`$FFE0`** | Tiny stub in fixed bank 15. |
| IRQ/BRK | `$FFFE/F` | **`$D1FE`** | Same address as NMI (shared handler in bank 14). |

### RESET stub (`$FFE0`, file `0x1FFF0`)
`78 A9 00 8D 00 80 8D 01 A0 8D 00 E0 4C 00 C0`
```
FFE0  SEI
FFE1  LDA #$00
FFE3  STA $8000     ; MMC3 bank-select = 0 (R0, PRG mode bit6=0, CHR mode bit7=0)
FFE6  STA $A001     ; PRG-RAM protect register
FFE9  STA $E000     ; disable MMC3 IRQ
FFEC  JMP $C000     ; jump to main init at start of fixed bank 14
```
The `STA $8000` with `#$00` sets the MMC3 bank-select register to 0, which fixes **PRG bank mode bit6 = 0** and **CHR mode bit7 = 0**.

### Main init entry (`$C000`, start of bank 14, file `0x1C010`)
```
C000  SEI / LDX #$FF / TXS / LDA #$00 / STA $2000 / STA $2001 / STA $4010
C00F  LDA #$1F / STA $0027 / STA $4015 / LDA #$C0 / STA $4017 ...
```
Standard NES hardware bring-up (stack, PPU off, APU init). Confirms bank 14 is fixed at `$C000`.

### NMI/IRQ handler (`$D1FE`, in bank 14, file `0x1C010 + 0x11FE = 0x1D20E`)
```
D1FE  PHA / TXA / PHA / TYA / PHA
D203  LDA $2002 / STA $26
D208  LDA #$00 / STA $2003 / LDA #$02 / STA $4014   ; OAM DMA from $0200
D212  LDA $28 ...
```

## 3. MMC3 Bank Model

MMC3 banks PRG in **8 KiB** units (16 banks here) and CHR in **1 KiB** units (64 banks).

Evidence from code scan of all PRG (count of `STA abs` to MMC3 registers):
`$8000` bank-select x15, `$8001` bank-data x22, `$A000` mirroring x1, `$A001` PRG-RAM x1, `$E000` IRQ-disable x1.

Bank-register routines in the fixed last bank (file `0x1FD87`):
```
LDA #$06 / STA $8000 / LDA $34 / STA $8001     ; R6 -> $8000 window, from zp $34
LDA #$07 / STA $8000 / LDA $35 / STA $8001     ; R7 -> $A000 window, from zp $35
LDA #$06 / STA $8000 / LDA $30 / STA $8001     ; alt: R6 from zp $30
LDA #$07 / STA $8000 / LDA $31 / STA $8001     ; alt: R7 from zp $31
```
The code programs **R6 (=$8000 window)** and **R7 (=$A000 window)**, never an alternate-mode arrangement. With bank-select bit6 = 0, this means:

- **R6 controls `$8000-$9FFF`** (swappable)
- **R7 controls `$A000-$BFFF`** (swappable)
- **`$C000-$DFFF` is FIXED to the second-to-last bank (bank 14)**
- **`$E000-$FFFF` is FIXED to the last bank (bank 15)**

CHR registers R0/R1 (2 KiB) and R2-R5 (1 KiB) are also written (the `STA $8000` with `#$00,#$01,#$04,#$05` values seen near file `0x1D3A0`).

### PRG bank table (16 x 8 KiB)

| Bank | File offset range | Maps to CPU window | Mapping |
|------|-------------------|--------------------|---------|
| 0  | `0x00010..0x02010` | `$8000` or `$A000` | swappable (R6/R7) |
| 1  | `0x02010..0x04010` | `$8000` or `$A000` | swappable |
| 2  | `0x04010..0x06010` | `$8000` or `$A000` | swappable |
| 3  | `0x06010..0x08010` | `$8000` or `$A000` | swappable |
| 4  | `0x08010..0x0A010` | `$8000` or `$A000` | swappable |
| 5  | `0x0A010..0x0C010` | `$8000` or `$A000` | swappable |
| 6  | `0x0C010..0x0E010` | `$8000` or `$A000` | swappable |
| 7  | `0x0E010..0x10010` | `$8000` or `$A000` | swappable |
| 8  | `0x10010..0x12010` | `$8000` or `$A000` | swappable |
| 9  | `0x12010..0x14010` | `$8000` or `$A000` | swappable |
| 10 | `0x14010..0x16010` | `$8000` or `$A000` | swappable |
| 11 | `0x16010..0x18010` | `$8000` or `$A000` | swappable |
| 12 | `0x18010..0x1A010` | `$8000` or `$A000` | swappable |
| 13 | `0x1A010..0x1C010` | `$8000` or `$A000` | swappable |
| 14 | `0x1C010..0x1E010` | **`$C000-$DFFF`** | **FIXED (second-to-last)** |
| 15 | `0x1E010..0x20010` | **`$E000-$FFFF`** | **FIXED (last; holds vectors)** |

Note: MMC3 R6/R7 are 6-bit, but with 128 KiB PRG only bank indices 0-15 are valid; banks 14 and 15 are reachable as swappable values too but are conventionally left in their fixed positions. The two fixed windows (`$C000`, `$E000`) are bank 14 and bank 15 regardless of R6/R7.

### CHR
- CHR-ROM = 64 KiB = **64 banks of 1 KiB** (`0x20010..0x30010`).
- Banked via MMC3 R0-R5 into PPU `$0000-$1FFF`.

## 4. Proposed ld65 Segment Model

One 8 KiB segment per PRG bank, plus a CHR segment. Memory areas reflect the *runtime* CPU window each bank loads into; banks that share a swappable window all assemble against `$8000` (or `$A000`) origin. A practical scheme: assign every swappable bank an origin of `$8000` (most code/data banks page in at `$8000`); fixed banks use their true origins.

```
# --- lotw.cfg (ld65) ---
MEMORY {
    # iNES 16-byte header
    HDR:    start=$0000, size=$0010, fill=yes, fillval=$00;

    # Swappable PRG banks (origin $8000; some may instead page to $A000)
    PRG00:  start=$8000, size=$2000, fill=yes;
    PRG01:  start=$8000, size=$2000, fill=yes;
    PRG02:  start=$8000, size=$2000, fill=yes;
    PRG03:  start=$8000, size=$2000, fill=yes;
    PRG04:  start=$8000, size=$2000, fill=yes;
    PRG05:  start=$8000, size=$2000, fill=yes;
    PRG06:  start=$8000, size=$2000, fill=yes;
    PRG07:  start=$8000, size=$2000, fill=yes;
    PRG08:  start=$8000, size=$2000, fill=yes;
    PRG09:  start=$8000, size=$2000, fill=yes;
    PRG10:  start=$8000, size=$2000, fill=yes;
    PRG11:  start=$8000, size=$2000, fill=yes;
    PRG12:  start=$8000, size=$2000, fill=yes;
    PRG13:  start=$8000, size=$2000, fill=yes;

    # Fixed banks
    PRG14:  start=$C000, size=$2000, fill=yes;   # second-to-last, fixed $C000-$DFFF
    PRG15:  start=$E000, size=$2000, fill=yes;   # last, fixed $E000-$FFFF (vectors)

    CHR:    start=$0000, size=$10000, fill=yes;  # 64 KiB CHR-ROM
}

SEGMENTS {
    HEADER:  load=HDR,   type=ro;
    CODE00:  load=PRG00, type=ro;
    CODE01:  load=PRG01, type=ro;
    CODE02:  load=PRG02, type=ro;
    CODE03:  load=PRG03, type=ro;
    CODE04:  load=PRG04, type=ro;
    CODE05:  load=PRG05, type=ro;
    CODE06:  load=PRG06, type=ro;
    CODE07:  load=PRG07, type=ro;
    CODE08:  load=PRG08, type=ro;
    CODE09:  load=PRG09, type=ro;
    CODE10:  load=PRG10, type=ro;
    CODE11:  load=PRG11, type=ro;
    CODE12:  load=PRG12, type=ro;
    CODE13:  load=PRG13, type=ro;
    CODE14:  load=PRG14, type=ro;
    CODE15:  load=PRG15, type=ro;   # contains RESET stub @ $FFE0 and VECTORS @ $FFFA
    VECTORS: load=PRG15, type=ro, start=$FFFA;
    CHRROM:  load=CHR,   type=ro;
}
```

Notes for the disassembly driver:
- The 16-byte iNES HEADER must be emitted first (`4E 45 53 1A 08 08 40 08 00 00 00 00 00 00 00 01`).
- Segment concatenation order = HEADER, CODE00..CODE15, CHRROM — reproducing the exact `0x00..0x30010` byte layout.
- Bank 15 holds the 6 vector bytes at `$FFFA`; keep them in a dedicated `VECTORS` segment anchored at `$FFFA`.
- Whether a given swappable bank should be disassembled at origin `$8000` vs `$A000` must be decided per bank by tracing R6/R7 writes (cross-bank handoff analysis); the table above lists both as possible windows. Default to `$8000` unless evidence shows a bank is only ever paged to `$A000`.
