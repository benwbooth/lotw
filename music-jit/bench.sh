#!/usr/bin/env bash
# Measure the live-edit reload latency: write the song source, recompile the
# `music-jit` cdylib, dlopen it, render every song/SFX. Run inside `nix develop`.
#
# The key to speed is NOT cranelift or the codegen backend — rustc's frontend +
# codegen for the whole song file is ~30 ms. It's the *link* step: the dev
# shell's NIX_LDFLAGS is ~30 KB (every Qt/SDL/GL/KDE lib path) and the linker
# wrapper grinds through all of it (~1 s) even though the JIT only needs std.
# Clearing NIX_LDFLAGS + using mold + no debuginfo brings a full rebuild to
# ~200 ms on stable rustc via cargo, or ~130 ms if the server invokes rustc
# directly (cargo's fingerprinting is ~64 ms of pure overhead for one crate).
# Breakdown: rustc startup+frontend+codegen ~60 ms, link (mold, clean env)
# ~40 ms. Song count barely matters (one song ~175 ms vs all 60 ~200 ms), so no
# need to compile per-song. cranelift would not help — codegen was never the
# cost.
set -euo pipefail
cd "$(dirname "$0")/.."

export NIX_LDFLAGS=""
export RUSTFLAGS="-C debuginfo=0 -C link-arg=-fuse-ld=mold"

cp src/music/songs.rs music-jit/src/songs.rs
t0=$(date +%s%N)
cargo build -p music-jit >/dev/null 2>&1
echo "warm build: $(( ($(date +%s%N) - t0)/1000000 )) ms"

for i in 1 2 3 4 5; do
  printf '// live edit %d %s\n' "$i" "$RANDOM" >>music-jit/src/songs.rs
  s=$(date +%s%N)
  cargo build -p music-jit >/dev/null 2>&1
  ms=$(( ($(date +%s%N) - s)/1000000 ))
  so="/tmp/jit_$i.so"
  cp target/debug/libmusic_jit.so "$so"
  val=$(python3 -c "import ctypes,sys; l=ctypes.CDLL(sys.argv[1]); l.render_total_bytes.restype=ctypes.c_uint64; print(l.render_total_bytes())" "$so")
  echo "edit $i: rebuild ${ms} ms   render=${val} bytes"
done
