//! Native (egui) asset editor for the LotW ROM.
//!
//! Loads a ROM, decodes the rooms (incl. the special home/outdoor/shrine rows),
//! and presents a world grid of live room thumbnails plus per-room metatile
//! editing. Left pane: tools, the metatile palette, and the clickable world map.
//! Tools: paint (drag), eyedropper, line, rectangle, ellipse. Pinch / ctrl+
//! scroll zooms. Save writes a rebuilt ROM (byte-identical where unedited).
//!
//! Run: `cargo run --features editor --bin lotw-editor -- [rom] [out.nes]`

use std::collections::HashSet;

use eframe::egui;

#[path = "assets/mod.rs"]
mod assets;
use assets::render;

const COLS: usize = 64;
const ROWS: usize = 12;
const TILES: usize = COLS * ROWS;
const ROOM: usize = 1024;
const RW: usize = COLS * 16; // room pixel width (1024)
const RH: usize = ROWS * 16; // room pixel height (192)
const MAP_COLS: usize = 4;
// Rows 0-15 = the main dungeon/overworld grid (banks 0-7). Rows 16-17 (bank 8)
// are the special "home" area: interiors + the outdoor surface (room 16-3) +
// the fragment shrine (row 17).
const MAP_ROWS: usize = 18;
const WW: usize = MAP_COLS * RW; // world pixel width (4096)
const WH: usize = MAP_ROWS * RH; // world pixel height
const WCOLS: i32 = (MAP_COLS * COLS) as i32; // world metatile columns
const WROWS: i32 = (MAP_ROWS * ROWS) as i32; // world metatile rows

// Title screen data (PRG, with banks 12@$8000 / 13@$A000 mapped at the title).
const TITLE_NT: usize = 0x19EC9; // nametable (1024 bytes)
const TITLE_PAL: usize = 0x1A2C9; // 32-byte palette
const TITLE_CHR: usize = 0x1A2E9; // chr0,chr1 bytes

#[derive(PartialEq, Clone, Copy)]
enum Tool {
    Paint,
    Eyedrop,
    Line,
    Rect,
    Ellipse,
    Object,
}

struct RoomData {
    mapx: usize,
    mapy: usize,
    off: usize,
    header: Vec<u8>,
    grid: Vec<Vec<u8>>,      // ROWS x COLS
    pal: Vec<u8>,
    records: Vec<[u8; 16]>, // 12 actor-spawn records; all-zero = empty slot
}

impl RoomData {
    fn active(&self, i: usize) -> bool {
        self.records[i].iter().any(|&b| b != 0)
    }
}

struct App {
    prg: Vec<u8>,
    chr: Vec<u8>,
    ines_header: Vec<u8>,
    rooms: Vec<RoomData>,
    thumbs: Vec<Option<egui::TextureHandle>>,
    room_tex: Option<egui::TextureHandle>,
    atlas_tex: Option<egui::TextureHandle>,
    selected: usize,
    sel_metatile: u8,
    out_path: String,
    status: String,
    world_view: bool,
    world_tex: Option<egui::TextureHandle>,
    world_rgb: Vec<u8>,
    title_view: bool,
    title_tex: Option<egui::TextureHandle>,
    room_zoom: f32,
    world_zoom: f32,
    tool: Tool,
    drag_start: Option<(i32, i32)>, // metatile cell where a shape drag began (view-local)
    pending_nav: Option<usize>,     // room idx to scroll the world view to
    sel_object: Option<usize>,      // selected actor slot in the selected room
    obj_kind: u8,                   // type byte for newly-created objects
}

fn room_offset(mapx: usize, mapy: usize) -> usize {
    let bank = mapy / 2;
    let slot = (mapy & 1) * 4 + mapx;
    bank * 0x2000 + slot * 0x400
}

fn blit_room(world: &mut [u8], world_w: usize, ox: usize, oy: usize, room: &[u8]) {
    for y in 0..RH {
        let src = (y * RW) * 3;
        let dst = ((oy + y) * world_w + ox) * 3;
        world[dst..dst + RW * 3].copy_from_slice(&room[src..src + RW * 3]);
    }
}

// --- shape cell rasterizers (return metatile cell coords) ---
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

fn rect_cells(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let (xa, xb) = (x0.min(x1), x0.max(x1));
    let (ya, yb) = (y0.min(y1), y0.max(y1));
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

fn ellipse_cells(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let (xa, xb) = (x0.min(x1), x0.max(x1));
    let (ya, yb) = (y0.min(y1), y0.max(y1));
    let (cx, cy) = ((xa + xb) as f32 / 2.0, (ya + yb) as f32 / 2.0);
    let (rx, ry) = (((xb - xa) as f32 / 2.0).max(0.5), ((yb - ya) as f32 / 2.0).max(0.5));
    let mut set = HashSet::new();
    let steps = (((rx + ry) * 8.0) as i32).max(16);
    for i in 0..steps {
        let t = i as f32 / steps as f32 * std::f32::consts::TAU;
        let x = (cx + rx * t.cos()).round() as i32;
        let y = (cy + ry * t.sin()).round() as i32;
        set.insert((x, y));
    }
    set.into_iter().collect()
}

impl App {
    fn new(rom_path: &str, out_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let rom = std::fs::read(rom_path)?;
        let prg_len = rom[4] as usize * 16_384;
        let ines_header = rom[0..16].to_vec();
        let prg = rom[16..16 + prg_len].to_vec();
        let chr = rom[16 + prg_len..].to_vec();
        let mut rooms = Vec::new();
        for mapy in 0..MAP_ROWS {
            for mapx in 0..MAP_COLS {
                let off = room_offset(mapx, mapy);
                let tiles = &prg[off..off + TILES];
                let meta = &prg[off + TILES..off + ROOM];
                let grid: Vec<Vec<u8>> = (0..ROWS).map(|r| (0..COLS).map(|c| tiles[c * ROWS + r]).collect()).collect();
                let records = (0..12)
                    .map(|i| {
                        let mut a = [0u8; 16];
                        a.copy_from_slice(&meta[0x20 + i * 16..0x20 + (i + 1) * 16]);
                        a
                    })
                    .collect();
                rooms.push(RoomData { mapx, mapy, off, header: meta[0..0x20].to_vec(), grid, pal: meta[0xE0..0x100].to_vec(), records });
            }
        }
        let thumbs = (0..rooms.len()).map(|_| None).collect();
        Ok(App {
            prg,
            chr,
            ines_header,
            rooms,
            thumbs,
            room_tex: None,
            atlas_tex: None,
            selected: 0,
            sel_metatile: 0,
            out_path,
            status: String::new(),
            world_view: false,
            world_tex: None,
            world_rgb: Vec::new(),
            title_view: false,
            title_tex: None,
            room_zoom: 2.0,
            world_zoom: 0.5,
            tool: Tool::Paint,
            drag_start: None,
            pending_nav: None,
            sel_object: None,
            obj_kind: 0x51,
        })
    }

    fn tex(&self, ctx: &egui::Context, name: &str, w: usize, h: usize, rgb: &[u8]) -> egui::TextureHandle {
        ctx.load_texture(name, egui::ColorImage::from_rgb([w, h], rgb), egui::TextureOptions::NEAREST)
    }

    fn render_room_tex(&mut self, ctx: &egui::Context) {
        let r = &self.rooms[self.selected];
        let rgb = render::render_room(&self.prg, &self.chr, &r.header, &r.grid, &r.pal);
        self.room_tex = Some(self.tex(ctx, "room", RW, RH, &rgb));
        let atlas = render::render_metatile_atlas(&self.prg, &self.chr, &r.header, &r.pal);
        self.atlas_tex = Some(self.tex(ctx, "atlas", 256, 256, &atlas));
    }

    fn render_title_tex(&mut self, ctx: &egui::Context) {
        let rgb = render::render_nametable(&self.chr, &self.prg[TITLE_NT..TITLE_NT + 1024], self.prg[TITLE_CHR], self.prg[TITLE_CHR + 1], &self.prg[TITLE_PAL..TITLE_PAL + 32]);
        self.title_tex = Some(self.tex(ctx, "title", 256, 240, &rgb));
    }

    fn build_world(&mut self, ctx: &egui::Context) {
        let mut rgb = vec![0u8; WW * WH * 3];
        for room in &self.rooms {
            let img = render::render_room(&self.prg, &self.chr, &room.header, &room.grid, &room.pal);
            blit_room(&mut rgb, WW, room.mapx * RW, room.mapy * RH, &img);
        }
        self.world_rgb = rgb;
        self.world_tex = Some(self.tex(ctx, "world", WW, WH, &self.world_rgb));
    }

    fn refresh_world_rooms(&mut self, rooms: &HashSet<usize>, ctx: &egui::Context) {
        for &idx in rooms {
            let room = &self.rooms[idx];
            let img = render::render_room(&self.prg, &self.chr, &room.header, &room.grid, &room.pal);
            blit_room(&mut self.world_rgb, WW, room.mapx * RW, room.mapy * RH, &img);
        }
        self.world_tex = Some(self.tex(ctx, "world", WW, WH, &self.world_rgb));
    }

    fn select(&mut self, idx: usize, ctx: &egui::Context) {
        self.selected = idx;
        self.render_room_tex(ctx);
    }

    fn save(&mut self) {
        let mut prg = self.prg.clone();
        for r in &self.rooms {
            for c in 0..COLS {
                for row in 0..ROWS {
                    prg[r.off + c * ROWS + row] = r.grid[row][c];
                }
            }
            // Meta page: header + 12 actor records + room palette.
            let m = r.off + TILES;
            prg[m..m + 0x20].copy_from_slice(&r.header);
            for i in 0..12 {
                prg[m + 0x20 + i * 16..m + 0x20 + (i + 1) * 16].copy_from_slice(&r.records[i]);
            }
            prg[m + 0xE0..m + 0x100].copy_from_slice(&r.pal);
        }
        let mut rom = self.ines_header.clone();
        rom.extend_from_slice(&prg);
        rom.extend_from_slice(&self.chr);
        match std::fs::write(&self.out_path, &rom) {
            Ok(()) => self.status = format!("saved {} ({} bytes)", self.out_path, rom.len()),
            Err(e) => self.status = format!("save failed: {e}"),
        }
    }

    /// Cells a tool produces between drag start `s` and current `e` (or just `e`).
    fn tool_cells(&self, s: Option<(i32, i32)>, e: (i32, i32)) -> Vec<(i32, i32)> {
        match self.tool {
            Tool::Paint | Tool::Eyedrop | Tool::Object => vec![e],
            Tool::Line => line_cells(s.unwrap_or(e).0, s.unwrap_or(e).1, e.0, e.1),
            Tool::Rect => rect_cells(s.unwrap_or(e).0, s.unwrap_or(e).1, e.0, e.1),
            Tool::Ellipse => ellipse_cells(s.unwrap_or(e).0, s.unwrap_or(e).1, e.0, e.1),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.room_tex.is_none() {
            self.render_room_tex(ctx);
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save ROM").clicked() {
                    self.save();
                }
                if ui.toggle_value(&mut self.world_view, "World").clicked() && self.world_view {
                    self.title_view = false;
                    if self.world_tex.is_none() {
                        self.build_world(ctx);
                    }
                }
                if ui.toggle_value(&mut self.title_view, "Title").clicked() && self.title_view {
                    self.world_view = false;
                    if self.title_tex.is_none() {
                        self.render_title_tex(ctx);
                    }
                }
                // Zoom controls for the active view (pinch/ctrl+scroll also work).
                if !self.title_view {
                    ui.separator();
                    let world = self.world_view;
                    let (lo, hi) = if world { (0.1, 4.0) } else { (0.5, 8.0) };
                    ui.label("zoom");
                    if ui.button("−").clicked() {
                        if world {
                            self.world_zoom = (self.world_zoom / 1.25).max(lo);
                        } else {
                            self.room_zoom = (self.room_zoom / 1.25).max(lo);
                        }
                    }
                    if world {
                        ui.add(egui::Slider::new(&mut self.world_zoom, lo..=hi).fixed_decimals(2));
                    } else {
                        ui.add(egui::Slider::new(&mut self.room_zoom, lo..=hi).fixed_decimals(2));
                    }
                    if ui.button("+").clicked() {
                        if world {
                            self.world_zoom = (self.world_zoom * 1.25).min(hi);
                        } else {
                            self.room_zoom = (self.room_zoom * 1.25).min(hi);
                        }
                    }
                }
                ui.separator();
                ui.label(&self.status);
            });
        });

        egui::SidePanel::left("left").resizable(true).default_width(280.0).show(ctx, |ui| {
            // Tools
            ui.horizontal_wrapped(|ui| {
                for (t, name) in [
                    (Tool::Paint, "✏ Paint"),
                    (Tool::Eyedrop, "💉 Pick"),
                    (Tool::Line, "Line"),
                    (Tool::Rect, "Rect"),
                    (Tool::Ellipse, "Ellipse"),
                    (Tool::Object, "Object"),
                ] {
                    ui.selectable_value(&mut self.tool, t, name);
                }
            });
            ui.label(format!("metatile {} (room {:02}-{})", self.sel_metatile, self.rooms[self.selected].mapy, self.rooms[self.selected].mapx));

            // Object tool: type for new objects + selected object's fields.
            if self.tool == Tool::Object {
                ui.horizontal(|ui| {
                    ui.label("new type:");
                    ui.add(egui::DragValue::new(&mut self.obj_kind).hexadecimal(2, false, false));
                });
                ui.label("click empty = create, click obj = select,\ndrag = move, Delete = remove");
                if let Some(i) = self.sel_object {
                    let rec = &mut self.rooms[self.selected].records[i];
                    ui.horizontal(|ui| {
                        ui.label(format!("obj {i}:"));
                        ui.label("type");
                        ui.add(egui::DragValue::new(&mut rec[0]).hexadecimal(2, false, false));
                        ui.label("hp");
                        ui.add(egui::DragValue::new(&mut rec[4]));
                        ui.label("dmg");
                        ui.add(egui::DragValue::new(&mut rec[5]));
                    });
                }
            }

            // Metatile palette (always visible; uses the selected room's tileset).
            if let Some(atlas) = &self.atlas_tex {
                let resp = ui.add(egui::Image::new(atlas).fit_to_exact_size(egui::vec2(256.0, 256.0)).sense(egui::Sense::click()));
                let sel = self.sel_metatile as usize;
                let cell = resp.rect.min + egui::vec2((sel % 16) as f32 * 16.0, (sel / 16) as f32 * 16.0);
                ui.painter_at(resp.rect).rect_stroke(egui::Rect::from_min_size(cell, egui::vec2(16.0, 16.0)), 0.0, egui::Stroke::new(2.0, egui::Color32::GREEN));
                if resp.clicked() {
                    if let Some(pos) = resp.interact_pointer_pos() {
                        let l = pos - resp.rect.min;
                        let (mx, my) = ((l.x / 16.0) as usize, (l.y / 16.0) as usize);
                        if mx < 16 && my < 16 {
                            self.sel_metatile = (my * 16 + mx) as u8;
                        }
                    }
                }
            }
            ui.separator();
            ui.label("World map (click a room):");
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut clicked = None;
                for mapy in 0..MAP_ROWS {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(1.0, 1.0);
                        for mapx in 0..MAP_COLS {
                            let idx = mapy * MAP_COLS + mapx;
                            if self.thumbs[idx].is_none() {
                                let r = &self.rooms[idx];
                                let rgb = render::render_room(&self.prg, &self.chr, &r.header, &r.grid, &r.pal);
                                self.thumbs[idx] = Some(self.tex(ctx, &format!("thumb{idx}"), RW, RH, &rgb));
                            }
                            let t = self.thumbs[idx].as_ref().unwrap();
                            let resp = ui.add(egui::Image::new(t).fit_to_exact_size(egui::vec2(64.0, 12.0)).sense(egui::Sense::click()));
                            if idx == self.selected {
                                ui.painter().rect_stroke(resp.rect, 0.0, egui::Stroke::new(2.0, egui::Color32::YELLOW));
                            }
                            if resp.clicked() {
                                clicked = Some(idx);
                            }
                        }
                    });
                }
                if let Some(idx) = clicked {
                    self.select(idx, ctx);
                    if self.world_view {
                        self.pending_nav = Some(idx);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.title_view {
                ui.label("Title screen — nametable $19EC9 (view)");
                if let Some(tex) = &self.title_tex {
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(512.0, 480.0)));
                    });
                }
                return;
            }

            if self.world_view {
                self.world_canvas(ui, ctx);
            } else {
                self.room_canvas(ui, ctx);
            }
        });
    }
}

impl App {
    fn room_canvas(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let r = &self.rooms[self.selected];
        ui.label(format!("Room {:02}-{} @ PRG {:#07x} — zoom {:.1}x (pinch / ctrl+scroll)", r.mapy, r.mapx, r.off, self.room_zoom));
        let z = self.room_zoom;
        let mut commit: Vec<(i32, i32)> = Vec::new();
        let mut eyedrop: Option<(i32, i32)> = None;
        let mut preview: Vec<(i32, i32)> = Vec::new();
        let mut obj_click: Option<(i32, i32)> = None;
        let mut obj_drag: Option<(i32, i32)> = None;
        let object_tool = self.tool == Tool::Object;
        egui::ScrollArea::both().show(ui, |ui| {
            let Some(tex) = &self.room_tex else { return };
            let resp = ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(RW as f32 * z, RH as f32 * z)).sense(egui::Sense::click_and_drag()));
            if resp.hovered() {
                let zd = ui.input(|i| i.zoom_delta());
                if zd != 1.0 {
                    self.room_zoom = (self.room_zoom * zd).clamp(0.5, 8.0);
                }
            }
            let cell = |p: egui::Pos2| -> (i32, i32) {
                let l = p - resp.rect.min;
                ((l.x / (16.0 * z)) as i32, (l.y / (16.0 * z)) as i32)
            };
            match self.tool {
                Tool::Paint => {
                    if resp.dragged() || resp.clicked() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            commit.push(cell(p));
                        }
                    }
                }
                Tool::Eyedrop => {
                    if resp.clicked() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            eyedrop = Some(cell(p));
                        }
                    }
                }
                Tool::Object => {
                    if resp.clicked() {
                        obj_click = resp.interact_pointer_pos().map(cell);
                    }
                    if resp.dragged() {
                        obj_drag = resp.interact_pointer_pos().map(cell);
                    }
                }
                Tool::Line | Tool::Rect | Tool::Ellipse => {
                    if resp.drag_started() {
                        self.drag_start = resp.interact_pointer_pos().map(cell);
                    }
                    if resp.dragged() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            preview = self.tool_cells(self.drag_start, cell(p));
                        }
                    }
                    if resp.drag_stopped() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            commit = self.tool_cells(self.drag_start, cell(p));
                        }
                        self.drag_start = None;
                    }
                }
            }
            let p = ui.painter_at(resp.rect);
            // objects: boxes + type byte (selected = orange)
            let room = &self.rooms[self.selected];
            for i in 0..12 {
                if !room.active(i) {
                    continue;
                }
                let rec = &room.records[i];
                let o = resp.rect.min + egui::vec2(rec[2] as f32 * 16.0 * z, rec[3] as f32 * z);
                let rect = egui::Rect::from_min_size(o, egui::vec2(16.0 * z, 16.0 * z));
                let col = if Some(i) == self.sel_object { egui::Color32::from_rgb(255, 130, 0) } else { egui::Color32::YELLOW };
                p.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, col));
                p.text(rect.min, egui::Align2::LEFT_TOP, format!("{:02x}", rec[0]), egui::FontId::monospace(9.0), col);
            }
            // shape preview
            for (cx, cy) in &preview {
                let o = resp.rect.min + egui::vec2(*cx as f32 * 16.0 * z, *cy as f32 * 16.0 * z);
                p.rect_stroke(egui::Rect::from_min_size(o, egui::vec2(16.0 * z, 16.0 * z)), 0.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255)));
            }
            // tile-snapped cursor (white outer + black inner for contrast on any bg)
            if let Some(hp) = resp.hover_pos() {
                let l = hp - resp.rect.min;
                let o = resp.rect.min + egui::vec2((l.x / (16.0 * z)).floor() * 16.0 * z, (l.y / (16.0 * z)).floor() * 16.0 * z);
                let rect = egui::Rect::from_min_size(o, egui::vec2(16.0 * z, 16.0 * z));
                p.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
                p.rect_stroke(rect.shrink(2.0), 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
                ui.ctx().set_cursor_icon(egui::CursorIcon::None);
            }
        });
        // apply paint/eyedrop
        if let Some((c, rr)) = eyedrop {
            if (0..COLS as i32).contains(&c) && (0..ROWS as i32).contains(&rr) {
                self.sel_metatile = self.rooms[self.selected].grid[rr as usize][c as usize];
            }
        }
        if !commit.is_empty() {
            for (c, rr) in commit {
                if (0..COLS as i32).contains(&c) && (0..ROWS as i32).contains(&rr) {
                    self.rooms[self.selected].grid[rr as usize][c as usize] = self.sel_metatile;
                }
            }
            self.render_room_tex(ctx);
        }
        // object: select existing / create new
        if let Some((cx, cy)) = obj_click {
            let room = &self.rooms[self.selected];
            let hit = (0..12).find(|&i| room.active(i) && room.records[i][2] as i32 == cx && (room.records[i][3] as i32) / 16 == cy);
            if let Some(i) = hit {
                self.sel_object = Some(i);
            } else if let Some(free) = (0..9).find(|&i| !self.rooms[self.selected].active(i)) {
                self.rooms[self.selected].records[free] = [self.obj_kind, 0x02, cx as u8, (cy * 16) as u8, 0x10, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                self.sel_object = Some(free);
            }
        }
        // object: drag selected to a new tile
        if let (Some((cx, cy)), Some(i)) = (obj_drag, self.sel_object) {
            let rec = &mut self.rooms[self.selected].records[i];
            rec[2] = cx as u8;
            rec[3] = (cy * 16) as u8;
        }
        // object: delete selected
        if object_tool {
            if let Some(i) = self.sel_object {
                if ui.input(|inp| inp.key_pressed(egui::Key::Delete) || inp.key_pressed(egui::Key::Backspace)) {
                    self.rooms[self.selected].records[i] = [0; 16];
                    self.sel_object = None;
                }
            }
        }
    }

    fn world_canvas(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(format!("Continuous world {WW}x{WH} — zoom {:.2}x (pinch / ctrl+scroll). Paint metatile {} (per-room tileset).", self.world_zoom, self.sel_metatile));
        let z = self.world_zoom;
        let mut commit: Vec<(i32, i32)> = Vec::new();
        let mut eyedrop: Option<(i32, i32)> = None;
        let mut preview: Vec<(i32, i32)> = Vec::new();
        let nav = self.pending_nav.take();
        egui::ScrollArea::both().show(ui, |ui| {
            let Some(tex) = &self.world_tex else { return };
            let resp = ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(WW as f32 * z, WH as f32 * z)).sense(egui::Sense::click_and_drag()));
            if resp.hovered() {
                let zd = ui.input(|i| i.zoom_delta());
                if zd != 1.0 {
                    self.world_zoom = (self.world_zoom * zd).clamp(0.1, 4.0);
                }
            }
            let cell = |p: egui::Pos2| -> (i32, i32) {
                let l = p - resp.rect.min;
                ((l.x / (16.0 * z)) as i32, (l.y / (16.0 * z)) as i32)
            };
            match self.tool {
                Tool::Paint => {
                    if resp.dragged() || resp.clicked() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            commit.push(cell(p));
                        }
                    }
                }
                Tool::Eyedrop => {
                    if resp.clicked() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            eyedrop = Some(cell(p));
                        }
                    }
                }
                Tool::Object => {} // object editing is done in the single-room view
                Tool::Line | Tool::Rect | Tool::Ellipse => {
                    if resp.drag_started() {
                        self.drag_start = resp.interact_pointer_pos().map(cell);
                    }
                    if resp.dragged() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            preview = self.tool_cells(self.drag_start, cell(p));
                        }
                    }
                    if resp.drag_stopped() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            commit = self.tool_cells(self.drag_start, cell(p));
                        }
                        self.drag_start = None;
                    }
                }
            }
            // room boundary grid + preview
            let p = ui.painter_at(resp.rect);
            let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 90));
            for mx in 0..=MAP_COLS {
                let x = resp.rect.min.x + mx as f32 * RW as f32 * z;
                p.line_segment([egui::pos2(x, resp.rect.min.y), egui::pos2(x, resp.rect.max.y)], stroke);
            }
            for my in 0..=MAP_ROWS {
                let y = resp.rect.min.y + my as f32 * RH as f32 * z;
                p.line_segment([egui::pos2(resp.rect.min.x, y), egui::pos2(resp.rect.max.x, y)], stroke);
            }
            for (cx, cy) in &preview {
                let o = resp.rect.min + egui::vec2(*cx as f32 * 16.0 * z, *cy as f32 * 16.0 * z);
                p.rect_stroke(egui::Rect::from_min_size(o, egui::vec2(16.0 * z, 16.0 * z)), 0.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255)));
            }
            // tile-snapped cursor
            if let Some(hp) = resp.hover_pos() {
                let o = resp.rect.min + egui::vec2((((hp.x - resp.rect.min.x) / (16.0 * z)).floor()) * 16.0 * z, (((hp.y - resp.rect.min.y) / (16.0 * z)).floor()) * 16.0 * z);
                let rect = egui::Rect::from_min_size(o, egui::vec2(16.0 * z, 16.0 * z));
                p.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
                p.rect_stroke(rect.shrink(2.0), 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
                ui.ctx().set_cursor_icon(egui::CursorIcon::None);
            }
            // navigate to a clicked room
            if let Some(idx) = nav {
                let (mx, my) = (idx % MAP_COLS, idx / MAP_COLS);
                let target = egui::Rect::from_min_size(resp.rect.min + egui::vec2(mx as f32 * RW as f32 * z, my as f32 * RH as f32 * z), egui::vec2(RW as f32 * z, RH as f32 * z));
                ui.scroll_to_rect(target, Some(egui::Align::Center));
            }
        });
        if let Some((c, rr)) = eyedrop {
            if (0..WCOLS).contains(&c) && (0..WROWS).contains(&rr) {
                let idx = (rr as usize / ROWS) * MAP_COLS + c as usize / COLS;
                self.sel_metatile = self.rooms[idx].grid[rr as usize % ROWS][c as usize % COLS];
            }
        }
        if !commit.is_empty() {
            let mut affected = HashSet::new();
            for (c, rr) in commit {
                if (0..WCOLS).contains(&c) && (0..WROWS).contains(&rr) {
                    let idx = (rr as usize / ROWS) * MAP_COLS + c as usize / COLS;
                    self.rooms[idx].grid[rr as usize % ROWS][c as usize % COLS] = self.sel_metatile;
                    affected.insert(idx);
                }
            }
            self.refresh_world_rooms(&affected, ctx);
        }
    }
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let rom = args.get(1).cloned().unwrap_or_else(|| "rom/lotw.nes".into());
    let out = args.get(2).cloned().unwrap_or_else(|| "build/lotw-edited.nes".into());
    let app = match App::new(&rom, out) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("failed to load {rom}: {e}");
            std::process::exit(1);
        }
    };
    eframe::run_native("LotW asset editor", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(app))))
}
