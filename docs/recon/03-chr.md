# 03 — CHR-ROM Characterization

ROM: `rom/lotw.nes` (Legacy of the Wizard / Dragon Slayer IV: Drasle Family).
CHR-ROM region: file offset **0x020010 .. 0x030010** (131088..196624), **65536 bytes** =
**4096 tiles** (8x8, 2bpp, 16 bytes/tile) = **64 CHR banks** of 1 KiB / 64 tiles each (MMC3 banks CHR in 1 KiB units).

Tile decode: standard NES 2bpp planar. For each row y in 0..7, plane0 = byte[y], plane1 = byte[y+8];
pixel = bit(plane0) | (bit(plane1)<<1). Rendered with neutral grayscale palette [0x00, 0x55, 0xAA, 0xFF].

## Proof-of-concept renders (artifacts)
- `assets/chr/chr_first256.png` — tiles 0..255 (banks 0-3), 16-wide, 4x scale, bank separators.
- `assets/chr/chr_all4096.png` — ALL 4096 tiles, 64-wide (one CHR bank per row), 2x scale. Single overview.
- `assets/chr/chr_font_region.png` — tiles 1664..2303 (banks 26-35), the font/text region, 4x.
- `assets/chr/chr_sprite_region.png` — tiles 2560..2815 (banks 40-43), an object/sprite region, 4x.

## Quantitative summary
- Blank tiles (all 0x00): **348** (8.5%).
- Solid tiles (all 0xFF): **5**.
- Other (meaningful) tiles: **3743** (91.4%).
- Distinct tile patterns: **2886** of 4096 (the most common non-blank tile repeats only 27x), so there is
  very little filler duplication — the CHR is densely packed with real graphics.

Per-bank fill (set-pixel-bit average, "avgfill") and non-blank tile counts show only two near-empty banks:
- **Bank 31** (file 0x027C10..0x028010, tiles 1984-2047): 0/64 non-blank — fully empty padding.
- **Bank 30** (file 0x027810..0x027C10, tiles 1920-1983): 4/64 non-blank, avgfill 0.02 — near-empty.

These two empty banks sit at the seam between the background region and the sprite region (see below),
i.e. they pad the font/text block out to a bank boundary. A few other banks are partially filled
(banks 21, 22, 33, 34, 35 are 40-60% non-blank) but still contain real graphics.

## Content grouping by bank (visually confirmed from renders)
The CHR is clearly organized into three contiguous functional regions:

### 1. Background / map tiles — banks 0-25 (file 0x020010..0x026810, tiles 0-1663)
Dense overworld/dungeon background graphics. `chr_first256.png` shows: stone bridges, brick/block walls,
trees, mountains, building facades, doors/arches, and readable in-tile labels **"SHOP"** and **"INN"** (the
game's shop/inn entrance signage, drawn directly into background tiles). The overview shows these same motifs
(buildings, terrain, "SHOP"/"INN" signs repeating) filling rows for banks 0 through ~25. High avgfill (0.34-0.46)
consistent with detailed tiled scenery.

### 2. Font / text / title region — banks 26-35 (file 0x026810..0x029010, tiles 1664-2303)
Multiple character sets, confirmed in `chr_font_region.png`:
- A title-logo letter set (large stylized glyphs: "A H O U 1 B I P V 2 C J Q W 3 D K R X 4 E L S Y 5 F M T Z 6 G N ...").
- Japanese kana (hiragana/katakana rows) — residue/shared art from the Japanese "Drasle Family" original.
- A status/menu font: digits "0123456789", uppercase "ABCDEFGHIJKLMNOPQRSTUVWXYZ", punctuation, and the
  copyright string **"1978 BRODERBUND SOFTWARE INC"** plus a "© ... TM" line.
- Big title-screen word art (fragments reading "...OF THE...", part of "LEGACY OF THE WIZARD").
- A second, complete ASCII font (lower in the region): symbols `!"#$%&'()*+,-./`, `0123456789:;<=>?`,
  uppercase `@ABCDEFGHIJKLMNO PQRSTUVWXYZ[¥]^_`, and lowercase `abcdefghijklmno pqrstuvwxyz{|}~`.
Banks 30-31 (the two empty banks) terminate this block at the boundary before sprites begin.

### 3. Object / sprite graphics — banks 36-63 (file 0x029010..0x030010, tiles 2304-4095)
The largest region. `chr_sprite_region.png` (banks 40-43) shows clearly object/character sprite art:
large multi-tile creatures/enemies and player-family characters, laid out as metasprites with repeated
columns that look like animation frames. The overview's bottom half is uniformly this kind of rounded,
shaded sprite content (avgfill 0.30-0.51), distinct from the rectilinear background tiles of region 1.
A small amount of additional small-font/HUD glyphs also appears interleaved at the top of this region
(banks 36-37 show a tiny font row), but the bulk is sprite art.

## Conclusions
- ~91% of the 4096 tiles are meaningful graphics; only ~8.5% blank and effectively just **2 fully-padding banks (30, 31)**.
- Tiles are strongly grouped into recognizable, contiguous bank ranges: **backgrounds (banks 0-25)**,
  **fonts/text/title art (banks 26-35)**, and **sprites/objects (banks 36-63)**.
- The presence of full Japanese kana plus a "1978 BRODERBUND SOFTWARE INC" string confirms the US localization
  of the Falcom original, with both kana and Latin fonts retained in CHR.

Palette is intentionally NOT resolved here (separate agent); all renders use a neutral 4-gray ramp.
