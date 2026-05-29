# Recon SUMMARY — Legacy of the Wizard (lotw.nes)

Consolidated by the lead RE analyst from recon files `01`–`08`. All eight source files were
read in full and the load-bearing claims were re-verified against the raw ROM bytes
(`python3` on `rom/lotw.nes`). This file flags every spot where an agent's *file offset*,
bank attribution, or interpretation looked like a guess or an arithmetic slip.

ROM: `rom/lotw.nes` — *Legacy of the Wizard* (US, Broderbund). JP original *Dragon Slayer IV:
Drasle Family* (Falcom). 196,624 bytes = 16 B iNES header + 128 KiB PRG + 64 KiB CHR.

> **Offset conventions used below (and a recurring agent error to watch):**
> - **file** = absolute byte offset into `lotw.nes`.
> - **PRG** = offset within PRG-ROM. `file = PRG + 0x10`.
> - **CHR** = offset within CHR-ROM. `file = CHR + 0x20010`.
> - CPU addr -> PRG offset for the fixed banks: `$C000-$FFFF` maps to PRG `0x1C000-0x1FFFF`.
> - **Several agents miscomputed the file offset of the CPU vectors and the font.** The
>   PRG-relative numbers were right; the +0x10/CHR translations were wrong. Use the verified
>   values in section 1. This is the single most important correction in this summary.

---

## 1. Consolidated ROM map (verified)

### iNES header (file `0x00..0x10`)
`4E 45 53 1A 08 08 40 08 00 ...` — PRG 8x16 KiB = 128 KiB, CHR 8x8 KiB = 64 KiB, **mapper 4
(MMC3)**, horizontal mirroring (flags6 bit0=0), no battery/trainer/4-screen. flags7 bits2-3
= NES 2.0 identifier. **Verified.** (Charter says "horizontal"; the boot code then sets the
MMC3 mirroring register to **vertical** at runtime — see below. No contradiction: flags6 is
the power-on default, MMC3 overrides it.)

### Bank model (MMC3, PRG mode 0 / CHR mode 0 — verified)
The only `$8000` select immediates seen anywhere are `{0,1,4,5,6,7}` -> **bit6 (PRG mode) and
bit7 (CHR mode) are never set.** Therefore:

| CPU window | Size | Banking |
|---|---|---|
| `$8000-$9FFF` | 8 KiB | swappable via **R6** (zp shadow `$30`) |
| `$A000-$BFFF` | 8 KiB | swappable via **R7** (zp shadow `$31`) |
| `$C000-$DFFF` | 8 KiB | **FIXED to bank 14 (0x0E)** |
| `$E000-$FFFF` | 8 KiB | **FIXED to bank 15 (0x0F)** — holds vectors |
| PPU `$0000/$0800` | 2 KiB ea | R0/R1 (zp `$2A/$2B`) |
| PPU `$1000-$1C00` | 1 KiB x4 | R2-R5 (zp `$2C-$2F`) |

CHR = 64x1 KiB banks. **MMC3 register zero-page shadow is contiguous at `$2A..$31` (R0..R7),
with `$25` shadowing the `$8000` select latch.** The full register file is re-committed from
this shadow **every frame** in the NMI tail (`LD41D` @ `$D41D`). The authoritative live bank
in any window is the zp shadow, not individual `$8001` stores — any static tracker must model
`$2A-$31`. (Files 01 + 02 agree; verified by the consistent reset/NMI flow.)

### PRG bank file map
`bank N` = file `0x10 + N*0x2000` .. `0x10 + (N+1)*0x2000`. Banks 0-13 swappable; 14 fixed
`$C000`; 15 fixed `$E000`.

### CPU vectors — **CORRECTED file offset**
CPU `$FFFA-$FFFF` = PRG `0x1FFFA` = **file `0x2000A`** (NOT `0x1FFFA`, NOT `0x3000A` as
several agents wrote). Verified bytes at file `0x2000A`: `FE D1 E0 FF FE D1`.

- **NMI = `$D1FE`**, **RESET = `$FFE0`**, **IRQ = `$D1FE`** (NMI and IRQ share one handler).
- RESET stub at file `0x1FFF0` verified: `78 A9 00 8D 00 80 8D 01 A0 8D 00 E0 4C 00 C0` =
  `SEI; LDA #$00; STA $8000; STA $A001; STA $E000 (IRQ off); JMP $C000`.
- **Bank attribution caveat:** `$D1FE` lives in the `$C000-$DFFF` window = **bank 14 (0x0E)**.
  Files 01 and 02 say this correctly; **file 04's prose "vectors point inside bank 15 ...
  confirming bank 15 holds the NMI/IRQ code" is wrong** — bank 15 holds only the *vector
  bytes* and the RESET stub/sound driver; the NMI/IRQ handler body is in bank 14.

### Code vs data regions (cross-agent consensus, verified where checked)

| PRG / file | Bank(s) | Contents | Confidence |
|---|---|---|---|
| `0x00000-0x11FFF` / `0x00010` | 0-8 | **72 room blocks**, 0x400 stride: tile-map body (`+0x000`), props/enemy table, 32 B palette (`+0x3E0`) | HIGH (72/72 palettes verified) |
| `0x12000-~0x13FFF` / `0x12010` | 9 | **2x2 metatile-assembly table** + menu/status/password nametable text + dragon map (`PRG $13800`) | HIGH |
| `0x14000-0x17FFF` / `0x14010` | 10-11 | **music + SFX data (songs 0-9), sprite/metasprite tables**, parallel record arrays | HIGH |
| `0x18000-0x1BFFF` / `0x18010` | 12-13 | music (songs >=10), title screen, code (GAME OVER, sprite/OAM engine, credits) | HIGH |
| `0x1C000-0x1DFFF` / `0x1C010` | 14 | **fixed: main init, NMI/IRQ handler, bank dispatcher, per-frame committer** | HIGH |
| `0x1E000-0x1FFFF` / `0x1E010` | 15 | **fixed: sound driver, HUD labels, RESET stub, vectors** | HIGH |

CHR-ROM `file 0x20010-0x30010` = 4096 tiles. Three contiguous content regions (visual, not
yet code-correlated): backgrounds **banks 0-25**, fonts/title/Latin+kana **banks 26-35** (incl.
"1978 BRODERBUND SOFTWARE INC"), sprites/objects **banks 36-63**. ~91% non-blank; only banks
30-31 are padding. The ASCII font is CHR section 8 (CHR `0x8000` = **file `0x28010`**, *not*
`0x139088` as file 05 wrote — that absolute offset is past end-of-file; the PRG/CHR-relative
description is fine).

---

## 2. Does an existing community resource change the Stage 1 plan?

**Partially — for DATA only, not for CODE.** (File 08, verified.)

- **No public disassembly/decompilation of LotW / Dragon Slayer IV exists.** Two NES disasm
  collections (cyneprepou4uk, benljbrooks) were checked and do not include it. **All CPU/code
  RE, the password algorithm, and the sound driver remain ours to do from scratch.**
- **`lotwtool` (bbbradsmith, C#) + Data Crystal ROM/RAM maps already solve the data formats:**
  room/map layout, metatiles, enemy records, palettes, title, credits, and a RAM-map of
  game-state variables. These were independently re-derived by recon agents 04/05/07 and
  **match**, which raises confidence. Adopt the Data Crystal field layout (verified: credits
  ASCII at file `0x1B7AC`, title `0x1F`-fill at file `0x19ED9`) and label rather than blind-RE.
- Data Crystal's **per-room field map is more precise than the recon agents'**: it names
  `$300` metatile page, `$301` enemy CHR bank, `$305/$306` terrain CHR banks, `$307-30B`
  treasure, `$30B` music track, `$320-3AF` = 9 enemy slots x 16 B with named fields. This
  directly answers several recon "open" items (room CHR-bank selection; object record fields).
  **Net effect: Phase 4 (asset extraction) is largely unblocked for maps/CHR/palettes.**

Caveat: Data Crystal uses **PRG-relative (header-excluded) offsets**; convert with `+0x10`.
Trust level MEDIUM-HIGH (community wiki) — keep verifying each field against bytes.

---

## 3. Highest-confidence findings vs. open questions / gaps

### Highest confidence (multiple agents and/or byte-verified)
1. MMC3 mapper 4, PRG mode 0 / CHR mode 0; R6/R7 = `$8000/$A000`, banks 14/15 fixed.
2. Vectors NMI=IRQ=`$D1FE` (bank 14), RESET=`$FFE0` (bank 15). **File offset `0x2000A`.**
3. **Scanline IRQ is unused.** Zero `$C000/$C001/$E001` writes; the status-bar split is a
   software **sprite-0-hit poll + mid-frame CHR swap** inside the NMI (`LD36E`). Strong
   evidence (exhaustive store scan). Only NMI needs emulating for PRG control flow.
4. zp `$2A-$31` MMC3 shadow + per-frame `LD41D` committer; bank-switched far-call dispatcher
   `LCC9C` (`$CC9C`, target in `$0E/$0F`, forces R6=$0C/R7=$0D).
5. 72 room blocks @ 0x400 with `+0x3E0` 32-byte palette (72/72 verified) and `+0x380` object
   records; `+0x000` metatile-index body indexing the 2x2 table at PRG `0x12000`.
6. Text = tile-index encoding `byte = ASCII+0xA0` (verified: GAME OVER/RETRY/CONTINUE decodes
   cleanly). Draw primitive `$CCE4` with `$0E/$0F` pointer. Two formats (plain runs; nametable
   2 B/cell menus).
7. Custom Falcom sound driver in bank 15 (`$E000-$FFFF`); per-frame tick `$F89A` from NMI;
   song-init `$FC08`; 4-channel 8-byte descriptors; `[dur][note]` grammar with `$FF cc aa`
   commands; equal-tempered period table at `$FDB1` (verified file `0x1FDC1`).

### Cross-finding that resolves an "open" item
**File 04's "headline two-level pointer table" at PRG `0x14000` IS the sound engine's
song+SFX pointer tables** (file 06: song headers at `$8000`, SFX table at `$8014`, both bank
10). File 04 read it as a generic "object/entity record" table — that interpretation is
**superseded**: the `80xx` group = song-header pointers, the strictly-ascending `B3xx..B9xx`
group = SFX/stream-data pointers. *However* file 07 reads the SAME `0x14000` region as the
**metasprite animation/frame tables** (`06 NN 86` records at `0x153AB`). These two claims
**conflict** and both cannot be fully right for the same bytes — see open questions.

### Biggest open questions / gaps
1. **Bank 10 `0x14000` table identity conflict (HIGH priority).** Audio agent (06) says it's
   song/SFX pointers; graphics agent (07) says it's metasprite frame tables; pointer agent
   (04) says generic records. The `$8062`/`$80A2`/... header pointers and the `06 NN 86`
   records are real bytes, but their *purpose* is contested. Resolve by disassembling the
   consumers: sound init `$FC08` (bank 14/15) vs the OAM engine at PRG `0x1A400` (bank 13).
   One indexes `0x14000`; tracing the bank-swap + index call settles it. **This is the
   clearest case of two agents confidently labeling the same data differently.**
2. **Per-bank window assignment (`$8000` vs `$A000`) for swappable banks 0-13** — needed to
   pick the right disassembly origin. Default `$8000` is a *guess* for banks that may only
   ever page to `$A000` (e.g. bank 10 read as `$Bxxx` for SFX). Resolve by tracing R6/R7.
3. **Password / save algorithm — entirely unreversed.** Save block `$0300-$0321` is the
   payload (Data Crystal RAM map). The translation table at file `0x17d07` (verified: 0x01
   filler) maps input-char codes -> display tiles but is NOT the checksum. LSD4 generator
   (`lsd4.starfree.jp`) is the best oracle but was unreachable; retry via archive.org.
4. **Room tile-map body width/packing.** 896 bytes != clean 16x15 screen; Data Crystal says
   **64 cols x 12** (= 768) tile grid + props — *adopt this*, it likely resolves file 07's
   open "width" question (the 896 figure mixes grid + props/palette boundaries).
5. **CHR bank-per-room mapping** — which 1 KiB CHR banks load per room. Data Crystal names the
   room fields (`$301`,`$305`,`$306`) that hold these — adopt and verify.
6. **NMI dispatch jump table at `$D244`** (mode handlers) only partially decoded (file 02).
7. Sound: song count in bank 0x0C, descriptor param bytes `[+6,+7]`, cmd1 tempo math, the
   `index 5 = period 0x0000` hole (verified present: `...4d05 0000 0105...`).
8. Several agents' **absolute file offsets are unreliable** (vectors, font). Re-derive file
   offsets from PRG/CHR-relative values; do not trust the agents' `0x3000A`/`0x139088` numbers.

---

## 4. Recommended next steps

### Phase 2 — Tracer + coverage (do first)
1. Build/instrument an MMC3 emulator harness (charter notes `fceux` via `nix develop`).
   **Model only NMI (no MMC3 IRQ).** Track R0..R7 from the zp shadow `$2A-$31` and `$25`.
2. **Coverage trace** through boot -> title -> password screen -> in-game (each of the 5
   characters) -> menu/equipment -> shop/inn -> death/GAME OVER -> an enemy-drop. Capture
   per-PC execution and the live R6/R7 per region. This directly produces:
   - the **per-bank `$8000` vs `$A000` window assignment** (open Q2),
   - which routine indexes PRG `0x14000` (settles the audio-vs-sprite conflict, open Q1),
   - CHR-bank-per-room (open Q5), and the password code path (open Q3).
3. Log every `JSR $CC9C`/`$C833` with `$0E/$0F` target + forced banks to map far-call edges.

### Phase 3 — Disassembler
1. Seed labels from the verified anchors: RESET `$FFE0`, NMI/IRQ `$D1FE`, `LD41D $D41D`,
   `LD36E $D36E`, `LCC9C $CC9C`, `LCD08 $CD08`, `LC833 $C833`, `$CCE4` (text), `$F89A`/`$FC08`
   (sound). Disassemble the two fixed banks (14, 15) first — all are resident code.
2. Use the proposed ld65 config in `01-banks-vectors.md` section 4 (HEADER + CODE00..15 +
   CHRROM, concatenation reproduces file `0x00..0x30010` exactly). **Defer per-bank origin
   choice to the Phase 2 trace results** rather than defaulting everything to `$8000`.
3. Feed coverage from Phase 2 to da65 as code/data hints; da65 misaligned tables near `$FDB1`
   and `$FDB1`-adjacent helpers per file 02 — drive those with traced labels.

### Phase 4 — Asset extractors now unblocked
- **CHR tiles** — fully unblocked (decode verified; renders already in `assets/chr/`).
- **Room maps / metatiles / palettes / object+enemy records** — unblocked via the verified
  72-block layout + Data Crystal field map (`$300-$3AF`, `$3E0`). Build a room exporter that
  decodes body->metatile->CHR using the per-room CHR-bank fields.
- **Palettes** — unblocked (72 verified room palettes + 3 aux palette blocks at PRG
  `0x13BE0/0x17BE0/0x1BFE0`; confirm aux ones).
- **Music / SFX -> MIDI/PLAY-DSL** — unblocked enough to start: driver, grammar, and period
  table (file `0x1FDC1`) are mapped; finish tempo math + descriptor params during extraction.
- **Text** — unblocked: `byte = ASCII+0xA0`, font = CHR section 8 (file `0x28010`).
- **Still blocked:** password generate/validate extractor (needs Phase 2 trace of the
  bank-9/menu state machine + the `$0300-$0321` oracle).
