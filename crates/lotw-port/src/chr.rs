const TILE_BYTES: usize = 16;
const TILE_SIDE: usize = 8;
const PREVIEW_TILES_PER_ROW: usize = 64;
const SHADES: [[u8; 3]; 4] = [
    [12, 18, 28],
    [76, 98, 116],
    [155, 172, 186],
    [242, 245, 248],
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChrError {
    NoChrRom,
    IncompleteTile { len: usize },
}

impl std::fmt::Display for ChrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoChrRom => write!(f, "ROM has no CHR ROM to preview"),
            Self::IncompleteTile { len } => write!(
                f,
                "CHR ROM size must be a multiple of 16 bytes, got {len} bytes"
            ),
        }
    }
}

impl std::error::Error for ChrError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChrPreview {
    pub width: usize,
    pub height: usize,
    pub tile_count: usize,
    pub rgb: Vec<u8>,
}

pub fn decode_tile(chr: &[u8], tile_index: usize) -> Option<[[u8; TILE_SIDE]; TILE_SIDE]> {
    let start = tile_index.checked_mul(TILE_BYTES)?;
    let tile = chr.get(start..start + TILE_BYTES)?;
    let mut pixels = [[0u8; TILE_SIDE]; TILE_SIDE];

    for y in 0..TILE_SIDE {
        let lo = tile[y];
        let hi = tile[y + 8];
        for (x, pixel) in pixels[y].iter_mut().enumerate() {
            let bit = 7 - x;
            *pixel = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);
        }
    }

    Some(pixels)
}

pub fn preview(chr: &[u8]) -> Result<ChrPreview, ChrError> {
    if chr.is_empty() {
        return Err(ChrError::NoChrRom);
    }
    if !chr.len().is_multiple_of(TILE_BYTES) {
        return Err(ChrError::IncompleteTile { len: chr.len() });
    }

    let tile_count = chr.len() / TILE_BYTES;
    let rows = tile_count.div_ceil(PREVIEW_TILES_PER_ROW);
    let width = PREVIEW_TILES_PER_ROW * TILE_SIDE;
    let height = rows * TILE_SIDE;
    let mut rgb = vec![0u8; width * height * 3];

    for tile in 0..tile_count {
        let tile_x = (tile % PREVIEW_TILES_PER_ROW) * TILE_SIDE;
        let tile_y = (tile / PREVIEW_TILES_PER_ROW) * TILE_SIDE;
        let pixels = decode_tile(chr, tile).expect("tile index derived from CHR length");

        for y in 0..TILE_SIDE {
            for x in 0..TILE_SIDE {
                let shade = SHADES[pixels[y][x] as usize];
                let dst = ((tile_y + y) * width + tile_x + x) * 3;
                rgb[dst..dst + 3].copy_from_slice(&shade);
            }
        }
    }

    Ok(ChrPreview {
        width,
        height,
        tile_count,
        rgb,
    })
}

pub fn preview_ppm(chr: &[u8]) -> Result<(ChrPreview, Vec<u8>), ChrError> {
    let preview = preview(chr)?;
    let mut ppm = format!("P6\n{} {}\n255\n", preview.width, preview.height).into_bytes();
    ppm.extend_from_slice(&preview.rgb);
    Ok((preview, ppm))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_2bpp_tile_pixels() {
        let mut chr = [0u8; TILE_BYTES];
        chr[0] = 0b1000_0001;
        chr[8] = 0b0100_0001;

        let pixels = decode_tile(&chr, 0).unwrap();

        assert_eq!(pixels[0][0], 1);
        assert_eq!(pixels[0][1], 2);
        assert_eq!(pixels[0][7], 3);
        assert_eq!(pixels[1][0], 0);
    }

    #[test]
    fn writes_ppm_preview() {
        let chr = vec![0u8; TILE_BYTES * 2];

        let (preview, ppm) = preview_ppm(&chr).unwrap();

        assert_eq!(preview.tile_count, 2);
        assert_eq!(preview.width, 512);
        assert_eq!(preview.height, 8);
        assert!(ppm.starts_with(b"P6\n512 8\n255\n"));
        assert_eq!(ppm.len(), b"P6\n512 8\n255\n".len() + 512 * 8 * 3);
    }
}
