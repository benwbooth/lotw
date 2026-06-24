//! CHR (pattern-table graphics) <-> indexed PNG.
//!
//! NES CHR is 8x8 tiles, 16 bytes each: 8 bytes of bit-plane 0 (low bit) then 8
//! bytes of bit-plane 1 (high bit); pixel value = plane0_bit | (plane1_bit << 1)
//! in 0..3. Each pixel maps directly to a palette index in an indexed PNG, so
//! the round-trip is exactly lossless (the PNG's 4-grey palette is only for
//! viewing; the stored value is the 2-bit pattern, not a colour).
//!
//! The 64 KB CHR (4096 tiles) is split into 8 sheets of 512 tiles (one per 8 KB
//! CHR region), laid out 16 tiles wide x 32 tall = 128x256 px.

use std::error::Error;
use std::fs;
use std::io::BufWriter;
use std::path::Path;

const TILE_BYTES: usize = 16;
const TILES_PER_SHEET: usize = 512; // 8 KB
const COLS: usize = 16;
const ROWS: usize = TILES_PER_SHEET / COLS; // 32
const SHEET_W: usize = COLS * 8; // 128
const SHEET_H: usize = ROWS * 8; // 256

/// 4-grey viewing palette (RGB) for the indexed PNGs; indices 0..3 are the
/// 2-bit pattern values.
const GREYS: [u8; 12] = [0, 0, 0, 85, 85, 85, 170, 170, 170, 255, 255, 255];

/// Decode the CHR blob into `outdir/bank-N.png` indexed sheets.
pub fn extract(chr: &[u8], outdir: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(outdir)?;
    let sheets = chr.len() / TILE_BYTES / TILES_PER_SHEET;
    for s in 0..sheets {
        let mut px = vec![0u8; SHEET_W * SHEET_H];
        for t in 0..TILES_PER_SHEET {
            let off = (s * TILES_PER_SHEET + t) * TILE_BYTES;
            let tile = &chr[off..off + TILE_BYTES];
            let (tx, ty) = ((t % COLS) * 8, (t / COLS) * 8);
            for y in 0..8 {
                let (p0, p1) = (tile[y], tile[y + 8]);
                for x in 0..8 {
                    let bit = 7 - x;
                    let v = ((p0 >> bit) & 1) | (((p1 >> bit) & 1) << 1);
                    px[(ty + y) * SHEET_W + (tx + x)] = v;
                }
            }
        }
        write_indexed_png(&outdir.join(format!("bank-{s}.png")), SHEET_W, SHEET_H, &px)?;
    }
    Ok(())
}

/// Re-encode the `bank-N.png` sheets back into the CHR blob (byte-identical).
pub fn build(indir: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut chr = Vec::new();
    let mut s = 0;
    loop {
        let path = indir.join(format!("bank-{s}.png"));
        if !path.exists() {
            break;
        }
        let (w, h, px) = read_indexed_png(&path)?;
        let (cols, rows) = (w / 8, h / 8);
        for t in 0..(cols * rows) {
            let (tx, ty) = ((t % cols) * 8, (t / cols) * 8);
            let mut tile = [0u8; TILE_BYTES];
            for y in 0..8 {
                let (mut p0, mut p1) = (0u8, 0u8);
                for x in 0..8 {
                    let v = px[(ty + y) * w + (tx + x)];
                    let bit = 7 - x;
                    p0 |= (v & 1) << bit;
                    p1 |= ((v >> 1) & 1) << bit;
                }
                tile[y] = p0;
                tile[y + 8] = p1;
            }
            chr.extend_from_slice(&tile);
        }
        s += 1;
    }
    Ok(chr)
}

fn write_indexed_png(path: &Path, w: usize, h: usize, px: &[u8]) -> Result<(), Box<dyn Error>> {
    let file = BufWriter::new(fs::File::create(path)?);
    let mut enc = png::Encoder::new(file, w as u32, h as u32);
    enc.set_color(png::ColorType::Indexed);
    enc.set_depth(png::BitDepth::Eight);
    enc.set_palette(GREYS.to_vec());
    let mut writer = enc.write_header()?;
    writer.write_image_data(px)?;
    Ok(())
}

/// Read an 8-bit indexed PNG, returning (width, height, one-index-per-pixel).
fn read_indexed_png(path: &Path) -> Result<(usize, usize, Vec<u8>), Box<dyn Error>> {
    let decoder = png::Decoder::new(fs::File::open(path)?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    if info.color_type != png::ColorType::Indexed || info.bit_depth != png::BitDepth::Eight {
        return Err(format!(
            "{}: expected 8-bit indexed PNG, got {:?}/{:?}",
            path.display(),
            info.color_type,
            info.bit_depth
        )
        .into());
    }
    buf.truncate(info.buffer_size());
    Ok((info.width as usize, info.height as usize, buf))
}
