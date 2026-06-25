//! Native (egui) asset editor for the LotW ROM.
//!
//! Loads a ROM, decodes the 64 rooms, and presents a world grid (4 map columns
//! x 16 rows — the spatial connectivity) of live room thumbnails. Selecting a
//! room opens it for metatile painting against its tile_table atlas, with actor
//! spawns overlaid. Save writes a rebuilt ROM (byte-identical where unedited).
//!
//! Run: `cargo run --features editor --bin lotw-editor -- [rom] [out.nes]`

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

struct Actor {
    kind: u8,
    x: u8,
    y: u8,
}

struct RoomData {
    mapx: usize,
    mapy: usize,
    off: usize,
    header: Vec<u8>,
    grid: Vec<Vec<u8>>, // ROWS x COLS
    pal: Vec<u8>,
    actors: Vec<Actor>,
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
    world_zoom: f32,
    title_view: bool,
    title_tex: Option<egui::TextureHandle>,
}

// Title screen data (PRG, with banks 12@$8000 / 13@$A000 mapped at the title).
const TITLE_NT: usize = 0x19EC9; // nametable (1024 bytes)
const TITLE_PAL: usize = 0x1A2C9; // 32-byte palette
const TITLE_CHR: usize = 0x1A2E9; // chr0,chr1 bytes

fn room_offset(mapx: usize, mapy: usize) -> usize {
    let bank = mapy / 2;
    let slot = (mapy & 1) * 4 + mapx;
    bank * 0x2000 + slot * 0x400
}

/// Copy a room RGB image into a wider world buffer at pixel (ox, oy).
fn blit_room(world: &mut [u8], world_w: usize, ox: usize, oy: usize, room: &[u8]) {
    for y in 0..RH {
        let src = (y * RW) * 3;
        let dst = ((oy + y) * world_w + ox) * 3;
        world[dst..dst + RW * 3].copy_from_slice(&room[src..src + RW * 3]);
    }
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
                let grid: Vec<Vec<u8>> = (0..ROWS)
                    .map(|r| (0..COLS).map(|c| tiles[c * ROWS + r]).collect())
                    .collect();
                let actors = (0..12)
                    .map(|i| {
                        let rec = &meta[0x20 + i * 16..0x20 + (i + 1) * 16];
                        Actor { kind: rec[0], x: rec[2], y: rec[3] }
                    })
                    .collect();
                rooms.push(RoomData {
                    mapx,
                    mapy,
                    off,
                    header: meta[0..0x20].to_vec(),
                    grid,
                    pal: meta[0xE0..0x100].to_vec(),
                    actors,
                });
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
            world_zoom: 1.0,
            title_view: false,
            title_tex: None,
        })
    }

    fn tex(&self, ctx: &egui::Context, name: &str, w: usize, h: usize, rgb: &[u8]) -> egui::TextureHandle {
        let img = egui::ColorImage::from_rgb([w, h], rgb);
        ctx.load_texture(name, img, egui::TextureOptions::NEAREST)
    }

    fn render_room_tex(&mut self, ctx: &egui::Context) {
        let r = &self.rooms[self.selected];
        let rgb = render::render_room(&self.prg, &self.chr, &r.header, &r.grid, &r.pal);
        self.room_tex = Some(self.tex(ctx, "room", RW, RH, &rgb));
        let atlas = render::render_metatile_atlas(&self.prg, &self.chr, &r.header, &r.pal);
        self.atlas_tex = Some(self.tex(ctx, "atlas", 256, 256, &atlas));
    }

    fn select(&mut self, idx: usize, ctx: &egui::Context) {
        self.selected = idx;
        self.render_room_tex(ctx);
    }

    fn render_title_tex(&mut self, ctx: &egui::Context) {
        let rgb = render::render_nametable(
            &self.chr,
            &self.prg[TITLE_NT..TITLE_NT + 1024],
            self.prg[TITLE_CHR],
            self.prg[TITLE_CHR + 1],
            &self.prg[TITLE_PAL..TITLE_PAL + 32],
        );
        self.title_tex = Some(self.tex(ctx, "title", 256, 240, &rgb));
    }

    /// Stitch all 64 rooms into one continuous world image (4096x3072).
    fn build_world(&mut self, ctx: &egui::Context) {
        let mut rgb = vec![0u8; WW * WH * 3];
        for room in &self.rooms {
            let img = render::render_room(&self.prg, &self.chr, &room.header, &room.grid, &room.pal);
            blit_room(&mut rgb, WW, room.mapx * RW, room.mapy * RH, &img);
        }
        self.world_rgb = rgb;
        self.world_tex = Some(self.tex(ctx, "world", WW, WH, &self.world_rgb));
    }

    /// Re-render one room into the cached world image and refresh the texture.
    fn refresh_world_room(&mut self, idx: usize, ctx: &egui::Context) {
        let room = &self.rooms[idx];
        let img = render::render_room(&self.prg, &self.chr, &room.header, &room.grid, &room.pal);
        blit_room(&mut self.world_rgb, WW, room.mapx * RW, room.mapy * RH, &img);
        self.world_tex = Some(self.tex(ctx, "world", WW, WH, &self.world_rgb));
    }

    fn save(&mut self) {
        let mut prg = self.prg.clone();
        for r in &self.rooms {
            for c in 0..COLS {
                for row in 0..ROWS {
                    prg[r.off + c * ROWS + row] = r.grid[row][c];
                }
            }
        }
        let mut rom = self.ines_header.clone();
        rom.extend_from_slice(&prg);
        rom.extend_from_slice(&self.chr);
        match std::fs::write(&self.out_path, &rom) {
            Ok(()) => self.status = format!("saved {} ({} bytes)", self.out_path, rom.len()),
            Err(e) => self.status = format!("save failed: {e}"),
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
                if ui.toggle_value(&mut self.world_view, "World view").clicked() {
                    if self.world_view {
                        self.title_view = false;
                        if self.world_tex.is_none() {
                            self.build_world(ctx);
                        }
                    }
                }
                if ui.toggle_value(&mut self.title_view, "Title").clicked() {
                    if self.title_view {
                        self.world_view = false;
                        if self.title_tex.is_none() {
                            self.render_title_tex(ctx);
                        }
                    }
                }
                if self.world_view {
                    ui.label("zoom");
                    ui.add(egui::Slider::new(&mut self.world_zoom, 0.25..=2.0).fixed_decimals(2));
                }
                ui.label(format!("out: {}", self.out_path));
                ui.separator();
                ui.label(&self.status);
            });
        });

        // World grid (4 columns x 16 rows): the connectivity map. Each cell is a
        // live room thumbnail; click to open.
        egui::SidePanel::left("world").resizable(true).default_width(300.0).show(ctx, |ui| {
            ui.heading("World (mapX -> , mapY v)");
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut clicked = None;
                for mapy in 0..MAP_ROWS {
                    ui.horizontal(|ui| {
                        for mapx in 0..MAP_COLS {
                            let idx = mapy * MAP_COLS + mapx;
                            if self.thumbs[idx].is_none() {
                                let r = &self.rooms[idx];
                                let rgb = render::render_room(&self.prg, &self.chr, &r.header, &r.grid, &r.pal);
                                self.thumbs[idx] = Some(self.tex(ctx, &format!("thumb{idx}"), RW, RH, &rgb));
                            }
                            let t = self.thumbs[idx].as_ref().unwrap();
                            let size = egui::vec2(64.0, 12.0);
                            let sense = if idx == self.selected { egui::Sense::click() } else { egui::Sense::click() };
                            let resp = ui.add(egui::Image::new(t).fit_to_exact_size(size).sense(sense));
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
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.title_view {
                ui.label("Title screen — nametable $19EC9, CHR banks $1A2E9, palette $1A2C9");
                if let Some(tex) = &self.title_tex {
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.add(egui::Image::new(tex).fit_to_exact_size(egui::vec2(512.0, 480.0)));
                    });
                }
                return;
            }
            if self.world_view {
                ui.label(format!(
                    "Continuous world ({MAP_COLS}x{MAP_ROWS} rooms, {WW}x{WH}px) — paint metatile {} (uses each room's own tileset)",
                    self.sel_metatile
                ));
                let z = self.world_zoom;
                let mut wpaint: Option<(usize, usize, usize)> = None;
                egui::ScrollArea::both().show(ui, |ui| {
                    if let Some(tex) = &self.world_tex {
                        let resp = ui.add(
                            egui::Image::new(tex)
                                .fit_to_exact_size(egui::vec2(WW as f32 * z, WH as f32 * z))
                                .sense(egui::Sense::click()),
                        );
                        // Room boundary grid.
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
                        if resp.clicked() {
                            if let Some(pos) = resp.interact_pointer_pos() {
                                let local = pos - resp.rect.min;
                                let (wx, wy) = ((local.x / z / 16.0) as usize, (local.y / z / 16.0) as usize);
                                let (mapx, mapy) = (wx / COLS, wy / ROWS);
                                if mapx < MAP_COLS && mapy < MAP_ROWS {
                                    wpaint = Some((mapy * MAP_COLS + mapx, wy % ROWS, wx % COLS));
                                }
                            }
                        }
                    }
                });
                if let Some((idx, row, c)) = wpaint {
                    self.rooms[idx].grid[row][c] = self.sel_metatile;
                    self.refresh_world_room(idx, ctx);
                }
                return;
            }

            let r = &self.rooms[self.selected];
            ui.label(format!("Room {:02}-{} @ PRG {:#07x} — selected metatile {}", r.mapy, r.mapx, r.off, self.sel_metatile));

            let mut paint: Option<(usize, usize)> = None;
            let mut pick: Option<u8> = None;
            egui::ScrollArea::both().show(ui, |ui| {
                // Room canvas (paintable). 2x zoom by default.
                if let Some(tex) = &self.room_tex {
                    let scale = 2.0;
                    let size = egui::vec2(RW as f32 * scale, RH as f32 * scale);
                    let resp = ui.add(egui::Image::new(tex).fit_to_exact_size(size).sense(egui::Sense::click()));
                    let p = ui.painter_at(resp.rect);
                    for a in &self.rooms[self.selected].actors {
                        if a.x == 0 && a.y == 0 {
                            continue;
                        }
                        let c = resp.rect.min + egui::vec2(a.x as f32 * 16.0 * scale + 8.0 * scale, a.y as f32 * scale);
                        p.circle_stroke(c, 6.0, egui::Stroke::new(1.5, egui::Color32::YELLOW));
                        p.text(c + egui::vec2(0.0, -10.0), egui::Align2::CENTER_CENTER, a.kind, egui::FontId::monospace(9.0), egui::Color32::YELLOW);
                    }
                    if resp.clicked() {
                        if let Some(pos) = resp.interact_pointer_pos() {
                            let local = pos - resp.rect.min;
                            let (c, row) = ((local.x / (16.0 * scale)) as usize, (local.y / (16.0 * scale)) as usize);
                            if c < COLS && row < ROWS {
                                paint = Some((row, c));
                            }
                        }
                    }
                }

                ui.separator();
                ui.label("Metatile palette (click to pick):");
                if let Some(atlas) = &self.atlas_tex {
                    let z = 32.0; // 2x metatile cells
                    let resp = ui.add(egui::Image::new(atlas).fit_to_exact_size(egui::vec2(16.0 * z, 16.0 * z)).sense(egui::Sense::click()));
                    let sel = self.sel_metatile as usize;
                    let cell = resp.rect.min + egui::vec2((sel % 16) as f32 * z, (sel / 16) as f32 * z);
                    ui.painter_at(resp.rect).rect_stroke(egui::Rect::from_min_size(cell, egui::vec2(z, z)), 0.0, egui::Stroke::new(2.0, egui::Color32::GREEN));
                    if resp.clicked() {
                        if let Some(pos) = resp.interact_pointer_pos() {
                            let local = pos - resp.rect.min;
                            let (mx, my) = ((local.x / z) as usize, (local.y / z) as usize);
                            if mx < 16 && my < 16 {
                                pick = Some((my * 16 + mx) as u8);
                            }
                        }
                    }
                }
            });
            if let Some(m) = pick {
                self.sel_metatile = m;
            }
            if let Some((row, c)) = paint {
                self.rooms[self.selected].grid[row][c] = self.sel_metatile;
                self.render_room_tex(ctx);
            }
        });
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
    eframe::run_native(
        "LotW asset editor",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
