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
}

fn room_offset(mapx: usize, mapy: usize) -> usize {
    let bank = mapy / 2;
    let slot = (mapy & 1) * 4 + mapx;
    bank * 0x2000 + slot * 0x400
}

impl App {
    fn new(rom_path: &str, out_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let rom = std::fs::read(rom_path)?;
        let prg_len = rom[4] as usize * 16_384;
        let ines_header = rom[0..16].to_vec();
        let prg = rom[16..16 + prg_len].to_vec();
        let chr = rom[16 + prg_len..].to_vec();
        let mut rooms = Vec::new();
        for mapy in 0..16 {
            for mapx in 0..4 {
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
                for mapy in 0..16 {
                    ui.horizontal(|ui| {
                        for mapx in 0..4 {
                            let idx = mapy * 4 + mapx;
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
