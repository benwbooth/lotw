use crate::rom::InesRom;
use crate::video::{self, Frame};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootFrame {
    pub frame: Frame,
    pub page: usize,
    pub page_count: usize,
}

pub fn render_boot_frame(rom: &InesRom) -> BootFrame {
    let page = 0usize;
    let page_count = video::chr_page_count(rom.chr_rom());
    let frame = video::render_chr_page(rom.chr_rom(), page);

    BootFrame {
        frame,
        page,
        page_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + 0x4000 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 1;
        bytes[5] = 1;
        bytes[6] = 0x40;
        bytes[16 + 0x4000] = 0xff;
        bytes
    }

    #[test]
    fn renders_initial_runtime_frame_at_nes_resolution() {
        let rom = InesRom::parse(&ines_fixture()).unwrap();

        let boot = render_boot_frame(&rom);

        assert_eq!(boot.frame.width, video::SCREEN_WIDTH);
        assert_eq!(boot.frame.height, video::SCREEN_HEIGHT);
        assert_eq!(boot.page, 0);
        assert_eq!(boot.page_count, 1);
        assert_eq!(
            boot.frame.rgb.len(),
            video::SCREEN_WIDTH * video::SCREEN_HEIGHT * 3
        );
    }
}
