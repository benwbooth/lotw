use lotw_port::sha256;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const RUNTIME_TRACE_HEADER: &str = "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tfirst_frame\tinitial_pc\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\toracle_final_pc\toracle_cycles\toracle_final_a\toracle_final_x\toracle_final_y\toracle_final_p\toracle_final_s\toracle_final_ram_sha256\tstate_match";
const LABEL_STATES_HEADER: &str =
    "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff";
const EXPECTED_EXTERNAL_HEADER: &str = "kind\tframe\taddr\tvalue";

#[derive(Debug, Clone)]
struct TraceRow {
    frame: String,
    pc: String,
    a: String,
    x: String,
    y: String,
    p: String,
    s: String,
    ram_hash: String,
}

#[derive(Debug, Default)]
struct Stats {
    trace_bad_header: u64,
    label_bad_header: u64,
    trace_rows: u64,
    label_rows: u64,
    trace_state_mismatch_rows: u64,
    extra_label_rows: u64,
    label_mismatch_rows: u64,
    label_ram_length_mismatch_rows: u64,
    ram_hash_mismatch_rows: u64,
    missing_hash_rows: u64,
}

#[derive(Debug, Default)]
struct ExternalStats {
    blank_external_trace_files: u64,
    expected_external_bad_header: u64,
    expected_external_rows: u64,
    expected_ppu_write_count: u64,
    expected_apu_write_count: u64,
    expected_mapper_write_count: u64,
    actual_ppu_write_count: u64,
    actual_apu_write_count: u64,
    actual_mapper_write_count: u64,
    external_trace_mismatches: u64,
}

pub fn run(
    run_dir: &Path,
    trace_dir: &Path,
    out_dir: &Path,
    expected_runtime: &str,
    expected_external: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = run_dir.join("manifest.txt");
    let runtime_trace = run_dir.join("native_block_runtime_trace.tsv");
    let summary = trace_dir.join("port_trace_summary.txt");
    let label_states = trace_dir.join("port_label_states.tsv");
    for path in [&manifest, &runtime_trace, &summary, &label_states] {
        require_file(path)?;
    }
    if let Some(path) = expected_external {
        if !path.is_file() {
            return Err(format!(
                "runtime_native_trace_verify: missing expected external write file: {}",
                path.display()
            )
            .into());
        }
    }

    let mapper_writes = trace_dir.join("port_mapper_writes.tsv");
    let apu_writes = trace_dir.join("port_apu_writes.tsv");
    let oam_dma = trace_dir.join("port_oam_dma.tsv");
    let ppu_writes = trace_dir.join("port_ppu_writes.tsv");
    let ppu_vram_writes = trace_dir.join("port_ppu_vram_writes.tsv");
    for path in [
        &mapper_writes,
        &apu_writes,
        &oam_dma,
        &ppu_writes,
        &ppu_vram_writes,
    ] {
        require_port_trace_file(path)?;
    }

    fs::create_dir_all(out_dir)?;
    let report = out_dir.join("runtime_native_trace_verify.txt");
    let manifest_values = read_key_values(&manifest)?;
    let summary_values = read_key_values(&summary)?;
    let case_count = require_value(&manifest_values, "case_count", &manifest)?;
    let matched = require_value(&manifest_values, "matched", &manifest)?;
    let mismatches = require_value(&manifest_values, "mismatches", &manifest)?;
    let runtime = require_value(&summary_values, "runtime", &summary)?;
    let label_state_count = require_value(&summary_values, "label_state_count", &summary)?;
    let mapper_count = require_value(&summary_values, "mapper_write_count", &summary)?;
    let apu_count = require_value(&summary_values, "apu_write_count", &summary)?;
    let oam_dma_count = require_value(&summary_values, "oam_dma_count", &summary)?;
    let ppu_count = require_value(&summary_values, "ppu_write_count", &summary)?;
    let ppu_vram_count = require_value(&summary_values, "ppu_vram_write_count", &summary)?;

    let stats = collect_stats(&runtime_trace, &label_states)?;
    let external_stats = collect_external_stats(
        trace_dir,
        expected_external,
        [
            &mapper_writes,
            &apu_writes,
            &oam_dma,
            &ppu_writes,
            &ppu_vram_writes,
        ],
    )?;

    let values = ReportValues {
        run_dir,
        trace_dir,
        expected_external,
        case_count,
        matched,
        mismatches,
        runtime,
        expected_runtime,
        label_state_count,
        mapper_count,
        apu_count,
        oam_dma_count,
        ppu_count,
        ppu_vram_count,
    };
    write_report(&report, &values, &stats, &external_stats)?;
    validate(&values, &stats, &external_stats)?;

    println!("runtime_native_trace_verify: wrote {}", report.display());
    Ok(())
}

struct ReportValues<'a> {
    run_dir: &'a Path,
    trace_dir: &'a Path,
    expected_external: Option<&'a Path>,
    case_count: &'a str,
    matched: &'a str,
    mismatches: &'a str,
    runtime: &'a str,
    expected_runtime: &'a str,
    label_state_count: &'a str,
    mapper_count: &'a str,
    apu_count: &'a str,
    oam_dma_count: &'a str,
    ppu_count: &'a str,
    ppu_vram_count: &'a str,
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "runtime_native_trace_verify: missing input: {}",
                path.display()
            ),
        ))
    }
}

fn require_port_trace_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "runtime_native_trace_verify: missing port trace file: {}",
                path.display()
            ),
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

fn require_value<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values.get(key).map(String::as_str).ok_or_else(|| {
        format!(
            "runtime_native_trace_verify: missing {key} in {}",
            path.display()
        )
        .into()
    })
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

fn collect_stats(runtime_trace: &Path, label_states: &Path) -> io::Result<Stats> {
    let (trace_rows, mut stats) = read_runtime_trace(runtime_trace)?;
    read_label_states(label_states, &trace_rows, &mut stats)?;
    Ok(stats)
}

fn read_runtime_trace(runtime_trace: &Path) -> io::Result<(Vec<TraceRow>, Stats)> {
    let text = fs::read_to_string(runtime_trace)?;
    let mut lines = text.lines();
    let mut stats = Stats::default();
    if lines.next().unwrap_or("") != RUNTIME_TRACE_HEADER {
        stats.trace_bad_header = 1;
    }

    let mut rows = Vec::new();
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 23 {
            return invalid_tsv(runtime_trace, line_no + 2, fields.len(), 23);
        }
        stats.trace_rows += 1;
        if fields[22] != "1" {
            stats.trace_state_mismatch_rows += 1;
        }
        rows.push(TraceRow {
            frame: fields[4].to_string(),
            pc: fields[6].to_string(),
            a: fields[8].to_string(),
            x: fields[9].to_string(),
            y: fields[10].to_string(),
            p: fields[11].to_string(),
            s: fields[12].to_string(),
            ram_hash: fields[13].to_string(),
        });
    }
    Ok((rows, stats))
}

fn read_label_states(
    label_states: &Path,
    trace_rows: &[TraceRow],
    stats: &mut Stats,
) -> io::Result<()> {
    let text = fs::read_to_string(label_states)?;
    let mut lines = text.lines();
    if lines.next().unwrap_or("") != LABEL_STATES_HEADER {
        stats.label_bad_header = 1;
    }

    let mut label_ram_count = 0usize;
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 10 {
            return invalid_tsv(label_states, line_no + 2, fields.len(), 10);
        }
        stats.label_rows += 1;
        match trace_rows.get(line_no) {
            Some(trace) => {
                if fields[0] != trace.pc
                    || fields[2] != trace.frame
                    || fields[3] != trace.pc
                    || fields[4] != trace.a
                    || fields[5] != trace.x
                    || fields[6] != trace.y
                    || fields[7] != trace.p
                    || fields[8] != trace.s
                {
                    stats.label_mismatch_rows += 1;
                }
                if fields[9].len() != 4096 {
                    stats.label_ram_length_mismatch_rows += 1;
                }
                let actual_hash = sha256::digest_hex(&hex_to_bytes(fields[9]).map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "{}:{} invalid RAM hex: {err}",
                            label_states.display(),
                            line_no + 2
                        ),
                    )
                })?);
                if actual_hash != trace.ram_hash {
                    stats.ram_hash_mismatch_rows += 1;
                }
            }
            None => {
                stats.extra_label_rows += 1;
                stats.missing_hash_rows += 1;
            }
        }
        label_ram_count += 1;
    }
    if trace_rows.len() > label_ram_count {
        stats.missing_hash_rows += 1;
    }
    Ok(())
}

fn hex_to_bytes(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) {
        return Err("odd length".to_string());
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    let raw = value.as_bytes();
    for chunk in raw.chunks_exact(2) {
        let hi = hex_nibble(chunk[0])?;
        let lo = hex_nibble(chunk[1])?;
        bytes.push((hi << 4) | lo);
    }
    Ok(bytes)
}

fn hex_nibble(value: u8) -> Result<u8, String> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(format!("invalid hex byte {}", value as char)),
    }
}

fn collect_external_stats(
    trace_dir: &Path,
    expected_external: Option<&Path>,
    external_trace_files: [&Path; 5],
) -> Result<ExternalStats, Box<dyn std::error::Error>> {
    let mut stats = ExternalStats {
        blank_external_trace_files: if external_trace_files
            .iter()
            .all(|path| line_count(path).unwrap_or(usize::MAX) == 1)
        {
            1
        } else {
            0
        },
        ..ExternalStats::default()
    };

    let Some(expected_external) = expected_external else {
        return Ok(stats);
    };

    let expected = read_expected_external(expected_external, &mut stats)?;
    let actual_ppu = read_actual_ppu(&trace_dir.join("port_ppu_writes.tsv"))?;
    let actual_apu = read_actual_apu(&trace_dir.join("port_apu_writes.tsv"))?;
    let actual_mapper = read_actual_mapper(&trace_dir.join("port_mapper_writes.tsv"))?;

    stats.expected_external_rows =
        (expected.ppu.len() + expected.apu.len() + expected.mapper.len()) as u64;
    stats.expected_ppu_write_count = expected.ppu.len() as u64;
    stats.expected_apu_write_count = expected.apu.len() as u64;
    stats.expected_mapper_write_count = expected.mapper.len() as u64;
    stats.actual_ppu_write_count = actual_ppu.len() as u64;
    stats.actual_apu_write_count = actual_apu.len() as u64;
    stats.actual_mapper_write_count = actual_mapper.len() as u64;
    if expected.ppu != actual_ppu {
        stats.external_trace_mismatches += 1;
    }
    if expected.apu != actual_apu {
        stats.external_trace_mismatches += 1;
    }
    if expected.mapper != actual_mapper {
        stats.external_trace_mismatches += 1;
    }
    Ok(stats)
}

#[derive(Debug, Default)]
struct ExpectedExternal {
    ppu: Vec<String>,
    apu: Vec<String>,
    mapper: Vec<String>,
}

fn read_expected_external(path: &Path, stats: &mut ExternalStats) -> io::Result<ExpectedExternal> {
    let text = fs::read_to_string(path)?;
    let mut lines = text.lines();
    if lines.next().unwrap_or("") != EXPECTED_EXTERNAL_HEADER {
        stats.expected_external_bad_header = 1;
    }
    let mut expected = ExpectedExternal::default();
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 4 {
            return invalid_tsv(path, line_no + 2, fields.len(), 4);
        }
        let normalized = format!(
            "{}\t{}\t{}",
            fields[1],
            fields[2].to_ascii_uppercase(),
            fields[3].to_ascii_uppercase()
        );
        match fields[0] {
            "ppu" => expected.ppu.push(normalized),
            "apu" => expected.apu.push(normalized),
            "mapper" => expected.mapper.push(normalized),
            _ => {}
        }
    }
    Ok(expected)
}

fn read_actual_ppu(path: &Path) -> io::Result<Vec<String>> {
    read_actual_rows(path, 5, |fields| {
        format!(
            "{}\t{}\t{}",
            fields[0],
            fields[2].to_ascii_uppercase(),
            fields[4].to_ascii_uppercase()
        )
    })
}

fn read_actual_apu(path: &Path) -> io::Result<Vec<String>> {
    read_actual_rows(path, 4, |fields| {
        format!(
            "{}\t{}\t{}",
            fields[0],
            fields[2].to_ascii_uppercase(),
            fields[3].to_ascii_uppercase()
        )
    })
}

fn read_actual_mapper(path: &Path) -> io::Result<Vec<String>> {
    read_actual_rows(path, 3, |fields| {
        format!(
            "{}\t{}\t{}",
            fields[0],
            fields[1].to_ascii_uppercase(),
            fields[2].to_ascii_uppercase()
        )
    })
}

fn read_actual_rows<F>(path: &Path, min_fields: usize, normalize: F) -> io::Result<Vec<String>>
where
    F: Fn(&[&str]) -> String,
{
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < min_fields {
            return invalid_tsv(path, line_no + 1, fields.len(), min_fields);
        }
        rows.push(normalize(&fields));
    }
    Ok(rows)
}

fn line_count(path: &Path) -> io::Result<usize> {
    Ok(fs::read_to_string(path)?.lines().count())
}

fn write_report(
    report: &Path,
    values: &ReportValues<'_>,
    stats: &Stats,
    external: &ExternalStats,
) -> io::Result<()> {
    let mut file = fs::File::create(report)?;
    writeln!(file, "native_block_run={}", values.run_dir.display())?;
    writeln!(file, "port_trace={}", values.trace_dir.display())?;
    writeln!(
        file,
        "expected_external_trace={}",
        values
            .expected_external
            .map(|path| path.display().to_string())
            .unwrap_or_default()
    )?;
    writeln!(file, "manifest_case_count={}", values.case_count)?;
    writeln!(file, "manifest_matched={}", values.matched)?;
    writeln!(file, "manifest_mismatches={}", values.mismatches)?;
    writeln!(file, "port_runtime={}", values.runtime)?;
    writeln!(file, "expected_runtime={}", values.expected_runtime)?;
    writeln!(file, "port_label_state_count={}", values.label_state_count)?;
    writeln!(file, "port_mapper_write_count={}", values.mapper_count)?;
    writeln!(file, "port_apu_write_count={}", values.apu_count)?;
    writeln!(file, "port_oam_dma_count={}", values.oam_dma_count)?;
    writeln!(file, "port_ppu_write_count={}", values.ppu_count)?;
    writeln!(file, "port_ppu_vram_write_count={}", values.ppu_vram_count)?;
    writeln!(file, "trace_bad_header={}", stats.trace_bad_header)?;
    writeln!(file, "label_bad_header={}", stats.label_bad_header)?;
    writeln!(file, "trace_rows={}", stats.trace_rows)?;
    writeln!(file, "label_rows={}", stats.label_rows)?;
    writeln!(
        file,
        "trace_state_mismatch_rows={}",
        stats.trace_state_mismatch_rows
    )?;
    writeln!(file, "extra_label_rows={}", stats.extra_label_rows)?;
    writeln!(file, "label_mismatch_rows={}", stats.label_mismatch_rows)?;
    writeln!(
        file,
        "label_ram_length_mismatch_rows={}",
        stats.label_ram_length_mismatch_rows
    )?;
    writeln!(
        file,
        "ram_hash_mismatch_rows={}",
        stats.ram_hash_mismatch_rows
    )?;
    writeln!(file, "missing_hash_rows={}", stats.missing_hash_rows)?;
    writeln!(
        file,
        "blank_external_trace_files={}",
        external.blank_external_trace_files
    )?;
    writeln!(
        file,
        "expected_external_bad_header={}",
        external.expected_external_bad_header
    )?;
    writeln!(
        file,
        "expected_external_rows={}",
        external.expected_external_rows
    )?;
    writeln!(
        file,
        "expected_ppu_write_count={}",
        external.expected_ppu_write_count
    )?;
    writeln!(
        file,
        "expected_apu_write_count={}",
        external.expected_apu_write_count
    )?;
    writeln!(
        file,
        "expected_mapper_write_count={}",
        external.expected_mapper_write_count
    )?;
    writeln!(
        file,
        "actual_ppu_write_count={}",
        external.actual_ppu_write_count
    )?;
    writeln!(
        file,
        "actual_apu_write_count={}",
        external.actual_apu_write_count
    )?;
    writeln!(
        file,
        "actual_mapper_write_count={}",
        external.actual_mapper_write_count
    )?;
    writeln!(
        file,
        "external_trace_mismatches={}",
        external.external_trace_mismatches
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn validate(
    values: &ReportValues<'_>,
    stats: &Stats,
    external: &ExternalStats,
) -> Result<(), Box<dyn std::error::Error>> {
    let case_count = parse_u64(values.case_count, "case_count")?;
    let matched = parse_u64(values.matched, "matched")?;
    let mismatches = parse_u64(values.mismatches, "mismatches")?;
    let label_state_count = parse_u64(values.label_state_count, "label_state_count")?;
    let mapper_count = parse_u64(values.mapper_count, "mapper_write_count")?;
    let apu_count = parse_u64(values.apu_count, "apu_write_count")?;
    let oam_dma_count = parse_u64(values.oam_dma_count, "oam_dma_count")?;
    let ppu_count = parse_u64(values.ppu_count, "ppu_write_count")?;
    let ppu_vram_count = parse_u64(values.ppu_vram_count, "ppu_vram_write_count")?;

    let mut failures = Vec::new();
    if values.runtime != values.expected_runtime || label_state_count != case_count {
        failures.push("trace summary mismatch");
    }
    if values.expected_external.is_some() {
        if external.expected_external_bad_header != 0 {
            failures.push("bad expected external write header");
        }
        if ppu_count != external.expected_ppu_write_count
            || apu_count != external.expected_apu_write_count
            || mapper_count != external.expected_mapper_write_count
            || external.actual_ppu_write_count != external.expected_ppu_write_count
            || external.actual_apu_write_count != external.expected_apu_write_count
            || external.actual_mapper_write_count != external.expected_mapper_write_count
            || oam_dma_count != 0
            || ppu_vram_count != 0
        {
            failures.push("external write trace count mismatch");
        }
        if external.external_trace_mismatches != 0 {
            failures.push("external write trace rows do not match expected rows");
        }
    } else if mapper_count != 0
        || apu_count != 0
        || oam_dma_count != 0
        || ppu_count != 0
        || ppu_vram_count != 0
    {
        failures.push("unexpected external write trace rows");
    }
    if matched != case_count || mismatches != 0 {
        failures.push("native run manifest mismatch");
    }
    if stats.trace_bad_header != 0 || stats.label_bad_header != 0 {
        failures.push("bad header");
    }
    if stats.trace_rows != case_count || stats.label_rows != case_count {
        failures.push("row count mismatch");
    }
    if stats.trace_state_mismatch_rows != 0
        || stats.extra_label_rows != 0
        || stats.label_mismatch_rows != 0
    {
        failures.push("label state rows do not match runtime trace");
    }
    if stats.label_ram_length_mismatch_rows != 0
        || stats.ram_hash_mismatch_rows != 0
        || stats.missing_hash_rows != 0
    {
        failures.push("label state RAM does not match runtime trace");
    }
    if values.expected_external.is_none() && external.blank_external_trace_files != 1 {
        failures.push("expected blank external trace files");
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!("runtime_native_trace_verify: {}", failures.join("; ")).into())
    }
}

fn parse_u64(value: &str, name: &str) -> Result<u64, Box<dyn std::error::Error>> {
    value
        .parse::<u64>()
        .map_err(|err| format!("runtime_native_trace_verify: invalid {name}: {err}").into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_blank_and_external_trace_reports() {
        let root = std::env::temp_dir().join(format!(
            "lotw_runtime_native_trace_verify_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let run_dir = root.join("run");
        let trace_dir = root.join("trace");
        let verify_dir = root.join("verify");
        let external_verify_dir = root.join("verify_external");
        let expected_external = root.join("expected_external_writes.tsv");
        fs::create_dir_all(&run_dir).unwrap();
        fs::create_dir_all(&trace_dir).unwrap();

        let zero_ram = "0".repeat(4096);
        let zero_sha = sha256::digest_hex(&vec![0u8; 2048]);
        write_base_inputs(&run_dir, &trace_dir, &zero_ram, &zero_sha);

        run(
            &run_dir,
            &trace_dir,
            &verify_dir,
            "lotw_runtime_native_blocks",
            None,
        )
        .unwrap();
        let report =
            fs::read_to_string(verify_dir.join("runtime_native_trace_verify.txt")).unwrap();
        assert!(report.contains("complete=1\n"));
        assert!(report.contains("manifest_case_count=2\n"));
        assert!(report.contains("port_label_state_count=2\n"));
        assert!(report.contains("label_rows=2\n"));
        assert!(report.contains("ram_hash_mismatch_rows=0\n"));
        assert!(report.contains("blank_external_trace_files=1\n"));

        fs::write(
            trace_dir.join("port_trace_summary.txt"),
            "runtime=lotw_runtime_native_blocks\n\
             frames=8\n\
             mapper_write_count=1\n\
             apu_write_count=1\n\
             ppu_write_count=1\n\
             ppu_vram_write_count=0\n\
             oam_dma_count=0\n\
             label_state_count=2\n\
             complete=1\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n8\t8000\t06\tnative\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_apu_writes.tsv"),
            "frame\tcycle\taddr\tvalue\n8\tnative\t4008\t00\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n7\tnative\t2000\tPPUCTRL\t80\n",
        )
        .unwrap();
        fs::write(
            &expected_external,
            "kind\tframe\taddr\tvalue\nppu\t7\t2000\t80\napu\t8\t4008\t00\nmapper\t8\t8000\t06\n",
        )
        .unwrap();

        run(
            &run_dir,
            &trace_dir,
            &external_verify_dir,
            "lotw_runtime_native_blocks",
            Some(&expected_external),
        )
        .unwrap();
        let report =
            fs::read_to_string(external_verify_dir.join("runtime_native_trace_verify.txt"))
                .unwrap();
        assert!(report.contains("expected_external_rows=3\n"));
        assert!(report.contains("expected_ppu_write_count=1\n"));
        assert!(report.contains("expected_apu_write_count=1\n"));
        assert!(report.contains("expected_mapper_write_count=1\n"));
        assert!(report.contains("external_trace_mismatches=0\n"));

        let _ = fs::remove_dir_all(root);
    }

    fn write_base_inputs(run_dir: &Path, trace_dir: &Path, zero_ram: &str, zero_sha: &str) {
        fs::write(
            run_dir.join("manifest.txt"),
            "runtime=lotw_runtime_native_blocks\n\
             cases=/tmp/native_block_run_cases.tsv\n\
             run_report=native_block_run.tsv\n\
             runtime_trace=native_block_runtime_trace.tsv\n\
             case_count=2\n\
             matched=2\n\
             mismatches=0\n\
             scope=pc-dispatched generated native block runs and translated-native final RAM/register trace versus block-exec oracle\n\
             complete=1\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            format!(
                "{RUNTIME_TRACE_HEADER}\n\
                 smoke\t2\t2\t0,1\t7\tC000\tC010\t11\t01\t02\t03\t24\tF8\t{zero_sha}\tC010\t11\t01\t02\t03\t24\tF8\t{zero_sha}\t1\n\
                 smoke\t3\t3\t0,1,2\t8\tC010\tC020\t17\t04\t05\t06\t25\tF7\t{zero_sha}\tC020\t17\t04\t05\t06\t25\tF7\t{zero_sha}\t1\n"
            ),
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_trace_summary.txt"),
            "runtime=lotw_runtime_native_blocks\n\
             frames=8\n\
             mapper_write_count=0\n\
             apu_write_count=0\n\
             ppu_write_count=0\n\
             ppu_vram_write_count=0\n\
             oam_dma_count=0\n\
             label_state_count=2\n\
             complete=1\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_label_states.tsv"),
            format!(
                "{LABEL_STATES_HEADER}\n\
                 C010\t1C010\t7\tC010\t01\t02\t03\t24\tF8\t{zero_ram}\n\
                 C020\t1C020\t8\tC020\t04\t05\t06\t25\tF7\t{zero_ram}\n"
            ),
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_apu_writes.tsv"),
            "frame\tcycle\taddr\tvalue\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_oam_dma.tsv"),
            "frame\tcycle\tpage\tbytes_0000_00ff\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n",
        )
        .unwrap();
        fs::write(
            trace_dir.join("port_ppu_vram_writes.tsv"),
            "frame\tcycle\taddr\tregion\tvalue\n",
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
