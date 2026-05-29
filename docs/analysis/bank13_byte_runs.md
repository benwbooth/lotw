# Bank 13 (.byte residual) classification — disasm/bank13.s ($A000-$BFFF)

Bank 13 PRG offset 0x1A010 maps to CPU $A000. CPU $X -> file 0x1A010 + ($X - $A000).

There are exactly **6 contiguous `.byte` runs** in `disasm/bank13.s`. All 6 are
**genuine DATA**. No missed code was found.

Decoder verification: a full 6502 decoder was anchored to ROM offsets (not the .s
assembler layout, which drifts). Each run boundary was confirmed by ROM byte
matching against the surrounding decoded instructions. Bank 13 contains **zero
indirect `JMP (...)`** instructions, so there is no jump table that could dispatch
into any of these runs as code. Every run is reached only via indexed reads
(`LDA tbl,X`), pointer/PPU streaming, or is ASCII text — never as a branch/JSR/JMP
target.

## Run-by-run

### Run 1 — $A000-$A2EA (747 bytes) — DATA: title/credits nametable + palette
- File 0x1A010-0x1A2FA. Code resumes at $A2EB (`LDA #$18` = `A9 18`, line 53).
- Streamed to the PPU in 256-byte pages by the nametable uploader:
  `LDA $9FC9,X / STA PPUDATA` ... `LDA $A0C9,X`, `LDA $A1C9,X` (bank13.s lines
  2605-2620). $A2C9,X is copied to $0180 (palette buffer, line 2675), and
  $A2E9/$A2EA hold MMC3 R0/R1 bank values loaded into the shadows (lines 2627-2630).
- Tile bytes ($1F = blank frame tile, logo tiles $7A-$83), with a palette block at
  the tail (`0F 29 34 11 0F 27 16 06 ...`, standard NES palette indices).
- Decoding as 6502 is incoherent; structure is PPU tile/attr/palette data.

### Run 2 — $AAFC-$ABBB (192 bytes) — DATA: OAM sprite tables (3 x 64 bytes)
- File 0x1AB0C-0x1ABCB. Preceded by `RTS`; code resumes at $ABBC.
- Three 64-byte OAM blocks copied to $0240 (OAM shadow):
  `LDA $AAFC,X` (line 743, LDX #$3F), `LDA $AB3C,X` (line 751), `LDA $AB7C,X`
  (line 760).
- 48 sprite entries in classic OAM 4-byte format Y,tile,attr,X
  (e.g. `58 51 03 A0`); every attribute byte = $03. Pure indexed reads -> DATA.

### Run 3 — $B0AC-$B0B0 (5 bytes: `03 04 05 02 08`) — DATA: per-character item table
- File 0x1B0BC. Preceded by `JMP L_AE64`; consumer at $B0B1.
- `LDA $B0AC,X / STA carried_item0` (line 1698), X = character index (0-4, 5 entries).
  Starting carried-item per character. DATA.

### Run 4 — $B0FE-$B101 (4 bytes: `81 84 82 00`) — DATA: lookup table
- File 0x1B10E. Preceded by `RTS`; consumer label L_B102 follows at $B102.
- `LDA $B0FE,X / STA $20` indexed by `JSR $CC64` result (lines 1853-1854). DATA.

### Run 5 — $B4AF-$B4C4 (22 bytes) — DATA: HUD/menu text "GAME OVER/RETRY/CONTINUE"
- File 0x1B4BF. Preceded by `RTS`; consumer label L_B4C5 follows at $B4C5.
- Tile encoding = ASCII+$A0. Bytes `E7 E1 ED E5 C0 EF F6 E5 ...` decode to
  `GAME OVERRETRYCONTINUE` (three concatenated menu strings used by the game-over
  handler at $B307). DATA (text).

### Run 6 — $B6FC-$BFFF (2308 bytes) — DATA: OAM tables + ending credits nametable
- File 0x1B70C-0x1C00F (to end of bank). Preceded by `RTS`.
- $B6FC-$B71B (32 bytes): OAM sprite data (`80 C1 00 60 ...` Y,tile,attr,X),
  copied to $0240 by L_B110 (`LDA $B6FC,X`, line 1877, LDX #$1F).
- $B71C-$B79B (128 bytes): OAM sprite data, copied to $0240 by L_B104
  (`LDA $B71C,X`, line 1869, LDX #$7F); within it $B73C+ is an $F8 fill
  (off-screen sprite Y values).
- $B79C-$B7A7: $20 spaces (padding).
- $B7A8-$BFFF: ending CREDITS nametable text in **raw ASCII** (newline = $0D),
  "CREDITS / CAST / Warrior Xemn Worzen / ... / Monsters / ... / STAFF / ...",
  with some tile-code bytes ($C0/$C1) and $00 padding toward the end. Bank ends at
  $BFFF = $00.
- No code; not reached as a branch/JSR/JMP target.

## Confirmed: no missed code in bank 13
- All 6 .byte runs are reached exclusively via indexed table reads, PPU streaming,
  or are ASCII text.
- Bank 13 has no `JMP (indirect)` and no code-address pointer tables that would
  re-enter any run as code.
- The named code landmarks (oam_sprite_engine $A400, game-over handler $B307) are
  already decoded as instructions; the runs above are their *data*, not code.
