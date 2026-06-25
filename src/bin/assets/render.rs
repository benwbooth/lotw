//! Visual renderer: turns the byte-level data (CHR + metatile tile_table +
//! palette) into actual pixels, so rooms/sprites can be seen and edited. This is
//! a *view* on top of the lossless byte layer (not part of the roundtrip).
//!
//! Room pipeline: each room's descriptor gives a tile_table page (PRG bank 9 at
//! $A000, base 0x12000 + page*256) and CHR banks 0/1 (descriptor +5/+6). A room
//! grid byte is a metatile; the tile_table maps it to a 2x2 CHR quad
//! [TL,TR,BL,BR]; each CHR index goes through the MMC3 1KB windows formed from
//! CHR banks 0/1 to fetch 8x8 2bpp pixels, coloured by the room palette.

use std::error::Error;
use std::fs;
use std::io::BufWriter;
use std::path::Path;

use super::palettes::nes_rgb;

const TT_BASE: usize = 0x12000; // PRG bank 9 ($A000) holds the metatile tile_table
const ROWS: usize = 12;
const COLS: usize = 64;

/// Render one room to an RGB image (COLS*16 x ROWS*12*... = 1024x192).
/// `header` = the 32-byte room descriptor, `grid` = ROWS x COLS metatiles,
/// `pal` = the 32-byte room palette.
pub fn render_room(prg: &[u8], chr: &[u8], header: &[u8], grid: &[Vec<u8>], pal: &[u8]) -> Vec<u8> {
    let tt = TT_BASE + header[0] as usize * 256;
    // BG pattern table $0000 = MMC3 R0/R1 expanded to four 1 KB windows.
    let (cb0, cb1) = (header[5] & 0xFE, header[6] & 0xFE);
    let win = [cb0, cb0 | 1, cb1, cb1 | 1];
    let chr_tile = |t: usize| win[t / 64] as usize * 1024 + (t % 64) * 16;

    let (w, h) = (COLS * 16, ROWS * 16);
    let mut img = vec![0u8; w * h * 3];
    let mut put = |img: &mut [u8], px: usize, py: usize, pixel: u8, subpal: usize| {
        let (r, g, b) = nes_rgb(pal[subpal * 4 + pixel as usize]);
        let o = (py * w + px) * 3;
        img[o] = r;
        img[o + 1] = g;
        img[o + 2] = b;
    };
    for (ry, row) in grid.iter().enumerate() {
        for (cx, &mt) in row.iter().enumerate() {
            // A metatile's BG sub-palette is its top 2 bits (the attribute build
            // reads metatile_id & 0xC0); the low 6 bits select within the group.
            let subpal = (mt as usize >> 6) & 3;
            let quad = &prg[tt + mt as usize * 4..tt + mt as usize * 4 + 4];
            // quad is column-major: 0=TL 1=BL 2=TR 3=BR (the view writes entry
            // 0/1 down one nametable column, then 2/3 down the next).
            for (qi, &t) in quad.iter().enumerate() {
                let (sx, sy) = ((qi / 2) * 8, (qi & 1) * 8);
                let base = chr_tile(t as usize);
                for y in 0..8 {
                    let (p0, p1) = (chr.get(base + y).copied().unwrap_or(0), chr.get(base + y + 8).copied().unwrap_or(0));
                    for x in 0..8 {
                        let bit = 7 - x;
                        let v = ((p0 >> bit) & 1) | (((p1 >> bit) & 1) << 1);
                        put(&mut img, cx * 16 + sx + x, ry * 16 + sy + y, v, subpal);
                    }
                }
            }
        }
    }
    img
}

/// Render a metatile atlas (256 metatiles laid out 16x16 = 256x256 px) for a
/// room's tile_table/CHR/palette, for use as a paint palette in the editor.
pub fn render_metatile_atlas(prg: &[u8], chr: &[u8], header: &[u8], pal: &[u8]) -> Vec<u8> {
    let tt = TT_BASE + header[0] as usize * 256;
    let (cb0, cb1) = (header[5] & 0xFE, header[6] & 0xFE);
    let win = [cb0, cb0 | 1, cb1, cb1 | 1];
    let (w, h) = (256usize, 256usize);
    let mut img = vec![0u8; w * h * 3];
    for mt in 0..256usize {
        let (ax, ay) = ((mt % 16) * 16, (mt / 16) * 16);
        let subpal = (mt >> 6) & 3;
        let quad = &prg[tt + mt * 4..tt + mt * 4 + 4];
        for (qi, &t) in quad.iter().enumerate() {
            let (sx, sy) = ((qi / 2) * 8, (qi & 1) * 8);
            let base = win[t as usize / 64] as usize * 1024 + (t as usize % 64) * 16;
            for y in 0..8 {
                let (p0, p1) = (chr.get(base + y).copied().unwrap_or(0), chr.get(base + y + 8).copied().unwrap_or(0));
                for x in 0..8 {
                    let v = ((p0 >> (7 - x)) & 1) | (((p1 >> (7 - x)) & 1) << 1);
                    let (r, g, b) = nes_rgb(pal[subpal * 4 + v as usize]);
                    let o = ((ay + sy + y) * w + (ax + sx + x)) * 3;
                    img[o] = r;
                    img[o + 1] = g;
                    img[o + 2] = b;
                }
            }
        }
    }
    img
}

pub fn rgb_to_png_bytes(w: usize, h: usize, rgb: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut out = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, w as u32, h as u32);
        enc.set_color(png::ColorType::Rgb);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header()?.write_image_data(rgb)?;
    }
    Ok(out)
}

pub fn write_rgb_png(path: &Path, w: usize, h: usize, rgb: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }
    let file = BufWriter::new(fs::File::create(path)?);
    let mut enc = png::Encoder::new(file, w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()?.write_image_data(rgb)?;
    Ok(())
}

/// Render all rooms to `outdir/room-YY-X.png` using the extracted rooms manifest.
pub fn render_all_rooms(prg: &[u8], chr: &[u8], assets_dir: &Path, outdir: &Path) -> Result<(), Box<dyn Error>> {
    let rdir = assets_dir.join("rooms");
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(rdir.join("manifest.json"))?)?;
    fs::create_dir_all(outdir)?;
    let mut count = 0;
    for room in manifest["rooms"].as_array().ok_or("bad manifest")? {
        let (mapx, mapy) = (room["mapx"].as_u64().unwrap(), room["mapy"].as_u64().unwrap());
        let header = unhex(room["header_hex"].as_str().unwrap())?;
        let pal: Vec<u8> = room["palette"]["indices"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect();
        let csv = fs::read_to_string(rdir.join(format!("room-{mapy:02}-{mapx}.csv")))?;
        let grid: Vec<Vec<u8>> = csv.lines().filter(|l| !l.trim().is_empty())
            .map(|l| l.split(',').map(|t| t.trim().parse().unwrap()).collect()).collect();
        let img = render_room(prg, chr, &header, &grid, &pal);
        write_rgb_png(&outdir.join(format!("room-{mapy:02}-{mapx}.png")), COLS * 16, ROWS * 16, &img)?;
        count += 1;
    }
    println!("rendered {count} rooms -> {}", outdir.display());
    Ok(())
}

/// Stitch all rooms into one world map (4 map columns x 16 rows) — the spatial
/// connectivity of the world at a glance.
pub fn render_world(prg: &[u8], chr: &[u8], assets_dir: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
    let rdir = assets_dir.join("rooms");
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(rdir.join("manifest.json"))?)?;
    let (rw, rh) = (COLS * 16, ROWS * 16); // one room
    let (ww, wh) = (4 * rw, 16 * rh);
    let mut world = vec![0u8; ww * wh * 3];
    for room in manifest["rooms"].as_array().ok_or("bad manifest")? {
        let (mx, my) = (room["mapx"].as_u64().unwrap() as usize, room["mapy"].as_u64().unwrap() as usize);
        let header = unhex(room["header_hex"].as_str().unwrap())?;
        let pal: Vec<u8> = room["palette"]["indices"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect();
        let csv = fs::read_to_string(rdir.join(format!("room-{my:02}-{mx}.csv")))?;
        let grid: Vec<Vec<u8>> = csv.lines().filter(|l| !l.trim().is_empty())
            .map(|l| l.split(',').map(|t| t.trim().parse().unwrap()).collect()).collect();
        let img = render_room(prg, chr, &header, &grid, &pal);
        let (ox, oy) = (mx * rw, my * rh);
        for y in 0..rh {
            let src = (y * rw) * 3;
            let dst = ((oy + y) * ww + ox) * 3;
            world[dst..dst + rw * 3].copy_from_slice(&img[src..src + rw * 3]);
        }
    }
    write_rgb_png(out, ww, wh, &world)?;
    println!("world map {ww}x{wh} -> {}", out.display());
    Ok(())
}

fn unhex(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    (0..s.len() / 2).map(|i| Ok(u8::from_str_radix(&s[2 * i..2 * i + 2], 16)?)).collect()
}
