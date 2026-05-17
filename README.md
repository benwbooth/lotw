# Legacy of the Wizard Rust Port

This repository is a Rust-first PC port effort for the NES game `Legacy of the
Wizard`. The target runtime is `crates/lotw-runtime`: Rust code, SDL for the
window path, and Rust-owned tooling for ROM validation, replay parsing, trace
comparison, static analysis, and proof reports.

This is not an emulator project. The target is Rust-only tooling and a native
Rust runtime while game behavior is reconstructed from verified traces.

## Layout

- `crates/lotw-port`: shared Rust library for ROM, CHR, replay, trace, video,
  and runtime support.
- `crates/lotw-runtime`: Rust PC runtime. Use `--features sdl` for a window.
- `crates/lotw-tools`: Rust command-line tooling and the `goal` runner.
- `tools/`: FCEUX Lua scripts for reference trace capture.

## Quick Start

```sh
nix develop
cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal status
cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal rust-rom
cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal progress
cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal test
```

Run the Rust SDL window:

```sh
nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal run
```

## Main Commands

- `build`: Cargo workspace build.
- `test`: source audit, `cargo fmt --check`, `cargo test --workspace`, and
  clippy including the SDL runtime feature.
- `run`: Rust SDL runtime.
- `progress`: writes game logic, raw CHR tile decode, assembled sprite PNG,
  room/background PNG, trace-frame render, and music/SFX DSL conversion metrics
  to `build/progress/progress_summary.txt`.
- `rust-runtime`: headless Rust runtime capture.
- `rust-port-capture`: Rust runtime frame/trace report.
- `rust-trace-compare`: FCEUX reference trace compared with Rust runtime output.
- `extract`: Rust ROM artifact extraction into `build/generated`, including
  CHR PNG output.
- `static-*`, `native-block-*`, `whole-program-report`: analysis and proof
  pipeline. Commands whose native-code proof path has not been rewritten are
  disabled instead of falling back to another language.
- `block-exec`: Rust block execution oracle for replay and explicit-state block
  execution.

Rust unit tests use Rust-compiled fixture executables for command-path checks;
they do not embed shell fixtures.

ROM files and generated ROM-derived artifacts are intentionally ignored and must
not be committed.
