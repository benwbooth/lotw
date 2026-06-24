//! Software NES PPU (Picture Processing Unit) shim.
//!
//! This module is a from-scratch software re-implementation of the subset of
//! NES PPU behavior that *Legacy of the Wizard* actually relies on. Rather than
//! emulating the PPU cycle-by-cycle, it renders whole frames on demand from the
//! current PPU register/memory state. It covers:
//!
//! - Background rendering from the nametables + attribute tables, with
//!   horizontal/vertical scrolling and a two-screen mirrored nametable layout.
//! - Sprite rendering from OAM (8x8 or 8x16 sprites) with H/V flip and back-to
//!   -front priority.
//! - Palette translation through the fixed 64-entry NES master palette.
//! - MMC3-style CHR bank switching (the cartridge mapper) feeding tile fetches.
//! - The status-bar scanline split: the top of the screen shows a fixed HUD
//!   drawn from a separate CHR/nametable region, the rest scrolls normally.

use std::{fs::File, io::Write, path::Path};

use crate::engine::{PPU_H, PPU_W};

/// The fixed 64-entry NES master palette as RGB triples.
///
/// Each NES palette index (0..63) maps to one hardware color. The PPU's own
/// palette RAM stores indices *into* this table; [`Ppu::put`] performs the final
/// index-to-RGB lookup here. This is a specific NTSC palette variant chosen to
/// match the reference rendering of the game.
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

/// Number of color entries in the NES master palette.
const PALETTE_COUNT: usize = 64;
/// Mask applied to a palette index to wrap it into `0..PALETTE_COUNT`.
const PALETTE_MASK: u8 = 63;

/// Bytes in the on-board PPU nametable VRAM (two 1KB nametables).
const VRAM_SIZE: usize = 2048;
/// Bytes in PPU palette RAM (background + sprite, 32 entries).
const PAL_SIZE: usize = 32;
/// Bytes of Object Attribute Memory: 64 sprites x 4 bytes each.
const OAM_SIZE: usize = 256;
/// Maximum CHR (pattern/tile) ROM/RAM the shim can hold.
const CHR_SIZE: usize = 65536;
/// Maximum PRG (program) ROM the shim can hold.
const PRG_SIZE: usize = 262144;

/// Size of one CHR bank window in bytes (1KB), as switched by MMC3.
const CHR_BANK_SIZE: usize = 1024;
/// Number of 1KB CHR windows mapped into the 8KB pattern space.
const CHR_WIN_COUNT: usize = 8;
/// Mask for an address within the 8KB CHR address space (0x0000..0x1FFF).
const CHR_ADDR_MASK: usize = 8191;
/// Mask for the offset within a single 1KB CHR window.
const CHR_BANK_MASK: usize = 1023;
/// Right-shift to convert a CHR address into its 1KB window index.
const CHR_WIN_SHIFT: usize = 10;

/// Number of bytes per pattern-table tile plane row pair: 8 low-plane bytes
/// followed by 8 high-plane bytes = 16 bytes per 8x8 tile.
const TILE_BYTES: i32 = 16;
/// Pixel dimension of an NES tile (8x8).
const TILE_SIZE: i32 = 8;
/// Byte offset from a tile's low bit-plane to its high bit-plane (8 rows).
const PLANE_HI_OFF: i32 = 8;

/// One nametable's tile grid is 32 tiles wide.
const NT_WIDTH: i32 = 32;
/// Byte size of one nametable (tile map + attribute table) = 1KB.
const NT_BYTES: usize = 1024;
/// Byte offset within a nametable where the attribute table begins
/// (32x30 = 960 tile bytes precede it).
const ATTR_OFFSET: usize = 960;
/// Attribute table is 8 entries wide; each entry covers a 4x4 tile (32x32 px)
/// region split into four 2x2-tile quadrants.
const ATTR_WIDTH: i32 = 8;

/// PPUCTRL bit: base nametable X select (adds one nametable horizontally).
const CTRL_NT_X: u8 = 1;
/// PPUCTRL bit: base nametable Y select (adds one nametable vertically).
const CTRL_NT_Y: u8 = 2;
/// PPUCTRL bit: sprite pattern table select (0x1000 when set, 8x8 mode only).
const CTRL_SPR_PT: u8 = 8;
/// PPUCTRL bit: background pattern table select (0x1000 when set).
const CTRL_BG_PT: u8 = 16;
/// PPUCTRL bit: sprite size; set = 8x16 tall sprites, clear = 8x8.
const CTRL_TALL: u8 = 32;

/// PPUMASK bit: enable background rendering.
const MASK_SHOW_BG: u8 = 8;
/// PPUMASK bit: enable sprite rendering.
const MASK_SHOW_SPR: u8 = 16;

/// PPUSTATUS bit: vblank flag.
const STATUS_VBLANK: u8 = 128;
/// PPUSTATUS bit: sprite-0 hit flag.
const STATUS_SPR0: u8 = 64;
/// PPUSTATUS bit: sprite overflow flag.
const STATUS_OVERFLOW: u8 = 32;

/// Address (offset by CPU $2000) at which the PPU nametable space begins.
const NT_BASE_ADDR: u16 = 8192;
/// Mask for an address within the 4KB nametable mirror region.
const NT_ADDR_MASK: usize = 4095;

/// Pattern-table select offset: 0x1000 (4096) selects the upper half.
const PT_HIGH: i32 = 4096;

/// Number of hardware sprites in OAM.
const SPRITE_COUNT: usize = 64;
/// Bytes per OAM sprite entry (Y, tile, attributes, X).
const OAM_ENTRY: usize = 4;
/// Sprite attribute byte: palette select (low 2 bits).
const SPR_ATTR_PAL: u8 = 3;
/// Sprite attribute byte: horizontal flip.
const SPR_ATTR_HFLIP: u8 = 64;
/// Sprite attribute byte: vertical flip.
const SPR_ATTR_VFLIP: u8 = 128;
/// Palette RAM base index for sprite palettes (entries 16..31).
const SPR_PAL_BASE: i32 = 16;
/// Colors per palette (one transparent + three opaque).
const PAL_ENTRIES: i32 = 4;

/// Width in pixels of the debug tilesheet (16 tiles * 8 px).
const TILESHEET_W: usize = 128;
/// Tiles per row in the debug tilesheet view.
const TILESHEET_COLS: i32 = 16;
/// Total tiles drawn in a debug tilesheet (one full pattern table).
const TILESHEET_TILES: i32 = 256;

/// Software model of the NES PPU plus the cartridge mapper/controller state
/// needed to render a frame. All fields mirror real PPU registers/memory so the
/// CPU-side engine can poke them exactly as the original code does.
pub struct Ppu {
    /// On-board nametable VRAM: two physical 1KB nametables.
    pub vram: [u8; VRAM_SIZE],
    /// Palette RAM: 16 background + 16 sprite indices into [`LOTW_PALETTE`].
    pub pal: [u8; PAL_SIZE],
    /// Object Attribute Memory: 64 sprites x 4 bytes.
    pub oam: [u8; OAM_SIZE],
    /// PPUCTRL ($2000): pattern table selects, sprite size, NT base, etc.
    pub ctrl: u8,
    /// PPUMASK ($2001): background/sprite enable bits.
    pub mask: u8,
    /// Fine/coarse X scroll (first PPUSCROLL write).
    pub scroll_x: u8,
    /// Fine/coarse Y scroll (second PPUSCROLL write).
    pub scroll_y: u8,
    /// PPUSTATUS ($2002): vblank/sprite-0/overflow flags.
    pub status: u8,
    /// Last value seen on the PPU data bus (open-bus reads).
    pub openbus: u8,
    /// OAMADDR ($2003): current OAM write pointer.
    pub oamaddr: u8,
    /// PPUADDR ($2006): current VRAM access address.
    pub vaddr: u16,
    /// PPUSCROLL/PPUADDR write toggle (first vs. second write).
    pub wtoggle: u8,
    /// PPUDATA read buffer (reads are delayed by one access).
    pub readbuf: u8,
    /// CHR pattern data (tile bitmaps), backing all tile fetches.
    pub chr: [u8; CHR_SIZE],
    /// Number of valid bytes loaded into [`Ppu::chr`].
    pub chr_len: usize,
    /// Resolved CHR bank window table: maps each of the 8 1KB windows of the
    /// pattern address space to a physical bank number.
    pub chr_win: [i32; CHR_WIN_COUNT],
    /// MMC3 bank-select register ($8000): controls bank layout/inversion.
    pub mmc3_sel: u8,
    /// MMC3 bank registers ($8001 targets): R0..R7 bank numbers.
    pub mmc3_bank: [u8; CHR_WIN_COUNT],
    /// Nametable mirroring mode (0 = vertical, else horizontal).
    pub mirror: i32,
    /// PRG program ROM.
    pub prg: [u8; PRG_SIZE],
    /// Number of valid bytes loaded into [`Ppu::prg`].
    pub prg_len: usize,
    /// Currently held controller buttons (bitmask).
    pub buttons: u8,
    /// Controller shift-register latch for serial button reads.
    pub ctrl_latch: u8,
    /// Controller strobe bit ($4016): reloads the latch while high.
    pub strobe: u8,
}

impl Default for Ppu {
    /// Defaults to a freshly reset PPU via [`Ppu::new`].
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    /// Construct a zero-initialized PPU and run [`Ppu::reset`] to bring all
    /// registers/banks to their power-on state.
    pub fn new() -> Self {
        // Zero every register, VRAM/OAM/palette buffer, and ROM/RAM region.
        let mut ppu = Self {
            vram: [0; VRAM_SIZE],
            pal: [0; PAL_SIZE],
            oam: [0; OAM_SIZE],
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
            chr: [0; CHR_SIZE],
            chr_len: 0,
            chr_win: [0; CHR_WIN_COUNT],
            mmc3_sel: 0,
            mmc3_bank: [0; CHR_WIN_COUNT],
            mirror: 0,
            prg: [0; PRG_SIZE],
            prg_len: 0,
            buttons: 0,
            ctrl_latch: 0,
            strobe: 0,
        };
        // Apply the power-on reset (clears state, seeds banks, resolves CHR).
        ppu.reset();
        ppu
    }

    /// Reset all volatile PPU/mapper state to power-on values. Does not clear the
    /// loaded CHR/PRG ROM contents (those are loaded once and persist).
    pub fn reset(&mut self) {
        // Clear the writable PPU buffers.
        self.vram = [0; VRAM_SIZE];
        self.pal = [0; PAL_SIZE];
        self.oam = [0; OAM_SIZE];
        // Clear the PPU register file.
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
        // Clear mapper selection state and mirroring.
        self.mmc3_sel = 0;
        self.mirror = 0;
        // Seed the MMC3 bank registers with the identity mapping R[i] = i.
        for i in 0..CHR_WIN_COUNT {
            self.mmc3_bank[i] = i as u8;
        }
        // Resolve the CHR window table from the seeded bank registers.
        self.recompute_chr();
    }

    /// Copy CHR (pattern/tile) data into the PPU, truncating to the buffer size.
    pub fn load_chr(&mut self, chr: &[u8]) {
        // Never write past the fixed CHR buffer.
        let len = chr.len().min(self.chr.len());
        self.chr[..len].copy_from_slice(&chr[..len]);
        self.chr_len = len;
    }

    /// Copy PRG (program) ROM into the PPU shim, truncating to the buffer size.
    pub fn load_prg(&mut self, prg: &[u8]) {
        // Never write past the fixed PRG buffer.
        let len = prg.len().min(self.prg.len());
        self.prg[..len].copy_from_slice(&prg[..len]);
        self.prg_len = len;
    }

    /// Rebuild [`Ppu::chr_win`] (the 8x1KB window->bank map) from the current
    /// MMC3 bank registers and the bank-inversion bit.
    pub fn recompute_chr(&mut self) {
        // MMC3 bit 7 of the bank-select register swaps the 2KB and 1KB regions.
        let inv = (self.mmc3_sel & 128) != 0;
        // R0/R1 select 2KB banks; expand each into two consecutive 1KB windows
        // by forcing the low bit (even base, then base|1).
        let two = [
            self.mmc3_bank[0] & 254,
            (self.mmc3_bank[0] & 254) | 1,
            self.mmc3_bank[1] & 254,
            (self.mmc3_bank[1] & 254) | 1,
        ];
        // R2..R5 each select a single 1KB bank directly.
        let one = [
            self.mmc3_bank[2],
            self.mmc3_bank[3],
            self.mmc3_bank[4],
            self.mmc3_bank[5],
        ];
        if !inv {
            // Normal layout: two 2KB banks at $0000-$0FFF, four 1KB at $1000-$1FFF.
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
            // Inverted layout: the 1KB banks move to $0000-$0FFF and vice versa.
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

    /// Fetch one CHR byte at pattern address `a`, routed through the active bank
    /// window table. Out-of-range physical offsets read as 0.
    fn chr_at(&self, a: i32) -> u8 {
        // Wrap into the 8KB pattern address space.
        let a = (a as usize) & CHR_ADDR_MASK;
        // Translate via the 1KB window: window index = a >> 10, offset within = low 10 bits.
        let off = self.chr_win[a >> CHR_WIN_SHIFT] as usize * CHR_BANK_SIZE + (a & CHR_BANK_MASK);
        // Bounds-check against loaded CHR; absent data reads as transparent 0.
        if off < self.chr.len() {
            self.chr[off]
        } else {
            0
        }
    }

    /// Compute the VRAM byte offset of the tile-map entry at coarse tile coords
    /// `(tx, ty)`, applying the current nametable mirroring.
    fn nt_offset(&self, tx: i32, ty: i32) -> usize {
        // Logical nametable selectors: which of the 2x2 virtual nametables.
        let ntx = (tx >> 5) & 1;
        let nty = (ty >> 5) & 1;
        // Mirroring picks the physical nametable: vertical (mirror==0) mirrors
        // along Y, horizontal mirrors along X.
        let phys = if self.mirror == 0 { nty } else { ntx };
        // Coarse coordinates within the 32x32 tile grid of one nametable.
        let cx = tx & 31;
        let cy = ty & 31;
        // Physical base (0 or second 1KB nametable) plus row-major tile index.
        (if phys != 0 { NT_BYTES } else { 0 }) + (cy as usize * NT_WIDTH as usize + cx as usize)
    }

    /// Decode the 2-bit palette select for the tile at coarse coords `(tx, ty)`
    /// from the attribute table of the mirrored physical nametable.
    fn attr_bits(&self, tx: i32, ty: i32) -> i32 {
        // Resolve which physical nametable, identical to nt_offset.
        let ntx = (tx >> 5) & 1;
        let nty = (ty >> 5) & 1;
        let phys = if self.mirror == 0 { nty } else { ntx };
        let cx = tx & 31;
        let cy = ty & 31;
        // Attribute table sits at offset 960 within the chosen nametable.
        let base = (if phys != 0 { NT_BYTES } else { 0 }) + ATTR_OFFSET;
        // Each attribute byte covers a 4x4 tile block: index by (cy/4, cx/4).
        let ab = self.vram[base + ((cy >> 2) * ATTR_WIDTH + (cx >> 2)) as usize];
        // Within that block, select the 2x2-tile quadrant (bit pair) by the
        // tile's low coarse bits: bit1 = Y quadrant, bit0 = X quadrant.
        let quad = (if (cy & 2) != 0 { 2 } else { 0 }) + if (cx & 2) != 0 { 1 } else { 0 };
        // Extract the 2-bit palette select for that quadrant.
        ((ab >> (quad * 2)) & 3) as i32
    }

    /// Map a CPU-side PPU address (in the $2000-$2FFF nametable range, biased by
    /// `NT_BASE_ADDR`) to a physical VRAM offset, applying mirroring.
    pub fn nt_addr_offset(&self, addr: u16) -> usize {
        // Bias to nametable space and wrap into the 4KB mirror region.
        let a = (addr.wrapping_sub(NT_BASE_ADDR) & NT_ADDR_MASK as u16) as usize;
        // Logical nametable index (0..3) and offset within it.
        let nt = (a >> 10) & 3;
        let off = a & 1023;
        // Vertical mirroring keys on the high index bit, horizontal on the low.
        let phys = if self.mirror == 0 { nt >> 1 } else { nt & 1 };
        // Physical nametable base plus the in-table offset.
        phys * NT_BYTES + off
    }

    /// Write one pixel at `(x, y)` into the RGB output buffer, resolving the NES
    /// palette index through [`LOTW_PALETTE`].
    fn put(out: &mut [u8], x: i32, y: i32, palidx: u8) {
        // Look up the RGB triple for this palette index (wrapped to 0..63).
        let c = LOTW_PALETTE[(palidx & PALETTE_MASK) as usize];
        // Compute the byte offset for this pixel in the row-major RGB buffer.
        let p = (y as usize * PPU_W + x as usize) * 3;
        out[p] = c[0];
        out[p + 1] = c[1];
        out[p + 2] = c[2];
    }

    /// Sample a background pixel at *world* coordinates `(wx, wy)` (post-scroll,
    /// across the 512x480 four-nametable space), using background pattern table
    /// `bg_pt`. Returns the resolved palette index (palette entry 0 if the tile
    /// pixel is transparent).
    fn bg_pixel(&self, wx: i32, wy: i32, bg_pt: i32) -> u8 {
        // Wrap into the 2x2 nametable world: 512 px wide, 480 px tall.
        let wx = wx & 511;
        let mut wy = wy % 480;
        if wy < 0 {
            wy += 480;
        }
        // Determine which nametable (X/Y) and the local pixel within it.
        let ntx = (wx >> 8) & 1;
        let nty = if wy >= 240 { 1 } else { 0 };
        let lx = wx & 255;
        let ly = if wy >= 240 { wy - 240 } else { wy };
        // Apply mirroring to choose the physical nametable base.
        let phys = if self.mirror == 0 { nty } else { ntx };
        let nt = if phys != 0 { NT_BYTES } else { 0 };
        // Split local pixel into coarse tile (cx, cy) and fine pixel (fx, fy).
        let cx = lx >> 3;
        let cy = ly >> 3;
        let fx = lx & 7;
        let fy = ly & 7;
        // Fetch the tile index from the tile map.
        let tile = self.vram[nt + (cy * NT_WIDTH + cx) as usize] as i32;
        // Fetch and decode the attribute byte to get this tile's 2-bit palette.
        let ab = self.vram[nt + ATTR_OFFSET + ((cy >> 2) * ATTR_WIDTH + (cx >> 2)) as usize] as i32;
        let pal = (ab
            >> (((if (cy & 2) != 0 { 2 } else { 0 }) + if (cx & 2) != 0 { 1 } else { 0 }) * 2))
            & 3;
        // Address of the tile's pixel row in CHR (16 bytes/tile, row fy).
        let a = bg_pt + tile * TILE_BYTES + fy;
        // Combine the two bit planes into a 2-bit pixel value for column fx.
        let v = ((self.chr_at(a) >> (7 - fx)) & 1)
            | (((self.chr_at(a + PLANE_HI_OFF) >> (7 - fx)) & 1) << 1);
        // Pixel value 0 is transparent -> universal background color (pal[0]).
        if v != 0 {
            self.pal[(pal * PAL_ENTRIES + v as i32) as usize]
        } else {
            self.pal[0]
        }
    }

    /// Render a full 256x240 frame into `out` (RGB), using the CPU-side `memory`
    /// to read game flags (notably the status-bar split flag). Draws background,
    /// the HUD split region, then sprites back-to-front.
    pub fn render(&mut self, memory: &[u8], out: &mut [u8]) {
        // Resolve pattern-table bases and sprite-size mode from PPUCTRL.
        let bg_pt = if (self.ctrl & CTRL_BG_PT) != 0 {
            PT_HIGH
        } else {
            0
        };
        let sp_pt = if (self.ctrl & CTRL_SPR_PT) != 0 {
            PT_HIGH
        } else {
            0
        };
        let tall = (self.ctrl & CTRL_TALL) != 0;
        // Status-bar sprite-0 split flag (GameState::STATUSBAR_SPLIT_FLAG, $29).
        let split = memory[crate::state::GameState::STATUSBAR_SPLIT_FLAG as usize] != 0;
        // When splitting, the scrolling area starts one scanline below sprite 0's
        // Y; otherwise the whole screen scrolls (split_y = 0).
        let mut split_y = if split { self.oam[0] as i32 + 1 } else { 0 };
        split_y = split_y.clamp(0, PPU_H as i32);

        if (self.mask & MASK_SHOW_BG) != 0 {
            // Base nametable offsets from PPUCTRL: +256 px X and/or +240 px Y.
            let bx = if (self.ctrl & CTRL_NT_X) != 0 { 256 } else { 0 };
            let by = if (self.ctrl & CTRL_NT_Y) != 0 { 240 } else { 0 };
            // Draw the scrolling background for every scanline below the split.
            for sy in split_y..PPU_H as i32 {
                // World Y for this screen line: base + scroll + offset within the
                // scrolling region; +6 px nudge applied when the HUD split is active.
                let wy = by + self.scroll_y as i32 + (sy - split_y) + if split { 6 } else { 0 };
                for sx in 0..PPU_W as i32 {
                    // World X = base + scroll + screen X; sample and plot.
                    Self::put(
                        out,
                        sx,
                        sy,
                        self.bg_pixel(bx + self.scroll_x as i32 + sx, wy, bg_pt),
                    );
                }
            }
        } else {
            // Background disabled: fill the lower region with the backdrop color.
            for sy in split_y..PPU_H as i32 {
                for sx in 0..PPU_W as i32 {
                    Self::put(out, sx, sy, self.pal[0]);
                }
            }
        }

        if split && (self.mask & MASK_SHOW_BG) != 0 {
            // The HUD/status-bar region uses fixed CHR banks. Save the live MMC3
            // banks, swap in the HUD banks, and re-resolve the CHR window table.
            let b1 = self.mmc3_bank[1];
            let b4 = self.mmc3_bank[4];
            let b5 = self.mmc3_bank[5];
            self.mmc3_bank[1] = 22;
            self.mmc3_bank[4] = 62;
            self.mmc3_bank[5] = 63;
            self.recompute_chr();
            // Draw the top split rows from a fixed world Y of 196 (no scrolling).
            for sy in 0..split_y {
                for sx in 0..PPU_W as i32 {
                    Self::put(out, sx, sy, self.bg_pixel(sx, 196 + sy, bg_pt));
                }
            }
            // Restore the live banks and CHR window table.
            self.mmc3_bank[1] = b1;
            self.mmc3_bank[4] = b4;
            self.mmc3_bank[5] = b5;
            self.recompute_chr();
        }

        if (self.mask & MASK_SHOW_SPR) != 0 {
            // Draw sprites back-to-front (OAM index 63 first, 0 last) so lower
            // indices win on overlap, matching NES sprite priority.
            for i in (0..SPRITE_COUNT).rev() {
                let o = i * OAM_ENTRY;
                // OAM byte 0 is Y-1 (sprites display one scanline below stored Y).
                let y = self.oam[o] as i32 + 1;
                // OAM byte 2: attribute flags; byte 3: X position.
                let at = self.oam[o + 2];
                let x = self.oam[o + 3] as i32;
                // Sprite palette index in palette RAM (16 + 4*palette_select).
                let pal = SPR_PAL_BASE + ((at & SPR_ATTR_PAL) as i32) * PAL_ENTRIES;
                let hflip = (at & SPR_ATTR_HFLIP) != 0;
                let vflip = (at & SPR_ATTR_VFLIP) != 0;
                // Sprite height: 16 px for tall 8x16 sprites, else 8.
                let h = if tall { 16 } else { TILE_SIZE };
                // Skip sprites entirely off the top or bottom of the screen.
                if y >= PPU_H as i32 || y + h <= 0 {
                    continue;
                }
                for row in 0..h {
                    let py = y + row;
                    // Clip rows outside the visible area.
                    if py < 0 || py >= PPU_H as i32 {
                        continue;
                    }
                    // Vertical flip reverses the source row within the sprite.
                    let sr = if vflip { h - 1 - row } else { row };
                    let a = if tall {
                        // 8x16: OAM tile byte bit0 selects the pattern table; the
                        // even tile index gives the top tile, the next the bottom.
                        let base = if (self.oam[o + 1] & 1) != 0 {
                            PT_HIGH
                        } else {
                            0
                        } + ((self.oam[o + 1] & 254) as i32) * TILE_BYTES;
                        // Rows 0..7 read the top tile; 8..15 read the next tile.
                        base + if sr < 8 {
                            sr
                        } else {
                            TILE_BYTES + (sr - PLANE_HI_OFF)
                        }
                    } else {
                        // 8x8: tile index times 16 bytes plus the source row.
                        sp_pt + self.oam[o + 1] as i32 * TILE_BYTES + sr
                    };
                    // Fetch the two bit planes for this sprite row.
                    let p0 = self.chr_at(a);
                    let p1 = self.chr_at(a + PLANE_HI_OFF);
                    for col in 0..8 {
                        let px = x + col;
                        // Clip columns outside the visible area.
                        if px < 0 || px >= PPU_W as i32 {
                            continue;
                        }
                        // Horizontal flip reverses which bit is sampled.
                        let sc = if hflip { col } else { 7 - col };
                        // Combine planes into the 2-bit sprite pixel value.
                        let v = ((p0 >> sc) & 1) | (((p1 >> sc) & 1) << 1);
                        // Value 0 is transparent for sprites.
                        if v == 0 {
                            continue;
                        }
                        // Plot through the sprite palette.
                        Self::put(out, px, py, self.pal[(pal + v as i32) as usize]);
                    }
                }
            }
        }
    }

    /// Render only the fixed status-bar HUD: `rows` scanlines drawn from the HUD
    /// CHR banks and the nametable region at world Y 196, without scrolling or
    /// sprites. Used to redraw the HUD independently of the play field.
    pub fn render_statusbar(&mut self, out: &mut [u8], rows: i32) {
        // Swap in the fixed HUD CHR banks (saving the live ones to restore later).
        let b1 = self.mmc3_bank[1];
        let b4 = self.mmc3_bank[4];
        let b5 = self.mmc3_bank[5];
        self.mmc3_bank[1] = 22;
        self.mmc3_bank[4] = 62;
        self.mmc3_bank[5] = 63;
        self.recompute_chr();
        // Background pattern table base for the HUD tiles.
        let bg_pt = if (self.ctrl & CTRL_BG_PT) != 0 {
            PT_HIGH
        } else {
            0
        };
        for sy in 0..rows.min(PPU_H as i32) {
            // The HUD reads from the nametable starting at world Y 196.
            let wy = sy + 196;
            // Split world Y into coarse tile row and fine pixel row.
            let ty = wy >> 3;
            let fy = wy & 7;
            for sx in 0..PPU_W as i32 {
                // Split screen X into coarse tile col and fine pixel col.
                let tx = sx >> 3;
                let fx = sx & 7;
                // Fetch the tile index from the (mirrored) nametable.
                let tile = self.vram[self.nt_offset(tx, ty)] as i32;
                // Address of this tile's pixel row in CHR.
                let a = bg_pt + tile * TILE_BYTES + fy;
                let bit = 7 - fx;
                // Combine the two bit planes into the 2-bit pixel value.
                let v = ((self.chr_at(a) >> bit) & 1)
                    | (((self.chr_at(a + PLANE_HI_OFF) >> bit) & 1) << 1);
                // Opaque pixels use the tile's attribute palette; 0 is backdrop.
                let idx = if v != 0 {
                    self.pal[(self.attr_bits(tx, ty) * PAL_ENTRIES + v as i32) as usize]
                } else {
                    self.pal[0]
                };
                Self::put(out, sx, sy, idx);
            }
        }
        // Restore the live CHR banks and window table.
        self.mmc3_bank[1] = b1;
        self.mmc3_bank[4] = b4;
        self.mmc3_bank[5] = b5;
        self.recompute_chr();
    }

    /// Render one full 256-tile pattern table into a 128x128 RGB image for
    /// debugging. `which` selects the low (0) or high pattern table; tiles are
    /// drawn with a fixed grayscale ramp rather than game palettes.
    pub fn debug_tilesheet(&self, which: i32, out: &mut [u8]) {
        // Fixed 4-step grayscale ramp (indices into LOTW_PALETTE) for the 2-bit
        // pixel values, so tiles are legible without a real palette.
        const GRAY: [u8; 4] = [0, 12, 16, 48];
        // Select which pattern table to view.
        let base = if which != 0 { PT_HIGH } else { 0 };
        for t in 0..TILESHEET_TILES {
            // Lay tiles out 16 across; compute the top-left pixel of tile t.
            let ox = (t & 15) * TILE_SIZE;
            let oy = (t >> 4) * TILE_SIZE;
            for row in 0..8 {
                // Fetch this tile row's two bit planes.
                let p0 = self.chr_at(base + t * TILE_BYTES + row);
                let p1 = self.chr_at(base + t * TILE_BYTES + PLANE_HI_OFF + row);
                for col in 0..8 {
                    // Decode the 2-bit pixel value (MSB = leftmost column).
                    let v = ((p0 >> (7 - col)) & 1) | (((p1 >> (7 - col)) & 1) << 1);
                    // Map through the grayscale ramp to an RGB color.
                    let c = LOTW_PALETTE[GRAY[v as usize] as usize];
                    // Plot into the 128-wide debug image buffer.
                    let p = ((oy + row) as usize * TILESHEET_W + (ox + col) as usize) * 3;
                    out[p] = c[0];
                    out[p + 1] = c[1];
                    out[p + 2] = c[2];
                }
            }
        }
    }

    /// Set or clear the PPUSTATUS vblank flag (bit 7).
    pub fn set_vblank(&mut self, on: bool) {
        if on {
            self.status |= STATUS_VBLANK;
        } else {
            self.status &= !STATUS_VBLANK;
        }
    }

    /// Set or clear the PPUSTATUS sprite-0-hit flag (bit 6).
    pub fn set_sprite0(&mut self, on: bool) {
        if on {
            self.status |= STATUS_SPR0;
        } else {
            self.status &= !STATUS_SPR0;
        }
    }

    /// Record the currently pressed controller buttons (bitmask).
    pub fn set_buttons(&mut self, buttons: i32) {
        self.buttons = buttons as u8;
    }

    /// Evaluate the PPUSTATUS sprite-overflow flag (bit 5): set it if more than
    /// 8 sprites occupy any single scanline, mirroring NES per-scanline limits.
    pub fn eval_sprite_overflow(&mut self) {
        // Sprite height by mode (8x16 vs 8x8).
        let h = if (self.ctrl & CTRL_TALL) != 0 {
            16
        } else {
            TILE_SIZE
        };
        // Per-scanline sprite counters (240 visible scanlines).
        let mut perline = [0u8; PPU_H];
        for s in 0..SPRITE_COUNT {
            let y = self.oam[s * OAM_ENTRY] as i32;
            // Sprites with Y >= 239 are effectively off-screen; ignore them.
            if y >= 239 {
                continue;
            }
            // Visible scanline span [top, bot): Y+1 .. Y+1+h, clamped to 240.
            let top = y + 1;
            let bot = (y + 1 + h).min(PPU_H as i32);
            for sl in top..bot {
                // Count this sprite on each covered scanline.
                perline[sl as usize] = perline[sl as usize].wrapping_add(1);
                // The 9th sprite on a line trips the overflow flag.
                if perline[sl as usize] > 8 {
                    self.status |= STATUS_OVERFLOW;
                    return;
                }
            }
        }
        // No overflow detected: clear the flag.
        self.status &= !STATUS_OVERFLOW;
    }
}

/// Write an RGB pixel buffer to disk as a binary (P6) PPM image, `w` by `h`
/// pixels. Used to dump rendered frames/tilesheets for inspection.
pub fn ppm_write(path: impl AsRef<Path>, rgb: &[u8], w: usize, h: usize) -> std::io::Result<()> {
    // Create the file and write the P6 header: magic, dimensions, max value.
    let mut f = File::create(path)?;
    write!(f, "P6\n{} {}\n255\n", w, h)?;
    // Write exactly w*h*3 bytes of raw RGB data.
    f.write_all(&rgb[..w * h * 3])?;
    Ok(())
}
