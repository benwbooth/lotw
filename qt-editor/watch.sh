#!/usr/bin/env bash
# Auto-rebuild + restart the Qt editor whenever its Rust/QML sources change
# (like cargo-watch). QML-only edits trigger a near-instant no-op rebuild.
#
# Run inside the dev shell so Qt/QMAKE env is set:
#   nix develop -c ./qt-editor/watch.sh
set -euo pipefail
cd "$(dirname "$0")/.."   # repo root, so rom/lotw.nes resolves

exec watchexec \
  --restart \
  --watch qt-editor/src \
  --watch qt-editor/qml \
  --watch src/render.rs \
  --exts rs,qml \
  -- 'cargo build --manifest-path qt-editor/Cargo.toml && QT_QPA_PLATFORM=wayland exec qt-editor/target/debug/qt-editor'
