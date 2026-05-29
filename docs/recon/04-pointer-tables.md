# Recon 04 — Pointer / Jump Tables and Major Data Regions

ROM: `rom/lotw.nes` (Legacy of the Wizard / Dragon Slayer IV: Drasle Family, MMC3).
File layout: header `[0,0x10)`, PRG-ROM `[0x10, 0x20010)` = 128 KiB (16 × 8 KiB banks),
CHR-ROM `[0x20010, 0x30010)`.

All "prg+0x….." offsets below are **PRG-relative** (subtract 0x10 from the file offset, or add
0x10 to get the file offset). CPU addresses assume MMC3 8 KiB windows: `$8000-$9FFF`, `$A000-$BFFF`,
`$C000-$DFFF` are swappable; `$E000-$FFFF` is **always** the last bank (bank 15).

Bank ↔ PRG offset: `bank b` occupies `prg+0x{b*2000}` .. `prg+0x{(b+1)*2000}` (8 KiB each).

## Method

Read raw PRG bytes with `python3`. Scanned for runs of little-endian 16-bit words whose value lies
in `$8000-$FFFF` (plausible pointers). Naive scanning produces huge false-positive "runs" because
uniform graphics/RLE data (e.g. bytes `9E 9E 9E …`, `C9 C9 C9 …`) parse as repeated words like
`0x9E9E`. To separate real pointer tables from data, applied successively stricter filters:

1. value diversity (distinct/total) and "hi-byte == lo-byte" fraction (rejects uniform fills);
2. **single 8 KiB target window** (real per-bank tables point inside one window);
3. **strictly increasing** distinct values (a near-certain table signature).

## Headline result: the bank-10 two-level pointer table (best candidate)

**`prg+0x14000` (file `0x14010`), at the very start of bank 10.** This is the single cleanest,
largest, most regular pointer structure in the ROM. It is a **data pointer table** (two-level
record table), *not* a code jump table.

Layout (LE16 words):

```
prg+0x14000  80xx group (11 words): 8062 80A2 8102 8082 80E2 8142 80C2 8122 8162 8182 8122
prg+0x14016  B3xx..B9xx group (37 words, strictly ASCENDING): B3AB B3BB B3D4 B3E7 B3FE B415 ...
                                                                ... B980 B9D6 B9E9   (span 0x063E)
prg+0x1405E  trailing: B6ED 8004 81A2 81A2 ...   (back-references / start of records)
```

- The **80xx group** points into this same bank when mapped at `$8000-$9FFF`. Resolving e.g.
  `$8062 -> prg+0x14062` yields `04 80 A2 81 A2 81 BA 00` — i.e. records that *themselves* contain
  pointers (`$8004`, `$81A2`, `$81A2`) terminated by a marker (`00BA`/`BA 00`). So this is a table
  **of pointers to pointer-records** (structured object/entity records).
- The **B3xx group** points into the same bank mapped at `$A000-$BFFF`. Resolving `$B3AB ->
  prg+0x153AB` yields tuple data `06 30 86 06 20 86 06 20 86 …` — repeating 3-byte records, not
  code. This is a table of pointers to **byte-data records** (likely sprite/animation/metatile or
  per-object property streams).

Confidence this is a real table: **high** — it sits on a bank boundary, is strictly ascending across
37 entries inside one window, and every resolved target lands on plausible structured data.

## Other strictly-increasing single-window candidates (weaker)

From the strictest filter (strictly increasing, distinct, single window, ≥8 entries):

| prg off | file off | bank | count | window | range | note |
|---|---|---|---|---|---|---|
| 0x14016 | 0x14026 | 10 | 37 | $A000-$BFFF | $B3AB..$B9E9 | sub-table of the headline structure |
| 0x12583 | 0x12593 | 9  | 10 | $E000-$FFFF | $E2F1..$EAF9 | small table, data-heavy bank 9 |
| 0x1EAAD | 0x1EABD | 15 | 10 | $E000-$FFFF | $EAFD..$F0A5 | in fixed last bank; near code |
| 0x1253F | 0x1254F | 9  |  9 | $A000-$BFFF | $A000..$A8B7 | small table in bank 9 |
| 0x17C98 | 0x17CA8 | 11 |  8 | $8000-$9FFF | $914C..$9F9E | three parallel windowed runs (see below) |
| 0x17CB8 | 0x17CC8 | 11 |  8 | $A000-$BFFF | $A15C..$AFAE | parallel run, same deltas (span 0xE52) |
| 0x17D71 | 0x17D81 | 11 |  8 | $C000-$DFFF | $D001..$DEDD | parallel run, same deltas (span 0xE52) |

The three bank-11 runs at `0x17C98 / 0x17CB8 / 0x17D71` share an identical span (0x0E52), suggesting
parallel record arrays. Confidence: **medium** (real but small).

## False positives explicitly ruled out

Large "runs" reported by the naive scan that are **NOT pointer tables**:

- `prg+0x03544` (file 0x03554), 120 bytes of `0xAF` → uniform data, parses as `0xAFAF` repeated.
- `prg+0x0840B` `C9 C9 …`, `prg+0x0740B` `9E 9E …`, `prg+0x126BF` `E8 E8 …` → uniform/RLE graphics
  data, not pointers.
- `prg+0x1A91F`, `prg+0x1B198` (last bank) → **6502 code**, e.g. `A9 CC 85 12` = `LDA #$CC; STA $12`;
  `A9 F0 85 20` = `LDA #$F0; STA $20`. These parse as "pointers" by accident. Bank 15 is code.

## Uniform / zero (padding & bulk-data) regions

Runs of one repeated byte, length ≥ 32 (top of list):

| prg off | file off | len | byte | interpretation |
|---|---|---|---|---|
| 0x13C00 | 0x13C10 | 768 | 0x00 | zero padding at end of bank 9 |
| 0x1BD89 | 0x1BD99 | 539 | 0x00 | zero padding (bank 13) |
| 0x10980 | 0x10990 | 384 | 0x42 | uniform data block (bank 8) |
| 0x10314/0x10714/0x10B14/0x10F14 | +0x10 | 204 ea | 0x00 | four equal-stride zero blocks in bank 8 (regular 0x400 stride → table/grid rows) |
| 0x108C0 | 0x108D0 | 192 | 0x5F | uniform data (bank 8) |
| 0x1A1E1 / 0x19EC9 | +0x10 | 168 / 100 | 0x1F | uniform data (bank 13) |
| 0x13800/0x138BF/0x1397F/0x13A3F | +0x10 | ~104 ea | 0x40 | regular-stride uniform blocks in bank 9 |

The repeated equal-stride blocks in **bank 8** (`0x10314`, `0x10714`, `0x10B14`, `0x10F14`, stride
0x400) and **bank 9** (`0x40`-fills at stride ~0xC0) indicate fixed-size record/grid tables, not
code.

## Code vs data boundaries (bank-level)

Per-8 KiB-bank statistics:

| bank | prg off | zero% | 0xFF% | distinct bytes | character |
|---|---|---|---|---|---|
| 0-7 | 0x00000-0x0E000 | ~11% | 0% | 160-200 | mixed code+data |
| 8 | 0x10000 | 18% | 0% | 216 | **data-heavy** (record/grid tables, uniform blocks) |
| 9 | 0x12000 | 29% | 0% | 256 | **most data-heavy** (zero padding, fill blocks, small tables) |
| 10 | 0x14000 | 5% | 6% | 161 | starts with the headline pointer table; dense data |
| 11 | 0x16000 | 10% | 7% | 238 | mixed (parallel record arrays at 0x17Cxx) |
| 12 | 0x18000 | 13% | 6% | 178 | mixed |
| 13 | 0x1A000 | 12% | 0% | 244 | mixed code+data (uniform 0x1F blocks) |
| 14 | 0x1C000 | 2% | 0% | 253 | **code-heavy** (very low zero%, high distinct) |
| 15 | 0x1E000 | 3% | 0% | 254 | **code-heavy / fixed last bank** (CPU vectors, confirmed code) |

No bank is dominated by ASCII (max printable ratio 42%); this game encodes text as tiles, so do not
expect ASCII string tables.

## CPU vectors (last bank, fixed at $E000-$FFFF)

Read from `prg+0x1FFFA..0x1FFFF` (file 0x3000A..0x3000F):

- **NMI = $D1FE**
- **RESET = $FFE0**
- **IRQ = $D1FE** (NMI and IRQ share the same handler address $D1FE)

These point inside bank 15 (`$E000-$FFFF`), confirming bank 15 holds the reset/NMI/IRQ code; the
near-vector candidate table at `prg+0x1EAAD` lives in this code bank.

## Summary of best leads for further RE

1. **`prg+0x14000` two-level data pointer table** (bank 10) — highest priority; decode the 80xx
   record format (pointer-records ending `BA 00`) and the B3xx 3-byte-tuple data records.
2. Bank 8/9 fixed-stride zero/fill grids — likely level/map or stat tables.
3. Bank 11 parallel record arrays at `0x17C98 / 0x17CB8 / 0x17D71` (split lo/mid/hi or 3 fields).
4. Vector handlers at $D1FE (NMI/IRQ) and $FFE0 (RESET) in bank 15 for control-flow rooting.
