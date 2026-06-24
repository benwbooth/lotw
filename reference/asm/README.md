# LotW 6502 reference disassembly

Reference disassembly of `rom/lotw.nes` (the NES game this project ports), kept
on hand for comparing Rust routines against the original assembly. Regenerate
with `tools/gen_reference.sh`.

## Layout

- `bank-NN.asm` — one file per 8KB MMC3 PRG bank (16 total). Banks 0–13 are
  switchable and are shown at base `$8000`; bank 14 is fixed at `$C000`, bank 15
  (reset/NMI vectors) at `$E000`. Internal branches are base-relative, so a
  switchable bank's structure reads correctly even though it can also map at
  `$A000`.
- `fieldmap.txt` — `$00XX <field>`: zero-page offset ↔ `GameState` field name
  (from the `offset_of!` asserts in `src/state.rs`). `engine.state.<field>`
  corresponds to 6502 zero-page `$XX`.
- `zp_xref.txt` — for each zero-page `$XX`, the instruction addresses that
  reference it. Use it to locate a routine by its distinctive memory accesses.
- `jsr_targets.txt` — every `JSR` target address (i.e. routine entry points).

## Finding a Rust routine's 6502 code

1. Read the Rust routine; map its `engine.state.<field>` accesses to `$XX` via
   `fieldmap.txt`, and note its immediate constants.
2. Look up 2–3 distinctive `$XX` in `zp_xref.txt`; the address where they
   co-occur is the routine.
3. Disassemble it cleanly from that entry with the on-demand tool:

   ```
   LOTW_ROM=rom/lotw.nes python3 tools/dis6502.py <START_HEX> <END_HEX>   # $C000–$FFFF
   LOTW_ROM=rom/lotw.nes python3 tools/dis6502.py bank <N> <BASE_HEX>     # any 8KB bank
   ```

## Caveat

The per-bank files are a **linear** disassembly: where data bytes are
interpreted as code the instruction stream desyncs until it realigns. When a
region looks wrong, re-run `dis6502.py` starting at a known entry point (a
`JSR` target) to re-sync. The tool itself is verified — it reproduces the
`$D8E3` / `$8C76` routines exactly.
