# RAM + Hardware-Register Symbol Map — Legacy of the Wizard (NES, MMC3)

Integration-ready symbol map for `rom/lotw.nes`. Sources:
- Data Crystal RAM map (via `docs/recon/08-external-research.md`) and `docs/recon/SUMMARY.md`.
- Standard NES hardware register definitions (PPU/APU/controller).
- MMC3 (mapper 4) register definitions.
- Byte-verified spot checks against `disasm/bankfix.s` (FIXED banks 14/15).

Conventions: addresses are CPU hex. Zero-page game vars are listed at their canonical
zp address (Data Crystal writes them as `$00xx`; the code uses the short `$xx` form —
same location). Confidence: HIGH = byte-verified in disasm or unambiguous hardware spec;
MEDIUM = documented by Data Crystal and consistent but not re-traced here; LOW = guess.

---

## 1. Verification highlights (byte-checked against `disasm/bankfix.s`)

- **RNG** `L_CC64` @ `$CC64` (file `0x1CC64`): `STA $38` (modulus) then loops over
  `$3B/$3A` with `ASL/ROL/ADC`, `AND #$7F`, `CMP $38` / `BCS` — exactly the Data Crystal
  RNG. Confirms `$38`=modulus, `$39/$3A/$3B`=state.
- **MMC3 shadow committer** `L_D41D` @ `$D41D` (file `0x1D41D`):
  `LDX #$07 / LDA $2A,X / STX $8000 / STA $8001 / DEX / BPL` — commits the contiguous
  `$2A..$31` shadow (R0..R7) to MMC3 BANK_SELECT/BANK_DATA every frame. Confirms
  `$2A-$31` = R0..R7 and the `$8000/$8001` register identities.
- **`$25`** = `$8000` BANK_SELECT shadow: in `L_C833` @ `$C833`,
  `LDA #$07 / STA $25 / STA $8000` — `$25` mirrors the value written to BANK_SELECT.
- **Save block** `L_D0A5` @ `$D0A5`: `$0300-$0307`→`$0308-$030F`; `$60-$6F`→`$0310-$031F`;
  `$5A`→`$0321`; `$5B`→`$0320`. Restore is the inverse (`L_D0C5` @ `$D0C5`). This both
  confirms and refines the Data Crystal save layout and confirms `$5A`=gold, `$5B`=keys,
  `$60-$6F`=inventory.
- **Player pos** `$43/$44` (X fine/tile), `$45` (Y): read/written in the boot/move code
  (e.g. `STA $43/$44/$45` near file `0x1C02B`, and `SBC $45`/`SBC a:$0043` in collision math).
- **Sprite tables** `$0400,Y` read/written across the OAM/sprite engine (e.g. `LDA $0400,Y`
  @ lines 411/417, `STA $040F,Y`).
- **MMC3 mirroring** `STA $A000` @ file `0x1C01C` region (RESET stub `STA $A001` etc.).

---

## 2. Zero-page game variables (Data Crystal RAM map, spot-verified)

| Addr | Name | Width | Meaning |
|---|---|---|---|
| `$0025` | MMC3_BANK_SELECT_SHADOW | 1 | last value written to `$8000` (BANK_SELECT) — HIGH (verified) |
| `$0026-$0028` | NMI_SCRATCH | 3 | NMI/handler scratch + re-entrancy guard (`$28` spin-flag in `L_CC90`) — MEDIUM |
| `$002A-$0031` | MMC3_SHADOW_R0..R7 | 8 | per-frame MMC3 bank shadow committed by `L_D41D`; `$30`=R6($8000), `$31`=R7($A000), `$2A/$2B`=R0/R1 (CHR 2K), `$2C-$2F`=R2-R5 (CHR 1K) — HIGH (verified) |
| `$0038` | RNG_MODULUS | 1 | RNG exclusive upper bound — HIGH (verified) |
| `$0039-$003B` | RNG_STATE | 3 | RNG state; result `AND #$7F` left in `$3B` — HIGH (verified) |
| `$0040` | CUR_CHARACTER | 1 | active character 0-4 (6 = selecting) — MEDIUM |
| `$0043` | PLAYER_X_FINE | 1 | player X sub-tile — HIGH (verified) |
| `$0044` | PLAYER_X_TILE | 1 | player X tile — HIGH (verified) |
| `$0045` | PLAYER_Y | 1 | player Y — HIGH (verified) |
| `$0047` | MAP_SCREEN_X | 1 | current map screen X — MEDIUM |
| `$0048` | MAP_SCREEN_Y | 1 | current map screen Y — MEDIUM |
| `$0051-$0053` | CARRIED_ITEMS | 3 | carried items — MEDIUM |
| `$0055` | EQUIPPED_ITEM | 1 | equipped item — MEDIUM |
| `$0058` | HEALTH | 1 | player health — HIGH (verified write) |
| `$0059` | MAGIC | 1 | player magic — HIGH (verified write) |
| `$005A` | GOLD | 1 | gold (saved to `$0321`) — HIGH (verified) |
| `$005B` | KEYS | 1 | keys (saved to `$0320`) — HIGH (verified) |
| `$005C` | JUMP | 1 | jump stat — MEDIUM |
| `$005D` | STRENGTH | 1 | strength stat — MEDIUM |
| `$005E` | SHOTS_ALLOWED | 1 | max simultaneous shots — MEDIUM |
| `$005F` | RANGE | 1 | shot range — MEDIUM |
| `$0060-$006F` | INVENTORY_COUNTS | 16 | inventory counts (saved to `$0310-$031F`) — HIGH (verified copy) |
| `$007B` | SCROLL_X_FINE | 1 | scroll X fine — MEDIUM |
| `$007C` | SCROLL_X_TILE | 1 | scroll X tile — MEDIUM |
| `$00F2` | BOSS_LIFE | 1 | boss life — MEDIUM |

---

## 3. Save / state block ($0300-$0321) — verified layout

| Addr | Name | Width | Meaning |
|---|---|---|---|
| `$0300-$031F` | SAVE_INVENTORY | 32 | last-save inventory payload — HIGH |
| `$0308-$030F` | SAVE_SCRATCH_0300 | 8 | backup copy of `$0300-$0307` (save), restore source (load) — HIGH (verified) |
| `$0310-$031F` | SAVE_INVENTORY_COUNTS | 16 | backup copy of `$60-$6F` — HIGH (verified) |
| `$0320` | SAVE_KEYS | 1 | saved keys (`$5B`) — HIGH (verified) |
| `$0321` | SAVE_GOLD | 1 | saved gold (`$5A`) — HIGH (verified) |

Note: `$0300-$0321` is the canonical password/save payload (Data Crystal). Password
generate/validate is still unreversed.

---

## 4. Sprite tables ($0400-$048F)

| Addr | Name | Width | Meaning |
|---|---|---|---|
| `$0400-$048F` | SPRITE_TABLES | 144 | 9 sprites x 16 bytes (per Data Crystal); indexed `$0400,Y` / `$040F,Y` by the sprite engine — HIGH (indexing verified) |

(OAM shadow page `$0200-$02FF` is separate; OAMDMA source — see hardware regs.)

---

## 5. NES PPU registers ($2000-$2007)

| Addr | Name | Meaning |
|---|---|---|
| `$2000` | PPUCTRL | NMI enable, NT base, increment, sprite/bg pattern, sprite size |
| `$2001` | PPUMASK | rendering enable + emphasis |
| `$2002` | PPUSTATUS | vblank / sprite-0 / overflow flags (read clears) |
| `$2003` | OAMADDR | OAM address |
| `$2004` | OAMDATA | OAM data port |
| `$2005` | PPUSCROLL | scroll (write x2) |
| `$2006` | PPUADDR | VRAM address (write x2) |
| `$2007` | PPUDATA | VRAM data port |

All HIGH (hardware spec).

---

## 6. NES APU + I/O registers ($4000-$4017)

| Addr | Name | Meaning |
|---|---|---|
| `$4000-$4003` | APU_PULSE1 | pulse channel 1 |
| `$4004-$4007` | APU_PULSE2 | pulse channel 2 |
| `$4008-$400B` | APU_TRIANGLE | triangle channel |
| `$400C-$400F` | APU_NOISE | noise channel |
| `$4010-$4013` | APU_DMC | delta-modulation channel |
| `$4014` | OAMDMA | write page -> OAM DMA |
| `$4015` | APU_STATUS | channel enable (write) / status (read) |
| `$4016` | JOY1 | controller 1 (+ strobe on write) |
| `$4017` | JOY2 | controller 2 / APU frame counter (write) |

All HIGH (hardware spec).

---

## 7. MMC3 (mapper 4) registers

| Addr | Name | Meaning |
|---|---|---|
| `$8000` | MMC3_BANK_SELECT | even: bank/CHR select latch (shadow `$25`) — HIGH (verified) |
| `$8001` | MMC3_BANK_DATA | odd: bank data for selected register — HIGH (verified) |
| `$A000` | MMC3_MIRROR | even: nametable mirroring — HIGH (write verified in RESET stub) |
| `$A001` | MMC3_PRGRAM_PROTECT | odd: PRG-RAM protect/enable — HIGH (write verified in RESET stub) |
| `$C000` | MMC3_IRQ_LATCH | even: IRQ counter reload value (unused by this game) — HIGH (spec) |
| `$C001` | MMC3_IRQ_RELOAD | odd: IRQ counter reload trigger (unused) — HIGH (spec) |
| `$E000` | MMC3_IRQ_DISABLE | even: ack + disable IRQ (written in RESET stub) — HIGH |
| `$E001` | MMC3_IRQ_ENABLE | odd: enable IRQ (unused) — HIGH (spec) |

Note: the scanline IRQ is unused (`SUMMARY.md`); the status-bar split is a software
sprite-0 poll + mid-frame CHR swap inside the NMI.
</content>
</invoke>
