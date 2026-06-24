//! Text / nametable-template extraction.
//!
//! LotW has no string system; readable text is baked into nametable templates
//! as font-tile indices. We recover the font map (tile <-> character, derived
//! from the known HUD labels: A..Z = $E1..$FA, space = $C0) into `assets/
//! font.json`, and extract the named text-bearing templates into `assets/
//! text.json` as editable grids: font tiles render as their character, all
//! other (structural / graphic) tiles as `[hh]`. Editing a label and rebuilding
//! re-encodes to the exact bytes.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs;
use std::path::Path;

/// (name, PRG offset, byte length, columns).
const TEMPLATES: &[(&str, usize, usize, usize)] = &[
    // HUD status bar ($FECB): 5 rows x 32 cols uploaded to nametable $2320.
    ("hud", 0x1_FECB, 160, 32),
];

#[derive(Serialize, Deserialize)]
struct TextFile {
    templates: Vec<Template>,
}

#[derive(Serialize, Deserialize)]
struct Template {
    name: String,
    prg_offset: usize,
    cols: usize,
    /// One string per row; a cell is a font character or `[hh]` raw tile.
    rows: Vec<String>,
}

/// The canonical font: A..Z at $E1..$FA, space at $C0.
fn canonical_font() -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for i in 0..26u8 {
        m.insert(format!("{:02X}", 0xE1 + i), ((b'A' + i) as char).to_string());
    }
    m.insert("C0".into(), " ".into());
    m
}

fn load_font(dir: &Path) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let path = dir.join("font.json");
    if path.exists() {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    } else {
        Ok(canonical_font())
    }
}

pub fn extract(prg: &[u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let font = canonical_font();
    fs::write(dir.join("font.json"), serde_json::to_string_pretty(&font)? + "\n")?;

    let templates = TEMPLATES
        .iter()
        .map(|&(name, off, len, cols)| {
            let rows = prg[off..off + len]
                .chunks(cols)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|&b| match font.get(&format!("{b:02X}")) {
                            Some(c) => c.clone(),
                            None => format!("[{b:02x}]"),
                        })
                        .collect::<String>()
                })
                .collect();
            Template { name: name.into(), prg_offset: off, cols, rows }
        })
        .collect();
    fs::write(
        dir.join("text.json"),
        serde_json::to_string_pretty(&TextFile { templates })? + "\n",
    )?;
    Ok(())
}

pub fn apply(prg: &mut [u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let path = dir.join("text.json");
    if !path.exists() {
        return Ok(());
    }
    let font = load_font(dir)?;
    // Invert to char -> tile byte.
    let mut inv: HashMap<char, u8> = HashMap::new();
    for (hex, ch) in &font {
        let tile = u8::from_str_radix(hex, 16)?;
        let c = ch.chars().next().ok_or("empty font char")?;
        inv.insert(c, tile);
    }
    let tf: TextFile = serde_json::from_str(&fs::read_to_string(&path)?)?;
    for t in &tf.templates {
        let mut bytes = Vec::new();
        for row in &t.rows {
            let mut it = row.chars().peekable();
            while let Some(c) = it.next() {
                if c == '[' {
                    let h1 = it.next().ok_or("truncated [hh]")?;
                    let h2 = it.next().ok_or("truncated [hh]")?;
                    if it.next() != Some(']') {
                        return Err(format!("malformed tile escape in template '{}'", t.name).into());
                    }
                    bytes.push(u8::from_str_radix(&format!("{h1}{h2}"), 16)?);
                } else {
                    let &tile = inv
                        .get(&c)
                        .ok_or_else(|| format!("char {c:?} not in font.json (template '{}')", t.name))?;
                    bytes.push(tile);
                }
            }
        }
        prg[t.prg_offset..t.prg_offset + bytes.len()].copy_from_slice(&bytes);
    }
    Ok(())
}
