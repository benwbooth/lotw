# 08 — External Research: Existing LotW / Dragon Slayer IV References

ROM: `rom/lotw.nes` (Legacy of the Wizard, US Broderbund; JP "Dragon Slayer IV: Drasle
Family", Falcom/Namco). iNES mapper 4 (MMC3), 128 KiB PRG, 64 KiB CHR. Verified against
ROM bytes (header `4e45531a 08 08 40 ...` → PRG=8×16KiB, CHR=8×8KiB, mapper 4, horizontal
mirroring, no battery).

## Bottom line

- **No full disassembly / decompilation of this game exists publicly.** The closest
  community asset is a *data editor* (lotwtool) plus two well-maintained Data Crystal
  format pages. This does NOT replace Stage 1 (we still own the code RE), but it
  **substantially shortcuts the DATA-format reverse engineering**: map/room layout,
  enemy records, metatiles, palettes, title screen, and credits are already mapped, and
  a RAM map gives us labels for most game-state variables. The RNG and enemy-drop
  algorithm are documented at the behavioral level.
- Treat lotwtool's C# source and the Data Crystal pages as the authoritative format
  references; cross-check every offset against the ROM (I verified the load-bearing ones
  below and they matched exactly).

## Sources (ranked by usefulness/trust)

### 1. lotwtool — Brad Smith (bbbradsmith) — HIGH trust, HIGH value
- URL: https://github.com/bbbradsmith/lotwtool
- A GUI ROM-data editor (C#/.NET 4) for the NES and Famicom versions (plus reduced MSX1/
  MSX2 builds). Brad Smith is a well-known, reliable NES reverse-engineer.
- **Not a disassembly** — no code-level annotation; it parses ROM data structures.
- C# source files are themselves format documentation: `MapEdit*.cs`,
  `MapEditItem.cs`, `MapEditProperties.cs`, `MapEditTile.cs`, `Metatile.cs`,
  `Nametable.cs`, `CHREdit.cs`, `CHRSelect.cs`, `PalettePick.cs`, `Credits.cs`,
  `MiscCheat.cs`. **There is NO Save/Password source file** → the tool does not touch the
  password system, so it is not a reference for that.
- readme cites: Data Crystal (complete map format), NetBrian's "Leghack" map-format
  notes, and the LSD4 unofficial archive.

### 2. Data Crystal — ROM map — HIGH value, MEDIUM-HIGH trust (verified)
- URL: https://datacrystal.tcrf.net/wiki/Legacy_of_the_Wizard/ROM_map
- **Offset convention: PRG-relative (no 16-byte iNES header).** So file offset = `0x10 +
  PRG offset`. I confirmed this: DC says credits at PRG `$1B79C`; file `$1B7AC` contains
  ASCII "CREDITS … CAST … Warrior Xemn Worzen … Wizard Mayna Worzen … Ranger Roas Worzen
  … Elf Lyll Worzen … Monster Pochi". Exact match.
- PRG bank layout (8 KiB MMC3 banks):
  - Banks `$0–$8` (`$00000–$11FFF`): **level maps** (1 KiB per 4-screen dungeon)
  - Bank `$9` (`$12000–$13FFF`): **metatile sets**; **dragon map at PRG `$13800`**
  - Banks `$A–$B` (`$14000–$17FFF`): music, unused title screen
  - Banks `$C–$D` (`$18000–$1BFFF`): music, title screen, code, credits
  - Banks `$E–$F` (`$1C000–$1FFFF`): fixed upper bank (`$E000–$FFFF`), code + vectors
- **Map / room format (1 KiB):**
  - `$000–$2FF` tile grid: 64 columns × 12 tiles
  - `$300` metatile page; `$301` enemy CHR (PPU $1400); `$302/$303` secret wall tile/
    replacement; `$304` block replacement; `$305/$306` terrain CHR 0/1 (PPU $0000/$0800)
  - `$307` treasure active; `$308/$309` treasure X (grid 0–63)/Y (px 0–191); `$30A`
    treasure contents (0–23); `$30B` music track (0–15)
  - `$30C–$30F` Celina teleport target (map X/Y, player X/Y)
  - `$310–$313` shop items 0/1 + prices; `$314` demo bitfield; `$315` music-control
    bitfield; `$316` unused
  - `$320–$3AF` **9 enemy slots × 16 bytes**: `+0` first sprite index, `+1` draw attr,
    `+2` X (grid), `+3` Y, `+4` HP, `+5` damage, `+6` death sprite index, `+7` animation
    style, `+8` behaviour (0–8), `+9` speed
  - `$3E0–$3FF` 32-byte palette
- Title screen straddles banks $C/$D at PRG `$19EC9` (file `$19ED9`; I read `0x1f`-fill
  there, consistent with a nametable using tile $1F as blank). Unused title at PRG
  `$17BCA`. Credits PRG `$1B79C` (LotW) / `$1B7AA` (Dragon Slayer IV).

### 3. Data Crystal — RAM map — HIGH value, MEDIUM-HIGH trust
- URL: https://datacrystal.tcrf.net/wiki/Legacy_of_the_Wizard/RAM_map
- Key zero-page / game-state labels:
  - `$0038` RNG modulus (exclusive upper bound); `$0039–$003B` RNG state
  - `$0040` current character (0–4, 6 = selecting)
  - `$0043/$0044` player X fine/tile; `$0045` player Y; `$0047/$0048` map screen X/Y
  - `$0051–$0053` carried items; `$0055` equipped item
  - `$0058` health; `$0059` magic; `$005A` gold; `$005B` keys
  - `$005C` jump; `$005D` strength; `$005E` shots allowed; `$005F` range
  - `$0060–$006F` inventory counts; `$007B/$007C` X fine/tile scroll
  - `$00F2` boss life
  - `$0300–$0321` last-save block: inventory `$0300–$031F`, keys `$0320`, gold `$0321`
    — **this is the canonical password/save payload; the password almost certainly
    encodes this region.**
  - `$0400–$048F` sprite tables (9 sprites × 16 bytes)

### 4. Data Crystal — main page — MEDIUM value
- URL: https://datacrystal.tcrf.net/wiki/Legacy_of_the_Wizard
- Documents the **RNG algorithm and enemy-drop table** at a behavioral level:
  - RNG: loop of ASL/ROL/ADC/AND over `$0039–$003B`, count driven by `$0038`, final
    result `AND #$7F` stored to `$003B`.
  - Drops: guaranteed first (health<20→bread, magic<30→potion, keys<2→key), else roll
    0–19: `$00–$03` poison, `$04–$05` key, `$06` ring, `$07` cross, `$08` scroll,
    `$09–$13` bread/potion/money chosen by life vs magic vs gold checks.

### 5. LSD4 password generator — MEDIUM value, MEDIUM trust (could not fetch)
- URL: http://lsd4.starfree.jp/password_generator/ (ECONNREFUSED at fetch time; retry
  later, possibly via archive.org)
- Generates valid passwords for NES/FC/MSX1/MSX2. Password format is 32 chars in 8 groups
  of 4 (e.g. "C4TB RSSH 6RXC 1TJH CUTK 3NFT YWMC WJVU"). **Most promising lead for the
  password ALGORITHM**, which neither lotwtool nor Data Crystal covers. Pair with the
  `$0300–$0321` save block when we RE the password routine.

### 6. Supplementary (low/contextual)
- NetBrian "Leghack" map-format notes — referenced by lotwtool readme; older but
  corroborates the map format. (Find via LSD4 archive / romhacking.net.)
- TCRF article https://tcrf.net/Legacy_of_the_Wizard — unused content / regional diffs
  (landing page only at fetch time; retry).
- VGMPF https://www.vgmpf.com/Wiki/index.php/Legacy_of_the_Wizard_(NES) — sound credited
  to Yuzo Koshiro; **no sound-engine/driver disassembly found anywhere**. The music
  driver is unmapped territory we will have to RE ourselves (banks $A–$D hold music data).
- Other disassembly collections (cyneprepou4uk/NES-Games-Disassembly, benljbrooks/
  nes_disassembly) — checked; **neither contains LotW / Dragon Slayer IV.**
- RPGClassics / vgmaps (FlyingArmor) — gameplay maps, not technical.

## Verification log (ROM bytes vs claims)
- Header/mapper/CHR sizes match charter and DC. ✓
- DC PRG offsets are header-excluded (file = +0x10): credits ASCII at file `$1B7AC`. ✓
- Title region at file `$19ED9` is `1F`-filled (blank-tile nametable). ✓
- Vectors (last 6 bytes of PRG, file `$1FFFA–$1FFFF`): NMI=`$D1FE`, RESET=`$FFE0`,
  IRQ=`$D1FE`. RESET `$FFE0` → file `$1DFF0` (last bank). NMI/IRQ share `$D1FE`.

## Impact on Stage 1 plan
- **Data formats are largely solved** — adopt the Data Crystal map/room/enemy/palette
  layout and RAM labels directly; skip blind RE of these structures and instead verify
  the documented offsets against bytes, then label.
- **Still need original RE for:** the password/save algorithm (use LSD4 generator + save
  block `$0300–$0321` as oracles), the sound/music engine, and all CPU code/banking
  logic (no code disassembly exists anywhere).
- Use lotwtool's C# as a second opinion when a Data Crystal field is ambiguous.
