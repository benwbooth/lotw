# Decompilation plan

## Target

*Legacy of the Wizard* (USA), NES, mapper 4 (MMC3), 128 KiB PRG + 64 KiB CHR.
sha256 `079f648d669966357fe4414a986573eacd7ecadf5c4f289c288427b8c5f491f1`.

## Strategy

Two stages, each with a hard, automatable verification loop.

### Stage 1 — Matching disassembly (6502 asm ⇒ byte-identical ROM)

A hand-assembled NES game cannot be reproduced byte-for-byte by any C compiler,
so the *matching* artifact is **ca65 assembly** that `ld65` links to a ROM whose
sha256 equals the original. This is the ground truth for everything downstream.

Approach:

1. **Bank/segment model.** MMC3 fixes the last 8 KiB PRG bank at `$E000-$FFFF`
   (holds the reset/NMI/IRQ vectors) and the bank before it at `$C000-$DFFF`;
   `$8000-$9FFF` and `$A000-$BFFF` are swappable 8 KiB windows. Build an
   `ld65` config with one segment per 8 KiB PRG bank + the CHR banks, so the
   assembler reproduces the exact byte layout.
2. **Code/data classification.** Combine:
   - *Dynamic coverage*: run the game in FCEUX over the replay fixtures (plus
     longer scripted play) logging executed `PC` + active bank ⇒ a code/data
     log (CDL-style). Bytes executed as opcodes are code; bytes read as
     operands/data are data.
   - *Static recursive traversal*: from the reset/NMI/IRQ vectors and every
     discovered call/jump target, follow control flow, tracking MMC3 bank state.
   The union, reconciled, gives a per-byte code/data map. Unreached bytes are
   conservatively treated as data until proven code.
3. **Disassemble + label.** Emit ca65 syntax with auto-labels, then progressively
   replace labels with meaningful names backed by evidence (see `docs/SYMBOLS.md`).
   Data tables get structured directives (`.byte`/`.word`/`.addr`).
4. **Round-trip gate.** `disasm/` assembles and links; CI-style check asserts
   the output sha256 matches the ROM. Nothing is "done" until this passes for
   the whole ROM.

### Stage 2 — Readable C port + open assets

Once a region is understood in asm, rewrite it in C and extract its assets.

1. **Assets** (parallelizable, high value, mostly independent of code):
   - CHR-ROM → PNG tile sheets (4096 tiles, 2bpp). *(attempt 1 reached 100%; redo cleanly.)*
   - Palettes → PNG swatches + data.
   - Metasprite/OAM assembly tables → composed sprite PNGs.
   - Room/level/map data → decoded room PNGs + a documented map format.
   - Text/password system → decoded strings + password algorithm.
   - Music/SFX: reverse the sound engine, then convert song/SFX data → MIDI and
     a PLAY-like DSL. Keep raw APU register traces as the oracle meanwhile.
2. **Code**, system by system (input, RNG, player physics, collision, camera/
   scrolling, enemy AI, items/inventory, doors/room transitions, HUD, sound
   driver). Each C module is verified by **differential testing**: drive both
   the original (in FCEUX) and the C reimplementation with identical input
   streams from the fixtures and assert matching RAM/PPU/APU effects.

## Verification

- Stage 1 gate: assembled ROM sha256 == original. (binary, non-negotiable)
- Stage 2 gate: per-system frame-accurate RAM/PPU/APU match on fixtures.
- Symbol naming gate: names require concrete evidence (trace/callsite/constant),
  never address-shape guesses. See `docs/SYMBOLS.md`.

## Phases (orchestrated)

- **P0 Foundation** *(done)*: archive attempt 1, fresh tree, cc65 + fceux toolchain,
  ROM pinned, plan written.
- **P1 Recon** *(done)*: parallel read-only analysis → `docs/recon/` knowledge base.
  No public code disasm exists; Data Crystal + lotwtool solve the DATA formats.
- **P2 Tracer + coverage** *(done)*: `tools/fceux_coverage.lua` + `run_coverage.py`.
  9-fixture sweep → `build/coverage/merged_coverage.tsv`. Code lives in banks 13/14/15.
- **P3 Disassembler** *(in progress)*: `tools/re/disasm6502.py` + `gen_disasm.py` emit
  byte-exact ca65. `make -C disasm verify` links a **byte-identical ROM**. Banks 14+15
  treated as one contiguous fixed `$C000-$FFFF` unit (~85% recovered as instructions);
  swappable code bank 13 partially; data banks stay `.byte`. Remaining: widen coverage
  (more gameplay → deeper code), resolve the bank-10 `0x14000` table identity, then
  progressively name/structure code + data tables while keeping the round-trip green.
- **P4 Asset extraction**: CHR/palette/metasprite/room/text/audio extractors (mostly
  unblocked via Data Crystal field maps).
- **P5 C port**: per-system reimplementation under differential test.

## Salvage from attempt 1

- `fixtures/reference/*.replay` — 9 gameplay replays (coverage).
- `tools/fceux_capture.lua`, `tools/fceux_trace.lua` — capture scaffolding.
- Reference links: Nes_Snd_Emu, game-music-emu (APU emulation for audio diffing).
- Everything else lives at tag `attempt-1` if needed.
