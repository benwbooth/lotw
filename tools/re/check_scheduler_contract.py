#!/usr/bin/env python3
"""Check the C-port scheduler/NMI contract.

The replay harness should expose timing bugs, not paper over them with ad hoc
gameplay yields. Ported routines may use nes_frame_wait() for explicit NMI waits,
but they must not call the raw vblank hook or a manual input-poll yield.
"""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PORTED = ROOT / "src" / "ported"

FORBIDDEN = (
    "nes_input_poll_yield",
    "nes_vblank_wait(",
)


def main() -> int:
    failures: list[str] = []
    for path in sorted(PORTED.glob("*.c")):
        text = path.read_text()
        for needle in FORBIDDEN:
            if needle in text:
                failures.append(f"{path.relative_to(ROOT)} contains forbidden scheduler API {needle!r}")

    if failures:
        print("Scheduler contract failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("Scheduler contract ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
