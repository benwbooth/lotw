#!/usr/bin/env bash
# Auto-rebuild + restart the Qt editor whenever its Rust/QML sources change
# (like cargo-watch). QML-only edits trigger a near-instant no-op rebuild.
#
# Run inside the dev shell so Qt/QMAKE env is set:
#   nix develop -c ./qt-editor/watch.sh
set -euo pipefail
cd "$(dirname "$0")"

exec watchexec \
  --restart \
  --watch src \
  --watch qml \
  --watch ../src/render.rs \
  --exts rs,qml \
  -- 'cargo build && QT_QPA_PLATFORM=wayland exec ./target/debug/qt-editor'
