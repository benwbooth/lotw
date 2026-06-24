//! Room layouts <-> per-room CSV metatile grids.
//!
//! The world map is 4 columns x 16 rows of rooms; each PRG bank holds two map
//! rows (bank = mapY/2), 8 rooms/bank at a 0x400 stride. A room is 1024 bytes:
//! 768 metatile indices (64 columns x 12 rows, column-major) followed by a
//! 256-byte palette/attribute/object page. Each metatile expands via the room's
//! `tile_table` into a 2x2 block of CHR tiles.
//!
//! We extract the metatile grid to an editable CSV (12 rows x 64 cols, in the
//! natural left-to-right / top-to-bottom orientation) and keep the 256-byte
//! page as hex (its object/actor portion is decoded by the object extractor).
//! Build re-encodes column-major and overlays the exact bytes.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

const COLS: usize = 64;
const ROWS: usize = 12;
const TILES: usize = COLS * ROWS; // 768
const META: usize = 256;
const ROOM: usize = TILES + META; // 1024
const MAP_ROWS: usize = 16;
const MAP_COLS: usize = 4;

#[derive(Serialize, Deserialize)]
struct Manifest {
    note: String,
    rooms: Vec<RoomMeta>,
}

#[derive(Serialize, Deserialize)]
struct RoomMeta {
    mapx: usize,
    mapy: usize,
    prg_offset: usize,
    cols: usize,
    rows: usize,
    /// The 256-byte palette/attribute/object page, verbatim hex.
    meta_hex: String,
}

fn room_offset(mapx: usize, mapy: usize) -> usize {
    let bank = mapy / 2;
    let slot = (mapy & 1) * 4 + mapx;
    bank * 0x2000 + slot * 0x400
}

pub fn extract(prg: &[u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let rdir = dir.join("rooms");
    fs::create_dir_all(&rdir)?;
    let mut rooms = Vec::new();
    for mapy in 0..MAP_ROWS {
        for mapx in 0..MAP_COLS {
            let off = room_offset(mapx, mapy);
            let tiles = &prg[off..off + TILES];
            let meta = &prg[off + TILES..off + ROOM];
            // Column-major source -> row-major grid for display.
            let mut csv = String::new();
            for r in 0..ROWS {
                let row: Vec<String> = (0..COLS).map(|c| tiles[c * ROWS + r].to_string()).collect();
                csv.push_str(&row.join(","));
                csv.push('\n');
            }
            fs::write(rdir.join(format!("room-{mapy:02}-{mapx}.csv")), csv)?;
            rooms.push(RoomMeta {
                mapx,
                mapy,
                prg_offset: off,
                cols: COLS,
                rows: ROWS,
                meta_hex: hex(meta),
            });
        }
    }
    let manifest = Manifest {
        note: "Per-room metatile grids in room-YY-X.csv (12 rows x 64 cols); meta_hex is the \
               room's 256-byte palette/attribute/object page (object data decoded separately)."
            .into(),
        rooms,
    };
    fs::write(rdir.join("manifest.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    Ok(())
}

pub fn apply(prg: &mut [u8], dir: &Path) -> Result<(), Box<dyn Error>> {
    let rdir = dir.join("rooms");
    let manifest_path = rdir.join("manifest.json");
    if !manifest_path.exists() {
        return Ok(());
    }
    let manifest: Manifest = serde_json::from_str(&fs::read_to_string(&manifest_path)?)?;
    for room in &manifest.rooms {
        let csv = fs::read_to_string(rdir.join(format!("room-{:02}-{}.csv", room.mapy, room.mapx)))?;
        let grid: Vec<Vec<u8>> = csv
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| line.split(',').map(|t| t.trim().parse()).collect())
            .collect::<Result<_, _>>()?;
        if grid.len() != ROWS || grid.iter().any(|r| r.len() != COLS) {
            return Err(format!("room {:02}-{}: expected {ROWS}x{COLS} grid", room.mapy, room.mapx).into());
        }
        let mut bytes = vec![0u8; ROOM];
        // Row-major grid -> column-major bytes.
        for r in 0..ROWS {
            for c in 0..COLS {
                bytes[c * ROWS + r] = grid[r][c];
            }
        }
        let meta = unhex(&room.meta_hex)?;
        if meta.len() != META {
            return Err(format!("room {:02}-{}: meta must be {META} bytes", room.mapy, room.mapx).into());
        }
        bytes[TILES..ROOM].copy_from_slice(&meta);
        prg[room.prg_offset..room.prg_offset + ROOM].copy_from_slice(&bytes);
    }
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err("odd-length hex".into());
    }
    (0..s.len() / 2)
        .map(|i| Ok(u8::from_str_radix(&s[2 * i..2 * i + 2], 16)?))
        .collect()
}
