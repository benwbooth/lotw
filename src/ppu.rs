use std::{fs::File, io::Write, path::Path};

use crate::engine::{PPU_H, PPU_W};

const LOTW_PALETTE: [[u8; 3]; 64] = [
    [84, 84, 84],
    [0, 30, 116],
    [8, 16, 144],
    [48, 0, 136],
    [68, 0, 100],
    [92, 0, 48],
    [84, 4, 0],
    [60, 24, 0],
    [32, 42, 0],
    [8, 58, 0],
    [0, 64, 0],
    [0, 60, 0],
    [0, 50, 60],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [152, 150, 152],
    [8, 76, 196],
    [48, 50, 236],
    [92, 30, 228],
    [136, 20, 176],
    [160, 20, 100],
    [152, 34, 32],
    [120, 60, 0],
    [84, 90, 0],
    [40, 114, 0],
    [8, 124, 0],
    [0, 118, 40],
    [0, 102, 120],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [236, 238, 236],
    [76, 154, 236],
    [120, 124, 236],
    [176, 98, 236],
    [228, 84, 236],
    [236, 88, 180],
    [236, 106, 100],
    [212, 136, 32],
    [160, 170, 0],
    [116, 196, 0],
    [76, 208, 32],
    [56, 204, 108],
    [56, 180, 204],
    [60, 60, 60],
    [0, 0, 0],
    [0, 0, 0],
    [236, 238, 236],
    [168, 204, 236],
    [188, 188, 236],
    [212, 178, 236],
    [236, 174, 236],
    [236, 174, 212],
    [236, 180, 176],
    [228, 196, 144],
    [204, 210, 120],
    [180, 222, 120],
    [168, 226, 144],
    [152, 226, 180],
    [160, 214, 228],
    [160, 162, 160],
    [0, 0, 0],
    [0, 0, 0],
];

pub struct Ppu {
    pub vram: [u8; 2048],
    pub pal: [u8; 32],
    pub oam: [u8; 256],
    pub ctrl: u8,
    pub mask: u8,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub status: u8,
    pub openbus: u8,
    pub oamaddr: u8,
    pub vaddr: u16,
    pub wtoggle: u8,
    pub readbuf: u8,
    pub chr: [u8; 65536],
    pub chr_len: usize,
    pub chr_win: [i32; 8],
    pub mmc3_sel: u8,
    pub mmc3_bank: [u8; 8],
    pub mirror: i32,
    pub prg: [u8; 262144],
    pub prg_len: usize,
    pub buttons: u8,
    pub ctrl_latch: u8,
    pub strobe: u8,
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    pub fn new() -> Self {
        let mut ppu = Self {
            vram: [0; 2048],
            pal: [0; 32],
            oam: [0; 256],
            ctrl: 0,
            mask: 0,
            scroll_x: 0,
            scroll_y: 0,
            status: 0,
            openbus: 0,
            oamaddr: 0,
            vaddr: 0,
            wtoggle: 0,
            readbuf: 0,
            chr: [0; 65536],
            chr_len: 0,
            chr_win: [0; 8],
            mmc3_sel: 0,
            mmc3_bank: [0; 8],
            mirror: 0,
            prg: [0; 262144],
            prg_len: 0,
            buttons: 0,
            ctrl_latch: 0,
            strobe: 0,
        };
        ppu.reset();
        ppu
    }

    pub fn reset(&mut self) {
        self.vram = [0; 2048];
        self.pal = [0; 32];
        self.oam = [0; 256];
        self.ctrl = 0;
        self.mask = 0;
        self.scroll_x = 0;
        self.scroll_y = 0;
        self.status = 0;
        self.openbus = 0;
        self.oamaddr = 0;
        self.vaddr = 0;
        self.wtoggle = 0;
        self.readbuf = 0;
        self.mmc3_sel = 0;
        self.mirror = 0;
        for i in 0..8 {
            self.mmc3_bank[i] = i as u8;
        }
        self.recompute_chr();
    }

    pub fn load_chr(&mut self, chr: &[u8]) {
        let len = chr.len().min(self.chr.len());
        self.chr[..len].copy_from_slice(&chr[..len]);
        self.chr_len = len;
    }

    pub fn load_prg(&mut self, prg: &[u8]) {
        let len = prg.len().min(self.prg.len());
        self.prg[..len].copy_from_slice(&prg[..len]);
        self.prg_len = len;
    }

    pub fn recompute_chr(&mut self) {
        let inv = (self.mmc3_sel & 128) != 0;
        let two = [
            self.mmc3_bank[0] & 254,
            (self.mmc3_bank[0] & 254) | 1,
            self.mmc3_bank[1] & 254,
            (self.mmc3_bank[1] & 254) | 1,
        ];
        let one = [
            self.mmc3_bank[2],
            self.mmc3_bank[3],
            self.mmc3_bank[4],
            self.mmc3_bank[5],
        ];
        if !inv {
            self.chr_win = [
                two[0] as i32,
                two[1] as i32,
                two[2] as i32,
                two[3] as i32,
                one[0] as i32,
                one[1] as i32,
                one[2] as i32,
                one[3] as i32,
            ];
        } else {
            self.chr_win = [
                one[0] as i32,
                one[1] as i32,
                one[2] as i32,
                one[3] as i32,
                two[0] as i32,
                two[1] as i32,
                two[2] as i32,
                two[3] as i32,
            ];
        }
    }

    fn chr_at(&self, a: i32) -> u8 {
        let a = (a as usize) & 8191;
        let off = self.chr_win[a >> 10] as usize * 1024 + (a & 1023);
        if off < self.chr.len() {
            self.chr[off]
        } else {
            0
        }
    }

    fn nt_offset(&self, tx: i32, ty: i32) -> usize {
        let ntx = (tx >> 5) & 1;
        let nty = (ty >> 5) & 1;
        let phys = if self.mirror == 0 { nty } else { ntx };
        let cx = tx & 31;
        let cy = ty & 31;
        (if phys != 0 { 1024 } else { 0 }) + (cy as usize * 32 + cx as usize)
    }

    fn attr_bits(&self, tx: i32, ty: i32) -> i32 {
        let ntx = (tx >> 5) & 1;
        let nty = (ty >> 5) & 1;
        let phys = if self.mirror == 0 { nty } else { ntx };
        let cx = tx & 31;
        let cy = ty & 31;
        let base = (if phys != 0 { 1024 } else { 0 }) + 960;
        let ab = self.vram[base + ((cy >> 2) * 8 + (cx >> 2)) as usize];
        let quad = (if (cy & 2) != 0 { 2 } else { 0 }) + if (cx & 2) != 0 { 1 } else { 0 };
        ((ab >> (quad * 2)) & 3) as i32
    }

    pub fn nt_addr_offset(&self, addr: u16) -> usize {
        let a = (addr.wrapping_sub(8192) & 4095) as usize;
        let nt = (a >> 10) & 3;
        let off = a & 1023;
        let phys = if self.mirror == 0 { nt >> 1 } else { nt & 1 };
        phys * 1024 + off
    }

    fn put(out: &mut [u8], x: i32, y: i32, palidx: u8) {
        let c = LOTW_PALETTE[(palidx & 63) as usize];
        let p = (y as usize * PPU_W + x as usize) * 3;
        out[p] = c[0];
        out[p + 1] = c[1];
        out[p + 2] = c[2];
    }

    fn bg_pixel(&self, wx: i32, wy: i32, bg_pt: i32) -> u8 {
        let wx = wx & 511;
        let mut wy = wy % 480;
        if wy < 0 {
            wy += 480;
        }
        let ntx = (wx >> 8) & 1;
        let nty = if wy >= 240 { 1 } else { 0 };
        let lx = wx & 255;
        let ly = if wy >= 240 { wy - 240 } else { wy };
        let phys = if self.mirror == 0 { nty } else { ntx };
        let nt = if phys != 0 { 1024 } else { 0 };
        let cx = lx >> 3;
        let cy = ly >> 3;
        let fx = lx & 7;
        let fy = ly & 7;
        let tile = self.vram[nt + (cy * 32 + cx) as usize] as i32;
        let ab = self.vram[nt + 960 + ((cy >> 2) * 8 + (cx >> 2)) as usize] as i32;
        let pal = (ab
            >> (((if (cy & 2) != 0 { 2 } else { 0 }) + if (cx & 2) != 0 { 1 } else { 0 }) * 2))
            & 3;
        let a = bg_pt + tile * 16 + fy;
        let v = ((self.chr_at(a) >> (7 - fx)) & 1) | (((self.chr_at(a + 8) >> (7 - fx)) & 1) << 1);
        if v != 0 {
            self.pal[(pal * 4 + v as i32) as usize]
        } else {
            self.pal[0]
        }
    }

    pub fn render(&mut self, memory: &[u8], out: &mut [u8]) {
        let bg_pt = if (self.ctrl & 16) != 0 { 4096 } else { 0 };
        let sp_pt = if (self.ctrl & 8) != 0 { 4096 } else { 0 };
        let tall = (self.ctrl & 32) != 0;
        // Status-bar sprite-0 split flag (GameState::STATUSBAR_SPLIT_FLAG, $29).
        let split = memory[crate::state::GameState::STATUSBAR_SPLIT_FLAG as usize] != 0;
        let mut split_y = if split { self.oam[0] as i32 + 1 } else { 0 };
        split_y = split_y.clamp(0, PPU_H as i32);

        if (self.mask & 8) != 0 {
            let bx = if (self.ctrl & 1) != 0 { 256 } else { 0 };
            let by = if (self.ctrl & 2) != 0 { 240 } else { 0 };
            for sy in split_y..PPU_H as i32 {
                let wy = by + self.scroll_y as i32 + (sy - split_y) + if split { 6 } else { 0 };
                for sx in 0..PPU_W as i32 {
                    Self::put(
                        out,
                        sx,
                        sy,
                        self.bg_pixel(bx + self.scroll_x as i32 + sx, wy, bg_pt),
                    );
                }
            }
        } else {
            for sy in split_y..PPU_H as i32 {
                for sx in 0..PPU_W as i32 {
                    Self::put(out, sx, sy, self.pal[0]);
                }
            }
        }

        if split && (self.mask & 8) != 0 {
            let b1 = self.mmc3_bank[1];
            let b4 = self.mmc3_bank[4];
            let b5 = self.mmc3_bank[5];
            self.mmc3_bank[1] = 22;
            self.mmc3_bank[4] = 62;
            self.mmc3_bank[5] = 63;
            self.recompute_chr();
            for sy in 0..split_y {
                for sx in 0..PPU_W as i32 {
                    Self::put(out, sx, sy, self.bg_pixel(sx, 196 + sy, bg_pt));
                }
            }
            self.mmc3_bank[1] = b1;
            self.mmc3_bank[4] = b4;
            self.mmc3_bank[5] = b5;
            self.recompute_chr();
        }

        if (self.mask & 16) != 0 {
            for i in (0..64).rev() {
                let o = i * 4;
                let y = self.oam[o] as i32 + 1;
                let at = self.oam[o + 2];
                let x = self.oam[o + 3] as i32;
                let pal = 16 + ((at & 3) as i32) * 4;
                let hflip = (at & 64) != 0;
                let vflip = (at & 128) != 0;
                let h = if tall { 16 } else { 8 };
                if y >= PPU_H as i32 || y + h <= 0 {
                    continue;
                }
                for row in 0..h {
                    let py = y + row;
                    if py < 0 || py >= PPU_H as i32 {
                        continue;
                    }
                    let sr = if vflip { h - 1 - row } else { row };
                    let a = if tall {
                        let base = if (self.oam[o + 1] & 1) != 0 { 4096 } else { 0 }
                            + ((self.oam[o + 1] & 254) as i32) * 16;
                        base + if sr < 8 { sr } else { 16 + (sr - 8) }
                    } else {
                        sp_pt + self.oam[o + 1] as i32 * 16 + sr
                    };
                    let p0 = self.chr_at(a);
                    let p1 = self.chr_at(a + 8);
                    for col in 0..8 {
                        let px = x + col;
                        if px < 0 || px >= PPU_W as i32 {
                            continue;
                        }
                        let sc = if hflip { col } else { 7 - col };
                        let v = ((p0 >> sc) & 1) | (((p1 >> sc) & 1) << 1);
                        if v == 0 {
                            continue;
                        }
                        Self::put(out, px, py, self.pal[(pal + v as i32) as usize]);
                    }
                }
            }
        }
    }

    pub fn render_statusbar(&mut self, out: &mut [u8], rows: i32) {
        let b1 = self.mmc3_bank[1];
        let b4 = self.mmc3_bank[4];
        let b5 = self.mmc3_bank[5];
        self.mmc3_bank[1] = 22;
        self.mmc3_bank[4] = 62;
        self.mmc3_bank[5] = 63;
        self.recompute_chr();
        let bg_pt = if (self.ctrl & 16) != 0 { 4096 } else { 0 };
        for sy in 0..rows.min(PPU_H as i32) {
            let wy = sy + 196;
            let ty = wy >> 3;
            let fy = wy & 7;
            for sx in 0..PPU_W as i32 {
                let tx = sx >> 3;
                let fx = sx & 7;
                let tile = self.vram[self.nt_offset(tx, ty)] as i32;
                let a = bg_pt + tile * 16 + fy;
                let bit = 7 - fx;
                let v = ((self.chr_at(a) >> bit) & 1) | (((self.chr_at(a + 8) >> bit) & 1) << 1);
                let idx = if v != 0 {
                    self.pal[(self.attr_bits(tx, ty) * 4 + v as i32) as usize]
                } else {
                    self.pal[0]
                };
                Self::put(out, sx, sy, idx);
            }
        }
        self.mmc3_bank[1] = b1;
        self.mmc3_bank[4] = b4;
        self.mmc3_bank[5] = b5;
        self.recompute_chr();
    }

    pub fn debug_tilesheet(&self, which: i32, out: &mut [u8]) {
        const GRAY: [u8; 4] = [0, 12, 16, 48];
        let base = if which != 0 { 4096 } else { 0 };
        for t in 0..256 {
            let ox = (t & 15) * 8;
            let oy = (t >> 4) * 8;
            for row in 0..8 {
                let p0 = self.chr_at(base + t * 16 + row);
                let p1 = self.chr_at(base + t * 16 + 8 + row);
                for col in 0..8 {
                    let v = ((p0 >> (7 - col)) & 1) | (((p1 >> (7 - col)) & 1) << 1);
                    let c = LOTW_PALETTE[GRAY[v as usize] as usize];
                    let p = ((oy + row) as usize * 128 + (ox + col) as usize) * 3;
                    out[p] = c[0];
                    out[p + 1] = c[1];
                    out[p + 2] = c[2];
                }
            }
        }
    }

    pub fn set_vblank(&mut self, on: bool) {
        if on {
            self.status |= 128;
        } else {
            self.status &= !128;
        }
    }

    pub fn set_sprite0(&mut self, on: bool) {
        if on {
            self.status |= 64;
        } else {
            self.status &= !64;
        }
    }

    pub fn set_buttons(&mut self, buttons: i32) {
        self.buttons = buttons as u8;
    }

    pub fn eval_sprite_overflow(&mut self) {
        let h = if (self.ctrl & 32) != 0 { 16 } else { 8 };
        let mut perline = [0u8; 240];
        for s in 0..64 {
            let y = self.oam[s * 4] as i32;
            if y >= 239 {
                continue;
            }
            let top = y + 1;
            let bot = (y + 1 + h).min(240);
            for sl in top..bot {
                perline[sl as usize] = perline[sl as usize].wrapping_add(1);
                if perline[sl as usize] > 8 {
                    self.status |= 32;
                    return;
                }
            }
        }
        self.status &= !32;
    }
}

pub fn ppm_write(path: impl AsRef<Path>, rgb: &[u8], w: usize, h: usize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    write!(f, "P6\n{} {}\n255\n", w, h)?;
    f.write_all(&rgb[..w * h * 3])?;
    Ok(())
}
