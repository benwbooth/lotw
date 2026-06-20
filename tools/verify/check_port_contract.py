#!/usr/bin/env python3
"""Check the native port runtime contract."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GAME_C_FILES = ROOT / "src" / "game"
IGNORED_TREE_PARTS = {".git", "build", "generated", "target", ".cache", "result"}

FORBIDDEN_RUNTIME_CALLS = (
    "ne" "s_" "input_" "poll_" "yield",
    "ne" "s_" "cpu_" "advance",
    "ne" "s_" "vblank_" "wait(",
    "nm" "i_" "handler(",
    "nm" "i_" "tail(",
)

FORBIDDEN_ASM_SUFFIXES = {".s", ".S", "." "asm", ".a" "65", ".ca" "65"}
FORBIDDEN_SOURCE_TEXT = (
    "65" "02",
    "m" "65" "02",
    "ca" "65",
    "ld" "65",
    "dis" "asse" "mbly",
    "matching " + "dis" "asse" "mbly",
    "dis" "asm/",
    "de" "comp",
    "u" "context",
    "swap" "context",
    "get" "context",
    "make" "context",
    "fc" "eux",
    "nm" "i_",
    "ne" "s_",
    "RAM" "8",
    "NES" "_MEM",
    "Re" "gs",
)
FORBIDDEN_SOURCE_PATTERNS = (
    re.compile("su" "b_[A-F0-9]{4}"),
    re.compile("L" "_[A-F0-9]{4}"),
    re.compile(r"\$[0-9A-Fa-f]{2,4}\b"),
)
SOURCE_TEXT_ROOTS = ("CMakeLists.txt", "README.md", "flake.nix", "docs/", "src/", "test/", "tools/")
SELF = "tools/verify/check_port_contract.py"
ALLOWED_GAME_CONTROLLER_SPINS = set()
ALLOWED_RAW_FRAME_WAIT = {
    "src/native/frame_wait_helpers.hpp",
    "src/ppu.c",
    "src/ppu.h",
}
ALLOWED_NATIVE_DIRECT_FRAME_COMMIT = {
    "src/native/routine_0058.cc",
}
ALLOWED_GAME_C_DIRECT_FRAME_COMMIT = set()
NATIVE_STATE_ACCESS_ALLOWLIST = {
    "GAME_MEM8(0x20)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x21)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x26)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x28)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x36)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x8C)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x8D)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x8F)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x90)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x85)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x58)": {"src/native/game_state.cc"},
    "GAME_MEM8(0x59)": {"src/native/game_state.cc"},
    "read_controllers": {"src/native/frame_wait_helpers.hpp"},
}


def relpath(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def active_source_files():
    seen: set[Path] = set()
    for root in SOURCE_TEXT_ROOTS:
        base = ROOT / root
        if root.endswith("/"):
            if not base.exists():
                continue
            for path in sorted(base.rglob("*")):
                if any(part in IGNORED_TREE_PARTS for part in path.relative_to(ROOT).parts):
                    continue
                if path.is_file() and path not in seen:
                    seen.add(path)
                    yield relpath(path), path
        elif base.exists() and base not in seen:
            seen.add(base)
            yield relpath(base), base


def repository_files():
    for path in sorted(ROOT.rglob("*")):
        if any(part in IGNORED_TREE_PARTS for part in path.relative_to(ROOT).parts):
            continue
        if path.is_file():
            yield relpath(path), path


def main() -> int:
    failures: list[str] = []
    for path in sorted(GAME_C_FILES.glob("*.c")):
        text = path.read_text()
        for needle in FORBIDDEN_RUNTIME_CALLS:
            if needle in text:
                failures.append(f"{path.relative_to(ROOT)} contains forbidden runtime API {needle!r}")

    for rel, path in repository_files():
        if Path(rel).suffix in FORBIDDEN_ASM_SUFFIXES:
            failures.append(f"{rel} is a repository ROM-derived source listing")
    for rel, path in active_source_files():
        if rel == SELF:
            continue
        lower_rel = rel.lower()
        for needle in FORBIDDEN_SOURCE_TEXT:
            if needle in lower_rel:
                failures.append(f"{rel} has forbidden legacy term in path {needle!r}")
        try:
            text = path.read_text()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        lower_text = text.lower()
        for needle in FORBIDDEN_SOURCE_TEXT:
            if needle in lower_text:
                failures.append(f"{rel} contains forbidden legacy term {needle!r}")
        for pattern in FORBIDDEN_SOURCE_PATTERNS:
            if pattern.search(text):
                failures.append(f"{rel} contains forbidden address-derived text {pattern.pattern!r}")

    for path in sorted(GAME_C_FILES.glob("*.c")):
        rel = str(path.relative_to(ROOT))
        if rel in ALLOWED_GAME_CONTROLLER_SPINS:
            continue
        text = path.read_text()
        if "do { read_controllers" in text:
            failures.append(f"{rel} contains game controller spin loop")
        if "lotw_frame_wait(" in text:
            failures.append(f"{rel} contains game frame wait")

    for rel, path in active_source_files():
        if rel == SELF or not rel.startswith(("src/", "test/", "tools/")):
            continue
        try:
            text = path.read_text()
        except UnicodeDecodeError:
            continue
        if "lotw_frame_wait(" in text and rel not in ALLOWED_RAW_FRAME_WAIT:
            failures.append(f"{rel} calls raw frame wait instead of coroutine helpers")
        if rel.startswith("src/native/"):
            if "routine_0058" in text and rel not in ALLOWED_NATIVE_DIRECT_FRAME_COMMIT:
                failures.append(f"{rel} uses direct frame-commit ABI instead of coroutine frame helpers")
            for needle, allowlist in NATIVE_STATE_ACCESS_ALLOWLIST.items():
                if needle in text and rel not in allowlist:
                    failures.append(f"{rel} uses raw native state {needle!r} instead of GameState/frame helpers")
        if rel.startswith("src/game/") and "routine_0058" in text and rel not in ALLOWED_GAME_C_DIRECT_FRAME_COMMIT:
            failures.append(f"{rel} adds a game frame-commit dependency instead of moving to src/native")

    if failures:
        print("Port contract failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("Port contract ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
