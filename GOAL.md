# Goal

Create a pixel-perfect PC port of `Legacy of the Wizard` as native Rust code:
same art, music, and gameplay behavior, with SDL for PC presentation and a
2A03-compatible audio path. Do not turn the runtime into a general NES emulator.

## Current Strategy

1. Keep the shipping target in Rust: `crates/lotw-runtime`.
2. Keep the codebase Rust-only. Disabled proof commands must be rewritten in
   Rust before they are re-enabled.
3. Use Rust tooling in `crates/lotw-tools` for ROM discovery, replay parsing,
   reference capture, trace comparison, static CFG work, proof ledgers, and
   whole-program reporting.
4. Use the Rust goal runner directly through Cargo; there is no source shell
   script entrypoint.
5. Do not commit ROMs or generated ROM-derived assets.

## Current Status

- Rust workspace exists with `lotw-port`, `lotw-runtime`, and `lotw-tools`.
- `lotw-tools goal` is the automation entrypoint.
- The old script fleet and compatibility path have been removed.
- CHR graphics decode to both PPM and PNG through Rust code.
- A Rust macro DSL scaffold exists for future high-level 2A03 music/SFX
  conversion.
- `progress` reports game logic, CHR graphics PNG decoding, and music/SFX DSL
  conversion percentages from current artifacts, including the generated CHR
  PNG path and hash.
- `build` and `test` are now Cargo/Rust commands.
- `extract` uses Rust `lotw-tools rom-extract`.
- Rust verifier tests now compile small Rust fixture executables instead of
  embedding shell fixtures.
- Rust `block-exec` handles replay block execution and explicit synthetic state
  cases.
- The native block codegen/verifier command path is currently disabled until
  rewritten in Rust.
- `whole-program-report` now emits `whole_program_remaining_units.tsv`, which
  classifies remaining logic work instead of hiding it behind one generic
  unplanned/data-split bucket.
- The default static proof horizon now covers the next 2,048 static entry/audit
  rows and uses 64-case verifier batches, so the normal Rust goal command keeps
  advancing past the old 1,024-row frontier.
- Static/native proof reports currently show:
  - 129 static leaf blocks proved.
  - 516 static leaf synthetic cases matched.
  - 1,489 native units matched by oracle proof.
  - 5,412/5,412 native oracle cases matched.
  - 369 static handoff units proved.
  - 311 static branch handoff units proved.
  - 520 static JSR handoff units proved.
  - 24 static return-prefix units proved.
  - 37 planned static frontier units remain in the current 2,048-row horizon.
- The repeatable FCEUX reference hash harness has been verified under
  `nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal reference-hash-harness`.
  Current artifact: `build/reference_hash_harness/manifest.txt` with
  `replay_count=9`, `frame_hash_count=16`, `ram_hash_count=16`,
  `rom_sha256_count=1`, and `complete=1`.
- Current progress tracks:
  - Game logic: 65.34% (`1,489/2,279` known reachable native units proved).
  - Remaining game-logic units: 790 total, including 662 replay-covered split
    candidates, 5 labels inside already verified native spans, 3 leaf entries,
    11 control-flow entries, 13 call/subroutine entries, 4 straight-line
    entries, and 92 labels not in the current static entry plan.
  - CHR graphics PNG decoding: 100% (`4,096/4,096` CHR tiles) at
    `build/rust_chr_preview/chr_tiles.png`.
  - Music/SFX DSL conversion: 0% converted; Rust 2A03 macro DSL scaffold is in
    place, but no game music or SFX programs have been translated yet.

## Useful Commands

```sh
nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal status
nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal rust-rom
nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal rust-chr-preview
nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal progress
nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal test
```

## Next Work

1. Rebuild native block codegen and verifier helpers in Rust, then re-enable
   the disabled proof commands.
2. Work through the 37 planned frontier units now visible in
   `build/semantic_match_report/static_frontier_match_status.tsv`: 4 linear
   handoffs, 12 branch handoffs currently failing outcome gates, 3 call-like
   leaf entries, 17 JSR handoffs, and one native-opcode support gap.
3. Replace the headless CHR-preview runtime frame with progressively translated
   gameplay systems.
4. Add or integrate a Rust 2A03 audio backend once enough APU behavior is
   replayed by native translated logic, and convert captured music/SFX streams
   into the Rust 2A03 macro DSL.
