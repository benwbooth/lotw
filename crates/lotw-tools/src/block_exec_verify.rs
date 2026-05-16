use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const BLOCK_EXEC_HEADER: &str = "id\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstatus\tsteps\tunsupported_opcode\tfinal_pc\tcycles\twrites\tppu_writes\tapu_writes\tmapper_writes\tunmapped_reads\tstate_applied\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256";
const BLOCK_CANDIDATE_HEADER: &str = "id\tcpu_addr\tprg_offset\tbytes\thit_count\tfirst_frame\tfirst_opcode\tstop_reason\trole\tcontrol_flow_ops\tterminator_opcode";
const EXTERNAL_WRITES_HEADER: &str =
    "id\tcpu_addr\tprg_offset\twrite_index\texternal_index\tkind\taddr\tvalue";

#[derive(Debug, Default)]
struct ExecStats {
    bad_header: u64,
    rows: u64,
    left_block: u64,
    stopped: u64,
    step_limit: u64,
    unsupported_opcode: u64,
    invalid_block: u64,
    state_applied_count: u64,
    total_writes: u64,
    total_ppu_writes: u64,
    total_apu_writes: u64,
    total_mapper_writes: u64,
    total_unmapped_reads: u64,
    final_ram_hash_rows: u64,
}

#[derive(Debug, Default)]
struct CandidateStats {
    bad_candidate_header: u64,
    candidate_rows: u64,
    candidate_exec_rows: u64,
    candidate_missing: u64,
    candidate_mismatches: u64,
    candidate_total_hit_count: u64,
}

#[derive(Debug, Default)]
struct ExternalStats {
    external_write_bad_header: u64,
    external_write_rows: u64,
    external_write_id_mismatch_rows: u64,
    external_write_malformed_rows: u64,
    external_ppu_rows: u64,
    external_apu_rows: u64,
    external_mapper_rows: u64,
    external_unknown_rows: u64,
}

pub fn run(block_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = block_dir.join("manifest.txt");
    let block_exec = block_dir.join("block_exec.tsv");
    let external_writes = block_dir.join("block_external_writes.tsv");
    let unsupported = block_dir.join("unsupported_opcodes.tsv");
    for path in [&manifest, &block_exec, &external_writes, &unsupported] {
        require_file(path)?;
    }

    let manifest_values = read_manifest(&manifest)?;
    let block_count = require_manifest_u64(&manifest_values, "block_count")?;
    let block_candidates = PathBuf::from(require_manifest(&manifest_values, "blocks")?);
    let manifest_left = require_manifest_u64(&manifest_values, "left_block")?;
    let manifest_stopped = require_manifest_u64(&manifest_values, "stopped")?;
    let manifest_step_limit = require_manifest_u64(&manifest_values, "step_limit")?;
    let manifest_unsupported = require_manifest_u64(&manifest_values, "unsupported_opcode")?;
    let manifest_invalid = require_manifest_u64(&manifest_values, "invalid_block")?;
    let manifest_external_write_rows =
        require_manifest_u64(&manifest_values, "external_write_rows")?;
    let manifest_external_write_alloc_failed =
        require_manifest_u64(&manifest_values, "external_write_alloc_failed")?;
    require_file(&block_candidates).map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "block_exec_verify: missing block candidates: {}",
                block_candidates.display()
            ),
        )
    })?;

    fs::create_dir_all(out_dir)?;
    let report = out_dir.join("block_exec_verify.txt");

    let exec_stats = read_exec_stats(&block_exec)?;
    let candidate_stats = read_candidate_stats(&block_candidates, &block_exec)?;
    let external_stats = read_external_stats(&block_exec, &external_writes)?;
    let unsupported_rows = count_unsupported_rows(&unsupported)?;

    write_report(
        &report,
        &block_exec,
        &block_candidates,
        &manifest,
        &exec_stats,
        &candidate_stats,
        &external_stats,
        manifest_external_write_alloc_failed,
        unsupported_rows,
    )?;

    let mut failures = Vec::new();
    if exec_stats.bad_header != 0 {
        failures.push("bad block execution header");
    }
    if exec_stats.rows != block_count {
        failures.push("row count mismatch");
    }
    if candidate_stats.bad_candidate_header != 0 {
        failures.push("bad candidate header");
    }
    if candidate_stats.candidate_rows != block_count {
        failures.push("candidate row count mismatch");
    }
    if candidate_stats.candidate_exec_rows != block_count {
        failures.push("candidate execution row count mismatch");
    }
    if candidate_stats.candidate_missing != 0 {
        failures.push("execution rows missing block candidates");
    }
    if candidate_stats.candidate_mismatches != 0 {
        failures.push("execution rows differ from block candidates");
    }
    if exec_stats.left_block != manifest_left {
        failures.push("left_block count mismatch");
    }
    if exec_stats.stopped != manifest_stopped {
        failures.push("stopped count mismatch");
    }
    if exec_stats.step_limit != manifest_step_limit {
        failures.push("step_limit count mismatch");
    }
    if exec_stats.unsupported_opcode != manifest_unsupported {
        failures.push("unsupported count mismatch");
    }
    if exec_stats.invalid_block != manifest_invalid {
        failures.push("invalid count mismatch");
    }
    if exec_stats.unsupported_opcode != 0 || manifest_unsupported != 0 || unsupported_rows != 0 {
        failures.push("unsupported opcodes remain");
    }
    if exec_stats.invalid_block != 0 || manifest_invalid != 0 {
        failures.push("invalid blocks remain");
    }
    if exec_stats.state_applied_count != block_count {
        failures.push("not every block used FCEUX first-hit state");
    }
    if exec_stats.final_ram_hash_rows != block_count {
        failures.push("not every block recorded final RAM hash");
    }
    if manifest_external_write_alloc_failed != 0 {
        failures.push("external write recording allocation failed");
    }
    if external_stats.external_write_bad_header != 0 {
        failures.push("bad external write header");
    }
    if external_stats.external_write_rows != manifest_external_write_rows {
        failures.push("external write row count mismatch");
    }
    if external_stats.external_write_rows
        != exec_stats.total_ppu_writes
            + exec_stats.total_apu_writes
            + exec_stats.total_mapper_writes
    {
        failures.push("external write rows do not match PPU/APU/mapper write totals");
    }
    if external_stats.external_write_id_mismatch_rows != 0 {
        failures.push("external write rows do not match per-block write totals");
    }
    if external_stats.external_write_malformed_rows != 0 {
        failures.push("malformed external write rows");
    }
    if external_stats.external_ppu_rows != exec_stats.total_ppu_writes {
        failures.push("PPU external write count mismatch");
    }
    if external_stats.external_apu_rows != exec_stats.total_apu_writes {
        failures.push("APU external write count mismatch");
    }
    if external_stats.external_mapper_rows != exec_stats.total_mapper_writes {
        failures.push("mapper external write count mismatch");
    }
    if external_stats.external_unknown_rows != 0 {
        failures.push("unknown external write kind");
    }

    if !failures.is_empty() {
        return Err(format!("block_exec_verify: {}", failures.join("; ")).into());
    }

    println!("block_exec_verify: wrote {}", report.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("block_exec_verify: missing input: {}", path.display()),
        ))
    }
}

fn read_manifest(path: &Path) -> io::Result<HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut values = HashMap::new();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.to_string(), value.to_string());
        }
    }
    Ok(values)
}

fn require_manifest<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values
        .get(key)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("block_exec_verify: missing manifest value: {key}").into())
}

fn require_manifest_u64(
    values: &HashMap<String, String>,
    key: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(require_manifest(values, key)?.parse::<u64>()?)
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn parse_u64(value: &str) -> u64 {
    value.parse::<u64>().unwrap_or(0)
}

fn read_exec_stats(path: &Path) -> io::Result<ExecStats> {
    let text = fs::read_to_string(path)?;
    let mut lines = text.lines();
    let mut stats = ExecStats::default();
    if lines.next() != Some(BLOCK_EXEC_HEADER) {
        stats.bad_header = 1;
        return Ok(stats);
    }
    for line in lines {
        let fields = split_tsv(line);
        if fields.len() < 22 {
            continue;
        }
        stats.rows += 1;
        match fields[5] {
            "left_block" => stats.left_block += 1,
            "stopped" => stats.stopped += 1,
            "step_limit" => stats.step_limit += 1,
            "unsupported_opcode" => stats.unsupported_opcode += 1,
            "invalid_block" => stats.invalid_block += 1,
            _ => {}
        }
        stats.total_writes += parse_u64(fields[10]);
        stats.total_ppu_writes += parse_u64(fields[11]);
        stats.total_apu_writes += parse_u64(fields[12]);
        stats.total_mapper_writes += parse_u64(fields[13]);
        stats.total_unmapped_reads += parse_u64(fields[14]);
        stats.state_applied_count += parse_u64(fields[15]);
        if is_lower_hex_sha256(fields[21]) {
            stats.final_ram_hash_rows += 1;
        }
    }
    Ok(stats)
}

fn read_candidate_stats(candidates: &Path, exec: &Path) -> io::Result<CandidateStats> {
    let mut stats = CandidateStats::default();
    let candidate_text = fs::read_to_string(candidates)?;
    let mut candidate_map = HashMap::<String, String>::new();
    let mut candidate_lines = candidate_text.lines();
    if candidate_lines.next() != Some(BLOCK_CANDIDATE_HEADER) {
        stats.bad_candidate_header = 1;
    }
    for line in candidate_lines {
        let fields = split_tsv(line);
        if fields.len() < 7 {
            continue;
        }
        stats.candidate_rows += 1;
        stats.candidate_total_hit_count += parse_u64(fields[4]);
        candidate_map.insert(
            fields[0].to_string(),
            format!("{}\t{}\t{}\t{}", fields[1], fields[2], fields[3], fields[6]),
        );
    }

    let exec_text = fs::read_to_string(exec)?;
    for line in exec_text.lines().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 5 {
            continue;
        }
        stats.candidate_exec_rows += 1;
        let expected = format!("{}\t{}\t{}\t{}", fields[1], fields[2], fields[3], fields[4]);
        match candidate_map.get(fields[0]) {
            Some(candidate) if candidate == &expected => {}
            Some(_) => stats.candidate_mismatches += 1,
            None => stats.candidate_missing += 1,
        }
    }
    Ok(stats)
}

fn read_external_stats(exec: &Path, external: &Path) -> io::Result<ExternalStats> {
    let exec_text = fs::read_to_string(exec)?;
    let mut expected = HashMap::<String, u64>::new();
    for line in exec_text.lines().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 14 {
            continue;
        }
        expected.insert(
            fields[0].to_string(),
            parse_u64(fields[11]) + parse_u64(fields[12]) + parse_u64(fields[13]),
        );
    }

    let external_text = fs::read_to_string(external)?;
    let mut lines = external_text.lines();
    let mut stats = ExternalStats::default();
    if lines.next() != Some(EXTERNAL_WRITES_HEADER) {
        stats.external_write_bad_header = 1;
    }
    let mut actual = HashMap::<String, u64>::new();
    let mut unknown_ids = 0;
    for line in lines {
        let fields = split_tsv(line);
        if fields.len() < 8 {
            stats.external_write_malformed_rows += 1;
            continue;
        }
        stats.external_write_rows += 1;
        *actual.entry(fields[0].to_string()).or_default() += 1;
        if !expected.contains_key(fields[0]) {
            unknown_ids += 1;
        }
        match fields[5] {
            "ppu" => stats.external_ppu_rows += 1,
            "apu" => stats.external_apu_rows += 1,
            "mapper" => stats.external_mapper_rows += 1,
            _ => stats.external_unknown_rows += 1,
        }
        if !is_decimal(fields[3])
            || !is_decimal(fields[4])
            || !is_upper_hex_width(fields[6], 4)
            || !is_upper_hex_width(fields[7], 2)
        {
            stats.external_write_malformed_rows += 1;
        }
    }
    let mut id_mismatches = 0;
    for (id, expected_count) in expected {
        if actual.get(&id).copied().unwrap_or(0) != expected_count {
            id_mismatches += 1;
        }
    }
    stats.external_write_id_mismatch_rows = id_mismatches + unknown_ids;
    Ok(stats)
}

fn count_unsupported_rows(path: &Path) -> io::Result<u64> {
    let text = fs::read_to_string(path)?;
    Ok(text
        .lines()
        .skip(1)
        .filter(|line| !line.split_whitespace().collect::<Vec<_>>().is_empty())
        .count() as u64)
}

#[allow(clippy::too_many_arguments)]
fn write_report(
    path: &Path,
    block_exec: &Path,
    block_candidates: &Path,
    manifest: &Path,
    exec: &ExecStats,
    candidates: &CandidateStats,
    external: &ExternalStats,
    external_write_alloc_failed: u64,
    unsupported_rows: u64,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "block_exec={}", block_exec.display())?;
    writeln!(file, "block_candidates={}", block_candidates.display())?;
    writeln!(file, "manifest={}", manifest.display())?;
    if exec.bad_header != 0 {
        writeln!(file, "bad_header={}", exec.bad_header)?;
    }
    writeln!(file, "rows={}", exec.rows)?;
    writeln!(file, "left_block={}", exec.left_block)?;
    writeln!(file, "stopped={}", exec.stopped)?;
    writeln!(file, "step_limit={}", exec.step_limit)?;
    writeln!(file, "unsupported_opcode={}", exec.unsupported_opcode)?;
    writeln!(file, "invalid_block={}", exec.invalid_block)?;
    writeln!(file, "state_applied_count={}", exec.state_applied_count)?;
    writeln!(file, "total_writes={}", exec.total_writes)?;
    writeln!(file, "total_ppu_writes={}", exec.total_ppu_writes)?;
    writeln!(file, "total_apu_writes={}", exec.total_apu_writes)?;
    writeln!(file, "total_mapper_writes={}", exec.total_mapper_writes)?;
    writeln!(file, "total_unmapped_reads={}", exec.total_unmapped_reads)?;
    writeln!(file, "final_ram_hash_rows={}", exec.final_ram_hash_rows)?;
    writeln!(
        file,
        "bad_candidate_header={}",
        candidates.bad_candidate_header
    )?;
    writeln!(file, "candidate_rows={}", candidates.candidate_rows)?;
    writeln!(
        file,
        "candidate_exec_rows={}",
        candidates.candidate_exec_rows
    )?;
    writeln!(file, "candidate_missing={}", candidates.candidate_missing)?;
    writeln!(
        file,
        "candidate_mismatches={}",
        candidates.candidate_mismatches
    )?;
    writeln!(
        file,
        "candidate_total_hit_count={}",
        candidates.candidate_total_hit_count
    )?;
    writeln!(
        file,
        "external_write_bad_header={}",
        external.external_write_bad_header
    )?;
    writeln!(file, "external_write_rows={}", external.external_write_rows)?;
    writeln!(
        file,
        "external_write_id_mismatch_rows={}",
        external.external_write_id_mismatch_rows
    )?;
    writeln!(
        file,
        "external_write_malformed_rows={}",
        external.external_write_malformed_rows
    )?;
    writeln!(file, "external_ppu_rows={}", external.external_ppu_rows)?;
    writeln!(file, "external_apu_rows={}", external.external_apu_rows)?;
    writeln!(
        file,
        "external_mapper_rows={}",
        external.external_mapper_rows
    )?;
    writeln!(
        file,
        "external_unknown_rows={}",
        external.external_unknown_rows
    )?;
    writeln!(
        file,
        "external_write_alloc_failed={external_write_alloc_failed}"
    )?;
    writeln!(file, "unsupported_opcode_rows={unsupported_rows}")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .chars()
            .all(|ch| ch.is_ascii_digit() || ('a'..='f').contains(&ch))
}

fn is_decimal(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit())
}

fn is_upper_hex_width(value: &str, width: usize) -> bool {
    value.len() == width
        && value
            .chars()
            .all(|ch| ch.is_ascii_digit() || ('A'..='F').contains(&ch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_block_exec_verify_report() {
        let root = std::env::temp_dir().join(format!(
            "lotw_rust_block_exec_verify_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let block = root.join("block_exec");
        let out = root.join("verify");
        fs::create_dir_all(&block).unwrap();
        fs::write(
            block.join("manifest.txt"),
            format!(
                "blocks={}\nblock_count=2\nleft_block=1\nstopped=0\nunsupported_opcode=0\nstep_limit=1\ninvalid_block=0\nexternal_write_rows=2\nexternal_write_alloc_failed=0\nscope=tooling-only semantic block execution\ncomplete=1\n",
                block.join("block_candidates.tsv").display()
            ),
        )
        .unwrap();
        fs::write(
            block.join("block_candidates.tsv"),
            "id\tcpu_addr\tprg_offset\tbytes\thit_count\tfirst_frame\tfirst_opcode\tstop_reason\trole\tcontrol_flow_ops\tterminator_opcode\n\
             0\t8000\t1C000\t3\t4\t1\tA9\tnext_trace_label\tsmoke\t0\t00\n\
             1\t8003\t1C003\t2\t5\t2\tD0\tterminator_60\tsmoke\t2\t60\n",
        )
        .unwrap();
        fs::write(
            block.join("block_exec.tsv"),
            "id\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstatus\tsteps\tunsupported_opcode\tfinal_pc\tcycles\twrites\tppu_writes\tapu_writes\tmapper_writes\tunmapped_reads\tstate_applied\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
             0\t8000\t1C000\t3\tA9\tleft_block\t1\t\t8003\t2\t0\t0\t0\t0\t0\t1\t01\t02\t03\t24\tFD\t0000000000000000000000000000000000000000000000000000000000000000\n\
             1\t8003\t1C003\t2\tD0\tstep_limit\t64\t\t8003\t128\t2\t1\t0\t1\t0\t1\t04\t05\t06\tA4\tFA\t1111111111111111111111111111111111111111111111111111111111111111\n",
        )
        .unwrap();
        fs::write(
            block.join("block_external_writes.tsv"),
            "id\tcpu_addr\tprg_offset\twrite_index\texternal_index\tkind\taddr\tvalue\n\
             1\t8003\t1C003\t1\t1\tppu\t2000\t80\n\
             1\t8003\t1C003\t2\t2\tmapper\t8000\t01\n",
        )
        .unwrap();
        fs::write(block.join("unsupported_opcodes.tsv"), "opcode\tcount\n").unwrap();

        run(&block, &out).unwrap();

        let report = fs::read_to_string(out.join("block_exec_verify.txt")).unwrap();
        assert!(report.contains("rows=2\n"));
        assert!(report.contains("candidate_total_hit_count=9\n"));
        assert!(report.contains("external_write_rows=2\n"));
        assert!(report.contains("complete=1\n"));
        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
