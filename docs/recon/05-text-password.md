# Recon 05 — Text & Password Systems

ROM: `rom/lotw.nes` (Legacy of the Wizard, USA; Falcom *Dragon Slayer IV: Drasle
Family*). 196,624 bytes = 16 B iNES header + 128 KiB PRG + 64 KiB CHR. Mapper 4
(MMC3). All file offsets below are **absolute file offsets** (include the 16-byte
header). "PRG offset" means the offset within PRG-ROM (file offset − 0x10).

---

## 1. Text encoding: direct CHR tile-index, NOT ASCII, NOT DTE

### 1.1 The font

CHR-ROM contains a complete ASCII-ordered font in the pattern-table region
**CHR tiles 0x800–0x8FF** (CHR bank section 8 = CHR file offset 0x800·0x10 =
`0x8000` *within CHR*, i.e. absolute file `0x131088 + 0x8000 = 0x139088`).
Rendered sheet: `assets/chr/font_sec08_indexed.png` (and the readable enlargement
`assets/chr/font_sec08_big.png`).

Glyph layout within that 256-tile section (local tile index → glyph):

| local idx | glyphs |
|-----------|--------|
| 0x40–0x4F | `(space) ! " # $ % & ' ( ) * + , - . /` |
| 0x50–0x5F | `0 1 2 3 4 5 6 7 8 9 : ; < = > ?` |
| 0x60–0x6F | `@ A B C D E F G H I J K L M N O` |
| 0x70–0x7F | `P Q R S T U V W X Y Z [ ¥ ] ^ _` |
| 0x80–0x8F | `(space) a b c d e f g h i j k l m n o` |
| 0x90–0x9F | `p q r s t u v w x y z { : | ~ s` |

So **glyph(local index) = ASCII − 0x20 + 0x40 = ASCII + 0x20** within the font
section. (Local 0x00–0x3F of section 8 are non-font game graphics.)

### 1.2 The text data byte = ASCII + 0xA0

In-game **text data** stores each character as a background-tile index of the
form **`byte = ASCII + 0xA0` (mod 0x100)**. Equivalently the tile is the section-8
font glyph (ASCII+0x20) with the high bit set (+0x80), which selects the font's
copy in the high half of the displayed pattern table at runtime.

Concrete map for the characters actually used in game text (uppercase only):

| char | byte | char | byte | char | byte |
|------|------|------|------|------|------|
| (space) | 0xC0 | `0`–`9` | 0xD0–0xD9 | `A`–`O` | 0xE1–0xEF |
| `!` | 0xC1 | `:` | 0xDA | `P`–`Z` | 0xF0–0xFA |
| `,` | 0xCC | `=` | 0xDD | `[` | 0xFB |
| `-` | 0xCD | `?` | 0xDF | `\` | 0xFC |
| `.` | 0xCE | `@` | 0xE0 | `]` | 0xFD, `^`=0xFE |

(Lowercase would be 0x81–0x9A and `a`+0xA0 wraps to 0x01, colliding with map
tiles, which is why game text is uppercase only.)

**There is no DTE / dictionary compression**: every decoded string is a 1-byte-
per-character run with no escape bytes or back-references. The encoding is a
straight tile-index map.

Verification method: a delta-signature search (differences between consecutive
character codes are invariant under any additive offset) located "CONTINUE" with
a constant +0xA0 offset, after which raw `byte−0xA0` decoding cleanly recovered
all strings below.

---

## 2. Two text storage formats

### 2.1 Format A — plain concatenated strings drawn by `$CCE4`

Strings are stored as plain `ASCII+0xA0` byte runs and rendered by a draw routine
at **CPU `$CCE4`** (in the bank mapped to `$C000`). Caller convention: zero-page
**`$0E`/`$0F` = 16-bit source pointer (lo/hi)**, then `JSR $CCE4`.

Example — the **GAME OVER screen** (bank 13):

- Strings at **file 0x1b4bf** (PRG 0x1b4af, bank 13):
  `e7 e1 ed e5 c0 ef f6 e5 f2  f2 e5 f4 f2 f9  e3 ef ee f4 e9 ee f5 e5`
  = **`GAME OVER` / `RETRY` / `CONTINUE`** concatenated (lengths 9, 5, 8; no
  terminators — selected by length/offset).
- Immediately preceding code: `… STA $0E / LDA #$C4 / STA $0F / JSR $CCE4 /
  LDX #$01 / LDY #$60 …` (file ~0x1b4ad), i.e. set pointer then draw.
- The same screen routine at file ~0x18F0 (bank 13) does
  `LDA #$F2 / STA $0E / LDA #$C8 / STA $0F / JSR $CCE4` (pointer = `$C8F2`) to
  paint the background, confirming the `$0E/$0F → $CCE4` convention. `$CCE4` is
  called from ≥9 sites in bank 13 alone.

### 2.2 Format B — pre-built nametable blocks (text interleaved with frame tiles)

The pause / status / equipment menu is stored as a ready-made nametable stream in
**bank 9** where each on-screen cell occupies 2 data bytes (text tile + a
border/attribute tile, commonly `0xFB`=`[` or `0xFC`=`\`). Decoding only the
text positions yields readable labels:

- **file 0x12fc2** (PRG 0x12fb2, bank 9): `JUMP  STRENGTH  DISTANCE  EQUIPMENT
  EXIT` (the equipment sub-menu).
- **file 0x12f40** region: `ELIXER`, and `INVENTORY` near 0x12fa0 (item /
  inventory labels, one char per 2 bytes).
- **file 0x12d8c** (PRG 0x12d7c, bank 9): the title **`PASSWORD`** —
  raw bytes `… fc f0 fc e1 fc f3 fc f3 fc f7 fc ef fc f2 fc e4 …`; the odd
  bytes are `f0 e1 f3 f3 f7 ef f2 e4` = `P A S S W O R D` (confirms `P`=0x50+0xA0
  =0xF0), with `0xFC` border tiles between letters.

### 2.3 HUD labels (fixed bank 15)

The status bar labels live in the **fixed last bank (bank 15)** at
**file 0x1fefc** (PRG 0x1feec):
`fb ec e9 e6 e5 c0  fb ed e1 e7 e9 e3  fb eb e5 f9 c0 c0  fb e7 ef ec e4 c0
fb e9 f4 e5 ed …` → **`[LIFE`, `[MAGIC`, `[KEY`, `[GOLD`, `[ITEM`**, each
prefixed by `0xFB`=`[`, followed by bar-meter tiles (`0xDA`,`0xDD`, …).

### 2.4 Shop / Inn signage is graphical, not text

The words `SHOP` and `INN` appear as **dedicated background tiles in CHR section
0** (`assets/chr/sec_00.png`, `sec_01.png`, `sec_02.png`), i.e. shop signage is
drawn as pre-composed graphics, not emitted through the text engine. No raw
"SHOP"/"INN"/"GOLD" ASCII (or ASCII+0xA0) string for signage exists in PRG.

---

## 3. Located text tables (summary)

| What | File offset | PRG offset | Bank | Format |
|------|-------------|-----------|------|--------|
| Font (CHR) | 0x139088 +0x40·0x10 | — | CHR sec 8 | tiles 0x840–0x89F = ASCII+0x20 |
| GAME OVER / RETRY / CONTINUE | 0x1b4bf | 0x1b4af | 13 | A (plain, via `$CCE4`) |
| Equipment menu (JUMP/STRENGTH/DISTANCE/EQUIPMENT/EXIT) | 0x12fc2 | 0x12fb2 | 9 | B (nametable) |
| ELIXER / INVENTORY item labels | ~0x12f40 | ~0x12f30 | 9 | B (nametable) |
| PASSWORD title | 0x12d8c | 0x12d7c | 9 | B (nametable, interleaved) |
| HUD: LIFE/MAGIC/KEY/GOLD/ITEM | 0x1fefc | 0x1feec | 15 (fixed) | embedded nametable |
| Tile→code translation table (256 B) | 0x17d07 | 0x17cf7 | 11 | lookup (see §4) |

---

## 4. Password system

### 4.1 Where it lives

LotW's save system is a displayed password (this US release; the JP "Drasle
Family" original is the same engine). The password screen is part of the
pause/status menu group:

- The **`PASSWORD` title** is drawn from the bank-9 nametable block at file
  0x12d8c (§2.2). The whole menu group (status, equipment, inventory, password)
  is co-located in bank 9, file ≈0x12000–0x13900.

### 4.2 Tile→character translation table (file 0x17d07, bank 11)

A 256-byte table at **file 0x17d07** (PRG 0x17cf7; bank 11; if bank 11 is mapped
to the `$8000` window this is CPU `$9CF7`). Indexed 0x00–0xFF, most entries are
the filler `0x01`; valid entries are runs of consecutive tile codes:

```
idx 0x60..0x63 -> e0 c1 c2 c3 c4          (@ ! " # $ region tiles)
idx 0x80..0x8E -> d0 d1 d2 d3 d4 .. df    (digits 0-9 : ; < = > ?)
idx 0xA0..0xAE -> e1 e2 .. ee             (A B C .. N)
idx 0xC0..0xCE -> f2 f3 .. fe             (R S .. ^)
```

This is a **map from a compact input/character code to the display tile value
(ASCII+0xA0)** — exactly what a password-entry cursor needs to turn the player's
selected character index into the tile to draw, and (read in reverse) to validate
which on-screen tiles correspond to legal password characters. The legal password
character set it encodes is: digits `0–9`, punctuation, and letters (`A–N`,
`R–^` runs visible). The trailing bytes `12 13 14 15 …` and `c5 c6 / 75 76`
entries map a few special/cursor tiles.

### 4.3 Generation / validation routine (location, not fully reversed)

Confirmed: the menu/password code is in **bank 9** (data) with the drawing
primitive `$CCE4` and zero-page string pointer `$0E/$0F` (§2.1). The byte-level
checksum/encode-decode math (how gold/items/progress are packed into the
character string) was **not** isolated within this pass — it requires tracing the
live menu state machine. Candidate code regions to disassemble next:

- Bank 9 code interleaved with the menu nametable data (file ≈0x12000–0x13900).
- The translation table consumer in bank 11 around file 0x17cf7 (CPU `$9CF7`
  when bank 11 ∈ `$8000`).

No password-specific checksum constant or CRC table was found by inspection in
this pass; the password is most likely a positional character encoding of game
state (LotW uses a long on-screen password) validated against the translation
table in §4.2 rather than a hash.

---

## 5. Method notes / reproducibility

- All decoding done by reading raw ROM bytes with `python3` and the rule
  `char = chr(byte − 0xA0)` for bytes 0xC0–0xFF.
- Font located by rendering CHR sections to PNG (`assets/chr/sec_00..15.png`,
  `font_sec08_indexed.png`, `font_text_C0_FF.png`).
- Encoding offset (+0xA0) discovered via additive-offset-invariant delta search
  on the word "CONTINUE".
- Disassembly of the fixed bank and bank 13 via `da65` confirmed the
  `$0E/$0F → JSR $CCE4` print convention and the GAME OVER screen.

## 6. Open questions

1. Exact byte format of the password string and its checksum/validation math
   (which RAM variables map to which password positions).
2. The runtime CHR/PRG bank numbers in each window during the menu (to fix the
   `$9CF7` vs `$BCF7` ambiguity for the translation table).
3. Whether item names (POTION/BREAD/etc.) are stored anywhere as Format-A text or
   are only graphical/menu-embedded (none found as plain strings this pass).
