use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QColor, QFont, QImage, QImageFormat, QPen, QPoint, QRect, QString};

// View modes.
pub const ROOM: i32 = 0;
pub const ATLAS: i32 = 1;
pub const WORLD: i32 = 2;
pub const TITLE: i32 = 3;
pub const SHEET: i32 = 4; // single sprite tab: labeled subsections per character/creature

const FAMILY_PAL: usize = 0x1FFC5; // PRG: FAMILY_PALETTE_TABLE $FFC5, 6 x 4 bytes
const PLAYER_BANK0: usize = 56; // CHR bank for character 0; char c uses 56+c
// Drasle family ("Drasle" = "Dragon Slayer"; surname Worzen), character_index
// order (Pochi the pet = the round creature at index 4). Names + roles per the
// USA NES manual.
const CHAR_NAMES: [&str; 6] = ["Xemn", "Meyna", "Roas", "Lyll", "Pochi", "char5"];
const CHAR_ROLES: [&str; 6] = ["Father", "Mother", "Son", "Daughter", "Pet", ""];

/// The four crown-guardian bosses, indexed by enemy CHR bank 48..51. Mapping is
/// by ascending boss HP (100/150/200/255), which matches the documented
/// difficulty order Taratunes < Erebone < Archwinger < Rockgaea (Archwinger's
/// winged body and Rockgaea's golem body are also visually confirmed).
fn boss_name(bank: u8) -> &'static str {
    match bank {
        48 => "Taratunes (spider)",
        49 => "Erebone (skeleton)",
        50 => "Archwinger (winged)",
        51 => "Rockgaea (golem)",
        _ => "boss",
    }
}

/// Name of the area creature occupying strip `k` (each creature spans a
/// 4-metasprite animation strip; 4 creatures per bank) of enemy CHR bank
/// `bank`. Names are from StrategyWiki's monster list, matched to our CHR by
/// sprite-shape correlation (the manual itself names no regular enemies); only
/// confident, uniquely-matched assignments are included, so some strips are
/// left unnamed.
fn area_creature_name(bank: u8, k: usize) -> Option<&'static str> {
    let n = match (bank, k) {
        (36, 0) => "Derudeathgadedo",
        (36, 1) => "Meta Black",
        (36, 2) => "Moricdo",
        (36, 3) => "Killer Bat",
        (37, 0) => "Tiger",
        (37, 2) => "Aryu",
        (37, 3) => "Garba",
        (38, 1) => "Yashinotkin",
        (38, 2) => "Orc",
        (38, 3) => "Slime",
        (39, 0) => "Giant",
        (39, 1) => "Lee",
        (39, 2) => "Golem",
        (40, 0) => "Memes",
        (40, 1) => "Mimic",
        (40, 3) => "Wizard",
        (41, 0) => "Lightball",
        (41, 1) => "Mayu",
        (41, 2) => "Kraugen",
        (42, 1) => "Snake Kid",
        (42, 3) => "Rock",
        (43, 1) => "Crawler",
        (43, 2) => "Skeleton",
        (43, 3) => "Rock Veest",
        (44, 0) => "Slug",
        (44, 1) => "Mummy",
        (44, 2) => "Cyclops",
        (44, 3) => "Mu",
        (45, 0) => "Lion",
        (45, 1) => "Kirru",
        (45, 2) => "Elemental",
        (45, 3) => "Dwarf",
        (46, 0) => "Roid Moon",
        (46, 1) => "Dorak",
        (46, 2) => "Gridel",
        (46, 3) => "Flail Snail",
        (47, 0) => "Roman",
        (47, 1) => "Lizard Man",
        (47, 2) => "Bupurch",
        _ => return None,
    };
    Some(n)
}

const MAP_ROWS: usize = 18;
const WW: i32 = 4 * 1024; // world width
const WH: i32 = MAP_ROWS as i32 * 192; // world height (18 rows)
const TITLE_NT: usize = 0x19EC9;
const TITLE_PAL: usize = 0x1A2C9;
const TITLE_CHR: usize = 0x1A2E9;

// Sprite-sheet layout.
const SS_COLS: usize = 16; // metasprites per row (= one 64-tile CHR bank)
const SS_CELL: usize = 24; // pixel cell (16px sprite + 8px gap), scaled by QML
const SS_LABEL_H: usize = 16; // section header strip height

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!(<QtQuick/QQuickPaintedItem>);
        type QQuickPaintedItem;
        include!("cxx-qt-lib/qpainter.h");
        type QPainter = cxx_qt_lib::QPainter;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qml_element]
        #[base = QQuickPaintedItem]
        #[qobject]
        #[qproperty(i32, selected)]
        #[qproperty(i32, mode)]
        #[qproperty(i32, sel_metatile)]
        #[qproperty(i32, cursor_col)]
        #[qproperty(i32, cursor_row)]
        #[qproperty(i32, obj_rev)] // bumped on any object edit -> QML reactivity
        #[qproperty(i32, pal_rev)] // bumped on any palette edit -> QML reactivity
        #[qproperty(i32, sprite_pal)] // 0 = greyscale, 1..4 = room sprite palettes
        #[qproperty(bool, show_solid)] // overlay tile passability/collision classes
        type RoomCanvas = super::RoomCanvasRust;

        #[cxx_override]
        unsafe fn paint(self: Pin<&mut RoomCanvas>, painter: *mut QPainter);

        #[inherit]
        fn update(self: Pin<&mut RoomCanvas>);
        #[inherit]
        fn width(self: &RoomCanvas) -> f64;
        #[inherit]
        fn height(self: &RoomCanvas) -> f64;

        #[qinvokable]
        fn refresh(self: Pin<&mut RoomCanvas>);
        #[qinvokable]
        fn set_anim(self: Pin<&mut RoomCanvas>, f: i32);
        #[qinvokable]
        fn set_world_hover(self: Pin<&mut RoomCanvas>, idx: i32);
        #[qinvokable]
        fn set_atlas_hover(self: Pin<&mut RoomCanvas>, idx: i32);
        #[qinvokable]
        fn paint_tile(self: Pin<&mut RoomCanvas>, col: i32, row: i32);
        #[qinvokable]
        fn erase_tile(self: Pin<&mut RoomCanvas>, col: i32, row: i32);
        #[qinvokable]
        fn paint_line(self: Pin<&mut RoomCanvas>, c0: i32, r0: i32, c1: i32, r1: i32);
        #[qinvokable]
        fn paint_rect(self: Pin<&mut RoomCanvas>, c0: i32, r0: i32, c1: i32, r1: i32);
        #[qinvokable]
        fn paint_ellipse(self: Pin<&mut RoomCanvas>, c0: i32, r0: i32, c1: i32, r1: i32);
        #[qinvokable]
        fn set_preview(self: Pin<&mut RoomCanvas>, kind: i32, c0: i32, r0: i32, c1: i32, r1: i32);
        #[qinvokable]
        fn clear_preview(self: Pin<&mut RoomCanvas>);
        #[qinvokable]
        fn metatile_at(self: &RoomCanvas, col: i32, row: i32) -> i32;
        #[qinvokable]
        fn obj_active(self: &RoomCanvas, slot: i32) -> bool;
        #[qinvokable]
        fn obj_kind(self: &RoomCanvas, slot: i32) -> i32;
        #[qinvokable]
        fn obj_x(self: &RoomCanvas, slot: i32) -> i32;
        #[qinvokable]
        fn obj_y(self: &RoomCanvas, slot: i32) -> i32;
        #[qinvokable]
        fn obj_byte(self: &RoomCanvas, slot: i32, i: i32) -> i32;
        #[qinvokable]
        fn obj_name(self: &RoomCanvas, slot: i32) -> QString;
        #[qinvokable]
        fn pal_byte(self: &RoomCanvas, i: i32) -> i32;
        #[qinvokable]
        fn nes_color(self: &RoomCanvas, c: i32) -> QString;
        #[qinvokable]
        fn set_pal(self: Pin<&mut RoomCanvas>, i: i32, c: i32);
        #[qinvokable]
        fn set_obj(self: Pin<&mut RoomCanvas>, slot: i32, kind: i32, x: i32, y: i32);
        #[qinvokable]
        fn delete_obj(self: Pin<&mut RoomCanvas>, slot: i32);
        #[qinvokable]
        fn create_obj(self: Pin<&mut RoomCanvas>, x: i32, y: i32, kind: i32) -> i32;
        #[qinvokable]
        fn copy_obj(self: Pin<&mut RoomCanvas>, slot: i32) -> i32;
        #[qinvokable]
        fn begin_edit(self: Pin<&mut RoomCanvas>);
        #[qinvokable]
        fn undo(self: Pin<&mut RoomCanvas>);
        #[qinvokable]
        fn redo(self: Pin<&mut RoomCanvas>);
        #[qinvokable]
        fn world_room_at(self: &RoomCanvas, x: i32, y: i32) -> i32;
        #[qinvokable]
        fn room_count(self: &RoomCanvas) -> i32;
        #[qinvokable]
        fn room_label(self: &RoomCanvas, idx: i32) -> QString;
        #[qinvokable]
        fn img_w(self: &RoomCanvas) -> i32;
        #[qinvokable]
        fn img_h(self: &RoomCanvas) -> i32;
        #[qinvokable]
        fn tile_info(self: &RoomCanvas, x: i32, y: i32) -> QString;
        #[qinvokable]
        fn save_rom(self: &RoomCanvas, path: QString) -> QString;
    }

    impl cxx_qt::Constructor<()> for RoomCanvas {}
}

pub struct RoomCanvasRust {
    selected: i32,
    mode: i32,
    sel_metatile: i32,
    cursor_col: i32,
    cursor_row: i32,
    header: Vec<u8>,
    prg: Vec<u8>,
    chr: Vec<u8>,
    rooms: Vec<lotw::render::Room>,
    orig_rooms: Vec<lotw::render::Room>, // pristine copy for the eraser
    world_cache: Option<Vec<u8>>,
    room_cache: Option<Vec<u8>>,
    cache_sel: i32,
    pv: (i32, i32, i32, i32, i32), // (kind, c0, r0, c1, r1) shape-tool preview
    obj_rev: i32,
    pal_rev: i32,
    sprite_pal: i32,
    anim_frame: u8, // sprite animation offset (0 or 4), driven by a QML timer
    world_hover: i32, // room index hovered in the World view (-1 = none)
    atlas_hover: i32, // metatile index hovered in the metatile atlas (-1 = none)
    show_solid: bool, // overlay tile passability/collision classes
    sections: Vec<Section>, // sprite-sheet subsections (players / area enemies / banks)
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
}

/// One cell in a sprite-sheet section.
#[derive(Clone, Copy)]
enum Cell {
    /// Player/family pose: 4 consecutive CHR tiles, fixed family palette, animates.
    Family { base_tile: usize, pal4: [(u8, u8, u8); 4] },
    /// Placed/area actor: spawn tile + attr, drawn with a real room palette + the
    /// room's per-area sprite banks (so the colours match the game), animates.
    Actor { tile: u8, attr: u8, room: usize },
    /// Large actor (boss): a 32x32 body assembled from four 16x16 pieces, the way
    /// `compose_large_actor_body_slots` lays them out (TL=base, TR=base|4,
    /// BL=base|0x20, BR=base|0x24). `base` is the top-left animation frame tile.
    Boss { base: u8, attr: u8, room: usize },
    /// Raw CHR-bank metasprite: 4 consecutive tiles, palette chosen live from the
    /// `sprite_pal` selector. Used for shared object/boss banks. Static.
    Bank { base_tile: usize },
}

/// A labeled sprite-sheet subsection (a row of related metasprites). `cell_px`
/// is the pixel pitch of each cell (large for boss rows).
struct Section {
    label: String,
    cells: Vec<Cell>,
    cell_px: usize,
}

impl Section {
    fn rows(&self) -> usize {
        self.cells.len().div_ceil(SS_COLS).max(1)
    }
    fn height(&self) -> usize {
        SS_LABEL_H + self.rows() * self.cell_px
    }
}

/// Pre-edit snapshot of a single room (grid + actor records) for undo/redo.
struct Snapshot {
    idx: usize,
    grid: Vec<Vec<u8>>,
    records: Vec<[u8; 16]>,
}

fn rom_path() -> String {
    if let Ok(p) = std::env::var("LOTW_ROM") {
        return p;
    }
    for p in ["rom/lotw.nes", "../rom/lotw.nes", "../../rom/lotw.nes"] {
        if std::path::Path::new(p).exists() {
            return p.to_string();
        }
    }
    "rom/lotw.nes".to_string()
}

fn greyscale4() -> [(u8, u8, u8); 4] {
    [(0, 0, 0), (85, 85, 85), (170, 170, 170), (255, 255, 255)]
}

/// Build the sprite-sheet subsections by reverse-engineering how the game maps
/// sprites to CHR banks:
///   * Players: per character, bank `56+c`, 16 pose metasprites, family palette.
///   * Area enemies: the enemy sprite window (tiles 0x40-0x7F) is CHR slot 3,
///     loaded per room from descriptor byte +1 (`header[1]`). Each distinct
///     bank holds one area's creatures; render all 16 of its metasprites with
///     the palette of a representative placed enemy.
///   * Shared banks 62/63 (object/projectile tiles) and 61 (boss bodies).
fn build_sections(prg: &[u8], chr: &[u8], rooms: &[lotw::render::Room]) -> Vec<Section> {
    let mut sections = Vec::new();
    // A creature's 4-metasprite strip (strip k = metasprites 4k..4k+3 = tiles
    // 16k..16k+16 of the bank) is "real" only if it has non-empty graphics.
    let strip_nonempty = |bank: u8, k: usize| -> bool {
        let start = (bank as usize * 64 + k * 16) * 16;
        chr.get(start..start + 16 * 16).map(|s| s.iter().any(|&b| b != 0)).unwrap_or(false)
    };

    // --- Players (named, family palettes) ---
    for c in 0..6 {
        let fp = &prg[FAMILY_PAL + c * 4..FAMILY_PAL + c * 4 + 4];
        if !fp.iter().any(|&b| b != 0) {
            continue; // char 5 = empty palette, not a real character
        }
        let pal4 = [
            (0, 0, 0),
            lotw::render::nes_rgb(fp[1]),
            lotw::render::nes_rgb(fp[2]),
            lotw::render::nes_rgb(fp[3]),
        ];
        let cells = (0..SS_COLS)
            .map(|k| Cell::Family { base_tile: (PLAYER_BANK0 + c) * 64 + k * 4, pal4 })
            .collect();
        sections.push(Section { label: format!("{} — {} (player)", CHAR_NAMES[c], CHAR_ROLES[c]), cells, cell_px: SS_CELL });
    }

    // --- Area enemies, one section per distinct header[1] CHR bank ---
    // For each bank: a representative room (first that uses it) for the palette,
    // and a per-metasprite palette from the placed enemies that use it.
    use std::collections::BTreeMap;
    struct BankInfo {
        rep_room: usize,
        attr_by_m: [Option<u8>; 16],
        any_attr: u8,
    }
    let mut banks: BTreeMap<u8, BankInfo> = BTreeMap::new();
    for (idx, room) in rooms.iter().enumerate() {
        let b = room.header[1];
        if b == 0 {
            continue; // sentinel / no enemy bank
        }
        let info = banks.entry(b).or_insert(BankInfo { rep_room: idx, attr_by_m: [None; 16], any_attr: 0 });
        for i in 0..12 {
            if room.active(i) {
                let rec = room.records[i];
                // window-1 spawn tile -> metasprite index within the bank.
                if (0x40..0x80).contains(&rec[0]) {
                    let m = ((rec[0] as usize) % 64) / 4;
                    info.attr_by_m[m] = Some(rec[1] & 3);
                    info.any_attr = rec[1] & 3;
                }
            }
        }
    }
    for (b, info) in &banks {
        // Boss rooms are exactly those whose enemy bank (CHR slot 3) is >= 48
        // (see update_room_actors: "CHR bank 3 >= 48 selects a boss room").
        // Their actor is a 32x32 four-piece body, so show the four animation
        // frames composed, not the 16x16 slices.
        if *b >= 48 {
            let attr = info.any_attr;
            // animate_large_actor_body_tiles: base = 0x41 | (frame bits3-4).
            let cells = [0x41u8, 0x49, 0x51, 0x59]
                .iter()
                .map(|&base| Cell::Boss { base, attr, room: info.rep_room })
                .collect();
            sections.push(Section { label: format!("{} — boss (bank {b})", boss_name(*b)), cells, cell_px: 40 });
            continue;
        }
        // One row per creature: each creature is a 4-metasprite animation strip.
        for k in 0..4 {
            if !strip_nonempty(*b, k) {
                continue;
            }
            // Palette: prefer the base frame's placed attr, else any frame's, else
            // the bank default.
            let attr = (0..4).find_map(|j| info.attr_by_m[k * 4 + j]).unwrap_or(info.any_attr);
            let cells = (0..4)
                .map(|j| Cell::Actor { tile: (0x41 + (k * 4 + j) * 4) as u8, attr, room: info.rep_room })
                .collect();
            let label = match area_creature_name(*b, k) {
                Some(n) => format!("{n}  (bank {b})"),
                None => format!("Bank {b} creature {k}"),
            };
            sections.push(Section { label, cells, cell_px: SS_CELL });
        }
    }

    // --- Shared sprite banks (object/projectile + boss tiles) ---
    for (bank, name) in [(61usize, "Boss bodies — bank 61"), (62, "Objects/projectiles — bank 62"), (63, "Objects/projectiles — bank 63")] {
        let cells = (0..SS_COLS).map(|m| Cell::Bank { base_tile: bank * 64 + m * 4 }).collect();
        sections.push(Section { label: name.to_string(), cells, cell_px: SS_CELL });
    }

    sections
}

impl Default for RoomCanvasRust {
    fn default() -> Self {
        let path = rom_path();
        let rom = std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        let prg_len = rom[4] as usize * 16_384;
        let header = rom[0..16].to_vec();
        let prg = rom[16..16 + prg_len].to_vec();
        let chr = rom[16 + prg_len..].to_vec();
        let rooms = lotw::render::decode_rooms(&prg, MAP_ROWS);
        let orig_rooms = rooms.clone();
        let sections = build_sections(&prg, &chr, &rooms);
        Self {
            selected: 0,
            mode: 0,
            sel_metatile: 0,
            cursor_col: -1,
            cursor_row: -1,
            header,
            prg,
            chr,
            rooms,
            orig_rooms,
            world_cache: None,
            room_cache: None,
            cache_sel: -1,
            pv: (0, 0, 0, 0, 0),
            obj_rev: 0,
            pal_rev: 0,
            sprite_pal: 0,
            anim_frame: 0,
            world_hover: -1,
            atlas_hover: -1,
            show_solid: false,
            sections,
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }
}

impl RoomCanvasRust {
    fn sel(&self) -> usize {
        (self.selected.max(0) as usize).min(self.rooms.len().saturating_sub(1))
    }
    fn world_rgb(&mut self) -> &[u8] {
        if self.world_cache.is_none() {
            let mut buf = vec![0u8; (WW * WH) as usize * 3];
            for room in &self.rooms {
                let img = room.render(&self.prg, &self.chr);
                let (ox, oy) = (room.mapx * 1024, room.mapy * 192);
                for y in 0..192 {
                    let src = y * 1024 * 3;
                    let dst = ((oy + y) * WW as usize + ox) * 3;
                    buf[dst..dst + 1024 * 3].copy_from_slice(&img[src..src + 1024 * 3]);
                }
            }
            self.world_cache = Some(buf);
        }
        self.world_cache.as_ref().unwrap()
    }
    fn set_cell(&mut self, col: i32, row: i32) {
        if col >= 0 && row >= 0 && col < 64 && row < 12 {
            let mt = self.sel_metatile as u8;
            let s = self.sel();
            self.rooms[s].grid[row as usize][col as usize] = mt;
            self.room_cache = None;
            self.world_cache = None;
        }
    }

    /// Total pixel height of the stacked sprite-sheet sections.
    fn sheet_height(&self) -> i32 {
        self.sections.iter().map(|s| s.height()).sum::<usize>() as i32
    }

    /// Live palette for `Cell::Bank` cells from the `sprite_pal` selector.
    fn bank_pal4(&self) -> [(u8, u8, u8); 4] {
        if self.sprite_pal == 0 {
            return greyscale4();
        }
        let p = &self.rooms[self.sel()].pal;
        let b = (4 + (self.sprite_pal as usize - 1).min(3)) * 4;
        [
            (0, 0, 0),
            lotw::render::nes_rgb(p[b + 1]),
            lotw::render::nes_rgb(p[b + 2]),
            lotw::render::nes_rgb(p[b + 3]),
        ]
    }

    /// Render the whole sprite tab into one tall RGB image, returning it with the
    /// per-section header positions so the caller can draw text labels on top.
    fn render_sheet(&self) -> (Vec<u8>, i32, i32, Vec<(String, i32)>) {
        let w = SS_COLS * SS_CELL;
        let h = self.sheet_height().max(SS_CELL as i32) as usize;
        let mut buf = vec![0u8; w * h * 3];
        // Dark background.
        for px in buf.chunks_exact_mut(3) {
            px[0] = 24;
            px[1] = 24;
            px[2] = 30;
        }
        let f = self.anim_frame;
        let bank_pal4 = self.bank_pal4();
        let mut labels = Vec::new();
        let mut y = 0usize;
        for sec in &self.sections {
            // Header strip.
            for yy in y..(y + SS_LABEL_H).min(h) {
                for xx in 0..w {
                    let o = (yy * w + xx) * 3;
                    buf[o] = 46;
                    buf[o + 1] = 46;
                    buf[o + 2] = 64;
                }
            }
            labels.push((sec.label.clone(), y as i32));
            let band_y = y + SS_LABEL_H;
            let cp = sec.cell_px;
            // Sprite band: faint checkerboard so transparency reads.
            for yy in band_y..(band_y + sec.rows() * cp).min(h) {
                for xx in 0..w {
                    if ((xx / 8 + yy / 8) & 1) == 0 {
                        continue;
                    }
                    let o = (yy * w + xx) * 3;
                    buf[o] = 34;
                    buf[o + 1] = 34;
                    buf[o + 2] = 42;
                }
            }
            for (i, cell) in sec.cells.iter().enumerate() {
                let cx = (i % SS_COLS) * cp + 4;
                let cy = band_y + (i / SS_COLS) * cp + 4;
                match *cell {
                    Cell::Family { base_tile, pal4 } => {
                        lotw::render::blit_metasprite_raw(&self.chr, &pal4, base_tile + f as usize, &mut buf, w, cx, cy);
                    }
                    Cell::Bank { base_tile } => {
                        lotw::render::blit_metasprite_raw(&self.chr, &bank_pal4, base_tile, &mut buf, w, cx, cy);
                    }
                    Cell::Actor { tile, attr, room } => {
                        let r = &self.rooms[room];
                        let banks = lotw::render::sprite_banks(&r.header);
                        lotw::render::blit_sprite(&self.chr, &r.pal, tile.wrapping_add(f), attr, &banks, &mut buf, w, cx, cy);
                    }
                    Cell::Boss { base, attr, room } => {
                        let r = &self.rooms[room];
                        let banks = lotw::render::sprite_banks(&r.header);
                        // 2x2 of 16x16 pieces -> 32x32 (compose_large_actor_body_slots).
                        for (dx, dy, t) in [(0, 0, base), (16, 0, base | 4), (0, 16, base | 0x20), (16, 16, base | 0x24)] {
                            lotw::render::blit_sprite(&self.chr, &r.pal, t, attr, &banks, &mut buf, w, cx + dx, cy + dy);
                        }
                    }
                }
            }
            y += sec.height();
        }
        (buf, w as i32, h as i32, labels)
    }
}

/// Invert the 1px border ring of a `size`x`size` cell at (px,py) in an RGB image.
fn invert_border(rgb: &mut [u8], w: usize, px: usize, py: usize, size: usize) {
    let mut inv = |x: usize, y: usize| {
        let o = (y * w + x) * 3;
        if o + 2 < rgb.len() {
            rgb[o] = 255 - rgb[o];
            rgb[o + 1] = 255 - rgb[o + 1];
            rgb[o + 2] = 255 - rgb[o + 2];
        }
    };
    for d in 0..size {
        inv(px + d, py);
        inv(px + d, py + size - 1);
        inv(px, py + d);
        inv(px + size - 1, py + d);
    }
}

/// Passability/collision tint for a metatile shape id (`mt & 0x3F`), per the
/// RE'd terrain rules (probe_player_solid_tile / dispatch_room_tile_action):
///   2  = locked door (key)        -> orange
///   3/4/5 = door / portal / shop  -> blue
///   62 = item-interactable/breakable -> yellow
///   >=48 = solid wall / hazard    -> red
///   else (0, 1, 6..47)            -> passable background (no tint)
/// Returns None for passable tiles so the room art shows through unchanged.
fn tile_class_tint(shape: u8) -> Option<(u8, u8, u8)> {
    match shape {
        2 => Some((240, 150, 30)),         // locked door
        3 | 4 | 5 => Some((60, 120, 240)), // door / portal / shop
        62 => Some((240, 230, 40)),        // interactable / breakable
        s if s >= 48 => Some((220, 40, 40)), // solid wall / hazard
        _ => None,                          // passable
    }
}

/// Blend a translucent tint over a 16x16 cell at (px,py).
fn tint_cell(rgb: &mut [u8], w: usize, px: usize, py: usize, (tr, tg, tb): (u8, u8, u8)) {
    for y in 0..16 {
        for x in 0..16 {
            let o = ((py + y) * w + px + x) * 3;
            if o + 2 < rgb.len() {
                rgb[o] = ((rgb[o] as u16 * 3 + tr as u16 * 2) / 5) as u8;
                rgb[o + 1] = ((rgb[o + 1] as u16 * 3 + tg as u16 * 2) / 5) as u8;
                rgb[o + 2] = ((rgb[o + 2] as u16 * 3 + tb as u16 * 2) / 5) as u8;
            }
        }
    }
}

/// Invert a `t`-pixel-thick border of a `rw`x`rh` rect at (px,py) in an RGB image.
fn invert_rect_border(rgb: &mut [u8], w: usize, px: usize, py: usize, rw: usize, rh: usize, t: usize) {
    let mut inv = |x: usize, y: usize| {
        let o = (y * w + x) * 3;
        if o + 2 < rgb.len() {
            rgb[o] = 255 - rgb[o];
            rgb[o + 1] = 255 - rgb[o + 1];
            rgb[o + 2] = 255 - rgb[o + 2];
        }
    };
    for k in 0..t {
        for d in 0..rw {
            inv(px + d, py + k);
            inv(px + d, py + rh - 1 - k);
        }
        for d in 0..rh {
            inv(px + k, py + d);
            inv(px + rw - 1 - k, py + d);
        }
    }
}

impl cxx_qt::Constructor<()> for qobject::RoomCanvas {
    type NewArguments = ();
    type BaseArguments = ();
    type InitializeArguments = ();
    fn route_arguments(_: ()) -> (Self::NewArguments, Self::BaseArguments, Self::InitializeArguments) {
        ((), (), ())
    }
    fn new(_: ()) -> RoomCanvasRust {
        RoomCanvasRust::default()
    }
}

fn line_cells(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let (mut x0, mut y0) = (x0, y0);
    let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
    let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
    let mut err = dx + dy;
    let mut out = Vec::new();
    loop {
        out.push((x0, y0));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    out
}

fn rect_cells(c0: i32, r0: i32, c1: i32, r1: i32) -> Vec<(i32, i32)> {
    let (xa, xb) = (c0.min(c1), c0.max(c1));
    let (ya, yb) = (r0.min(r1), r0.max(r1));
    let mut out = Vec::new();
    for x in xa..=xb {
        out.push((x, ya));
        out.push((x, yb));
    }
    for y in ya..=yb {
        out.push((xa, y));
        out.push((xb, y));
    }
    out
}

fn ellipse_cells(c0: i32, r0: i32, c1: i32, r1: i32) -> Vec<(i32, i32)> {
    let (xa, xb) = (c0.min(c1), c0.max(c1));
    let (ya, yb) = (r0.min(r1), r0.max(r1));
    let (cx, cy) = ((xa + xb) as f32 / 2.0, (ya + yb) as f32 / 2.0);
    let (rx, ry) = (((xb - xa) as f32 / 2.0).max(0.5), ((yb - ya) as f32 / 2.0).max(0.5));
    let steps = (((rx + ry) * 8.0) as i32).max(16);
    (0..steps)
        .map(|i| {
            let t = i as f32 / steps as f32 * std::f32::consts::TAU;
            ((cx + rx * t.cos()).round() as i32, (cy + ry * t.sin()).round() as i32)
        })
        .collect()
}

fn shape_cells(kind: i32, c0: i32, r0: i32, c1: i32, r1: i32) -> Vec<(i32, i32)> {
    match kind {
        1 => line_cells(c0, r0, c1, r1),
        2 => rect_cells(c0, r0, c1, r1),
        3 => ellipse_cells(c0, r0, c1, r1),
        _ => Vec::new(),
    }
}

impl qobject::RoomCanvas {
    fn paint(mut self: Pin<&mut Self>, painter: *mut qobject::QPainter) {
        let mut painter = match unsafe { painter.as_mut() } {
            Some(p) => unsafe { Pin::new_unchecked(p) },
            None => return,
        };
        let mode = self.rust().mode;

        // Single sprite tab: stacked labeled subsections, with text labels drawn
        // directly via QPainter on top of the rendered sheet.
        if mode == SHEET {
            let (rgb, w, h, labels) = self.rust().render_sheet();
            let img = unsafe { QImage::from_raw_bytes(rgb, w, h, QImageFormat::Format_RGB888) };
            painter.as_mut().draw_image(&QRect::new(0, 0, w, h), &img);
            let mut font = QFont::default();
            font.set_pixel_size(11);
            font.set_bold(true);
            painter.as_mut().set_font(&font);
            let pen = QPen::from(&QColor::from_rgb(245, 240, 150));
            painter.as_mut().set_pen(&pen);
            for (text, y) in labels {
                painter.as_mut().draw_text(&QPoint::new(4, y + 12), &QString::from(&text));
            }
            return;
        }

        let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
        let (rgb, w, h) = match mode {
            ATLAS => {
                let s = rust.sel();
                let room = &rust.rooms[s];
                let mut rgb = lotw::render::render_metatile_atlas(&rust.prg, &rust.chr, &room.header, &room.pal);
                // inverse-colour border on the hovered metatile (16x16 cells, 16/row)
                if rust.atlas_hover >= 0 && rust.atlas_hover < 256 {
                    let (tx, ty) = (rust.atlas_hover as usize % 16, rust.atlas_hover as usize / 16);
                    invert_border(&mut rgb, 256, tx * 16, ty * 16, 16);
                }
                (rgb, 256, 256)
            }
            WORLD => {
                let mut rgb = rust.world_rgb().to_vec();
                // inverse-colour border around the hovered room
                if rust.world_hover >= 0 {
                    let idx = rust.world_hover as usize;
                    let (mx, my) = (idx % 4, idx / 4);
                    invert_rect_border(&mut rgb, WW as usize, mx * 1024, my * 192, 1024, 192, 3);
                }
                (rgb, WW, WH)
            }
            TITLE => (
                lotw::render::render_nametable(
                    &rust.chr,
                    &rust.prg[TITLE_NT..TITLE_NT + 1024],
                    rust.prg[TITLE_CHR],
                    rust.prg[TITLE_CHR + 1],
                    &rust.prg[TITLE_PAL..TITLE_PAL + 32],
                ),
                256,
                240,
            ),
            _ => {
                let s = rust.sel();
                if rust.room_cache.is_none() || rust.cache_sel != s as i32 {
                    let img = rust.rooms[s].render(&rust.prg, &rust.chr);
                    rust.room_cache = Some(img);
                    rust.cache_sel = s as i32;
                }
                let mut rgb = rust.room_cache.clone().unwrap();
                // collision overlay: tint each cell by its metatile's passability
                // class (terrain solidity is read from the shape id = mt & 0x3F).
                if rust.show_solid {
                    let grid = &rust.rooms[s].grid;
                    for (row, line) in grid.iter().enumerate() {
                        for (col, &mt) in line.iter().enumerate() {
                            if let Some(c) = tile_class_tint(mt & 0x3F) {
                                tint_cell(&mut rgb, 1024, col * 16, row * 16, c);
                            }
                        }
                    }
                }
                // draw object spawn sprites (real entity graphics, transparent bg)
                // using the room's per-area sprite banks.
                let banks = lotw::render::sprite_banks(&rust.rooms[s].header);
                let af = rust.anim_frame;
                for i in 0..12 {
                    if rust.rooms[s].active(i) {
                        let rec = rust.rooms[s].records[i];
                        lotw::render::blit_sprite(&rust.chr, &rust.rooms[s].pal, rec[0].wrapping_add(af), rec[1], &banks, &mut rgb, 1024, rec[2] as usize * 16, rec[3] as usize);
                    }
                }
                // live shape-tool preview: blit the selected metatile into the cells.
                let (kind, c0, r0, c1, r1) = rust.pv;
                if kind != 0 {
                    let room = &rust.rooms[s];
                    let mt = rust.sel_metatile as u8;
                    for (c, r) in shape_cells(kind, c0, r0, c1, r1) {
                        if c >= 0 && r >= 0 && c < 64 && r < 12 {
                            lotw::render::blit_metatile(&rust.prg, &rust.chr, &room.header, &room.pal, mt, &mut rgb, 1024, c as usize * 16, r as usize * 16);
                        }
                    }
                }
                if rust.cursor_col >= 0 && rust.cursor_row >= 0 && rust.cursor_col < 64 && rust.cursor_row < 12 {
                    invert_border(&mut rgb, 1024, rust.cursor_col as usize * 16, rust.cursor_row as usize * 16, 16);
                }
                (rgb, 1024, 192)
            }
        };
        let img = unsafe { QImage::from_raw_bytes(rgb, w, h, QImageFormat::Format_RGB888) };
        // Draw at native size; QML scales the item on the GPU (smooth, cheap).
        painter.draw_image(&QRect::new(0, 0, w, h), &img);
    }

    fn refresh(self: Pin<&mut Self>) {
        self.update();
    }

    fn set_anim(mut self: Pin<&mut Self>, f: i32) {
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            rust.anim_frame = f as u8;
        }
        self.update();
    }

    /// Set the hovered room in the World view; only repaints on change (a World
    /// repaint clones a large cached image, so per-pixel hover moves must not).
    fn set_world_hover(mut self: Pin<&mut Self>, idx: i32) {
        if self.rust().world_hover == idx {
            return;
        }
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            rust.world_hover = idx;
        }
        self.update();
    }

    /// Set the hovered metatile in the atlas; only repaints on change.
    fn set_atlas_hover(mut self: Pin<&mut Self>, idx: i32) {
        if self.rust().atlas_hover == idx {
            return;
        }
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            rust.atlas_hover = idx;
        }
        self.update();
    }

    fn paint_tile(mut self: Pin<&mut Self>, col: i32, row: i32) {
        if self.rust().mode != ROOM {
            return;
        }
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            rust.set_cell(col, row);
        }
        self.update();
    }

    fn paint_line(mut self: Pin<&mut Self>, c0: i32, r0: i32, c1: i32, r1: i32) {
        if self.rust().mode != ROOM {
            return;
        }
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            for (c, r) in line_cells(c0, r0, c1, r1) {
                rust.set_cell(c, r);
            }
        }
        self.update();
    }

    fn paint_rect(mut self: Pin<&mut Self>, c0: i32, r0: i32, c1: i32, r1: i32) {
        if self.rust().mode != ROOM {
            return;
        }
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            for (c, r) in rect_cells(c0, r0, c1, r1) {
                rust.set_cell(c, r);
            }
        }
        self.update();
    }

    fn paint_ellipse(mut self: Pin<&mut Self>, c0: i32, r0: i32, c1: i32, r1: i32) {
        if self.rust().mode != ROOM {
            return;
        }
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            for (c, r) in ellipse_cells(c0, r0, c1, r1) {
                rust.set_cell(c, r);
            }
        }
        self.update();
    }

    fn set_preview(mut self: Pin<&mut Self>, kind: i32, c0: i32, r0: i32, c1: i32, r1: i32) {
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            rust.pv = (kind, c0, r0, c1, r1);
        }
        self.update();
    }

    fn clear_preview(mut self: Pin<&mut Self>) {
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            rust.pv = (0, 0, 0, 0, 0);
        }
        self.update();
    }

    fn erase_tile(mut self: Pin<&mut Self>, col: i32, row: i32) {
        if self.rust().mode != ROOM || col < 0 || row < 0 || col >= 64 || row >= 12 {
            return;
        }
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            let s = rust.sel();
            let orig = rust.orig_rooms[s].grid[row as usize][col as usize];
            rust.rooms[s].grid[row as usize][col as usize] = orig;
            rust.room_cache = None;
            rust.world_cache = None;
        }
        self.update();
    }

    fn metatile_at(&self, col: i32, row: i32) -> i32 {
        let r = self.rust();
        if r.mode != ROOM || col < 0 || row < 0 || col >= 64 || row >= 12 {
            return -1;
        }
        r.rooms[r.sel()].grid[row as usize][col as usize] as i32
    }

    fn obj_active(&self, slot: i32) -> bool {
        let r = self.rust();
        (0..12).contains(&slot) && r.rooms[r.sel()].active(slot as usize)
    }
    fn obj_kind(&self, slot: i32) -> i32 {
        let r = self.rust();
        if (0..12).contains(&slot) { r.rooms[r.sel()].records[slot as usize][0] as i32 } else { 0 }
    }
    fn obj_x(&self, slot: i32) -> i32 {
        let r = self.rust();
        if (0..12).contains(&slot) { r.rooms[r.sel()].records[slot as usize][2] as i32 } else { 0 }
    }
    fn obj_y(&self, slot: i32) -> i32 {
        let r = self.rust();
        if (0..12).contains(&slot) { r.rooms[r.sel()].records[slot as usize][3] as i32 } else { 0 }
    }

    fn obj_byte(&self, slot: i32, i: i32) -> i32 {
        let r = self.rust();
        if (0..12).contains(&slot) && (0..16).contains(&i) {
            r.rooms[r.sel()].records[slot as usize][i as usize] as i32
        } else {
            0
        }
    }

    /// Creature name for an object slot, from the room's enemy bank (header[1])
    /// + spawn tile, via the same RE'd mapping the sprite tab uses.
    fn obj_name(&self, slot: i32) -> QString {
        let r = self.rust();
        if !(0..12).contains(&slot) {
            return QString::from("");
        }
        let room = &r.rooms[r.sel()];
        if !room.active(slot as usize) {
            return QString::from("");
        }
        let rec = room.records[slot as usize];
        let bank = room.header[1];
        let name = if bank >= 48 {
            boss_name(bank).to_string()
        } else if (0x40..0x80).contains(&rec[0]) {
            // window-1 spawn tile -> creature strip k = (tile%64)/16 (4 per bank).
            let k = (rec[0] as usize % 64) / 16;
            area_creature_name(bank, k).map(|s| s.to_string()).unwrap_or_else(|| format!("bank {bank} #{k}"))
        } else {
            format!("tile 0x{:02x}", rec[0])
        };
        QString::from(&name)
    }

    /// One byte (NES colour index 0-63) of the selected room's 32-byte palette.
    /// Bytes 0-15 are the four BG sub-palettes, 16-31 the four sprite ones.
    fn pal_byte(&self, i: i32) -> i32 {
        let r = self.rust();
        *r.rooms[r.sel()].pal.get(i.max(0) as usize).unwrap_or(&0) as i32
    }

    /// "#rrggbb" for NES colour index `c` (0-63), for QML swatches/picker.
    fn nes_color(&self, c: i32) -> QString {
        let (r, g, b) = lotw::render::nes_rgb((c & 0x3f) as u8);
        QString::from(&format!("#{r:02x}{g:02x}{b:02x}"))
    }

    /// Set palette byte `i` (0-31) to NES colour `c` (0-63) in the current room.
    fn set_pal(mut self: Pin<&mut Self>, i: i32, c: i32) {
        if (0..32).contains(&i) {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            let s = rust.sel();
            rust.rooms[s].pal[i as usize] = (c & 0x3f) as u8;
            rust.room_cache = None;
            rust.world_cache = None;
        }
        let rev = self.rust().pal_rev;
        self.as_mut().set_pal_rev(rev + 1);
        self.update();
    }

    fn bump_obj_rev(mut self: Pin<&mut Self>) {
        let rev = self.rust().obj_rev;
        self.as_mut().set_obj_rev(rev + 1);
        self.update(); // redraw object sprites in the room
    }

    fn set_obj(mut self: Pin<&mut Self>, slot: i32, kind: i32, x: i32, y: i32) {
        if (0..12).contains(&slot) {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            let s = rust.sel();
            let rec = &mut rust.rooms[s].records[slot as usize];
            rec[0] = kind.clamp(0, 255) as u8;
            rec[2] = x.clamp(0, 63) as u8;
            rec[3] = y.clamp(0, 191) as u8;
        }
        self.bump_obj_rev();
    }

    fn delete_obj(mut self: Pin<&mut Self>, slot: i32) {
        if (0..12).contains(&slot) {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            let s = rust.sel();
            rust.rooms[s].records[slot as usize] = [0; 16];
        }
        self.bump_obj_rev();
    }

    fn create_obj(mut self: Pin<&mut Self>, x: i32, y: i32, kind: i32) -> i32 {
        let mut made = -1;
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            let s = rust.sel();
            if let Some(free) = (0..9).find(|&i| !rust.rooms[s].active(i)) {
                rust.rooms[s].records[free] =
                    [kind.clamp(0, 255) as u8, 0x02, x.clamp(0, 63) as u8, y.clamp(0, 191) as u8, 0x10, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                made = free as i32;
            }
        }
        self.bump_obj_rev();
        made
    }

    fn copy_obj(mut self: Pin<&mut Self>, slot: i32) -> i32 {
        let mut made = -1;
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            let s = rust.sel();
            if (0..12).contains(&slot) && rust.rooms[s].active(slot as usize) {
                if let Some(free) = (0..9).find(|&i| !rust.rooms[s].active(i)) {
                    let mut rec = rust.rooms[s].records[slot as usize];
                    rec[2] = (rec[2] + 2).min(63); // nudge so the copy is visible
                    rust.rooms[s].records[free] = rec;
                    made = free as i32;
                }
            }
        }
        self.bump_obj_rev();
        made
    }

    fn begin_edit(mut self: Pin<&mut Self>) {
        let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
        let s = rust.sel();
        let snap = Snapshot { idx: s, grid: rust.rooms[s].grid.clone(), records: rust.rooms[s].records.clone() };
        rust.undo.push(snap);
        rust.redo.clear();
    }

    fn undo(mut self: Pin<&mut Self>) {
        let mut sel = None;
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            if let Some(snap) = rust.undo.pop() {
                let cur = Snapshot { idx: snap.idx, grid: rust.rooms[snap.idx].grid.clone(), records: rust.rooms[snap.idx].records.clone() };
                rust.redo.push(cur);
                rust.rooms[snap.idx].grid = snap.grid;
                rust.rooms[snap.idx].records = snap.records;
                rust.room_cache = None;
                rust.world_cache = None;
                sel = Some(snap.idx as i32);
            }
        }
        if let Some(s) = sel {
            self.as_mut().set_selected(s);
            self.as_mut().bump_obj_rev();
            self.update();
        }
    }

    fn redo(mut self: Pin<&mut Self>) {
        let mut sel = None;
        {
            let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
            if let Some(snap) = rust.redo.pop() {
                let cur = Snapshot { idx: snap.idx, grid: rust.rooms[snap.idx].grid.clone(), records: rust.rooms[snap.idx].records.clone() };
                rust.undo.push(cur);
                rust.rooms[snap.idx].grid = snap.grid;
                rust.rooms[snap.idx].records = snap.records;
                rust.room_cache = None;
                rust.world_cache = None;
                sel = Some(snap.idx as i32);
            }
        }
        if let Some(s) = sel {
            self.as_mut().set_selected(s);
            self.as_mut().bump_obj_rev();
            self.update();
        }
    }

    fn world_room_at(&self, x: i32, y: i32) -> i32 {
        let (mx, my) = (x / 1024, y / 192);
        if mx < 0 || mx >= 4 || my < 0 || my >= MAP_ROWS as i32 {
            return -1;
        }
        my * 4 + mx
    }

    fn room_count(&self) -> i32 {
        self.rust().rooms.len() as i32
    }

    fn room_label(&self, idx: i32) -> QString {
        let r = self.rust();
        if let Some(room) = r.rooms.get(idx.max(0) as usize) {
            QString::from(&format!("{:02}-{}", room.mapy, room.mapx))
        } else {
            QString::from("")
        }
    }

    fn img_w(&self) -> i32 {
        match self.rust().mode {
            ATLAS => 256,
            SHEET => (SS_COLS * SS_CELL) as i32,
            WORLD => WW,
            TITLE => 256,
            _ => 1024,
        }
    }
    fn img_h(&self) -> i32 {
        let r = self.rust();
        match r.mode {
            ATLAS => 256,
            SHEET => r.sheet_height(),
            WORLD => WH,
            TITLE => 240,
            _ => 192,
        }
    }

    /// Hover text for the sprite tab: the section the cursor is over, plus the
    /// metasprite cell index within it.
    fn tile_info(&self, x: i32, y: i32) -> QString {
        let r = self.rust();
        if r.mode != SHEET {
            return QString::from("");
        }
        let mut top = 0i32;
        for sec in &r.sections {
            let band = top + SS_LABEL_H as i32;
            let bot = top + sec.height() as i32;
            if y >= top && y < bot {
                let col = (x as usize / sec.cell_px).min(SS_COLS - 1);
                let row = ((y - band).max(0) as usize) / sec.cell_px;
                let idx = row * SS_COLS + col;
                if idx < sec.cells.len() {
                    return QString::from(&format!("{}  — frame {idx}", sec.label));
                }
                return QString::from(&sec.label);
            }
            top = bot;
        }
        QString::from("")
    }

    fn save_rom(&self, path: QString) -> QString {
        let r = self.rust();
        let mut prg = r.prg.clone();
        for room in &r.rooms {
            lotw::render::encode_room(&mut prg, room);
        }
        let mut rom = r.header.clone();
        rom.extend_from_slice(&prg);
        rom.extend_from_slice(&r.chr);
        match std::fs::write(path.to_string(), &rom) {
            Ok(()) => QString::from(&format!("saved {} ({} bytes)", path, rom.len())),
            Err(e) => QString::from(&format!("save failed: {e}")),
        }
    }
}
