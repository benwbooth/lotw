use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const NATIVE_BLOCKS_HEADER: &str = "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason";

#[derive(Debug, Clone)]
struct StaticSource {
    name: &'static str,
    verified_reason: &'static str,
    blocks: PathBuf,
    summary: PathBuf,
    manifest: PathBuf,
    required: bool,
}

#[derive(Debug, Default, Clone)]
struct SourceStats {
    count: u64,
    appended: u64,
    duplicates: u64,
}

#[derive(Debug, Clone)]
struct NativeBlockRow {
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    replay_count: String,
    replays: String,
    observations: String,
    hit_count_total: String,
    writes_total: String,
    ppu_writes: String,
    apu_writes: String,
    mapper_writes: String,
    final_ram_hash_count: String,
    reason: String,
}

pub fn run(build_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let replay_blocks = build_dir.join("native_block_plan/native_blocks.tsv");
    require_file(&replay_blocks)?;

    let sources = static_sources(build_dir);
    let mut enabled_sources = Vec::new();
    for source in sources {
        if source.required
            || source.blocks.exists()
            || source.summary.exists()
            || source.manifest.exists()
        {
            validate_source(&source)?;
            enabled_sources.push(source);
        }
    }

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let native_blocks = out_dir.join("native_blocks.tsv");
    let merge_tsv = out_dir.join("native_block_static_merge.tsv");
    let summary = out_dir.join("native_block_static_merge_summary.txt");
    let manifest = out_dir.join("manifest.txt");

    let (replay_count, stats, merged_count) =
        write_merged_blocks(&replay_blocks, &enabled_sources, &native_blocks, &merge_tsv)?;
    write_summary(&summary, replay_count, &stats, merged_count)?;
    write_manifest(&manifest, build_dir, &replay_blocks, &enabled_sources)?;

    println!("native_block_static_merge: wrote {}", out_dir.display());
    Ok(())
}

fn static_sources(build_dir: &Path) -> Vec<StaticSource> {
    vec![
        StaticSource {
            name: "static_leaf",
            verified_reason: "static_verified_leaf",
            blocks: build_dir.join("static_leaf_verify/static_leaf_native_blocks.tsv"),
            summary: build_dir.join("static_leaf_verify/static_leaf_verify_summary.txt"),
            manifest: build_dir.join("static_leaf_verify/native_verify/manifest.txt"),
            required: true,
        },
        StaticSource {
            name: "static_handoff",
            verified_reason: "static_verified_handoff",
            blocks: build_dir.join("static_handoff_verify/static_handoff_native_blocks.tsv"),
            summary: build_dir.join("static_handoff_verify/static_handoff_verify_summary.txt"),
            manifest: build_dir.join("static_handoff_verify/native_verify/manifest.txt"),
            required: false,
        },
        StaticSource {
            name: "static_branch",
            verified_reason: "static_verified_branch",
            blocks: build_dir.join("static_branch_verify/static_branch_native_blocks.tsv"),
            summary: build_dir.join("static_branch_verify/static_branch_verify_summary.txt"),
            manifest: build_dir.join("static_branch_verify/native_verify/manifest.txt"),
            required: false,
        },
        StaticSource {
            name: "static_jsr",
            verified_reason: "static_verified_jsr",
            blocks: build_dir.join("static_jsr_verify/static_jsr_native_blocks.tsv"),
            summary: build_dir.join("static_jsr_verify/static_jsr_verify_summary.txt"),
            manifest: build_dir.join("static_jsr_verify/native_verify/manifest.txt"),
            required: false,
        },
        StaticSource {
            name: "static_return",
            verified_reason: "static_verified_return",
            blocks: build_dir.join("static_return_verify/static_return_native_blocks.tsv"),
            summary: build_dir.join("static_return_verify/static_return_verify_summary.txt"),
            manifest: build_dir.join("static_return_verify/native_verify/manifest.txt"),
            required: false,
        },
    ]
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "native_block_static_merge: missing input: {}",
                path.display()
            ),
        ))
    }
}

fn validate_source(source: &StaticSource) -> Result<(), Box<dyn std::error::Error>> {
    for path in [&source.blocks, &source.summary, &source.manifest] {
        if !path.is_file() {
            let kind = if source.required {
                "missing input"
            } else {
                "incomplete input"
            };
            return Err(format!("native_block_static_merge: {kind}: {}", path.display()).into());
        }
    }
    let summary = read_key_values(&source.summary)?;
    let manifest = read_key_values(&source.manifest)?;
    require_key_value(&summary, "complete", "1", &source.summary)?;
    require_key_value(&manifest, "complete", "1", &source.manifest)?;
    require_key_value(&manifest, "mismatches", "0", &source.manifest)?;
    require_key_value(
        &manifest,
        "external_write_mismatches",
        "0",
        &source.manifest,
    )?;
    Ok(())
}

fn read_key_values(path: &Path) -> io::Result<HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut values = HashMap::new();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.to_string(), value.to_string());
        }
    }
    Ok(values)
}

fn require_key_value(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = values.get(key).ok_or_else(|| {
        format!(
            "native_block_static_merge: missing {key} in {}",
            path.display()
        )
    })?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "native_block_static_merge: expected {key}={expected} in {}, got {actual}",
            path.display()
        )
        .into())
    }
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn invalid_tsv<T>(path: &Path, line_no: usize, actual: usize, expected: usize) -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "{}:{line_no} has {actual} fields, expected at least {expected}",
            path.display()
        ),
    ))
}

fn read_native_rows(path: &Path) -> io::Result<Vec<(String, NativeBlockRow)>> {
    let text = fs::read_to_string(path)?;
    let mut lines = text.lines();
    if lines.next() != Some(NATIVE_BLOCKS_HEADER) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}: bad native block header", path.display()),
        ));
    }
    let mut rows = Vec::new();
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 15 {
            return invalid_tsv(path, line_no + 2, fields.len(), 15);
        }
        rows.push((
            fields[0].to_string(),
            NativeBlockRow {
                cpu_addr: fields[1].to_string(),
                prg_offset: fields[2].to_string(),
                bytes: fields[3].to_string(),
                first_opcode: fields[4].to_string(),
                replay_count: fields[5].to_string(),
                replays: fields[6].to_string(),
                observations: fields[7].to_string(),
                hit_count_total: fields[8].to_string(),
                writes_total: fields[9].to_string(),
                ppu_writes: fields[10].to_string(),
                apu_writes: fields[11].to_string(),
                mapper_writes: fields[12].to_string(),
                final_ram_hash_count: fields[13].to_string(),
                reason: fields[14].to_string(),
            },
        ));
    }
    Ok(rows)
}

fn key_for(row: &NativeBlockRow) -> String {
    format!("{}\t{}", row.cpu_addr, row.prg_offset)
}

fn write_merged_blocks(
    replay_blocks: &Path,
    sources: &[StaticSource],
    native_blocks: &Path,
    merge_tsv: &Path,
) -> io::Result<(u64, BTreeMap<&'static str, SourceStats>, u64)> {
    let mut native = fs::File::create(native_blocks)?;
    let mut merge = fs::File::create(merge_tsv)?;
    writeln!(native, "{NATIVE_BLOCKS_HEADER}")?;
    writeln!(
        merge,
        "source\tsource_rank\toutput_rank\tcpu_addr\tprg_offset\taction\treason"
    )?;

    let mut seen = HashSet::<String>::new();
    let mut output_rank = 0u64;
    let mut replay_count = 0u64;
    let replay_rows = read_native_rows(replay_blocks)?;
    for (source_rank, row) in replay_rows {
        seen.insert(key_for(&row));
        output_rank += 1;
        replay_count += 1;
        write_native_row(&mut native, output_rank, &row)?;
        writeln!(
            merge,
            "replay\t{}\t{}\t{}\t{}\tkept\t{}",
            source_rank, output_rank, row.cpu_addr, row.prg_offset, row.reason
        )?;
    }

    let mut stats = BTreeMap::<&'static str, SourceStats>::new();
    for source in sources {
        let rows = read_native_rows(&source.blocks)?;
        for (source_rank, mut row) in rows {
            let entry = stats.entry(source.name).or_default();
            entry.count += 1;
            let key = key_for(&row);
            if seen.contains(&key) {
                entry.duplicates += 1;
                writeln!(
                    merge,
                    "{}\t{}\t\t{}\t{}\tduplicate_skipped\t{}",
                    source.name, source_rank, row.cpu_addr, row.prg_offset, row.reason
                )?;
                continue;
            }
            seen.insert(key);
            output_rank += 1;
            entry.appended += 1;
            row.reason = source.verified_reason.to_string();
            write_native_row(&mut native, output_rank, &row)?;
            writeln!(
                merge,
                "{}\t{}\t{}\t{}\t{}\tappended\t{}",
                source.name,
                source_rank,
                output_rank,
                row.cpu_addr,
                row.prg_offset,
                source.verified_reason
            )?;
        }
    }

    Ok((replay_count, stats, output_rank))
}

fn write_native_row(file: &mut fs::File, rank: u64, row: &NativeBlockRow) -> io::Result<()> {
    writeln!(
        file,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        rank,
        row.cpu_addr,
        row.prg_offset,
        row.bytes,
        row.first_opcode,
        row.replay_count,
        row.replays,
        row.observations,
        row.hit_count_total,
        row.writes_total,
        row.ppu_writes,
        row.apu_writes,
        row.mapper_writes,
        row.final_ram_hash_count,
        row.reason
    )
}

fn write_summary(
    path: &Path,
    replay_count: u64,
    stats: &BTreeMap<&'static str, SourceStats>,
    merged_count: u64,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "replay_block_count={replay_count}")?;
    for source in [
        "static_leaf",
        "static_handoff",
        "static_branch",
        "static_jsr",
        "static_return",
    ] {
        let item = stats.get(source).cloned().unwrap_or_default();
        let key = source.strip_prefix("static_").unwrap_or(source);
        writeln!(file, "static_{key}_count={}", item.count)?;
        writeln!(file, "static_{key}_appended={}", item.appended)?;
        writeln!(file, "static_{key}_duplicates={}", item.duplicates)?;
    }
    writeln!(file, "merged_block_count={merged_count}")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(
    path: &Path,
    build_dir: &Path,
    replay_blocks: &Path,
    enabled_sources: &[StaticSource],
) -> io::Result<()> {
    let source_map = enabled_sources
        .iter()
        .map(|source| (source.name, source))
        .collect::<HashMap<_, _>>();
    let all_sources = static_sources(build_dir);
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=native_block_static_merge")?;
    writeln!(file, "replay_blocks={}", replay_blocks.display())?;
    for source in &all_sources {
        match source.name {
            "static_leaf" => {
                writeln!(file, "static_blocks={}", source.blocks.display())?;
                writeln!(file, "static_summary={}", source.summary.display())?;
                writeln!(file, "static_verify_manifest={}", source.manifest.display())?;
            }
            "static_handoff" => {
                write_optional_manifest(&mut file, "static_handoff", source, &source_map)?
            }
            "static_branch" => {
                write_optional_manifest(&mut file, "static_branch", source, &source_map)?
            }
            "static_jsr" => write_optional_manifest(&mut file, "static_jsr", source, &source_map)?,
            "static_return" => {
                write_optional_manifest(&mut file, "static_return", source, &source_map)?
            }
            _ => {}
        }
    }
    writeln!(file, "native_blocks=native_blocks.tsv")?;
    writeln!(file, "merge=native_block_static_merge.tsv")?;
    writeln!(file, "summary=native_block_static_merge_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_optional_manifest(
    file: &mut fs::File,
    prefix: &str,
    source: &StaticSource,
    enabled: &HashMap<&'static str, &StaticSource>,
) -> io::Result<()> {
    writeln!(
        file,
        "{prefix}_enabled={}",
        u64::from(enabled.contains_key(source.name))
    )?;
    writeln!(file, "{prefix}_blocks={}", source.blocks.display())?;
    writeln!(file, "{prefix}_summary={}", source.summary.display())?;
    writeln!(file, "{prefix}_manifest={}", source.manifest.display())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_static_blocks() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_static_merge_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let build = root.join("build");
        let out = root.join("out");
        for dir in [
            "native_block_plan",
            "static_leaf_verify/native_verify",
            "static_handoff_verify/native_verify",
            "static_branch_verify/native_verify",
            "static_jsr_verify/native_verify",
            "static_return_verify/native_verify",
        ] {
            fs::create_dir_all(build.join(dir)).unwrap();
        }
        write_blocks(
            &build.join("native_block_plan/native_blocks.tsv"),
            &[
                "1\tAE6F\t1AE6F\t3\tA9\t2\ttitle,start\t5\t20\t1\t1\t0\t0\t2\tppu_control_external_write_leaf",
                "2\tF2D3\t1F2D3\t7\tB1\t1\troute\t1\t3\t0\t0\t0\t0\t1\treplay_verified_leaf",
            ],
        );
        write_blocks(
            &build.join("static_leaf_verify/static_leaf_native_blocks.tsv"),
            &[
                "1\tF2D3\t1F2D3\t7\tB1\t0\tstatic_entry_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_leaf_synthetic",
                "2\tF233\t1F233\t7\tB1\t0\tstatic_entry_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_leaf_synthetic",
            ],
        );
        write_source_gate(&build.join("static_leaf_verify/static_leaf_verify_summary.txt"));
        write_manifest_gate(&build.join("static_leaf_verify/native_verify/manifest.txt"));
        write_blocks(
            &build.join("static_handoff_verify/static_handoff_native_blocks.tsv"),
            &["1\tF0E1\t1F0E1\t3\tA5\t0\tstatic_handoff_plan\t4\t0\t4\t0\t0\t0\t4\tstatic_handoff_linear_ram_writes"],
        );
        write_source_gate(&build.join("static_handoff_verify/static_handoff_verify_summary.txt"));
        write_manifest_gate(&build.join("static_handoff_verify/native_verify/manifest.txt"));

        run(&build, &out).unwrap();

        let summary =
            fs::read_to_string(out.join("native_block_static_merge_summary.txt")).unwrap();
        assert!(summary.contains("replay_block_count=2\n"));
        assert!(summary.contains("static_leaf_appended=1\n"));
        assert!(summary.contains("static_leaf_duplicates=1\n"));
        assert!(summary.contains("static_handoff_appended=1\n"));
        assert!(summary.contains("merged_block_count=4\n"));
        let blocks = fs::read_to_string(out.join("native_blocks.tsv")).unwrap();
        assert!(blocks.contains(
            "3\tF233\t1F233\t7\tB1\t0\tstatic_entry_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_leaf\n"
        ));
        assert!(blocks.contains(
            "4\tF0E1\t1F0E1\t3\tA5\t0\tstatic_handoff_plan\t4\t0\t4\t0\t0\t0\t4\tstatic_verified_handoff\n"
        ));
        let _ = fs::remove_dir_all(root);
    }

    fn write_blocks(path: &Path, rows: &[&str]) {
        let mut text = String::from(NATIVE_BLOCKS_HEADER);
        text.push('\n');
        for row in rows {
            text.push_str(row);
            text.push('\n');
        }
        fs::write(path, text).unwrap();
    }

    fn write_source_gate(path: &Path) {
        fs::write(path, "complete=1\n").unwrap();
    }

    fn write_manifest_gate(path: &Path) {
        fs::write(
            path,
            "complete=1\nmismatches=0\nexternal_write_mismatches=0\n",
        )
        .unwrap();
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
