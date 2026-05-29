# Porting a routine (Stage 2)

Each engine routine becomes a self-contained, differential-tested C function.
The system is conflict-free: porting one routine touches only **two new files**,
so many routines can be ported in parallel.

## Steps

1. Pick a routine from `build/port_worklist.tsv` (easy leaves first: `easy_leaf=1`,
   small `size`, `deps=0`, `hw=0`).
2. Read its **real** disassembly in `disasm/bankfix.s` (or `disasm/bank13.s`).
   Use this, not a re-decode — addressing modes matter (`LDA ($0C),Y` ≠ `LDA $0C`,
   `INC $95,X` ≠ `INC $95`).
3. Write `src/ported/<name>.c` with the uniform ABI:
   ```c
   #include "ram.h"
   #include "regs.h"
   void <name>(Regs *r) {
       /* read inputs from r->a / r->x / r->y / r->c.., memory via RAM8(addr);
          write outputs back to r-> and RAM8(). Keep it readable. */
   }
   ```
   - Memory: `RAM8(addr)` (read/write). Hardware-register writes: `REG_W(addr, v)`.
   - Add named RAM symbols to `src/ram.h` as you learn them (mirrors `disasm/lotw.inc`).
4. Write `port/specs/<name>.json`:
   ```json
   { "name": "<name>", "addr": "CC64", "entry": "lda_a",
     "inputs": ["a"], "compare": ["ram"], "shaper": null, "notes": "..." }
   ```
   - `entry`: how the caller set the flags — `lda_a`/`lda_x`/`lda_y` (Z/N reflect
     that reg, the common case), `flags_input` (entry C/Z/N/V are real inputs,
     randomised), or `none`.
   - `compare`: which outputs must match — any of `ram`, `a`, `x`, `y`, `c`, `z`,
     `n`, `v`. Include every output the routine produces.
   - `shaper`: optional input constraint (e.g. `rng_count`) for routines that
     only terminate / behave on certain inputs.
5. Verify:
   ```sh
   nix develop --command python3 tools/re/bulkdiff.py --only <name> -n 30000
   ```
   Iterate until `PASS`. The harness runs the ORIGINAL bytes (m6502 oracle) vs
   your C on thousands of random states and compares the declared outputs.

## What needs more than a leaf

- **Sub-calls** (`deps>0`): port callees first, or model them. The oracle runs
  the real callees (they're in the mapped ROM), so your C must reproduce their
  effects — easiest once they're ported too.
- **Hardware** (`hw=1`): writes to `$2000-$401F`. Use `REG_W`; the host harness
  ignores them, so only compare RAM/regs the routine also produces. PPU/APU/
  controller *reads* need harness modelling (not yet built).
- **Spin-waits on NMI vars** (e.g. `$CC97` waits on `$28`): not isolatable — the
  oracle has no NMI. Port as a vblank-wait primitive; skip the diff-test or seed
  the awaited var to its post-NMI value.

## Worked examples
`src/ported/rng_update.c` (RAM out, loop, shaper), `sub_E41E.c` (reg out),
`sub_F233.c` (carry out, pointer deref), `inc16_95.c` (RAM out, `zp,X`).
