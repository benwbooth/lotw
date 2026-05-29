# Code map — matching disassembly completeness

*Legacy of the Wizard* (NES, MMC3). The matching disassembly in `disasm/`
re-assembles to a **byte-identical ROM** (`make -C disasm verify` →
sha `079f648d…`). This documents what is code vs data after the
"decompile all the code" pass.

## Where the code is

All executable code lives in **banks 13, 14, 15**. Banks 0–12 are pure data
(room maps, metatiles, music, title/credits nametables) — verified by decode
sampling and the absence of any `$8000`-window call/jump target anywhere.

| Unit | CPU window | Code | Data | Code % |
|---|---|---|---|---|
| Fixed banks 14+15 | `$C000-$FFFF` | 15,690 B | 694 B | **95.8%** |
| Bank 13 (swappable) | `$A000-$BFFF` | 4,914 B | 3,278 B | 60.0% |
| Banks 0–12 | data | — | 98,304 B | 0% |

The residual "data" inside the code banks is **genuine data**, not missed code:

- Fixed (694 B): jump/pointer tables (`$D244` NMI dispatch, `$DB06/$DB16` item
  handlers, `$EAAD` boss states, `$F033` phase, `$FBBB` sound commands),
  `note_period_table` + sound descriptors (`$FDB1`), HUD/menu text (`$FC00`,
  tile = ASCII+`$A0`), CPU vectors (`$FFFA`), reset padding, and a couple of
  unreferenced dead fragments (`$C034`, `$D5F2`).
- Bank 13 (3,278 B): metasprite/sprite/animation tables, title/credits
  nametable data, ending-credits text.

So **all genuine code is recovered as instructions**; the rest is delimited with
`; ==== name (kind) @ $addr ====` data-region comments.

## How completeness was reached

1. **Dynamic coverage** — FCEUX traces over fixtures + exploration personas
   (`tools/fceux_coverage.lua`, `run_coverage.py`) → confirmed-executed code.
2. **Recursive descent** from coverage + verified anchors (`disasm6502.py`).
3. **Far-call target extraction** — light dataflow resolves `STA $0E/$0F` +
   dispatcher `JSR` into cross-bank targets (banks 12/13).
4. **Cross-window xref routing** — direct `JSR`/`JMP` between the simultaneously
   mapped resident banks (`$A000-$FFFF`).
5. **Jump-table target recovery** — the completeness-audit workflow decoded every
   residual `.byte` run and extracted indirect/jump-table targets (the sound
   engine + boss state machines), seeded via `disasm/entries.txt`.

Coverage plateaued at ~6.6% of PRG (random play can't progress far in this game),
so steps 3–5 (static, deterministic) did most of the completion. The byte-identical
round-trip is the safety net: any decode that fails to re-encode breaks the build.

## Known caveats / next

- A few far-call / indirect targets set up non-literally aren't auto-resolved;
  none produced ambiguous `$8000`-window targets, so no code is believed missing.
- Bank 13 still has a handful of unlabeled entry points (decoded as code but no
  `L_`/named label). Naming/structuring is ongoing (`tools/re/symbols.py`).
- This is the *matching* (asm) stage. Stage 2 = readable C port per system,
  differential-tested (see `docs/PLAN.md`).
