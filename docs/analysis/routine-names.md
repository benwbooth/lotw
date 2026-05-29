# Routine Names — Fixed Banks 14/15 ($C000-$FFFF)

Proposed `snake_case` names for the major engine routines in the two fixed PRG banks
(bank 14 @ `$C000-$DFFF`, bank 15 @ `$E000-$FFFF`), derived by reading
`disasm/bankfix.s` and cross-checking `docs/recon/02-mmc3-bankswitch.md` and
`docs/recon/06-audio-engine.md`. All CPU addresses; PRG file offset = `CPU - $C000 + 0x1C010`
(i.e. `$C000` = file `0x1C010`, `$E000` = file `0x1E010`). Do NOT edit `disasm/` — analysis only.

## Boot / control flow

### `reset` — `$FFE0` (file `0x1FFF0`), bank 15
RESET vector target. `SEI; LDA #$00; STA $8000; STA $A001; STA $E000; JMP $C000`.
Clears MMC3 select/PRG-RAM-protect, disables the scanline IRQ (only `$E000` write in the ROM),
hands off to `main_init`. Confidence: HIGH (vectors byte-verified in SUMMARY; matches recon §2).

### `main_init` — `$C000` (file `0x1C010`), bank 14
Cold-boot entry from `JMP $C000`. `SEI; LDX #$FF; TXS`; zeroes `$2000/$2001/$4010`;
writes `$1F`→`$4015` (APU all channels enabled), `$C0`→`$4017` (frame counter); two
`BIT $2002` vblank-wait spins (`L_C01C`/`L_C021`); `STA $A000` = vertical mirroring;
`JSR L_CD08` (seed R6=$0C/R7=$0D); then far-calls into the title/menu code. Followed by the
main game loop at `L_C04F`/`L_C06D` which drives mode selection, room load, and per-frame update.
Confidence: HIGH (matches recon §2; APU/PPU init sequence is unambiguous).

### `main_loop_dispatch` — `$C06D` (file `0x1C07D`), bank 14
Top of the resident main loop. Reads `$58` (mode/character select), and either runs the
title/password path (`L_C04F` block, repeated far-calls into bank `$0C/$0D` via `L_CC9C`)
or the in-game path at `L_C093` (`JSR L_CC43` controller read, `JSR L_D42B` game update,
scroll/camera via `L_C15D`, room transition via `L_C2B1`). Confidence: MEDIUM (clear control
flow; exact mode semantics of `$58` not fully traced).

## Interrupt / per-frame PPU

### `nmi_handler` — `$D1FE` (file `0x1D20E`), bank 14
NMI=IRQ vector target (shared; scanline IRQ unused). Pushes A/X/Y, latches `$2002`→`$26`,
sets OAM addr `$2003`=0 and triggers OAM DMA `$4014`=`$02` (sprite page `$0200`).
Dispatches a queued VRAM-upload job through the jump table at `$D244` (index = `$28`, the
"PPU job id"; `$28>=7` or 0 skips to the tail `L_D351`). Tail commits MMC3 regs, does the
sprite-0 split, restores the select latch, `RTI`. Confidence: HIGH (matches recon §7).

### `nmi_vram_dispatch_table` — `$D244` (file `0x1D254`), bank 14 (data)
7-entry little-endian jump table of PPU-upload handlers reached by `nmi_handler`:
`$D351 $D252 $D25F $D290 $D2E5 $D334 $D344`. Each handler streams a different RAM buffer to
`$2007` (palette `$0180`, the `$0140`/`$0158` nametable strips, attribute updates `$0170`,
the `$0500`-page stack-blit at `$D2E5`, generic `($18)` run-copy at `$D334`, single word at
`$D344`). Confidence: HIGH (table bytes verified inline at `L_D221`+).

### `nmi_tail` — `$D351` (file `0x1D361`), bank 14
Common NMI exit: `JSR ppu_commit_banks` (`L_D41D`), `JSR statusbar_split` (`L_D36E`),
decrement frame flag `$36`, `JSR frame_counters` (`L_D408`), restore `$25`→`$8000`,
pull regs, `RTI`. Confidence: HIGH.

### `ppu_commit_banks` — `$D41D` (file `0x1D42D`), bank 14
The per-frame MMC3 register committer. `LDX #$07; loop: LDA $2A,X; STX $8000; STA $8001; DEX;
BPL` — re-asserts the full R0..R7 register file from the zp shadow `$2A..$31` every frame.
The zp shadow is the single source of truth for live banks. Confidence: HIGH (matches recon §3).

### `statusbar_split` — `$D36E` (file `0x1D37E`), bank 14
Sprite-0-hit software raster split for the status bar. Programs scroll (`$2005`) and, when
`$29`!=0, busy-polls the `$2002` sprite-0 flag (`L_D3C6`/`L_D3CB`), then mid-frame swaps CHR
regs R1/R4/R5: pre-split to fixed banks `$16/$3E/$3F`, post-split back to shadows `$2B/$2E/$2F`.
Calls `sound_tick` (`L_F89A`) before the split wait. Confidence: HIGH (matches recon §6).

### `frame_counters` — `$D408` (file `0x1D418`), bank 14
Once-per-`$84`-frames (reloads `$84`=`$3C`=60), decrements the 8 software timers `$85..$8C`
(skipping any already zero). General countdown-timer service. Confidence: HIGH (clear loop).

## Bank-switched far calls

### `farcall_bank_0C0D` — `$CC9C` (file `0x1CCAC`), bank 14
The primary far-call dispatcher. Saves R6/R7 shadows to `$32/$33`, pushes return `$CCC7`
(restore stub), forces R6=`$0C`/R7=`$0D` (updating `$25`/`$30`/`$31`), `JMP ($000E)`.
Caller stashes the target address in `$0E/$0F`. The embedded restore stub at `$CCC7`
restores R6/R7 from `$32/$33` and `RTS`. ~13 call sites. Confidence: HIGH (matches recon §5).

### `farcall_return_home` — `$CCE4` (file `0x1CCF4`), bank 14
NOTE: the charter's "text/string draw via $0E/$0F" label is INCORRECT for this address.
This is the inverse dispatcher: it pushes return `$CD07`, restores R6/R7 to the *saved* banks
`$32/$33` (the caller's prior banks), then `JMP ($000E)`. It has zero callers within the fixed
banks; it is invoked from code running in banks `$0C/$0D` to call back into the game's home
banks via `$0E/$0F`. Effectively a "far-call back into the previously-banked code" trampoline,
the complement of `farcall_bank_0C0D`. Confidence: HIGH (instruction-level read; corrects charter).

### `farcall_bank_0C0D_seed` — `$CD08` (file `0x1CD18`), bank 14
Boot/setup variant of the dispatcher with no call-through: saves R6/R7 to `$32/$33`, sets
R6=`$0C`/R7=`$0D`, `RTS`. Used by `main_init` to seed the windows. Confidence: HIGH (recon §5).

### `farcall_bank_09_r7` — `$C833` (file `0x1C843`), bank 14
R7-specific far-call wrapper: pushes R7 shadow `$31`, forces R7=`$09` (via `$25`/`$8000`/`$8001`),
runs `L_CA54`+`L_C871` (object/sprite record builders that read tables in bank `$09`), restores
R7. ~6 call sites. Confidence: HIGH (matches recon §5 "R7→bank $09 wrapper").

## Input / RNG / sync

### `read_controllers` — `$CC43` (file `0x1CC53`), bank 14
Standard NES dual-controller strobe-and-shift read. Strobes `$4016` (1 then 0), shifts 8 bits
from `$4016 ORA $4017` into `$20` (pad 1) and `$21` (pad 2), then `$20 = $20 ORA $21` (merged
held-buttons). Confidence: HIGH (textbook controller read; `$20` is the button state byte).

### `rng_update` — `$CC64` (file `0x1CC74`), bank 14
The RNG. State is the 24-bit value `$3A` (lo) / `$3B` (hi) with scratch `$39`. Argument in A
(stored to `$38`) is an inclusive upper bound: loops doing a 16-bit `ASL`/`ROL` of `$3A:$3B`,
`INY`/`INX`, then `ADC $3A` / `ADC $3B` plus `ADC $39`, `AND #$7F`, storing back to `$3B:$3A`,
repeating until the masked result `>= $38`. Returns a value in `[0,$38)` in X/`$3B`. Matches the
charter's "ASL/ROL/ADC/AND loop over $39-$3B". Confidence: HIGH (algorithm + state regs match).

### `queue_ppu_job_and_wait` — `$CC8F` (file `0x1CC9F`), bank 14
Wait-for-NMI / queue-a-VRAM-job primitive. Spins until prior job `$28`==0, stores the job id
(A) into `$28`, then spins until the NMI clears `$28` (i.e. waits for the upload to be consumed
in vblank). The job id selects a handler in `nmi_vram_dispatch_table`. The sub-entry `L_CC97`
($CC97) is the pure "wait until `$28`==0" form. Confidence: HIGH (clear vblank handshake).

## Text / nametable / state init

### `text_attr_build` — `$C909` (file `0x1C919`), bank 14
Reads a 0x16-byte room/menu descriptor via `($77)`, converting the first byte with `ADC #$A0`
(the verified text tile encoding `tile = ASCII + $A0`) and unpacking nametable address `$70/$71`,
CHR/scroll selectors `$2A/$2B/$2D`, and HUD-icon fields into the `$0400`/`$04A0` blit buffers.
Part of the screen/HUD assembly chain `L_C8F2` (`scene_assemble`). Confidence: MEDIUM
(text-encoding constant `$A0` is the load-bearing tell; full field map not exhaustively traced).

### `scene_assemble` — `$C8F2` (file `0x1C902`), bank 14
Top-level "build the current screen" routine: `JSR L_C9D2` (compute base nametable addr from
`$47/$48`), `L_C9A9` (copy 0x300 bytes of layout to `$0500-$07FF`), `text_attr_build`,
`L_C9FB` (copy `$A0..$FF` zp-shadow block + HUD palette). Confidence: MEDIUM.

### `ram_state_init` — `$D1C8` (file `0x1D1D8`), bank 14
Cold-init of game RAM from banked ROM init tables: copies 256 B `$9B9F`→`$0000` (zero page),
0x40 B `$9C9E`→`$0100`, fills `$0180` (palette buf) with `$0F`, 256 B `$9D3E`→`$0300`, 256 B
`$9DC9`→`$0400`. Called once from `main_init`. Confidence: HIGH (straight init copies).

### `game_update` — `$D42B` (file `0x1D43B`), bank 14
Per-frame game-state update entry (called from `main_loop_dispatch`). Sets `$E3`=`$FF`, branches
on `$EB` (paused/cutscene) to `L_D641`, else runs player/physics (`L_D64F`, `L_D596`), input flag
`$20` bit handling, and dispatches to the demo/play paths (`L_E00F`). Confidence: MEDIUM
(entry + first-level structure clear; deep logic not fully reversed).

### `metasprite_build` — `$C871` (file `0x1C881`), bank 14
Builds nametable/sprite strips for a 0x16-cell object from a record via `($79)`/`($0C)`,
writing into `$0140`/`$0158` (the strips uploaded by the NMI dispatch handlers) and computing
attribute bytes into `$0170`. Invoked through `farcall_bank_09_r7`. Confidence: MEDIUM
(buffer targets match the NMI upload buffers; exact record layout from bank 9 not traced).

## Sound driver (bank 15, $E000-$FFFF)

### `sound_tick` — `$F89A` (file `0x1F8AA`), bank 15
Per-frame sound update, called once per frame from `statusbar_split`. Maps music banks
(`L_FD74`), runs the SFX overlay voice (`L_FA60`), and if a song is active (`$8D`) updates the
four channel state machines `sound_ch_pulse1/2`/`triangle`/`noise` (`L_F8F0/L_F96E/L_FA09/
L_FB1F`), each selected via the channel offset `$02`, then restores game banks (`L_FD9C`).
Confidence: HIGH (matches recon §"Per-frame update"; APU writes verified).

### `song_init` — `$FC08` (file `0x1FC18`), bank 15
Song start. Picks music bank pair (`$0A/$0B` for song `$8E < $0A`, else `$0C/$0D`) into
`$34/$35`, maps them (`L_FD87`), indexes the `$8000` song-pointer table by `song*2` into `$0E/$0F`,
and copies 4 × 8-byte channel descriptors from `($0E)` into channel state starting at `$0093`,
zeroing the trailing 8 bytes of each. Ends with `JSR ppu_commit_banks`. Confidence: HIGH
(matches recon "LFC08 logic").

### `sound_set_default_banks` — `$FD74` (file `0x1FD84`), bank 15
Maps R6=`$0A`/R7=`$0B` (default music banks) into `$8000/$A000`. Confidence: HIGH.

### `sound_set_song_banks` — `$FD87` (file `0x1FD97`), bank 15
Maps R6=`$34`/R7=`$35` (the per-song bank pair) into `$8000/$A000`. Confidence: HIGH.

### `sound_restore_game_banks` — `$FD9C` (file `0x1FDAC`), bank 15
Restores R6=`$30`/R7=`$31` (the game's shadow banks) after the sound update. Confidence: HIGH.

### `sfx_overlay_voice` — `$FA60` (file `0x1FA70`), bank 15
The 5th "overlay" voice / SFX engine. Hijacks pulse-2 hardware ($4004-$4007) when an SFX
(`$8F`) is queued, using its own state `$D3-$DA` and priority `$90/$91`, restoring music after.
Confidence: HIGH (matches recon).

### `note_period_table` — `$FDB1` (file `0x1FDC1`), bank 15 (data)
12 little-endian period words, one equal-tempered chromatic octave at base C2; higher octaves
via right-shift (`L_FC81`). Index 5 = `$0000` (unused slot). Confidence: HIGH (byte-verified
in recon/SUMMARY).
