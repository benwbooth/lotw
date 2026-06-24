# LotW assets

Editable source for `rom/lotw.nes`. `assettool` (`cargo run --features assets
--bin assettool -- extract|build`) extracts the ROM into these files and rebuilds
a **byte-identical** ROM from them (the build asserts sha equality), so edits are
lossless and the FCEUX oracle keeps applying.

- `chr/bank-N.png` — pattern-table graphics (indexed PNG; pixel = 2-bit value).
- `palettes.json` — title + family palettes (NES indices + RGB).
- `font.json` / `text.json` — font map (A-Z=$E1..) + nametable text templates (HUD).
- `rooms/room-YY-X.csv` + `rooms/manifest.json` — 64 room metatile grids (64x12)
  plus each room's header, 12 actor-spawn records, and room palette.
- `audio.json` — 2A03 music streams as a note/command DSL (`note dur pitch`,
  `rest dur`, `cmd name arg`, `end`); compiles back byte-exact. DSL->binary is
  scripted; binary->musical-notation transcription is done by hand/agent.
- `header.bin` / `prg.bin` — iNES header + raw PRG; structured regions above
  overlay onto `prg.bin` on build, the rest (code, un-extracted data) is verbatim.

Rebuild + verify: `assettool build assets build/rebuilt.nes rom/lotw.nes`.
