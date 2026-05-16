# Audio Backend Notes

The port needs a Rust 2A03/APU path that can accept native game-code writes to
NES audio registers and, over time, replace raw write streams with high-level
music/SFX programs expressed through the Rust macro DSL in
`lotw_port::apu_program!`. The current automation produces reference input as
local generated event streams:

- `build/trace/<replay>/apu_writes.tsv`
- `build/audio_trace/<replay>/apu_events.tsv`

## Runtime Shape

The runtime-facing Rust API should stay small:

- initialize at the SDL audio sample rate;
- accept writes to `$4000-$4013`, `$4015`, and `$4017` with CPU-cycle timing;
- render interleaved audio samples into the SDL callback;
- reset deterministically for replay tests.
- expose a test-only way to advance/drain finalized samples deterministically.

## DSL Direction

- Use `apu_program!` for converted songs and sound effects.
- Keep raw APU register traces as the oracle while conversion is incomplete.
- Count converted DSL programs in `build/audio_dsl/manifest.txt` when those
  programs are added; `goal progress` reports that percentage.

## Next Step

Capture a reference APU stream:

```sh
nix develop --command cargo run --quiet --manifest-path Cargo.toml -p lotw-tools -- goal audio-trace
```

The remaining runtime task is to convert captured music and SFX behavior into
high-level Rust DSL programs, then feed those programs into a Rust 2A03 renderer.
The default/offline build still keeps a silent stub until the renderer exists.

References:

- https://slack.net/~ant/libs/audio.html
- https://github.com/jamesathey/Nes_Snd_Emu
- https://github.com/libgme/game-music-emu
