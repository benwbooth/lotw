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

// Meta-page ($E0-page) layout: header, actor records, room palette.
const HDR_LEN: usize = 0x20; // $00-$1F: CHR banks / tile_table / room metadata
const ACTOR_COUNT: usize = 12; // $20-$DF: 12 spawn records
const ACTOR_LEN: usize = 16;
const PAL_LEN: usize = 0x20; // $E0-$FF: 8 sub-palettes of 4

#[derive(Serialize, Deserialize)]
struct RoomMeta {
    mapx: usize,
    mapy: usize,
    prg_offset: usize,
    cols: usize,
    rows: usize,
    /// $00-$1F room header (CHR banks, tile-table pointer, metadata), verbatim.
    header_hex: String,
    /// $20-$DF: 12 actor-spawn records. kind/x/y are the editable position+type
    /// fields (byte 0 / +2 / +3); raw carries the full 16-byte record so unedited
    /// records round-trip exactly. (x|y == 0 means "random spawn".)
    actors: Vec<Actor>,
    /// $E0-$FF room palette: 8 sub-palettes of 4 NES indices.
    palette: Pal,
}

#[derive(Serialize, Deserialize)]
struct Actor {
    kind: u8,
    x: u8,
    y: u8,
    raw: String,
}

#[derive(Serialize, Deserialize)]
struct Pal {
    indices: Vec<u8>,
    #[serde(default)]
    rgb: Vec<String>,
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
            let actors = (0..ACTOR_COUNT)
                .map(|i| {
                    let rec = &meta[HDR_LEN + i * ACTOR_LEN..HDR_LEN + (i + 1) * ACTOR_LEN];
                    Actor { kind: rec[0], x: rec[2], y: rec[3], raw: hex(rec) }
                })
                .collect();
            let pal_bytes = &meta[META - PAL_LEN..META];
            rooms.push(RoomMeta {
                mapx,
                mapy,
                prg_offset: off,
                cols: COLS,
                rows: ROWS,
                header_hex: hex(&meta[0..HDR_LEN]),
                actors,
                palette: Pal {
                    indices: pal_bytes.to_vec(),
                    rgb: pal_bytes.iter().map(|&b| super::palettes::nes_rgb_hex(b)).collect(),
                },
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
        // Reassemble the meta page: header + 12 actor records + palette.
        let mut meta = unhex(&room.header_hex)?;
        for a in &room.actors {
            // raw is the source of truth; patch the editable type/position bytes
            // onto it so position edits take effect while the rest round-trips.
            let mut rec = unhex(&a.raw)?;
            rec[0] = a.kind;
            rec[2] = a.x;
            rec[3] = a.y;
            meta.extend_from_slice(&rec);
        }
        meta.extend_from_slice(&room.palette.indices);
        if meta.len() != META {
            return Err(format!(
                "room {:02}-{}: meta reassembled to {} bytes (expected {META})",
                room.mapy,
                room.mapx,
                meta.len()
            )
            .into());
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
    if !s.len().is_multiple_of(2) {
        return Err("odd-length hex".into());
    }
    (0..s.len() / 2)
        .map(|i| Ok(u8::from_str_radix(&s[2 * i..2 * i + 2], 16)?))
        .collect()
}
