use crate::rom::{InesRom, Mirroring};
use crate::video::{Frame, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::fs;
use std::path::{Path, PathBuf};

const NES_PALETTE: [[u8; 3]; 64] = [
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderInfo {
    pub frame: usize,
    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub chr_mode: u8,
    pub scroll_valid: bool,
    pub scroll_v: u16,
    pub scroll_x: usize,
    pub scroll_y: usize,
    pub applied_mapper_writes: usize,
    pub applied_ppu_register_writes: usize,
    pub applied_ppu_scroll_writes: usize,
    pub applied_ppu_vram_writes: usize,
    pub applied_oam_dma_writes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedTraceFrame {
    pub frame: Frame,
    pub info: RenderInfo,
}

#[derive(Debug)]
pub enum TraceRenderError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    MissingFrames {
        dir: PathBuf,
    },
    BadTsv {
        path: PathBuf,
        line: usize,
        message: String,
    },
    BadOamHex {
        path: PathBuf,
        line: usize,
    },
}

impl std::fmt::Display for TraceRenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::MissingFrames { dir } => {
                write!(
                    f,
                    "missing frames value in {}/trace_summary.txt",
                    dir.display()
                )
            }
            Self::BadTsv {
                path,
                line,
                message,
            } => write!(f, "{}:{line}: {message}", path.display()),
            Self::BadOamHex { path, line } => {
                write!(f, "{}:{line}: invalid OAM DMA hex row", path.display())
            }
        }
    }
}

impl std::error::Error for TraceRenderError {}

#[derive(Clone)]
struct PpuTraceState {
    nametable: [u8; 0x1000],
    palette: [u8; 0x20],
    oam: [u8; 0x100],
    ppu_ctrl: u8,
    ppu_mask: u8,
    bank_select: u8,
    chr_mode: u8,
    chr_regs: [u8; 8],
    scroll_t: u16,
    render_scroll_v: u16,
    scroll_x: u8,
    render_scroll_x: u8,
    scroll_latch: u8,
    scroll_valid: bool,
    applied_ppu_vram_writes: usize,
    applied_ppu_register_writes: usize,
    applied_ppu_scroll_writes: usize,
    applied_mapper_writes: usize,
    applied_oam_dma_writes: usize,
}

impl Default for PpuTraceState {
    fn default() -> Self {
        Self {
            nametable: [0; 0x1000],
            palette: [0; 0x20],
            oam: [0; 0x100],
            ppu_ctrl: 0,
            ppu_mask: 0,
            bank_select: 0,
            chr_mode: 0,
            chr_regs: [0, 2, 4, 5, 6, 7, 0, 1],
            scroll_t: 0,
            render_scroll_v: 0,
            scroll_x: 0,
            render_scroll_x: 0,
            scroll_latch: 0,
            scroll_valid: false,
            applied_ppu_vram_writes: 0,
            applied_ppu_register_writes: 0,
            applied_ppu_scroll_writes: 0,
            applied_mapper_writes: 0,
            applied_oam_dma_writes: 0,
        }
    }
}

pub fn render_trace_frame(
    rom: &InesRom,
    trace_dir: &Path,
    frame: Option<usize>,
) -> Result<RenderedTraceFrame, TraceRenderError> {
    let frame = match frame {
        Some(frame) if frame > 0 => frame,
        _ => read_trace_frames(trace_dir)?,
    };
    let mut state = PpuTraceState::default();

    load_mapper_trace(&mut state, trace_dir, frame)?;
    load_ppu_register_trace(&mut state, trace_dir, frame)?;
    load_ppu_vram_trace(&mut state, &rom.header().mirroring, trace_dir, frame)?;
    load_oam_dma_trace(&mut state, trace_dir, frame)?;

    let mut rgb = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3];
    write_ppu_frame(rom, &state, &mut rgb);
    let info = RenderInfo {
        frame,
        ppu_ctrl: state.ppu_ctrl,
        ppu_mask: state.ppu_mask,
        chr_mode: state.chr_mode,
        scroll_valid: state.scroll_valid,
        scroll_v: if state.scroll_valid {
            state.render_scroll_v
        } else {
            state.scroll_t
        },
        scroll_x: render_scroll_x_pixels(&state),
        scroll_y: render_scroll_y_pixels(&state),
        applied_mapper_writes: state.applied_mapper_writes,
        applied_ppu_register_writes: state.applied_ppu_register_writes,
        applied_ppu_scroll_writes: state.applied_ppu_scroll_writes,
        applied_ppu_vram_writes: state.applied_ppu_vram_writes,
        applied_oam_dma_writes: state.applied_oam_dma_writes,
    };

    Ok(RenderedTraceFrame {
        frame: Frame {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            rgb,
        },
        info,
    })
}

fn read_trace_frames(trace_dir: &Path) -> Result<usize, TraceRenderError> {
    let path = trace_file_path(trace_dir, "trace_summary.txt", "port_trace_summary.txt");
    let text = read_to_string(&path)?;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("frames=") {
            if let Ok(frame) = value.parse::<usize>() {
                if frame > 0 {
                    return Ok(frame);
                }
            }
        }
    }
    Err(TraceRenderError::MissingFrames {
        dir: trace_dir.to_path_buf(),
    })
}

fn load_mapper_trace(
    state: &mut PpuTraceState,
    trace_dir: &Path,
    target_frame: usize,
) -> Result<(), TraceRenderError> {
    let path = trace_file_path(trace_dir, "mapper_writes.tsv", "port_mapper_writes.tsv");
    for (line_no, fields) in read_tsv(&path)?.into_iter().enumerate().skip(1) {
        if fields.is_empty() || fields[0].is_empty() {
            continue;
        }
        require_fields(&path, line_no + 1, &fields, 3)?;
        let frame = parse_dec(&path, line_no + 1, &fields[0])?;
        let addr = parse_hex(&path, line_no + 1, &fields[1])?;
        let value = parse_hex(&path, line_no + 1, &fields[2])?;
        if frame > target_frame {
            continue;
        }
        match addr {
            0x8000 => {
                state.bank_select = (value & 0x07) as u8;
                state.chr_mode = u8::from(value & 0x80 != 0);
            }
            0x8001 => state.chr_regs[state.bank_select as usize] = value as u8,
            _ => {}
        }
        state.applied_mapper_writes += 1;
    }
    Ok(())
}

fn load_ppu_register_trace(
    state: &mut PpuTraceState,
    trace_dir: &Path,
    target_frame: usize,
) -> Result<(), TraceRenderError> {
    let path = trace_file_path(trace_dir, "ppu_writes.tsv", "port_ppu_writes.tsv");
    for (line_no, fields) in read_tsv(&path)?.into_iter().enumerate().skip(1) {
        if fields.is_empty() || fields[0].is_empty() {
            continue;
        }
        require_fields(&path, line_no + 1, &fields, 5)?;
        let frame = parse_dec(&path, line_no + 1, &fields[0])?;
        let addr = parse_hex(&path, line_no + 1, &fields[2])?;
        let value = parse_hex(&path, line_no + 1, &fields[4])? as u8;
        if frame > target_frame {
            continue;
        }
        match fields[3].as_str() {
            "PPUCTRL" => {
                state.ppu_ctrl = value;
                state.scroll_t = (state.scroll_t & 0xf3ff) | (((value & 0x03) as u16) << 10);
            }
            "PPUMASK" => state.ppu_mask = value,
            "PPUSCROLL" => {
                if state.scroll_latch == 0 {
                    state.scroll_t = (state.scroll_t & 0x7fe0) | (((value >> 3) & 0x1f) as u16);
                    state.scroll_x = value & 0x07;
                    state.scroll_latch = 1;
                } else {
                    state.scroll_t = (state.scroll_t & 0x0c1f)
                        | (((value & 0x07) as u16) << 12)
                        | (((value & 0xf8) as u16) << 2);
                    state.render_scroll_v = state.scroll_t;
                    state.render_scroll_x = state.scroll_x;
                    state.scroll_valid = true;
                    state.scroll_latch = 0;
                }
                state.applied_ppu_scroll_writes += 1;
            }
            "PPUADDR" => {
                if state.scroll_latch == 0 {
                    state.scroll_t = (state.scroll_t & 0x00ff) | (((value & 0x3f) as u16) << 8);
                    state.scroll_latch = 1;
                } else {
                    state.scroll_t = (state.scroll_t & 0x7f00) | value as u16;
                    state.scroll_latch = 0;
                }
            }
            _ if addr == 0x2002 => state.scroll_latch = 0,
            _ => {}
        }
        state.applied_ppu_register_writes += 1;
    }
    Ok(())
}

fn load_ppu_vram_trace(
    state: &mut PpuTraceState,
    mirroring: &Mirroring,
    trace_dir: &Path,
    target_frame: usize,
) -> Result<(), TraceRenderError> {
    let path = trace_file_path(trace_dir, "ppu_vram_writes.tsv", "port_ppu_vram_writes.tsv");
    for (line_no, fields) in read_tsv(&path)?.into_iter().enumerate().skip(1) {
        if fields.is_empty() || fields[0].is_empty() {
            continue;
        }
        require_fields(&path, line_no + 1, &fields, 5)?;
        let frame = parse_dec(&path, line_no + 1, &fields[0])?;
        let addr = parse_hex(&path, line_no + 1, &fields[2])?;
        let value = parse_hex(&path, line_no + 1, &fields[4])? as u8;
        if frame > target_frame {
            continue;
        }
        if (0x2000..0x3f00).contains(&addr) {
            state.nametable[mirror_nametable_addr(mirroring, addr as u16) as usize] = value;
        } else if addr >= 0x3f00 {
            state.palette[palette_addr_index(addr as u16) as usize] = value;
        }
        state.applied_ppu_vram_writes += 1;
    }
    Ok(())
}

fn load_oam_dma_trace(
    state: &mut PpuTraceState,
    trace_dir: &Path,
    target_frame: usize,
) -> Result<(), TraceRenderError> {
    let path = trace_file_path(trace_dir, "oam_dma.tsv", "port_oam_dma.tsv");
    for (line_no, fields) in read_tsv(&path)?.into_iter().enumerate().skip(1) {
        if fields.is_empty() || fields[0].is_empty() {
            continue;
        }
        require_fields(&path, line_no + 1, &fields, 4)?;
        let frame = parse_dec(&path, line_no + 1, &fields[0])?;
        if frame > target_frame {
            continue;
        }
        parse_hex_bytes(&fields[3], &mut state.oam).ok_or_else(|| TraceRenderError::BadOamHex {
            path: path.clone(),
            line: line_no + 1,
        })?;
        state.applied_oam_dma_writes += 1;
    }
    Ok(())
}

fn write_ppu_frame(rom: &InesRom, state: &PpuTraceState, image: &mut [u8]) {
    let mut bg_opaque = vec![false; SCREEN_WIDTH * SCREEN_HEIGHT];
    render_background(rom, state, image, &mut bg_opaque);
    render_sprites(rom, state, image, &bg_opaque);
}

fn render_background(
    rom: &InesRom,
    state: &PpuTraceState,
    image: &mut [u8],
    bg_opaque: &mut [bool],
) {
    let pattern_base = if state.ppu_ctrl & 0x10 != 0 {
        0x1000
    } else {
        0
    };
    let scroll_x = render_scroll_x_pixels(state);
    let scroll_y = render_scroll_y_pixels(state);

    for py in 0..SCREEN_HEIGHT {
        let world_y = (py + scroll_y) % 480;
        let nt_y = world_y / 240;
        let screen_y = world_y % 240;
        let tile_y = screen_y / 8;
        let fine_y = screen_y & 7;

        for px in 0..SCREEN_WIDTH {
            let world_x = (px + scroll_x) & 0x01ff;
            let nt_x = world_x / 256;
            let screen_x = world_x & 0xff;
            let tile_x = screen_x / 8;
            let fine_x = screen_x & 7;
            let nt_base = 0x2000 + ((nt_y * 2 + nt_x) * 0x0400);
            let tile_index_addr = nt_base + tile_y * 32 + tile_x;
            let attr_addr = nt_base + 0x03c0 + (tile_y / 4) * 8 + (tile_x / 4);
            let tile = nametable_read(rom, state, tile_index_addr as u16);
            let attr = nametable_read(rom, state, attr_addr as u16);
            let quadrant =
                if tile_y & 2 != 0 { 2 } else { 0 } + if tile_x & 2 != 0 { 1 } else { 0 };
            let palette_group = (attr >> (quadrant * 2)) & 0x03;
            let tile_addr = pattern_base + tile as u16 * 16;
            let color = tile_pixel(rom, state, tile_addr, fine_x, fine_y);
            let palette_slot = if color == 0 {
                0
            } else {
                palette_group * 4 + color
            };
            let nes_color = state.palette[palette_slot as usize] & 0x3f;
            put_pixel(image, px, py, &NES_PALETTE[nes_color as usize]);
            bg_opaque[py * SCREEN_WIDTH + px] = color != 0;
        }
    }
}

fn render_sprites(rom: &InesRom, state: &PpuTraceState, image: &mut [u8], bg_opaque: &[bool]) {
    let sprite_height = if state.ppu_ctrl & 0x20 != 0 { 16 } else { 8 };

    for sprite in (0..64).rev() {
        let entry = &state.oam[sprite * 4..sprite * 4 + 4];
        let top = entry[0] as isize + 1;
        let tile = entry[1];
        let attr = entry[2];
        let left = entry[3] as isize;
        let behind_background = attr & 0x20 != 0;
        let flip_h = attr & 0x40 != 0;
        let flip_v = attr & 0x80 != 0;
        let palette_base = 0x10 + (attr & 0x03) * 4;

        for sy in 0..sprite_height {
            let py = top + sy as isize;
            let src_y = if flip_v { sprite_height - 1 - sy } else { sy };
            if py < 0 || py >= SCREEN_HEIGHT as isize {
                continue;
            }
            for sx in 0..8 {
                let px = left + sx as isize;
                let src_x = if flip_h { 7 - sx } else { sx };
                if px < 0 || px >= SCREEN_WIDTH as isize {
                    continue;
                }
                let color = sprite_pixel(rom, state, tile, src_y, src_x);
                if color == 0 {
                    continue;
                }
                let py = py as usize;
                let px = px as usize;
                if behind_background && bg_opaque[py * SCREEN_WIDTH + px] {
                    continue;
                }
                let nes_color = state.palette[((palette_base + color) & 0x1f) as usize] & 0x3f;
                put_pixel(image, px, py, &NES_PALETTE[nes_color as usize]);
            }
        }
    }
}

fn sprite_pixel(rom: &InesRom, state: &PpuTraceState, tile: u8, mut row: usize, col: usize) -> u8 {
    let tile_addr = if state.ppu_ctrl & 0x20 != 0 {
        let pattern_base = if tile & 1 != 0 { 0x1000 } else { 0 };
        let mut tile_pair = tile & 0xfe;
        if row >= 8 {
            tile_pair = tile_pair.wrapping_add(1);
            row -= 8;
        }
        pattern_base + tile_pair as u16 * 16
    } else {
        let pattern_base = if state.ppu_ctrl & 0x08 != 0 {
            0x1000
        } else {
            0
        };
        pattern_base + tile as u16 * 16
    };
    tile_pixel(rom, state, tile_addr, col, row)
}

fn tile_pixel(rom: &InesRom, state: &PpuTraceState, tile_addr: u16, x: usize, y: usize) -> u8 {
    let lo = chr_read(rom, state, tile_addr + y as u16);
    let hi = chr_read(rom, state, tile_addr + y as u16 + 8);
    let bit = 7 - x;
    (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1)
}

fn chr_read(rom: &InesRom, state: &PpuTraceState, addr: u16) -> u8 {
    if rom.chr_rom().is_empty() {
        return 0;
    }
    let bank_count = rom.chr_rom().len() / 0x0400;
    if bank_count == 0 {
        return 0;
    }
    let bank = mapper_chr_bank(state, addr & 0x1fff) as usize;
    let offset = ((bank % bank_count) * 0x0400) + usize::from(addr & 0x03ff);
    rom.chr_rom()[offset % rom.chr_rom().len()]
}

fn mapper_chr_bank(state: &PpuTraceState, addr: u16) -> u8 {
    if state.chr_mode == 0 {
        if addr < 0x0800 {
            return (state.chr_regs[0] & 0xfe) + ((addr >> 10) & 1) as u8;
        }
        if addr < 0x1000 {
            return (state.chr_regs[1] & 0xfe) + ((addr >> 10) & 1) as u8;
        }
        if addr < 0x1400 {
            return state.chr_regs[2];
        }
        if addr < 0x1800 {
            return state.chr_regs[3];
        }
        if addr < 0x1c00 {
            return state.chr_regs[4];
        }
        return state.chr_regs[5];
    }

    if addr < 0x0400 {
        return state.chr_regs[2];
    }
    if addr < 0x0800 {
        return state.chr_regs[3];
    }
    if addr < 0x0c00 {
        return state.chr_regs[4];
    }
    if addr < 0x1000 {
        return state.chr_regs[5];
    }
    if addr < 0x1800 {
        return (state.chr_regs[0] & 0xfe) + ((addr >> 10) & 1) as u8;
    }
    (state.chr_regs[1] & 0xfe) + ((addr >> 10) & 1) as u8
}

fn nametable_read(rom: &InesRom, state: &PpuTraceState, addr: u16) -> u8 {
    state.nametable[mirror_nametable_addr(&rom.header().mirroring, addr) as usize]
}

fn mirror_nametable_addr(mirroring: &Mirroring, addr: u16) -> u16 {
    let index = (addr - 0x2000) & 0x0fff;
    let table = index / 0x0400;
    let offset = index & 0x03ff;
    match mirroring {
        Mirroring::Horizontal => {
            if table < 2 {
                offset
            } else {
                0x0400 + offset
            }
        }
        Mirroring::Vertical => {
            if table & 1 != 0 {
                0x0400 + offset
            } else {
                offset
            }
        }
        Mirroring::FourScreen => index,
    }
}

fn palette_addr_index(addr: u16) -> u8 {
    let mut index = ((addr - 0x3f00) & 0x1f) as u8;
    if matches!(index, 0x10 | 0x14 | 0x18 | 0x1c) {
        index -= 0x10;
    }
    index
}

fn render_scroll_x_pixels(state: &PpuTraceState) -> usize {
    let v = if state.scroll_valid {
        state.render_scroll_v
    } else {
        state.scroll_t
    };
    (((if v & 0x0400 != 0 { 256 } else { 0 }) + usize::from(v & 0x001f) * 8)
        + usize::from(if state.scroll_valid {
            state.render_scroll_x
        } else {
            state.scroll_x
        }))
        & 0x01ff
}

fn render_scroll_y_pixels(state: &PpuTraceState) -> usize {
    let v = if state.scroll_valid {
        state.render_scroll_v
    } else {
        state.scroll_t
    };
    ((if v & 0x0800 != 0 { 240 } else { 0 })
        + usize::from((v >> 5) & 0x1f) * 8
        + usize::from((v >> 12) & 0x07))
        % 480
}

fn put_pixel(image: &mut [u8], x: usize, y: usize, rgb: &[u8; 3]) {
    let dst = (y * SCREEN_WIDTH + x) * 3;
    image[dst..dst + 3].copy_from_slice(rgb);
}

fn trace_file_path(trace_dir: &Path, reference_name: &str, port_name: &str) -> PathBuf {
    let reference = trace_dir.join(reference_name);
    if reference.is_file() {
        reference
    } else {
        trace_dir.join(port_name)
    }
}

fn read_to_string(path: &Path) -> Result<String, TraceRenderError> {
    fs::read_to_string(path).map_err(|source| TraceRenderError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn read_tsv(path: &Path) -> Result<Vec<Vec<String>>, TraceRenderError> {
    Ok(read_to_string(path)?
        .lines()
        .map(|line| line.split('\t').map(str::to_string).collect())
        .collect())
}

fn require_fields(
    path: &Path,
    line: usize,
    fields: &[String],
    min: usize,
) -> Result<(), TraceRenderError> {
    if fields.len() >= min {
        Ok(())
    } else {
        Err(TraceRenderError::BadTsv {
            path: path.to_path_buf(),
            line,
            message: format!("expected at least {min} fields, got {}", fields.len()),
        })
    }
}

fn parse_dec(path: &Path, line: usize, text: &str) -> Result<usize, TraceRenderError> {
    text.parse::<usize>().map_err(|_| TraceRenderError::BadTsv {
        path: path.to_path_buf(),
        line,
        message: format!("invalid decimal value: {text}"),
    })
}

fn parse_hex(path: &Path, line: usize, text: &str) -> Result<usize, TraceRenderError> {
    usize::from_str_radix(text, 16).map_err(|_| TraceRenderError::BadTsv {
        path: path.to_path_buf(),
        line,
        message: format!("invalid hex value: {text}"),
    })
}

fn parse_hex_bytes(text: &str, out: &mut [u8]) -> Option<()> {
    if text.len() != out.len() * 2 {
        return None;
    }
    for (index, byte) in out.iter_mut().enumerate() {
        let start = index * 2;
        *byte = u8::from_str_radix(&text[start..start + 2], 16).ok()?;
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_port_ppu_trace_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    fn ines_fixture() -> InesRom {
        let mut bytes = vec![0u8; 16 + 0x4000 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 1;
        bytes[5] = 1;
        bytes[6] = 0x40;
        InesRom::parse(&bytes).unwrap()
    }

    #[test]
    fn renders_empty_trace_frame_and_counts_headers() {
        let root = temp_dir();
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("trace_summary.txt"), "frames=1\n").unwrap();
        fs::write(
            root.join("mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n",
        )
        .unwrap();
        fs::write(
            root.join("ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n",
        )
        .unwrap();
        fs::write(
            root.join("ppu_vram_writes.tsv"),
            "frame\tcycle\taddr\tregion\tvalue\n",
        )
        .unwrap();
        fs::write(
            root.join("oam_dma.tsv"),
            "frame\tcycle\tpage\tbytes_0000_00ff\n",
        )
        .unwrap();

        let rendered = render_trace_frame(&ines_fixture(), &root, None).unwrap();

        assert_eq!(rendered.frame.width, SCREEN_WIDTH);
        assert_eq!(rendered.frame.height, SCREEN_HEIGHT);
        assert_eq!(rendered.info.frame, 1);
        assert_eq!(rendered.info.applied_mapper_writes, 0);
        assert_eq!(rendered.info.applied_ppu_register_writes, 0);
        assert_eq!(rendered.info.applied_ppu_vram_writes, 0);
        assert_eq!(rendered.info.applied_oam_dma_writes, 0);

        fs::remove_dir_all(root).unwrap();
    }
}
