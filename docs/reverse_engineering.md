# Reverse-Engineering Notes

The goal of annotation in this repo is to turn the checked-in Rust port into
normal maintainable Rust without losing behavioral fidelity.

## Rules for Naming

Use the following bar before renaming a numbered routine or local variable:

1. Identify the persistent RAM addresses it reads and writes.
2. Identify its callers and whether it runs in the foreground loop or vblank.
3. Confirm whether `RoutineContext.value`, `index`, `offset`, or `carry` are
   inputs, outputs, or scratch.
4. Prefer a narrow test or replay capture when the routine mutates game state.
5. Preserve the numbered name as a comment or doc entry until all call sites in
   that subsystem have moved to semantic names.

## RoutineContext Conventions

`RoutineContext` is the translated register/scratch carrier used between
routines:

| Field | Typical meaning |
|---|---|
| `value` | accumulator-like input/output byte |
| `index` | x-index, slot offset, table index, or return index |
| `offset` | y-offset into a table, tile, or object slot |
| `carry` | boolean result for collision/resource checks |
| `zero`, `negative`, `overflow` | rarely used status bits preserved for port fidelity |

Do not give a context field a permanent semantic meaning globally. Its meaning
is local to the routine pair that passes it.

## Object Slots

Live actors, items, doors, and projectiles are 16-byte records under `0x0400`.
Most actor routines operate by copying one slot to scratch RAM `0xED..0xFC`,
mutating scratch, then copying it back.

Important offsets inside a slot:

| Slot offset | Scratch address | Meaning |
|---|---|---|
| `+0x00` | `0xED` | sprite/tile id and animation bits |
| `+0x01` | `0xEE` | active/state/lifetime byte |
| `+0x02` | `0xEF` | attributes/direction bits |
| `+0x03` | `0xF0` | tile replacement or movement scratch |
| `+0x04` | `0xF1` | cooldown/path scratch |
| `+0x05` | `0xF2` | health/damage threshold |
| `+0x06` | `0xF3` | timer/animation phase |
| `+0x08` | `0xF5` | x velocity low nibble |
| `+0x09` | `0xF6` | x velocity carry/sign |
| `+0x0A` | `0xF7` | y velocity |
| `+0x0B` | `0xF8` | damage/effect strength |
| `+0x0C` | `0xF9` | x sub-tile |
| `+0x0D` | `0xFA` | x tile |
| `+0x0E` | `0xFB` | y pixel |
| `+0x0F` | `0xFC` | extra y/sprite scratch |

The projectile bug fixed in `update_player_projectile_slot` came from violating this pattern:
the active byte was cleared in scratch but not copied back to the object slot.

## Foreground vs Vblank

Foreground code may wait for frames through `frame::frame_wait` and related
helpers. Vblank code enters through `vblank_commit`, saves `RoutineContext`,
commits PPU/APU/frame work, and restores the foreground context before return.

This is intentional interruption simulation. Avoid adding random yields inside
game routines; if a routine needs to span frames, model that wait explicitly in
the foreground task flow.
