# Recon 07 - PPU / Graphics Data

ROM: `rom/lotw.nes` (Legacy of the Wizard / Dragon Slayer IV: Drasle Family).
iNES mapper 4 (MMC3). 16-byte header + 128 KiB PRG + 64 KiB CHR.

File-offset conventions used below:
- `file=` is the absolute byte offset in `lotw.nes`.
- `PRG+` is the offset inside PRG-ROM (`file = PRG + 0x10`).
- `CHR+` is the offset inside CHR-ROM (`file = CHR + 0x10 + 0x20000`).
- PRG bank N (8 KiB) = `PRG + N*0x2000`. CPU vectors: NMI=$D1FE, RESET=$FFE0, IRQ=$D1FE.

All offsets verified by reading raw bytes with python3 and by rendering CHR/room data to PNG
(`assets/chr/*.png`).

---

## 1. Room / screen blocks (0x400-stride) -- PRG+0x00000 .. PRG+0x11FFF

The ROM opens with **72 contiguous 0x400 (1024-byte) blocks** (`0x12000 / 0x400 = 72`), each
describing one room/screen of the Metroidvania map. Every block has the same three-part layout:

| Sub-region (within each 0x400 block) | Contents |
|--------------------------------------|----------|
| `+0x000 .. +0x37F` (896 bytes)       | Tile/metatile map body (metatile indices) |
| `+0x380 .. +0x3DF`                   | Object/entity spawn table (see below)     |
| `+0x3E0 .. +0x3FF` (32 bytes)        | NES palette block (background + sprite)   |

Verification: a palette-validity scan over every offset with `(off & 0x3FF) == 0x3E0` returns a
single contiguous run of 72 valid 32-byte palettes from `PRG+0x003E0` through `PRG+0x11FE0`.

### 1a. Palette block (`+0x3E0`, 32 bytes per room)
Textbook NES palette: byte 0 is the universal backdrop (always `0x0F` = black) and is repeated at
positions 0,4,8,12,16,20,24,28 (the "reps=8" signature). Bytes are all <= 0x3F. First 16 bytes =
4 background sub-palettes; last 16 bytes = 4 sprite sub-palettes.

Example, room 0 at `PRG+0x003E0` (file 0x003F0):
```
0F 11 05 30  0F 00 10 30  0F 02 10 30  0F 0A 17 37   ; 4 BG sub-palettes
0F 05 30 36  0F 05 26 30  0F 02 22 30  0F 0F 26 30   ; 4 sprite sub-palettes
```
Note `0F 05 30 36` / `0F 05 26 30` recur as the first two sprite sub-palettes in almost every
room (a shared player/common-sprite palette).

### 1b. Object / entity spawn table (`+0x380`, up to 4 records of 16 bytes)
Records start at `+0x380`, `+0x390`, `+0x3A0`, `+0x3B0`; `+0x3C0`/`+0x3D0` are zero padding. Only
the first 10 bytes (cols 0-9) of each 16-byte record are used; cols 10-15 are always 0x00 (verified
across all 204 non-empty records in the 72 rooms). Field census across rooms:

- col0: object type/flags, 9 distinct, dominated by 0x41/0x51/0x61/0x71 (high nibble 4-7).
- col1: small count/variant 1-3.
- col2: 60 distinct values -- looks like an X or map coordinate.
- col3: coarse position, multiples of 0x10 (0x10..0xA0) -- X pixel/sub-tile.
- col4: 24 distinct -- Y coordinate.
- col5: 1-5 -- count or kind.
- col6: 0x4D/0x5D/0x6D/0x7D family -- mirrors col0 low nibble = 0xD (likely a paired type byte).
- col7..col9: small enums (0-8) -- sub-type / palette / behavior.

Example, room 0 (`PRG+0x00380`):
```
61 03 2E 10 1E 01 6D 02 00 02 00 00 00 00 00 00
41 01 38 90 14 01 4D 02 00 02 00 00 00 00 00 00
61 03 1D 30 17 01 6D 02 02 02 00 00 00 00 00 00
```

### 1c. Tile-map body (`+0x000 .. +0x37F`)
896 bytes of metatile indices (values include 0xFC, 0x6F, 0xA2, 0x9E, 0x40, 0x58, 0x49, 0xB6, 0xCE
... i.e. > 0x3F so NOT raw CHR tile ids). They index the metatile table in section 2. 896 bytes is
larger than a single 16x15-metatile screen (240), so the body is either a taller scroll column or a
lightly-packed/run layout; exact row width was not pinned (candidate widths 14/16 do not give a clean
240). Rendering the body through the metatile table produces coherent structured tile pairs, confirming
the indices are metatile ids, but the precise stride/compression is an open question.

---

## 2. Metatile (2x2 tile-assembly) table -- PRG+0x12000 .. ~PRG+0x12FFF

Immediately after the 72 rooms, `PRG+0x12000` (file 0x12010) begins a dense table of **4-byte
metatile definitions**. Each metatile = four CHR tile ids forming a 16x16 block. The bytes here are
real CHR tile ids (high fraction > 0x3F).

Example, `PRG+0x12000`:
```
2E 2E 2F 2F   8A 9A 8B 9B   88 98 89 99   86 96 87 97   ...
```
i.e. metatile 0 = tiles {2E,2E,2F,2F}, metatile 1 = {8A,9A,8B,9B}, etc. Near `PRG+0x12F00` there is
a linear identity-style run (`00 10 01 11 0E 1E 0F 1F ...`) used as a default/scratch mapping. The
room body bytes (section 1c) index into this table. Table is roughly `0x12000..0x13000` (~1024
metatiles) before density drops at `PRG+0x13000`.

---

## 3. CHR-ROM tile banks (64 KiB) -- file 0x20010 .. 0x30010

Rendered to PNGs in `assets/chr/`:
- `chr_full_0.png`, `chr_full_1.png`  (CHR+0x0000..0x7FFF): **background / terrain tiles** -- town
  buildings, dungeon walls, doorways, plus readable label tiles "SHOP" and "INN".
- `chr_full_2.png` (CHR+0x8000..0xBFFF): an **ASCII font** (digits `0123456789`, full upper/lower
  alphabet, punctuation) followed by **character / enemy / item sprites** (humanoid figures, monsters).
- `chr_full_3.png` (CHR+0xC000..0xFFFF): additional sprites/tiles.

So CHR is split background-tiles (lower half) vs font+sprites (upper half), selected per-room via
MMC3 CHR bank registers (the 1 KiB CHR windows).

Additional single-image dumps: `chr_bank0_pt0.png`, `chr_bank0_pt1.png`, `chr_mid.png`,
`room0_mt16.png` (a metatile-decoded view of room 0 -- structurally coherent, confirming the metatile
scheme; exact bank/width for a clean screen is unresolved).

---

## 4. Metasprite / OAM assembly tables -- PRG bank 10 ($8000/$A000 window), PRG+0x14000 region

`PRG+0x1A400` (PRG bank 13) is the **OAM/sprite drawing code** (e.g. `AA BD 01 04` = `TAX; LDA $0401,X`
writing to OAM page $0400, `9D 01 04` = `STA $0401,X`, `4C 3C A4` = `JMP $A43C`). It consumes data
tables that live in PRG bank 10:

### 4a. Frame-pointer table -- PRG+0x14016 (CPU $A000-window, addresses $B3xx-$B9xx)
A list of little-endian pointers, ascending: $B3AB, $B3BB, $B3D4, $B3E7, $B3FE, $B415, ... $B9E9
(at least ~70 entries). With bank 10 mapped at $A000, $B3AB resolves to `PRG+0x153AB`, where the
data is a stream of repeating 3-byte records `06 NN 86` (constant 0x06 / 0x86 framing, middle byte
0x30/0x20/0x1B/0x2B/0x23/0x33... = tile id or attribute). The variable pointer deltas (16, 25, 19,
... bytes) mean variable-length per-frame metasprite definitions.

Example, frame `$B3AB` (`PRG+0x153AB`):
```
06 30 86  06 20 86  06 20 86  06 20 86  06 30 86  06 20 86 ...
```

### 4b. Animation/dispatch pointer table -- PRG+0x14000 (CPU $8000-window, addresses $80xx-$81xx)
11 entries: $8062, $80A2, $8102, $8082, $80E2, $8142, $80C2, $8122, $8162, $8182, $8122. With bank
10 at $8000, $8062 -> `PRG+0x14062`, where records are 8 bytes like
`04 80 A2 81 A2 81 BA 00` -- containing embedded pointers ($81A2) plus terminator-looking bytes
(`BA 00`, `FF FF`, `80 03`). This is the upper level of a multi-tier sprite-animation system
(animation -> frame-list -> 3-byte tile records in 4a).

---

## 5. Other data regions noted (lower confidence / likely non-graphics)

- `PRG+0x13000 .. ~PRG+0x1A000`: mixed low-density data -- pointer tables and probable text/script
  data (the font at CHR+0x8000 implies in-game text). Some embedded 32-byte palette blocks exist at
  `PRG+0x13BE0`, `PRG+0x17BE0`, `PRG+0x1BFE0` (valid palette signature, possibly menu/UI palettes).
- `PRG+0x1A025`: runs of `0x1F` bytes -- looks like a collision/attribute bitmap rather than CHR.
- `PRG+0x1A400 .. PRG+0x1B7FF` high `>0x3F` density is **6502 code** (sprite engine), not a tile
  table -- do not mistake for graphics data.

---

## Summary table of candidate graphics-data offsets

| What | PRG offset | file offset | Structure |
|------|-----------|-------------|-----------|
| 72 room blocks | 0x00000-0x11FFF | 0x00010-0x1200F | 0x400 each: body + objects + palette |
| Room palettes | each `+0x3E0` | each `+0x3F0` | 32 bytes, 8 NES sub-palettes |
| Room object table | each `+0x380` | each `+0x390` | up to 4 x 10-of-16-byte records |
| Metatile (2x2) table | 0x12000-~0x12FFF | 0x12010 | 4 bytes/metatile (CHR tile ids) |
| Metasprite frame ptrs | 0x14016+ | 0x14026+ | LE pointers -> 3-byte tile records |
| Metasprite frame data | 0x153AB+ | 0x153BB+ | variable-len `06 NN 86` records |
| Anim dispatch ptrs | 0x14000 | 0x14010 | 11 LE pointers -> 8-byte records |
| Extra palette blocks | 0x13BE0,0x17BE0,0x1BFE0 | +0x10 | 32-byte NES palettes |
| BG/terrain CHR | CHR 0x0000-0x7FFF | 0x20010-0x27FFF | tiles (town, dungeon, "SHOP"/"INN") |
| Font + sprite CHR | CHR 0x8000-0xFFFF | 0x28010-0x3000F | ASCII font + character/enemy sprites |
