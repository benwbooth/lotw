use crate::chr;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

const TILE_SIDE: usize = 8;
const BACKGROUND: [u8; 3] = [0x11, 0x18, 0x27];
const SHADES: [[u8; 3]; 4] = [
    [12, 18, 28],
    [76, 98, 116],
    [155, 172, 186],
    [242, 245, 248],
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub rgb: Vec<u8>,
}

pub fn chr_page_count(chr_rom: &[u8]) -> usize {
    let tiles_per_page = (SCREEN_WIDTH / TILE_SIDE) * (SCREEN_HEIGHT / TILE_SIDE);
    let tile_count = chr_rom.len() / 16;
    if tile_count == 0 {
        1
    } else {
        tile_count.div_ceil(tiles_per_page)
    }
}

pub fn render_chr_page(chr_rom: &[u8], page: usize) -> Frame {
    let tiles_x = SCREEN_WIDTH / TILE_SIDE;
    let tiles_y = SCREEN_HEIGHT / TILE_SIDE;
    let tiles_per_page = tiles_x * tiles_y;
    let tile_count = chr_rom.len() / 16;
    let base_tile = page * tiles_per_page;
    let mut rgb = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3];

    for pixel in rgb.chunks_exact_mut(3) {
        pixel.copy_from_slice(&BACKGROUND);
    }

    for slot in 0..tiles_per_page {
        let tile = base_tile + slot;
        if tile >= tile_count {
            continue;
        }

        let tile_x = (slot % tiles_x) * TILE_SIDE;
        let tile_y = (slot / tiles_x) * TILE_SIDE;
        let pixels =
            chr::decode_tile(chr_rom, tile).expect("tile index checked against CHR length");

        for y in 0..TILE_SIDE {
            for x in 0..TILE_SIDE {
                let shade = SHADES[pixels[y][x] as usize];
                let dst = ((tile_y + y) * SCREEN_WIDTH + tile_x + x) * 3;
                rgb[dst..dst + 3].copy_from_slice(&shade);
            }
        }
    }

    Frame {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        rgb,
    }
}

pub fn frame_ppm(frame: &Frame) -> Vec<u8> {
    let mut ppm = format!("P6\n{} {}\n255\n", frame.width, frame.height).into_bytes();
    ppm.extend_from_slice(&frame.rgb);
    ppm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_page_count() {
        assert_eq!(chr_page_count(&[]), 1);
        assert_eq!(chr_page_count(&vec![0u8; 16 * 960]), 1);
        assert_eq!(chr_page_count(&vec![0u8; 16 * 961]), 2);
    }

    #[test]
    fn renders_chr_page_at_nes_resolution() {
        let mut chr_rom = vec![0u8; 16];
        chr_rom[0] = 0b1000_0000;
        let frame = render_chr_page(&chr_rom, 0);
        let ppm = frame_ppm(&frame);

        assert_eq!(frame.width, 256);
        assert_eq!(frame.height, 240);
        assert_eq!(frame.rgb.len(), 256 * 240 * 3);
        assert_eq!(&frame.rgb[0..3], &[76, 98, 116]);
        assert_eq!(&frame.rgb[3..6], &[12, 18, 28]);
        assert!(ppm.starts_with(b"P6\n256 240\n255\n"));
    }
}
