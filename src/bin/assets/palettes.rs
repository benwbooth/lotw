//! Palette tables <-> `assets/palettes.json`.
//!
//! NES palette bytes are 6-bit master-palette indices (0..$3F). We extract the
//! clearly-bounded tables (title + per-family sprite palettes) as editable index
//! lists, annotated with the real NES RGB for viewing. The indices are the
//! source of truth; on build they are written back over the PRG image, so the
//! round-trip is exact. (Room/level palettes use a paged scheme tied to room
//! metadata and are handled with the room extractor.)

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

/// (name, PRG file offset, byte length, group size for display).
const TABLES: &[(&str, usize, usize, usize)] = &[
    // Title screen full palette ($A2C9, bank 13): 8 sub-palettes of 4.
    ("title", 0x1_A2C9, 32, 4),
    // Drasle-family sprite palettes ($FFC5, fixed bank): 5 members x 4.
    ("family", 0x1_FFC5, 20, 4),
];

#[derive(Serialize, Deserialize)]
struct PaletteFile {
    note: String,
    tables: Vec<Table>,
}

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    prg_offset: usize,
    group: usize,
    /// NES master-palette indices (0..0x3F) — the editable source of truth.
    indices: Vec<u8>,
    /// Parallel hex RGB for viewing only; ignored on build.
    #[serde(default)]
    rgb: Vec<String>,
}

pub fn extract(prg: &[u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let tables = TABLES
        .iter()
        .map(|&(name, off, len, group)| {
            let indices = prg[off..off + len].to_vec();
            let rgb = indices.iter().map(|&i| nes_rgb_hex(i)).collect();
            Table { name: name.into(), prg_offset: off, group, indices, rgb }
        })
        .collect();
    let file = PaletteFile {
        note: "NES master-palette indices (0..0x3F); `indices` is the source, `rgb` is viewing-only."
            .into(),
        tables,
    };
    fs::write(dir.join("palettes.json"), serde_json::to_string_pretty(&file)? + "\n")?;
    Ok(())
}

/// Overlay the edited palette indices back onto the PRG image.
pub fn apply(prg: &mut [u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let path = dir.join("palettes.json");
    if !path.exists() {
        return Ok(());
    }
    let file: PaletteFile = serde_json::from_str(&fs::read_to_string(&path)?)?;
    for t in &file.tables {
        if t.indices.iter().any(|&i| i > 0x3F) {
            return Err(format!("palette '{}' has an index > 0x3F", t.name).into());
        }
        prg[t.prg_offset..t.prg_offset + t.indices.len()].copy_from_slice(&t.indices);
    }
    Ok(())
}

fn nes_rgb_hex(i: u8) -> String {
    let (r, g, b) = NES_PALETTE[(i & 0x3F) as usize];
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// Standard NES (2C02) master palette, 64 entries.
const NES_PALETTE: [(u8, u8, u8); 64] = [
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
