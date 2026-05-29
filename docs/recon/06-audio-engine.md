# 06 - Audio Engine (Sound Driver)

ROM: `rom/lotw.nes` (Legacy of the Wizard / Dragon Slayer IV: Drasle Family, Falcom / Broderbund).
iNES mapper 4 (MMC3), 128 KiB PRG (16 x 8 KiB banks), 64 KiB CHR.

All file offsets below are into `lotw.nes`; subtract `0x10` (iNES header) to get a pure-PRG
offset, or use the bank arithmetic given. CPU addresses use the MMC3 fixed/swappable windows.

## TL;DR

- The sound driver code is a compact, hand-written **Falcom/custom NES driver** living entirely
  in the **last PRG bank (bank 15, fixed at $E000-$FFFF)**, file offsets `0x1F73C..0x1FFF0`.
- It drives the 2 pulse channels, the triangle, and the noise channel (no DMC for music).
  APU register writes are concentrated in `0x1F873..0x1FB84` ($F873-$FB84).
- Per-frame update entry: **`$F89A`**, called once per frame from the NMI handler
  (`jsr $F89A` at file `0x1D3BE`, bank 14).
- Song-start / driver init: **`$FC08`** (`LFC08`), called from several sites; it banks in the
  music data and copies a song's 4 channel descriptors into zero page.
- **Music data lives in PRG banks `0x0A`/`0x0B`** (file `0x14000..0x17FFF`) for songs 0-9 and
  banks `0x0C`/`0x0D` (file `0x18000..0x1BFFF`) for songs >= 10. The driver swaps these into the
  `$8000`/`$A000` windows via MMC3 R6/R7.
- **~11 songs** in bank 0x0A (pointer table at `$8000`) plus additional songs in bank 0x0C.
- **~37 SFX** definitions (pointer table at `$8014`, data at `$B3AB..$BAxx` in bank 0x0A).
- Note pitch = 12-tone equal-tempered chromatic table at `$FDB1` (verified semitone ratio
  1.0595), base C2 = 65.4 Hz; octave via right-shift. This makes MIDI conversion straightforward.

## Driver location and bank model

The CPU vectors are in the last bank (`NMI=$D1FE`, `RESET=$FFE0`, `IRQ=$D1FE`; bytes at file
`0x1FFFA..0x1FFFF`). The whole driver is assembled into that last fixed bank ($E000-$FFFF).

The driver does NOT keep its own music tables in the fixed bank. Instead `LFC08` (`$FC08`)
programs MMC3 to place music banks at `$8000`/`$A000` and reads the song header table from
`$8000`:

- `LFD74` ($FD74, file `0x1FD74`): default music banks. Writes R6=`$0A` (-> `$8000`) and
  R7=`$0B` (-> `$A000`).
- `LFD87` ($FD87): switches in the per-song banks held in `$34/$35` (`$0A/$0B` for songs 0-9,
  `$0C/$0D` for songs >= 10).
- `LFD9C` ($FD9C): restores the game's banks (`$30/$31`) when the driver is done for the frame.

`LFC08` logic (`$FC08`, file `0x1FC08`):
```
ldx #$0A            ; default music bank low = $0A
lda $8E ; cmp #$0A ; bcc -> ldx #$0C   ; songs >=10 use bank $0C
stx $34 ; inx ; stx $35                ; $34/$35 = bank pair
jsr LFD87                              ; map music banks to $8000/$A000
lda $8E ; cmp #$0A ; bcc ; sec sbc #$0A  ; song index within bank
asl ; tax
lda $8000,x ; sta $0E ; lda $8001,x ; sta $0F   ; song header pointer
; then copy 4 channels x 8 bytes from (header) into channel state at $93,$A3,$B3,$C3...
```

`$8E` = requested song id (`$FF` = stop/none); `$8F` = requested SFX id. These are set by the
game logic, e.g. `lda #imm / sta $8E` and `sta $8F` (song ids seen: 02, 09, 0A, 0E, FF; SFX ids
seen up to `$21`).

## Per-frame update and channel routines

Per-frame entry `$F89A` does:
```
jsr LFD74           ; map music banks
lda #$40 ; sta $02  ; $02 = "current channel" selector / SFX-track flag
jsr LFA60           ; secondary track (uses pulse2 regs, the SFX/overlay engine)
; if a song is active ($8D):
jsr LF8F0           ; channel 0 update  -> Pulse 1   ($4000-$4003)
jsr LF96E           ; channel 1 update  -> Pulse 2   ($4004-$4007)
jsr LFA09           ; channel 2 update  -> Triangle  ($4008-$400B)
jsr LFB1F           ; channel 3 update  -> Noise     ($400C-$400F)
jsr LFD9C           ; restore game banks
rts
```

`$02` holds the per-channel offset (0/+1/+2/... applied to indexed zero-page state) so the four
channel routines share helper code. The four routines are near-identical state machines:
- decrement the active note-duration counter (`$93/$A3/$B3/$C3`);
- when it expires, read the next stream byte via `LFD6B` (pointer increment of `$95/$96`, etc.);
- `0x00` byte => end-of-track / silence (`LFCF9`); `0xFF` => command escape (`LFB8E`);
- otherwise it is a note event.

A fifth logical voice, **`LFA60` ($FA60)**, is the SFX / overlay engine. It also drives the
pulse-2 hardware registers ($4004-$4007) and steals the channel when an SFX (`$8F`) is queued,
restoring the music afterward (state in `$D3-$DA`, `$8F` = sfx id, `$90/$91` = priority).

### APU register write map (evidence)

| Channel | Routine | APU regs written | file offsets (examples) |
|---|---|---|---|
| Pulse 1 | `LF8F0` $F8F0 | $4000-$4003 | 0x1F954, 0x1F92B-0x1F939 |
| Pulse 2 | `LF96E` $F96E | $4004-$4007 | 0x1F9B6, 0x1F9BB-0x1F9C9 |
| Triangle | `LFA09` $FA09 | $4008-$400B | 0x1FA42-0x1FA50 |
| Noise | `LFB1F` $FB1F | $400C-$400F | 0x1FB79, 0x1FB57-0x1FB5C |
| SFX/overlay | `LFA60` $FA60 | $4004-$4007 | 0x1FADB-0x1FAE9 |
| Silence/init helpers | $F8A0 block | $4000/$4004/$4008/$400C | 0x1F8B4-0x1F8C7 |

APU init (`$4010` DMC, `$4015` enable, `$4017` frame counter) is done in the reset/boot path in
bank 14: `$4010` at file `0x1C00C`, `$4015` at `0x1C014`, `$4017` at `0x1C019`. `$4014` (OAM DMA)
at `0x1D20F` and `$4016` (controllers) at `0x1CC45/0x1CC49` are I/O, not music. The two `$4006`
hits at `0x16341/0x1635F` (bank 11) are incidental/data, not the music engine.

## Song header format

The song pointer table is at **`$8000`** in the mapped music bank. For bank 0x0A (file
`0x14000`) the first entries are:

```
song 0: $8062   song 1: $80A2   song 2: $8102   song 3: $8082
song 4: $80E2   song 5: $8142   song 6: $80C2   song 7: $8122
song 8: $8162   song 9: $8182   song 10(alias): $8122
```
(file `0x14000`: `62 80 a2 80 02 81 82 80 ...`). Data streams begin at `$8182`/`$81A2` upward, so
**~11 song headers** are present in bank 0x0A; bank 0x0C (file `0x18000`) holds the songs with
ids >= 10 (its `$8000` table starts `0c 80 2c 80 4c 80 ...`).

Each **song header is 4 channel descriptors of 8 bytes** (pulse1, pulse2, triangle, noise),
copied verbatim into the channel state blocks by `LFC08`. Example, song 0 header at `$8062`
(file `0x14062`):
```
04 80 | a2 81 | a2 81 | ba 00     ; ch0: ret-stub $8004, stream ptr $81A2, loop ptr $81A2, params $00BA
04 80 | 83 82 | 83 82 | ba 00     ; ch1: stream $8283
04 80 | 65 83 | 65 83 | 00 10     ; ch2: stream $8365
04 80 | d6 85 | d6 85 | 80 03     ; ch3: stream $85D6
```
Layout per 8-byte descriptor: `[+0,+1]` = a constant return/silence-stub pointer (`$8004`),
`[+2,+3]` = current stream pointer (copied to `$95/$96` etc.), `[+4,+5]` = stream start/loop
pointer, `[+6,+7]` = initial parameters (duration counter, flags/transpose). The `04 80`
($8004) stub is a 1-byte "stop" stream used to silence an unused channel.

## Note / event stream format

A channel stream is a byte stream of note events and `$FF` commands, terminated by `$00`.

- **Note event = 2 bytes: `[duration] [note]`.**
  - `duration` byte: bits 0-6 = ticks the note/rest lasts (stored to `$93` via `and #$7F`);
    bit 7 set => this is a rest/tie (taken via the `bmi` path `LF942`).
  - `note` byte: low nibble (`and #$0F`, *2) indexes the 12-entry semitone period table at
    `$FDB1`; high nibble (`lsr a x4`) = octave (right-shifts the period) via `LFC81`.
- **`$00`** => end of stream / channel silence (`LFCF9`, `$F8xx`).
- **`$FF cc aa`** => command escape (`LFB8E`, `$FB8E`): reads command index `cc` (`$04`) and
  argument `aa` (`$05`), `cc < 5`, then `jmp (table)` through the jump table at **`$FBBB`**:
  `$FBC5(0) $FBE2(1) $FBFF(2) $FC02(3) $FC05(4)`.
  - cmd 0 (`$FBC5`): set instrument/volume - splits the arg nibbles into duty/volume (`$99,x`)
    and instrument index (`$A2,x`), then loads the channel's base volume from `LFDD2`.
  - cmd 1 (`$FBE2`): tempo/length scaling (computes `$A0,x` note-length multiplier).
  - cmds 2-4 (`$FBFF/$FC02/$FC05`): direct stores of one byte to `$99,x` (duty/duty+vol),
    `$A1,x` (transpose/detune), `$9A,x` (volume).

Example (song 0, pulse 1, `$81A2`, file `0x141A2`):
```
ff 00 0b   -> cmd0 (set instrument 0x0B)
ff 01 ??   -> cmd1 (tempo)
48 10      -> dur 0x48, note 0x10 (octave1, semitone0 = C)
48 20      -> dur 0x48, note 0x20 (octave2, semitone0)
0c 28  0c 26  0c 28 ...   (short notes)
```

## Instruments / envelopes

Two instrument-related tables follow the period table, both in the fixed bank:

- **Period (pitch) table `$FDB1`** (file `0x1FDB1`), 12 little-endian words, one chromatic
  octave at base C2:
  `06AE 064E 05F4 059E 054D (0000) 0501 04B9 0475 0435 03F9 03C0 ...`
  Consecutive ratio = 1.0595 (equal temperament). Higher octaves are produced by shifting the
  period right N times (N = note high nibble). Index 5 = 0 is an unused/special slot.
- **Envelope/instrument table `$FDCB`** (`LFDCB`, file `0x1FDCB`) and the larger volume/macro
  table at **`$FDD2`** (`LFDD2`, file `0x1FDD2..0x1FFEF`). Entries are 12-byte records selected
  by the instrument index (`$A2,x`, set by cmd 0). Loader/stepper helpers: `LFCC4` ($FCC4) loads
  an envelope record into channel state (`$9B-$9F,x`), `LFCDF` ($FCDF) loads the sustain/loop
  portion (record+0x0C), `LFD11` ($FD11) steps the volume envelope each tick, `LFD45` ($FD45)
  steps the pitch/sweep portion. Record fields observed (e.g. `00 01 01 0f f7 01 01 00 ff 0d 82 00`):
  attack/decay rates, target levels, and a `$FF`-prefixed pitch-macro segment. The block at
  `$FF20+` also contains a small **pitch-bend / vibrato macro table** (sequences of `fb e? ..`,
  `fd fc fc ..`) referenced by the envelope stepper.

## SFX table

The SFX pointer table is at **`$8014`** in bank 0x0A (file `0x14014`), indexed by `$8F * 2` in
`LFA60`. Entry 0 (`$8122`) overlaps a song header (= "no sfx"); valid SFX run from index 1
(`$B3AB`) through ~index 37 (`$B9E9`), with index 39 (`$8004`) acting as a terminator. So there
are **~37 SFX**, all stored in bank 0x0A at file `0x153AB..0x15Axx` (`$B3AB..$BAxx`).

SFX streams use the same event grammar as music. Example SFX `$10` at `$B4D6` (file `0x154D6`):
```
ff 00 25   (cmd0 instrument 0x25)
ff 01 00   (cmd1)
01 28  01 34  01 40  01 30  0c 50   (very short notes - a sweep)
98 00      (rest/end)
```

## Driver family

This is a **custom Falcom-style NES driver**, not a known public engine (not GGSound/FamiTone/
NSD.Lib). Hallmarks: per-channel 8-byte descriptor table copied at song start, `$FF`-escape
command bytes with a 5-entry jump table, `[duration][note]` note pairs with packed octave nibble,
a single-octave 12-entry period table extended by right-shifting, and 12-byte envelope/instrument
macro records. The 5th "overlay" voice (`LFA60`) that hijacks pulse 2 for SFX is also
characteristic of Falcom's contemporaneous NES titles.

## Pointers for later conversion (MIDI / PLAY DSL)

- Driver code: file `0x1F73C..0x1FFF0` (bank 15, $E000-$FFFF). Disasm via
  `da65 --cpu 6502 --start-addr 0xE000` on bytes `0x1E000..0x20000`.
- Period table -> MIDI note mapping: `$FDB1` (file `0x1FDB1`), 12 words, base = MIDI 36 (C2).
  `midi = 36 + 12*octave + semitone(low nibble index into table)`.
- Song header tables: bank 0x0A `$8000` (file `0x14000`, songs 0-9, ~11 headers); bank 0x0C
  `$8000` (file `0x18000`, songs >= 10).
- SFX table: bank 0x0A `$8014` (file `0x14014`), ~37 entries.
- Event grammar: `[dur(7)+restbit] [note(octave<<4 | semitone)]`; `$00` end; `$FF cc aa` command
  (cc in 0..4). Map cmd0 -> program/volume change, cmd1 -> tempo, cmd2-4 -> duty/detune/volume.
- Instrument/envelope records: 12 bytes each at `$FDCB`/`$FDD2` (file `0x1FDCB`/`0x1FDD2`).

## Open items

- Exact semantics of header param bytes `[+6,+7]` (e.g. `BA 00`, `13 13`, `80 03`) - likely
  initial duration + transpose/loop-count; needs runtime trace to confirm.
- Precise tempo math in cmd1 (`$FBE2`, the `$A0,x` length multiplier) for accurate MIDI timing.
- Exact total song count in bank 0x0C and whether banks 0x0B/0x0D hold continuation data for
  long songs (stream pointers cross $A000).
