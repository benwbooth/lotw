use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QImage, QImageFormat, QRect, QString};

// View modes.
pub const ROOM: i32 = 0;
pub const ATLAS: i32 = 1;
pub const WORLD: i32 = 2;
pub const TITLE: i32 = 3;
pub const SPRITES: i32 = 4;

const MAP_ROWS: usize = 18;
const WW: i32 = 4 * 1024; // world width
const WH: i32 = MAP_ROWS as i32 * 192; // world height (18 rows)
const TITLE_NT: usize = 0x19EC9;
const TITLE_PAL: usize = 0x1A2C9;
const TITLE_CHR: usize = 0x1A2E9;

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
        #[qproperty(i32, sprite_pal)] // 0 = greyscale, 1..4 = room sprite palettes
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
        fn entity_count(self: &RoomCanvas) -> i32;
        #[qinvokable]
        fn entity_info(self: &RoomCanvas, i: i32) -> QString;
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
    sprite_pal: i32,
    anim_frame: u8, // sprite animation offset (0 or 4), driven by a QML timer
    entities: Vec<Entity>, // unique actor appearances across all rooms
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
}

/// A distinct actor appearance (sprite tile + attributes) and a room it appears
/// in (for the correct sprite palette + CHR banks).
#[derive(Clone, Copy)]
struct Entity {
    tile: u8,
    attr: u8,
    room: usize,
    behavior: u8,
}

const SS_COLS: usize = 12;
const SS_CELL: usize = 24;

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
        // Collect unique actor appearances (by tile + sub-palette) across rooms.
        let mut entities: Vec<Entity> = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for (idx, room) in rooms.iter().enumerate() {
            for i in 0..12 {
                if room.active(i) {
                    let rec = room.records[i];
                    if seen.insert((rec[0], rec[1] & 3)) {
                        entities.push(Entity { tile: rec[0], attr: rec[1], room: idx, behavior: rec[8] });
                    }
                }
            }
        }
        entities.sort_by_key(|e| (e.behavior, e.tile));
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
            sprite_pal: 0,
            anim_frame: 0,
            entities,
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
        let painter = match unsafe { painter.as_mut() } {
            Some(p) => unsafe { Pin::new_unchecked(p) },
            None => return,
        };
        let mode = self.rust().mode;
        let rust = unsafe { self.as_mut().rust_mut().get_unchecked_mut() };
        let (rgb, w, h) = match mode {
            ATLAS => {
                let s = rust.sel();
                let room = &rust.rooms[s];
                (lotw::render::render_metatile_atlas(&rust.prg, &rust.chr, &room.header, &room.pal), 256, 256)
            }
            WORLD => (rust.world_rgb().to_vec(), WW, WH),
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
            SPRITES => {
                // Full CHR tile dump: 64 cols (one 64-tile bank per row), 8x8 each.
                let cols = 64usize;
                let n = rust.chr.len() / 16;
                let rows = n.div_ceil(cols);
                let (w, h) = (cols * 8, rows * 8);
                let mut buf = vec![0u8; w * h * 3];
                // palette: 0 = greyscale, 1..4 = the selected room's sprite sub-palettes
                let sp = rust.sprite_pal;
                let s = rust.sel();
                let pal4: [(u8, u8, u8); 4] = if sp == 0 {
                    [(0, 0, 0), (85, 85, 85), (170, 170, 170), (255, 255, 255)]
                } else {
                    let p = &rust.rooms[s].pal;
                    let b = (4 + (sp as usize - 1).min(3)) * 4;
                    [lotw::render::nes_rgb(p[0]), lotw::render::nes_rgb(p[b + 1]), lotw::render::nes_rgb(p[b + 2]), lotw::render::nes_rgb(p[b + 3])]
                };
                for t in 0..n {
                    let base = t * 16;
                    let (ox, oy) = ((t % cols) * 8, (t / cols) * 8);
                    for y in 0..8 {
                        let (p0, p1) = (rust.chr[base + y], rust.chr[base + y + 8]);
                        for x in 0..8 {
                            let v = ((p0 >> (7 - x)) & 1) | (((p1 >> (7 - x)) & 1) << 1);
                            let (r, g, b) = pal4[v as usize];
                            let o = ((oy + y) * w + ox + x) * 3;
                            buf[o] = r;
                            buf[o + 1] = g;
                            buf[o + 2] = b;
                        }
                    }
                }
                (buf, w as i32, h as i32)
            }
            _ => {
                let s = rust.sel();
                if rust.room_cache.is_none() || rust.cache_sel != s as i32 {
                    let img = rust.rooms[s].render(&rust.prg, &rust.chr);
                    rust.room_cache = Some(img);
                    rust.cache_sel = s as i32;
                }
                let mut rgb = rust.room_cache.clone().unwrap();
                // draw object spawn sprites (real entity graphics, transparent bg).
                let banks = lotw::render::sprite_banks(rust.rooms[s].mapy);
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
            SPRITES => 64 * 8,
            WORLD => WW,
            TITLE => 256,
            _ => 1024,
        }
    }
    fn img_h(&self) -> i32 {
        let r = self.rust();
        match r.mode {
            ATLAS => 256,
            SPRITES => (r.chr.len() / 16).div_ceil(64) as i32 * 8,
            WORLD => WH,
            TITLE => 240,
            _ => 192,
        }
    }

    fn entity_count(&self) -> i32 {
        self.rust().entities.len() as i32
    }
    fn entity_info(&self, i: i32) -> QString {
        let r = self.rust();
        if let Some(e) = r.entities.get(i.max(0) as usize) {
            QString::from(&format!("tile 0x{:02x}  palette {}  behavior {}  (in room {:02}-{})", e.tile, e.attr & 3, e.behavior, r.rooms[e.room].mapy, r.rooms[e.room].mapx))
        } else {
            QString::from("")
        }
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
