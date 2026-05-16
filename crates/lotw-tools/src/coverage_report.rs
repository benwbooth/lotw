use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct ReplayCoverage {
    replay: String,
    frames: String,
    executed_labels: u64,
    mapper_writes: u64,
    apu_writes: u64,
    oam_dma: u64,
    ppu_writes: u64,
    ppu_vram_writes: u64,
    block_count: u64,
    left_block: u64,
    step_limit: u64,
    unsupported_opcode: u64,
    invalid_block: u64,
    replay_sha256: String,
}

#[derive(Debug, Default)]
struct Totals {
    executed_labels: u64,
    mapper_writes: u64,
    apu_writes: u64,
    oam_dma: u64,
    ppu_writes: u64,
    ppu_vram_writes: u64,
    block_count: u64,
    left_block: u64,
    step_limit: u64,
    unsupported_opcode: u64,
    invalid_block: u64,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    replays: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if replays.is_empty() {
        return Err("coverage_report: at least one replay is required".into());
    }

    fs::create_dir_all(out_dir)?;
    let summary = out_dir.join("coverage_summary.tsv");
    let manifest = out_dir.join("manifest.txt");

    let mut rows = Vec::new();
    let mut totals = Totals::default();
    for replay in replays {
        let row = read_replay(build_dir, replay)?;
        totals.add(&row);
        rows.push(row);
    }

    write_summary(&summary, &rows)?;
    write_manifest(&manifest, replays.len(), &totals)?;

    if totals.unsupported_opcode != 0 {
        return Err(format!(
            "coverage_report: unsupported opcodes remain: {}",
            totals.unsupported_opcode
        )
        .into());
    }
    if totals.invalid_block != 0 {
        return Err(format!(
            "coverage_report: invalid blocks remain: {}",
            totals.invalid_block
        )
        .into());
    }

    println!("coverage_report: wrote {}", summary.display());
    Ok(())
}

impl Totals {
    fn add(&mut self, row: &ReplayCoverage) {
        self.executed_labels += row.executed_labels;
        self.mapper_writes += row.mapper_writes;
        self.apu_writes += row.apu_writes;
        self.oam_dma += row.oam_dma;
        self.ppu_writes += row.ppu_writes;
        self.ppu_vram_writes += row.ppu_vram_writes;
        self.block_count += row.block_count;
        self.left_block += row.left_block;
        self.step_limit += row.step_limit;
        self.unsupported_opcode += row.unsupported_opcode;
        self.invalid_block += row.invalid_block;
    }
}

fn read_replay(
    build_dir: &Path,
    replay: &str,
) -> Result<ReplayCoverage, Box<dyn std::error::Error>> {
    let trace_summary = build_dir
        .join("trace")
        .join(replay)
        .join("trace_summary.txt");
    let block_manifest = build_dir
        .join("block_exec")
        .join(replay)
        .join("manifest.txt");
    require_file(&trace_summary)?;
    require_file(&block_manifest)?;

    let trace = read_key_values(&trace_summary)?;
    let block = read_key_values(&block_manifest)?;
    ensure_eq(&trace, "complete", "1", &trace_summary)?;
    ensure_eq(&block, "complete", "1", &block_manifest)?;

    Ok(ReplayCoverage {
        replay: replay.to_string(),
        frames: required(&trace, "frames", &trace_summary)?.to_string(),
        executed_labels: required_u64(&trace, "executed_label_count", &trace_summary)?,
        mapper_writes: required_u64(&trace, "mapper_write_count", &trace_summary)?,
        apu_writes: required_u64(&trace, "apu_write_count", &trace_summary)?,
        oam_dma: required_u64(&trace, "oam_dma_count", &trace_summary)?,
        ppu_writes: required_u64(&trace, "ppu_write_count", &trace_summary)?,
        ppu_vram_writes: required_u64(&trace, "ppu_vram_write_count", &trace_summary)?,
        replay_sha256: required(&trace, "replay_sha256", &trace_summary)?.to_string(),
        block_count: required_u64(&block, "block_count", &block_manifest)?,
        left_block: required_u64(&block, "left_block", &block_manifest)?,
        step_limit: required_u64(&block, "step_limit", &block_manifest)?,
        unsupported_opcode: required_u64(&block, "unsupported_opcode", &block_manifest)?,
        invalid_block: required_u64(&block, "invalid_block", &block_manifest)?,
    })
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("coverage_report: missing input: {}", path.display()),
        ))
    }
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

fn required<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("coverage_report: missing {key} in {}", path.display()).into())
}

fn required_u64(
    values: &HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(required(values, key, path)?.parse::<u64>()?)
}

fn ensure_eq(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = required(values, key, path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{} has {key}={actual}, expected {expected}", path.display()).into())
    }
}

fn write_summary(path: &Path, rows: &[ReplayCoverage]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "replay\tframes\texecuted_labels\tmapper_writes\tapu_writes\toam_dma\tppu_writes\tppu_vram_writes\tblock_count\tleft_block\tstep_limit\tunsupported_opcode\tinvalid_block\treplay_sha256"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.replay,
            row.frames,
            row.executed_labels,
            row.mapper_writes,
            row.apu_writes,
            row.oam_dma,
            row.ppu_writes,
            row.ppu_vram_writes,
            row.block_count,
            row.left_block,
            row.step_limit,
            row.unsupported_opcode,
            row.invalid_block,
            row.replay_sha256
        )?;
    }
    Ok(())
}

fn write_manifest(path: &Path, replay_count: usize, totals: &Totals) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "replay_count={replay_count}")?;
    writeln!(file, "total_executed_labels={}", totals.executed_labels)?;
    writeln!(file, "total_mapper_writes={}", totals.mapper_writes)?;
    writeln!(file, "total_apu_writes={}", totals.apu_writes)?;
    writeln!(file, "total_oam_dma={}", totals.oam_dma)?;
    writeln!(file, "total_ppu_writes={}", totals.ppu_writes)?;
    writeln!(file, "total_ppu_vram_writes={}", totals.ppu_vram_writes)?;
    writeln!(file, "total_block_count={}", totals.block_count)?;
    writeln!(file, "total_left_block={}", totals.left_block)?;
    writeln!(file, "total_step_limit={}", totals.step_limit)?;
    writeln!(
        file,
        "total_unsupported_opcode={}",
        totals.unsupported_opcode
    )?;
    writeln!(file, "total_invalid_block={}", totals.invalid_block)?;
    writeln!(file, "summary=coverage_summary.tsv")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn write(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_coverage_report_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_coverage_report() {
        let root = temp_dir();
        let build = root.join("build");
        let out = root.join("coverage");
        write(
            &build.join("trace/sample/trace_summary.txt"),
            "frames=12\nexecuted_label_count=3\nmapper_write_count=4\napu_write_count=5\noam_dma_count=6\nppu_write_count=6\nppu_vram_write_count=7\nreplay_sha256=0123456789abcdef\ncomplete=1\n",
        );
        write(
            &build.join("block_exec/sample/manifest.txt"),
            "block_count=3\nleft_block=2\nstep_limit=1\nunsupported_opcode=0\ninvalid_block=0\ncomplete=1\n",
        );

        run(&build, &out, &[String::from("sample")]).unwrap();

        let summary = fs::read_to_string(out.join("coverage_summary.tsv")).unwrap();
        assert!(summary.contains("replay\tframes\texecuted_labels\tmapper_writes\tapu_writes\toam_dma\tppu_writes\tppu_vram_writes\tblock_count"));
        assert!(summary.contains("sample\t12\t3\t4\t5\t6\t6\t7\t3\t2\t1\t0\t0\t0123456789abcdef\n"));
        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("total_oam_dma=6\n"));
        assert!(manifest.contains("total_ppu_writes=6\n"));
        assert!(manifest.contains("total_ppu_vram_writes=7\n"));
        assert!(manifest.contains("complete=1\n"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_invalid_blocks() {
        let root = temp_dir();
        let build = root.join("build");
        let out = root.join("coverage");
        write(
            &build.join("trace/sample/trace_summary.txt"),
            "frames=1\nexecuted_label_count=1\nmapper_write_count=0\napu_write_count=0\noam_dma_count=0\nppu_write_count=0\nppu_vram_write_count=0\nreplay_sha256=sha\ncomplete=1\n",
        );
        write(
            &build.join("block_exec/sample/manifest.txt"),
            "block_count=1\nleft_block=1\nstep_limit=0\nunsupported_opcode=0\ninvalid_block=1\ncomplete=1\n",
        );

        let err = run(&build, &out, &[String::from("sample")])
            .unwrap_err()
            .to_string();
        assert!(err.contains("invalid blocks remain: 1"));
        fs::remove_dir_all(root).unwrap();
    }
}
