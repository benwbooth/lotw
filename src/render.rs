//! Pure pixel rendering for the asset tooling/editors: metatile rooms,
//! tile_table atlases, and raw nametable screens -> RGB. No external deps (just
//! slice math), so both `assettool` and the Qt editor can share it.

pub const COLS: usize = 64;
pub const ROWS: usize = 12;
pub const TILES: usize = COLS * ROWS; // 768
pub const ROOM: usize = 1024;
pub const RW: usize = COLS * 16; // room pixel width (1024)
pub const RH: usize = ROWS * 16; // room pixel height (192)
pub const MAP_COLS: usize = 4;
const TT_BASE: usize = 0x12000; // PRG bank 9 ($A000) holds the metatile tile_table

/// Standard NES (2C02) master palette, 64 entries.
pub const NES_PALETTE: [(u8, u8, u8); 64] = [
    (84, 84, 84), (0, 30, 116), (8, 16, 144), (48, 0, 136), (68, 0, 100), (92, 0, 48),
    (84, 4, 0), (60, 24, 0), (32, 42, 0), (8, 58, 0), (0, 64, 0), (0, 60, 0),
    (0, 50, 60), (0, 0, 0), (0, 0, 0), (0, 0, 0),
    (152, 150, 152), (8, 76, 196), (48, 50, 236), (92, 30, 228), (136, 20, 176),
    (160, 20, 100), (152, 34, 32), (120, 60, 0), (84, 90, 0), (40, 114, 0), (8, 124, 0),
    (0, 118, 40), (0, 102, 120), (0, 0, 0), (0, 0, 0), (0, 0, 0),
    (236, 238, 236), (76, 154, 236), (120, 124, 236), (176, 98, 236), (228, 84, 236),
    (236, 88, 180), (236, 106, 100), (212, 136, 32), (160, 170, 0), (116, 196, 0),
    (76, 208, 32), (56, 204, 108), (56, 180, 204), (60, 60, 60), (0, 0, 0), (0, 0, 0),
    (236, 238, 236), (168, 204, 236), (188, 188, 236), (212, 178, 236), (236, 174, 236),
    (236, 174, 212), (236, 180, 176), (228, 196, 144), (204, 210, 120), (180, 222, 120),
    (168, 226, 144), (152, 226, 180), (160, 214, 228), (160, 162, 160), (0, 0, 0), (0, 0, 0),
];

pub fn nes_rgb(i: u8) -> (u8, u8, u8) {
    NES_PALETTE[(i & 0x3F) as usize]
}

/// PRG offset of a room: room bank = mapY/2, 8 rooms/bank at a 0x400 stride.
pub fn room_offset(mapx: usize, mapy: usize) -> usize {
    let bank = mapy / 2;
    let slot = (mapy & 1) * 4 + mapx;
    bank * 0x2000 + slot * 0x400
}

/// Render one room to an RGB image (1024x192). `header` = the 32-byte room
/// descriptor (byte0 = tile_table page, +5/+6 = BG CHR banks), `grid` = ROWS x
/// COLS metatiles, `pal` = the 32-byte room palette.
pub fn render_room(prg: &[u8], chr: &[u8], header: &[u8], grid: &[Vec<u8>], pal: &[u8]) -> Vec<u8> {
    let tt = TT_BASE + header[0] as usize * 256;
    let (cb0, cb1) = (header[5] & 0xFE, header[6] & 0xFE);
    let win = [cb0, cb0 | 1, cb1, cb1 | 1];
    let (w, h) = (COLS * 16, ROWS * 16);
    let mut img = vec![0u8; w * h * 3];
    for (ry, row) in grid.iter().enumerate() {
        for (cx, &mt) in row.iter().enumerate() {
            // metatile byte: low 6 bits = shape (64 of them; the view does
            // `(idx<<2) as u8`), high 2 bits = BG sub-palette.
            let subpal = (mt as usize >> 6) & 3;
            let shape = mt as usize & 0x3F;
            let quad = &prg[tt + shape * 4..tt + shape * 4 + 4];
            // column-major sub-tiles: 0=TL 1=BL 2=TR 3=BR
            for (qi, &t) in quad.iter().enumerate() {
                let (sx, sy) = ((qi / 2) * 8, (qi & 1) * 8);
                let base = win[t as usize / 64] as usize * 1024 + (t as usize % 64) * 16;
                blit_tile(&mut img, w, cx * 16 + sx, ry * 16 + sy, chr, base, pal, subpal);
            }
        }
    }
    img
}

/// Render a metatile atlas (256 metatiles, 16x16 layout = 256x256) for a room's
/// tile_table/CHR/palette, used as a paint palette.
pub fn render_metatile_atlas(prg: &[u8], chr: &[u8], header: &[u8], pal: &[u8]) -> Vec<u8> {
    let tt = TT_BASE + header[0] as usize * 256;
    let (cb0, cb1) = (header[5] & 0xFE, header[6] & 0xFE);
    let win = [cb0, cb0 | 1, cb1, cb1 | 1];
    let (w, _h) = (256usize, 256usize);
    let mut img = vec![0u8; w * 256 * 3];
    for mt in 0..256usize {
        let (ax, ay) = ((mt % 16) * 16, (mt / 16) * 16);
        let subpal = (mt >> 6) & 3;
        let shape = mt & 0x3F;
        let quad = &prg[tt + shape * 4..tt + shape * 4 + 4];
        for (qi, &t) in quad.iter().enumerate() {
            let (sx, sy) = ((qi / 2) * 8, (qi & 1) * 8);
            let base = win[t as usize / 64] as usize * 1024 + (t as usize % 64) * 16;
            blit_tile(&mut img, w, ax + sx, ay + sy, chr, base, pal, subpal);
        }
    }
    img
}

/// Render a raw 32x30 nametable (960 tiles + 64 attribute bytes) to 256x240 RGB,
/// for full-screen layouts that aren't metatile rooms (the title screen).
pub fn render_nametable(chr: &[u8], nt: &[u8], chr0: u8, chr1: u8, pal: &[u8]) -> Vec<u8> {
    let win = [chr0 & 0xFE, (chr0 & 0xFE) | 1, chr1 & 0xFE, (chr1 & 0xFE) | 1];
    let (w, h) = (256usize, 240usize);
    let mut img = vec![0u8; w * h * 3];
    for row in 0..30 {
        for col in 0..32 {
            let t = nt[row * 32 + col] as usize;
            let attr = nt[0x3C0 + (row / 4) * 8 + (col / 4)] as usize;
            let quad = ((row / 2) & 1) * 2 + ((col / 2) & 1);
            let sp = (attr >> (quad * 2)) & 3;
            let base = win[t / 64] as usize * 1024 + (t % 64) * 16;
            blit_tile(&mut img, w, col * 8, row * 8, chr, base, pal, sp);
        }
    }
    img
}

/// Render a single metatile `mt` (shape = low 6 bits, palette = high 2) into an
/// RGB buffer at pixel (px,py). Used for shape-tool previews.
pub fn blit_metatile(prg: &[u8], chr: &[u8], header: &[u8], pal: &[u8], mt: u8, dst: &mut [u8], dst_w: usize, px: usize, py: usize) {
    let tt = TT_BASE + header[0] as usize * 256;
    let (cb0, cb1) = (header[5] & 0xFE, header[6] & 0xFE);
    let win = [cb0, cb0 | 1, cb1, cb1 | 1];
    let subpal = (mt as usize >> 6) & 3;
    let shape = mt as usize & 0x3F;
    let quad = &prg[tt + shape * 4..tt + shape * 4 + 4];
    for (qi, &t) in quad.iter().enumerate() {
        let (sx, sy) = ((qi / 2) * 8, (qi & 1) * 8);
        let base = win[t as usize / 64] as usize * 1024 + (t as usize % 64) * 16;
        blit_tile(dst, dst_w, px + sx, py + sy, chr, base, pal, subpal);
    }
}

/// Draw one 8x8 2bpp CHR tile at (px,py) into an RGB buffer using sub-palette
/// `subpal` (pixel value 0 always uses the universal backdrop pal[0]).
fn blit_tile(img: &mut [u8], w: usize, px: usize, py: usize, chr: &[u8], base: usize, pal: &[u8], subpal: usize) {
    for y in 0..8 {
        let p0 = chr.get(base + y).copied().unwrap_or(0);
        let p1 = chr.get(base + y + 8).copied().unwrap_or(0);
        for x in 0..8 {
            let bit = 7 - x;
            let v = ((p0 >> bit) & 1) | (((p1 >> bit) & 1) << 1);
            let idx = if v == 0 { pal[0] } else { pal[subpal * 4 + v as usize] };
            let (r, g, b) = nes_rgb(idx);
            let o = ((py + y) * w + px + x) * 3;
            img[o] = r;
            img[o + 1] = g;
            img[o + 2] = b;
        }
    }
}

/// A decoded room: metatile grid + meta page (header, actor records, palette).
#[derive(Clone)]
pub struct Room {
    pub mapx: usize,
    pub mapy: usize,
    pub off: usize,
    pub header: Vec<u8>,
    pub grid: Vec<Vec<u8>>,    // ROWS x COLS, row-major
    pub pal: Vec<u8>,          // 32 bytes
    pub records: Vec<[u8; 16]>, // 12 actor-spawn records
}

impl Room {
    pub fn active(&self, i: usize) -> bool {
        self.records[i].iter().any(|&b| b != 0)
    }
    pub fn render(&self, prg: &[u8], chr: &[u8]) -> Vec<u8> {
        render_room(prg, chr, &self.header, &self.grid, &self.pal)
    }
}

/// Decode `map_rows` x 4 rooms from PRG (mapy 0-15 = dungeon, 16-17 = special).
pub fn decode_rooms(prg: &[u8], map_rows: usize) -> Vec<Room> {
    let mut rooms = Vec::new();
    for mapy in 0..map_rows {
        for mapx in 0..MAP_COLS {
            let off = room_offset(mapx, mapy);
            let tiles = &prg[off..off + TILES];
            let meta = &prg[off + TILES..off + ROOM];
            let grid = (0..ROWS).map(|r| (0..COLS).map(|c| tiles[c * ROWS + r]).collect()).collect();
            let records = (0..12)
                .map(|i| {
                    let mut a = [0u8; 16];
                    a.copy_from_slice(&meta[0x20 + i * 16..0x20 + (i + 1) * 16]);
                    a
                })
                .collect();
            rooms.push(Room { mapx, mapy, off, header: meta[0..0x20].to_vec(), grid, pal: meta[0xE0..0x100].to_vec(), records });
        }
    }
    rooms
}

/// Write a room's grid + meta page back into a PRG image (column-major).
pub fn encode_room(prg: &mut [u8], room: &Room) {
    for c in 0..COLS {
        for r in 0..ROWS {
            prg[room.off + c * ROWS + r] = room.grid[r][c];
        }
    }
    let m = room.off + TILES;
    prg[m..m + 0x20].copy_from_slice(&room.header);
    for i in 0..12 {
        prg[m + 0x20 + i * 16..m + 0x20 + (i + 1) * 16].copy_from_slice(&room.records[i]);
    }
    prg[m + 0xE0..m + 0x100].copy_from_slice(&room.pal);
}
