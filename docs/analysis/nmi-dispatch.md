# NMI dispatch jump table at `$D244`

Charter asked to decode "the main game-state dispatch jump table near `$D244`, reached from
the NMI handler `L_D1FE`," mapping each handler to a game state (title / select / gameplay /
password / shop / menu / game-over / ending), indexed by a zp "game mode" byte.

**Correction (high confidence):** the `$D244` table is *not* a game-state dispatcher. It is the
**NMI VRAM-update-routine table**, indexed by the zero-page byte **`$28` (an NMI "VRAM update
request type" selector)**, not by a game-mode variable. Each entry is a small PPU-transfer
routine that performs one kind of `$2007` write batch during vblank, then falls through to the
shared NMI tail `L_D351`. The recon note (`02-mmc3-bankswitch.md` line 151) labeled it
"dispatch via jump table @ `$D244`," and the charter inherited the assumption that this is the
game-mode dispatch. It is not. (The true game-mode/state machine is driven from the main loop
in swappable code bank 13 via the far-call mechanism `LCC9C`/`LC833`, and does not appear as a
jump table in the fixed banks — the only `JMP (indirect)` in banks 14/15 is this NMI one.)

## Where it lives

- Table base: CPU `$D244`, fixed **bank 14** (`$C000-$DFFF`).
  - PRG offset `0x1D244`, file offset **`0x1D254`** (`file = PRG + 0x10`).
- Reached only from the NMI/IRQ handler `L_D1FE` (`$D1FE`, file `0x1D20E`) via `JMP ($0006)`.
- In `disasm/bankfix.s` the 14 table bytes are the `.byte` block at line 2569
  (`51 D3 52 D2 5F D2 90 D2 E5 D2 34 D3 44 D3 ...`); the bytes after that are handler code that
  da65 left unlabeled because the targets are reached only through the indirect jump.

## Dispatch mechanism (NMI handler `L_D1FE`)

```
D1FE  PHA/TXA/PHA/TYA/PHA              ; save regs
      LDA $2002 / STA $26              ; latch PPUSTATUS
      LDA #$00 / STA $2003            ; OAM addr = 0
      LDA #$02 / STA $4014            ; OAM DMA from page $02 ($0200)
      LDA $28                         ; <-- selector: NMI VRAM-update request type
      BEQ  L_D21E                     ;   0 => no batch, go straight to tail
      LDX #$00 / STX $28              ;   consume/clear the request
      CMP #$07 / BCC L_D221           ;   only values 1..6 dispatch; >=7 also -> tail
L_D21E JMP  L_D351                    ; tail
L_D221 ASL A / TAX                    ; X = request*2
      LDA $D244,X / STA $06           ; load 16-bit handler pointer into $06/$07
      LDA $D245,X / STA $07
      LDA $2002
      LDX $17 / LDY $16              ; set PPU VRAM address ($17=hi, $16=lo)
      STX $2006 / STY $2006
      LDA $23 / AND #$04 / STA $2000  ; PPUCTRL (NMI off during batch, keep VRAM-incr bit)
      JMP ($0006)                     ; -> selected handler -> falls through to L_D351
```

So the index is `$28`. Valid range is `0..6` (guard `CMP #$07 / BCC`); `$28 = 0` means "no VRAM
work this frame." Each non-zero handler does its transfer and ends with `JMP L_D351`.

`$28` has **no static writers in the fixed banks (14/15)** — it is set by gameplay/UI code in
the swappable banks before the frame, queuing a specific kind of VRAM update for the NMI to
flush. The transfer parameters travel in zero page: `$16/$17` = VRAM dest addr (lo/hi),
`$18/$19` = source data (or pointer lo/hi), `$1A` = byte count, `$23` = PPUCTRL shadow.

## Table entries (decoded from ROM bytes at file `0x1D254`)

| `$28` | X | ptr bytes | Target | Routine kind (what it writes to `$2007`) |
|------|---|-----------|--------|------------------------------------------|
| 0 | 0 | `51 D3` | `$D351` | **No-op / null** — the shared NMI tail itself (no VRAM batch) |
| 1 | 2 | `52 D2` | `$D252` | **Tile run-fill** — write byte `$18`, `$1A` times to `$2007` |
| 2 | 4 | `5F D2` | `$D25F` | **Palette upload** — 32 bytes `$0180..$019F` -> PPU `$3F00` |
| 3 | 6 | `90 D2` | `$D290` | **Status-bar / HUD upload** — multi-row + attribute transfer from `$0140`/`$0158`/`$0170` |
| 4 | 8 | `E5 D2` | `$D2E5` | **Stack-streamed bulk transfer** — repoint SP, `PLA/STA $2007` x256 |
| 5 | 10 | `34 D3` | `$D334` | **Indirect run** — `$1A` bytes from `($18),Y` -> `$2007` |
| 6 | 12 | `44 D3` | `$D344` | **Two-byte poke** — write `$19` then `$18` to `$2007` |

### Per-handler detail

- **`$D351` (idx 0) `L_D351` — NMI tail (already labeled).**
  `JSR L_D41D` (commit MMC3 R0..R7 from zp shadow `$2A-$31`), `JSR L_D36E` (sprite-0 split +
  mid-frame CHR swap), `JSR L_D408`, restore select latch `$25 -> $8000`, pull regs, `RTI`.
  Every other handler reaches this by `JMP L_D351`.

- **`$D252` (idx 1) — tile run-fill.**
  `LDX $1A; LDA $18; loop: STA $2007; DEX; BNE` then `JMP L_D351`. Fills `$1A` cells with the
  single tile in `$18` at the pre-loaded VRAM addr (`$16/$17`). Used to clear/blank a strip.

- **`$D25F` (idx 2) — palette upload.**
  `LDA $2002; LDA #$3F; STA $2006; LDA #$00; STA $2006; LDX #$20; LDY #$00; loop $0180,Y ->
  $2007` (32 bytes), then re-points `$3F00` and writes `$00` x4 (mirror/reset), `JMP L_D351`.
  Source buffer is the RAM palette mirror at **`$0180-$019F`**, destination PPU `$3F00`.

- **`$D290` (idx 3) — status bar / HUD row upload.**
  `LDA $23; ORA #$04; STA $2000` (set +32 VRAM increment), then three transfers:
  `$0140..$0157` (0x18 bytes), an attribute pair via `$16/$17(+1)`, `$0158..$016F` (0x18 bytes),
  then an `$0170`-indexed read-modify-write loop combining `$18` mask with `$0171,X` attribute
  bits. This is the top HUD/status-line VRAM refresh.

- **`$D2E5` (idx 4) — stack-streamed bulk transfer.**
  `TSX; TXA; LDX #$FF; TXS; TAX; LDY #$04;` then an unrolled `PLA/STA $2007` x16 inner loop x4
  outer = **256 bytes** pulled off the stack page into `$2007`, then `TXS` restores SP and
  `JMP L_D351`. The "data" is whatever the caller pushed onto the stack (full-nametable / large
  block update). Fastest possible vblank transfer.

- **`$D334` (idx 5) — indirect run transfer.**
  `LDX $1A; LDY #$00; loop: LDA ($18),Y; STA $2007; INY; DEX; BNE` then `JMP L_D351`. Copies
  `$1A` bytes from the pointer in `$18/$19` to the pre-set VRAM addr. General run-length VRAM
  write from an arbitrary RAM/ROM source.

- **`$D344` (idx 6) — two-byte poke.**
  `LDA $19; STA $2007; LDA $18; STA $2007; JMP L_D351`. Writes exactly two bytes (`$19` then
  `$18`) to the pre-set VRAM addr. Used for tiny patches (e.g. a 2-tile counter update).

## Proposed symbols

Data:

- `$D244` `NMI_VRAM_UPDATE_TABLE` (data, 14 bytes / 7 LE pointers) — NMI VRAM-update routine
  jump table, indexed by `$28*2`.

Routines (fixed bank 14):

- `$D1FE` `nmi_irq_handler` (already `L_D1FE`) — NMI/IRQ entry; OAM DMA + VRAM dispatch.
- `$D351` `nmi_tail` (already `L_D351`) — table idx 0; commit banks, raster split, RTI.
- `$D252` `vram_fill_run` — idx 1.
- `$D25F` `vram_upload_palette` — idx 2.
- `$D290` `vram_upload_hud` — idx 3.
- `$D2E5` `vram_blit_stack` — idx 4.
- `$D334` `vram_copy_indirect` — idx 5.
- `$D344` `vram_poke2` — idx 6.

Zero-page (NMI transfer ABI):

- `$28` `NMI_VRAM_REQ` (1 B) — VRAM-update request type / table index (0 = none, 1..6 valid).
- `$16` `VRAM_DST_LO`, `$17` `VRAM_DST_HI` (1 B each) — destination PPU addr for the batch.
- `$18` `VRAM_SRC_LO` / fill-byte, `$19` `VRAM_SRC_HI` (1 B each) — source pointer / data.
- `$1A` `VRAM_LEN` (1 B) — transfer byte count.
- `$23` `PPUCTRL_SHADOW`, `$25` `MMC3_SELECT_SHADOW`, `$26` `PPUSTATUS_LATCH`.

## Note on the actual game-state machine

The real game-mode/state dispatch (title, character select, gameplay, password, shop/inn,
menu, game over, ending) is **not** this table and is **not** in the fixed banks as a jump
table. It is driven from the main loop after `JMP $C000` and routed into swappable code bank 13
through the far-call dispatcher `LCC9C`/`LC833` (target in `$0E/$0F`, forcing R6=`$0C`/R7=`$0D`).
Locating and decoding that state machine is a separate task; this document only covers the
`$D244` NMI VRAM-update table the charter pointed at.
