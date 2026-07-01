"""TAS composer: build a master input log segment by segment.

The product is a button-per-frame `.replay` that plays the game from power-on
through the objective DAG. Determinism makes this a pure search problem: from
the current prefix (a savestate), find a suffix that satisfies the next goal
predicate, stitch it on, repeat. Solvers, cheapest first:

  - `macro(...)`     scripted leg (walk N px, hold a button until a predicate)
  - `sample(...)`    N stochastic rollouts (policy- or distribution-guided),
                     keep the SHORTEST suffix that hits the goal

Goal predicates read privileged state (the TASer's RAM watch); the final log
is pure controller bytes. Key RAM offsets (zero page mirrors GameState):
  $40 character_index   $47/$48 map_screen_x/y   $49/$4A x_tile(+fine via $4A?)
  $58 health  $59 magic  $5A coins  $5B keys
  $51..$54 carried/equipped item slots   $60..$6F inventory counts (item ids 0..15)
"""

from __future__ import annotations

import numpy as np

import lotw_env

A, B, SELECT, START = 1, 2, 4, 8
UP, DOWN, LEFT, RIGHT = 16, 32, 64, 128
NOP = 0

# RAM offsets for predicates
CHAR, SCR_X, SCR_Y = 0x40, 0x47, 0x48
HEALTH, MAGIC, COINS, KEYS = 0x58, 0x59, 0x5A, 0x5B
INVENTORY = 0x60          # 16 bytes, item ids 0..15


class Composer:
    """Grows a master input log against a deterministic env."""

    def __init__(self, rom: str = "rom/lotw.nes", prefix: bytes = b""):
        self.env = lotw_env.Lotw(rom)
        self.log = bytearray(prefix)
        self.env.reset_replay(bytes(self.log))

    # ---- observation ----
    def state(self) -> dict:
        return self.env.state()

    def ram(self) -> bytes:
        return self.env.ram()

    def px(self) -> int:
        s = self.state()
        return s["player_x_tile"] * 16 + s["player_x_fine"]

    # ---- log management ----
    def mark(self) -> int:
        """Checkpoint the current log length (segment start)."""
        return len(self.log)

    def rewind(self, mark: int):
        """Drop everything after `mark` and replay back to it."""
        del self.log[mark:]
        self.env.reset_replay(bytes(self.log))

    def save(self, path: str, header: str = ""):
        names = [("A", A), ("B", B), ("select", SELECT), ("start", START),
                 ("up", UP), ("down", DOWN), ("left", LEFT), ("right", RIGHT)]
        out, i, data = [], 0, bytes(self.log)
        while i < len(data):
            j = i
            while j < len(data) and data[j] == data[i]:
                j += 1
            btns = " ".join(n for n, b in names if data[i] & b)
            out.append(f"frame {j - i}{(' ' + btns) if btns else ''}")
            i = j
        hdr = "".join(f"# {ln}\n" for ln in header.splitlines()) if header else ""
        with open(path, "w") as f:
            f.write(hdr + "\n".join(out) + "\n")
        return len(data)

    # ---- solvers ----
    def do(self, button: int, frames: int):
        """Append `frames` of `button` unconditionally."""
        for _ in range(frames):
            self.env.advance(button)
            self.log.append(button)

    def macro(self, button: int, until, max_frames: int = 2000) -> bool:
        """Hold `button` until `until(self)` is true. Rewinds on failure."""
        start = self.mark()
        for _ in range(max_frames):
            self.env.advance(button)
            self.log.append(button)
            if until(self):
                return True
        self.rewind(start)
        return False

    def walk_to_px(self, target: int, tol: int = 1, max_frames: int = 3000) -> bool:
        """Walk horizontally to pixel-x `target` (auto direction)."""
        start = self.mark()
        for _ in range(max_frames):
            p = self.px()
            if abs(p - target) <= tol:
                return True
            b = RIGHT if target > p else LEFT
            self.env.advance(b)
            self.log.append(b)
        self.rewind(start)
        return False

    def sample(self, goal, actions: list[int], rng_seed: int = 0,
               tries: int = 200, horizon: int = 800, frame_skip: int = 4,
               guard=None) -> bool:
        """Stochastic search: rollouts of random actions (from `actions`) until
        `goal(self)`; keeps the SHORTEST winning suffix. `guard(self)` aborts a
        rollout early (e.g. death). Rewinds between tries; stitches the win."""
        start = self.mark()
        best: bytearray | None = None
        rng = np.random.default_rng(rng_seed)
        for _ in range(tries):
            self.rewind(start)
            suffix = bytearray()
            hor = horizon if best is None else min(horizon, len(best) // frame_skip)
            ok = False
            for _ in range(hor):
                b = actions[int(rng.integers(0, len(actions)))]
                for _ in range(frame_skip):
                    self.env.advance(b)
                    suffix.append(b)
                if guard is not None and guard(self):
                    break
                if goal(self):
                    ok = True
                    break
            if ok and (best is None or len(suffix) < len(best)):
                best = suffix
        self.rewind(start)
        if best is None:
            return False
        for b in best:
            self.env.advance(b)
            self.log.append(b)
        return True


# ---- common predicates ----
def in_room(x: int, y: int):
    return lambda c: (c.ram()[SCR_X], c.ram()[SCR_Y]) == (x, y)


def has_item(item_id: int, count: int = 1):
    return lambda c: c.ram()[INVENTORY + item_id] >= count


def is_char(idx: int):
    return lambda c: c.ram()[CHAR] == idx


def dead_or_left(c) -> bool:
    return c.ram()[CHAR] > 4
