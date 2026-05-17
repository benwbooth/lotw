use lotw_port::audio_dsl;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq)]
struct TrackProgress {
    percent: f64,
    done: u64,
    total: u64,
}

#[derive(Debug, Clone, PartialEq)]
struct LogicProgress {
    track: TrackProgress,
    remaining: HashMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq)]
struct ChrTileProgress {
    track: TrackProgress,
    png_path: Option<PathBuf>,
    png_sha256: Option<String>,
    width: Option<u64>,
    height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
struct TraceFrameProgress {
    track: TrackProgress,
    compared: u64,
    matched: u64,
}

pub fn run(build_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let logic = logic_progress(build_dir);
    let chr_tiles = chr_tile_progress(build_dir);
    let sprite_png = sprite_png_progress(build_dir);
    let background_room_png = background_room_png_progress(build_dir);
    let trace_frames = trace_frame_progress(build_dir);
    let music = music_sfx_progress(build_dir);

    let summary = out_dir.join("progress_summary.txt");
    write_summary(
        &summary,
        logic,
        chr_tiles,
        sprite_png,
        background_room_png,
        trace_frames,
        music,
    )?;
    println!("progress_report: wrote {}", out_dir.display());
    Ok(())
}

fn logic_progress(build_dir: &Path) -> LogicProgress {
    let path = build_dir
        .join("whole_program_report")
        .join("whole_program_summary.txt");
    let values = read_key_values(&path).unwrap_or_default();
    let done = parse_u64(values.get("oracle_verified_native_units")).unwrap_or(0);
    let total = parse_u64(values.get("whole_program_known_reachable_units")).unwrap_or(0);
    let remaining = [
        "remaining_known_reachable_units",
        "remaining_replay_covered_needs_block_split",
        "remaining_inside_verified_native_block_span",
        "remaining_entry_plan_leaf_return_or_interrupt",
        "remaining_entry_plan_control_flow",
        "remaining_entry_plan_calls_subroutine",
        "remaining_entry_plan_straight_line_or_data",
        "remaining_entry_plan_other",
        "remaining_not_in_static_entry_plan",
    ]
    .into_iter()
    .filter_map(|key| parse_u64(values.get(key)).map(|value| (key.to_string(), value)))
    .collect();
    LogicProgress {
        track: TrackProgress {
            percent: percent(done, total),
            done,
            total,
        },
        remaining,
    }
}

fn chr_tile_progress(build_dir: &Path) -> ChrTileProgress {
    let manifest_path = build_dir.join("rust_chr_preview").join("manifest.txt");
    let values = read_key_values(&manifest_path).unwrap_or_default();
    let tile_count = parse_u64(values.get("tile_count")).unwrap_or(0);
    let png_name = values.get("png").map(String::as_str).unwrap_or("");
    let png_path = manifest_path
        .parent()
        .filter(|_| !png_name.is_empty())
        .map(|dir| dir.join(png_name));
    let png_exists = png_path.as_ref().is_some_and(|path| path.is_file());
    let complete = values.get("complete").is_some_and(|value| value == "1");
    let done = if complete && png_exists {
        tile_count
    } else {
        0
    };
    ChrTileProgress {
        track: TrackProgress {
            percent: percent(done, tile_count),
            done,
            total: tile_count,
        },
        png_path: png_path.filter(|path| path.is_file()),
        png_sha256: values.get("png_sha256").cloned(),
        width: parse_u64(values.get("width")),
        height: parse_u64(values.get("height")),
    }
}

fn sprite_png_progress(build_dir: &Path) -> TrackProgress {
    let path = build_dir.join("graphics_sprites").join("manifest.txt");
    let values = read_key_values(&path).unwrap_or_default();
    let done = first_u64(&values, &["sprite_png_count", "complete_png_count"]).unwrap_or(0);
    let total = first_u64(&values, &["known_sprite_count", "sprite_total"]).unwrap_or(0);
    TrackProgress {
        percent: percent(done, total),
        done,
        total,
    }
}

fn background_room_png_progress(build_dir: &Path) -> TrackProgress {
    let path = build_dir.join("graphics_backgrounds").join("manifest.txt");
    let values = read_key_values(&path).unwrap_or_default();
    let done = first_u64(&values, &["room_png_count", "complete_png_count"]).unwrap_or(0);
    let total = first_u64(&values, &["known_room_count", "room_total"]).unwrap_or(0);
    TrackProgress {
        percent: percent(done, total),
        done,
        total,
    }
}

fn trace_frame_progress(build_dir: &Path) -> TraceFrameProgress {
    let path = build_dir
        .join("ppu_render_compare")
        .join("replay_ppu_render_compare.tsv");
    let Ok(text) = fs::read_to_string(path) else {
        return TraceFrameProgress {
            track: TrackProgress {
                percent: 0.0,
                done: 0,
                total: 0,
            },
            compared: 0,
            matched: 0,
        };
    };

    let mut compared = 0;
    let mut matched = 0;
    for line in text.lines().skip(1) {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 6 {
            continue;
        }
        compared += 1;
        if fields[4] == "1" && fields[5] == "1" {
            matched += 1;
        }
    }
    TraceFrameProgress {
        track: TrackProgress {
            percent: percent(matched, compared),
            done: matched,
            total: compared,
        },
        compared,
        matched,
    }
}

fn music_sfx_progress(build_dir: &Path) -> TrackProgress {
    let converted = converted_audio_program_count(build_dir).unwrap_or(0);
    let total = reference_replay_count(build_dir).unwrap_or(9);
    TrackProgress {
        percent: percent(converted, total),
        done: converted,
        total,
    }
}

fn converted_audio_program_count(build_dir: &Path) -> io::Result<u64> {
    let path = build_dir.join("audio_dsl").join("manifest.txt");
    if !path.is_file() {
        return Ok(0);
    }
    let values = read_key_values(&path)?;
    Ok(parse_u64(values.get("converted_program_count")).unwrap_or(0))
}

fn reference_replay_count(build_dir: &Path) -> io::Result<u64> {
    let path = build_dir
        .join("reference_hash_harness")
        .join("manifest.txt");
    if !path.is_file() {
        return Ok(9);
    }
    let values = read_key_values(&path)?;
    Ok(parse_u64(values.get("replay_count")).unwrap_or(9))
}

fn write_summary(
    path: &Path,
    logic: LogicProgress,
    chr_tiles: ChrTileProgress,
    sprite_png: TrackProgress,
    background_room_png: TrackProgress,
    trace_frames: TraceFrameProgress,
    music: TrackProgress,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=progress_report")?;
    writeln!(
        file,
        "game_logic_percent={:.2}",
        clamp_percent(logic.track.percent)
    )?;
    writeln!(file, "game_logic_verified_units={}", logic.track.done)?;
    writeln!(file, "game_logic_total_units={}", logic.track.total)?;
    writeln!(
        file,
        "game_logic_metric=oracle_verified_native_units/whole_program_known_reachable_units"
    )?;
    for (source_key, output_key) in [
        (
            "remaining_known_reachable_units",
            "game_logic_remaining_units",
        ),
        (
            "remaining_replay_covered_needs_block_split",
            "game_logic_remaining_replay_covered_needs_block_split",
        ),
        (
            "remaining_inside_verified_native_block_span",
            "game_logic_remaining_inside_verified_native_block_span",
        ),
        (
            "remaining_entry_plan_leaf_return_or_interrupt",
            "game_logic_remaining_entry_plan_leaf_return_or_interrupt",
        ),
        (
            "remaining_entry_plan_control_flow",
            "game_logic_remaining_entry_plan_control_flow",
        ),
        (
            "remaining_entry_plan_calls_subroutine",
            "game_logic_remaining_entry_plan_calls_subroutine",
        ),
        (
            "remaining_entry_plan_straight_line_or_data",
            "game_logic_remaining_entry_plan_straight_line_or_data",
        ),
        (
            "remaining_entry_plan_other",
            "game_logic_remaining_entry_plan_other",
        ),
        (
            "remaining_not_in_static_entry_plan",
            "game_logic_remaining_not_in_static_entry_plan",
        ),
    ] {
        if let Some(value) = logic.remaining.get(source_key) {
            writeln!(file, "{output_key}={value}")?;
        }
    }
    writeln!(
        file,
        "chr_tile_decode_percent={:.2}",
        clamp_percent(chr_tiles.track.percent)
    )?;
    writeln!(
        file,
        "chr_tile_decode_scope=raw_8x8_chr_tiles_single_png_not_sprite_translation"
    )?;
    writeln!(file, "chr_tile_png_tiles={}", chr_tiles.track.done)?;
    writeln!(file, "chr_tile_total_tiles={}", chr_tiles.track.total)?;
    if let Some(png_path) = chr_tiles.png_path {
        writeln!(file, "chr_tile_png_path={}", png_path.display())?;
    }
    if let Some(sha256) = chr_tiles.png_sha256 {
        writeln!(file, "chr_tile_png_sha256={sha256}")?;
    }
    if let Some(width) = chr_tiles.width {
        writeln!(file, "chr_tile_png_width={width}")?;
    }
    if let Some(height) = chr_tiles.height {
        writeln!(file, "chr_tile_png_height={height}")?;
    }
    writeln!(
        file,
        "chr_tile_decode_metric=chr_rom_tiles_decoded_to_png/chr_rom_tiles"
    )?;
    writeln!(
        file,
        "graphics_percent={:.2}",
        clamp_percent(sprite_png.percent)
    )?;
    writeln!(file, "graphics_scope=assembled_palette_correct_sprite_pngs")?;
    writeln!(
        file,
        "graphics_sprite_png_percent={:.2}",
        clamp_percent(sprite_png.percent)
    )?;
    writeln!(file, "graphics_sprite_png_count={}", sprite_png.done)?;
    writeln!(file, "graphics_known_sprite_count={}", sprite_png.total)?;
    writeln!(
        file,
        "graphics_sprite_png_metric=assembled_palette_correct_sprite_pngs/known_sprite_assets"
    )?;
    writeln!(
        file,
        "graphics_background_room_png_percent={:.2}",
        clamp_percent(background_room_png.percent)
    )?;
    writeln!(
        file,
        "graphics_background_room_png_count={}",
        background_room_png.done
    )?;
    writeln!(
        file,
        "graphics_known_background_room_count={}",
        background_room_png.total
    )?;
    writeln!(
        file,
        "graphics_background_room_png_metric=palette_correct_room_pngs/known_room_assets"
    )?;
    writeln!(
        file,
        "graphics_trace_frame_render_match_percent={:.2}",
        clamp_percent(trace_frames.track.percent)
    )?;
    writeln!(
        file,
        "graphics_trace_frame_render_matched={}",
        trace_frames.matched
    )?;
    writeln!(
        file,
        "graphics_trace_frame_render_compared={}",
        trace_frames.compared
    )?;
    writeln!(
        file,
        "graphics_trace_frame_render_scope=full_frame_ppu_trace_renderer_not_asset_translation"
    )?;
    writeln!(
        file,
        "music_sfx_percent={:.2}",
        clamp_percent(music.percent)
    )?;
    writeln!(file, "music_sfx_converted_programs={}", music.done)?;
    writeln!(file, "music_sfx_reference_streams={}", music.total)?;
    writeln!(
        file,
        "music_sfx_metric=rust_2a03_dsl_programs/reference_replay_audio_streams"
    )?;
    writeln!(
        file,
        "music_sfx_dsl_available={}",
        u8::from(audio_dsl::dsl_available())
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn read_key_values(path: &Path) -> io::Result<HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    Ok(text
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect())
}

fn parse_u64(value: Option<&String>) -> Option<u64> {
    value?.parse().ok()
}

fn first_u64(values: &HashMap<String, String>, keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| parse_u64(values.get(*key)))
}

fn percent(done: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        done as f64 * 100.0 / total as f64
    }
}

fn clamp_percent(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_progress_report_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_three_track_progress() {
        let root = temp_dir();
        let build = root.join("build");
        let whole = build.join("whole_program_report");
        let chr = build.join("rust_chr_preview");
        let audio = build.join("audio_dsl");
        let reference = build.join("reference_hash_harness");
        let sprite_graphics = build.join("graphics_sprites");
        let background_graphics = build.join("graphics_backgrounds");
        let ppu_compare = build.join("ppu_render_compare");
        let out = root.join("out");
        fs::create_dir_all(&whole).unwrap();
        fs::create_dir_all(&chr).unwrap();
        fs::create_dir_all(&audio).unwrap();
        fs::create_dir_all(&reference).unwrap();
        fs::create_dir_all(&sprite_graphics).unwrap();
        fs::create_dir_all(&background_graphics).unwrap();
        fs::create_dir_all(&ppu_compare).unwrap();
        fs::write(
            whole.join("whole_program_summary.txt"),
            "oracle_verified_native_units=25\nwhole_program_known_reachable_units=100\nremaining_known_reachable_units=75\nremaining_replay_covered_needs_block_split=12\nremaining_inside_verified_native_block_span=3\nremaining_entry_plan_leaf_return_or_interrupt=4\nremaining_entry_plan_control_flow=5\nremaining_entry_plan_calls_subroutine=6\nremaining_entry_plan_straight_line_or_data=7\nremaining_entry_plan_other=1\nremaining_not_in_static_entry_plan=8\n",
        )
        .unwrap();
        fs::write(
            chr.join("manifest.txt"),
            "png=chr_tiles.png\ntile_count=64\ncomplete=1\n",
        )
        .unwrap();
        fs::write(chr.join("chr_tiles.png"), b"\x89PNG\r\n\x1a\n").unwrap();
        fs::write(audio.join("manifest.txt"), "converted_program_count=2\n").unwrap();
        fs::write(reference.join("manifest.txt"), "replay_count=8\n").unwrap();
        fs::write(
            sprite_graphics.join("manifest.txt"),
            "sprite_png_count=3\nknown_sprite_count=12\n",
        )
        .unwrap();
        fs::write(
            background_graphics.join("manifest.txt"),
            "room_png_count=2\nknown_room_count=10\n",
        )
        .unwrap();
        fs::write(
            ppu_compare.join("replay_ppu_render_compare.tsv"),
            "replay\tframe\treference_hash\tppu_render_hash\tmatch\tpixel_match\n\
             title\t180\ta\tb\t1\t1\n\
             game\t420\ta\tc\t0\t0\n",
        )
        .unwrap();

        run(&build, &out).unwrap();

        let summary = fs::read_to_string(out.join("progress_summary.txt")).unwrap();
        assert!(summary.contains("game_logic_percent=25.00\n"));
        assert!(summary.contains("game_logic_remaining_units=75\n"));
        assert!(summary.contains("game_logic_remaining_replay_covered_needs_block_split=12\n"));
        assert!(summary.contains("game_logic_remaining_inside_verified_native_block_span=3\n"));
        assert!(summary.contains("game_logic_remaining_entry_plan_control_flow=5\n"));
        assert!(summary.contains("game_logic_remaining_not_in_static_entry_plan=8\n"));
        assert!(summary.contains("chr_tile_decode_percent=100.00\n"));
        assert!(summary.contains(
            "chr_tile_decode_scope=raw_8x8_chr_tiles_single_png_not_sprite_translation\n"
        ));
        assert!(summary.contains("chr_tile_png_path="));
        assert!(summary.contains("graphics_percent=25.00\n"));
        assert!(summary.contains("graphics_scope=assembled_palette_correct_sprite_pngs\n"));
        assert!(summary.contains("graphics_sprite_png_count=3\n"));
        assert!(summary.contains("graphics_known_sprite_count=12\n"));
        assert!(summary.contains("graphics_background_room_png_percent=20.00\n"));
        assert!(summary.contains("graphics_trace_frame_render_match_percent=50.00\n"));
        assert!(summary.contains("music_sfx_percent=25.00\n"));
        assert!(summary.contains("music_sfx_dsl_available=1\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
