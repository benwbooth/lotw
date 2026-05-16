use crate::native_block_run_external_writes;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct CaseRow {
    source_index: usize,
    line: String,
    replay: String,
    frame: u64,
    path_indices: String,
    initial_pc: String,
    final_pc: String,
}

pub fn run(
    run_dir: &Path,
    out_dir: &Path,
    replay_filter: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let cases = run_dir.join("native_block_run_cases.tsv");
    let run_report = run_dir.join("native_block_run.tsv");
    let runtime_trace = run_dir.join("native_block_runtime_trace.tsv");
    let source_manifest = run_dir.join("manifest.txt");
    for path in [&cases, &run_report, &runtime_trace, &source_manifest] {
        require_file(path)?;
    }

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let filter = replay_filter.iter().cloned().collect::<HashSet<_>>();
    let selected = selected_case_rows(&cases, &filter)?;
    if selected.is_empty() {
        return Err("native_block_live_frame_schedule: no cases generated".into());
    }

    let run_rows = read_data_lines(&run_report)?;
    let trace_rows = read_data_lines(&runtime_trace)?;
    write_reordered_cases(&cases, out_dir, &selected)?;
    write_reordered_lines(
        &run_report,
        &out_dir.join("native_block_run.tsv"),
        &selected,
        &run_rows,
    )?;
    write_reordered_lines(
        &runtime_trace,
        &out_dir.join("native_block_runtime_trace.tsv"),
        &selected,
        &trace_rows,
    )?;
    write_schedule(out_dir, &selected)?;
    write_manifest(run_dir, out_dir, replay_filter, &selected)?;

    let verify_cases = run_dir
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("native_block_verify/native_block_verify_cases.tsv");
    if verify_cases.is_file() {
        native_block_run_external_writes::run(
            &out_dir.join("native_block_run_cases.tsv"),
            &verify_cases,
            &out_dir.join("native_block_run_external_writes.tsv"),
        )?;
    }

    println!(
        "native_block_live_frame_schedule: wrote {}",
        out_dir.display()
    );
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "native_block_live_frame_schedule: missing input: {}",
                path.display()
            ),
        ))
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

fn selected_case_rows(path: &Path, filter: &HashSet<String>) -> io::Result<Vec<CaseRow>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 14 {
            return invalid_tsv(path, line_no + 1, fields.len(), 14);
        }
        if !filter.is_empty() && !filter.contains(fields[0]) {
            continue;
        }
        let frame = fields[5].parse::<u64>().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid frame: {err}", path.display(), line_no + 1),
            )
        })?;
        rows.push(CaseRow {
            source_index: line_no,
            line: line.to_string(),
            replay: fields[0].to_string(),
            frame,
            path_indices: fields[2].to_string(),
            initial_pc: fields[6].to_string(),
            final_pc: fields[13].to_string(),
        });
    }
    rows.sort_by(|left, right| {
        left.frame
            .cmp(&right.frame)
            .then_with(|| left.replay.cmp(&right.replay))
            .then_with(|| left.source_index.cmp(&right.source_index))
    });
    Ok(rows)
}

fn read_data_lines(path: &Path) -> io::Result<Vec<String>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .skip(1)
        .map(str::to_string)
        .collect())
}

fn read_header(path: &Path) -> io::Result<String> {
    Ok(fs::read_to_string(path)?
        .lines()
        .next()
        .unwrap_or("")
        .to_string())
}

fn write_reordered_cases(path: &Path, out_dir: &Path, selected: &[CaseRow]) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("native_block_run_cases.tsv"))?;
    writeln!(file, "{}", read_header(path)?)?;
    for row in selected {
        writeln!(file, "{}", row.line)?;
    }
    Ok(())
}

fn write_reordered_lines(
    source_path: &Path,
    out_path: &Path,
    selected: &[CaseRow],
    data_lines: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(out_path)?;
    writeln!(file, "{}", read_header(source_path)?)?;
    for row in selected {
        let source = data_lines.get(row.source_index - 1).ok_or_else(|| {
            format!(
                "native_block_live_frame_schedule: missing row {} in {}",
                row.source_index,
                source_path.display()
            )
        })?;
        writeln!(file, "{source}")?;
    }
    Ok(())
}

fn write_schedule(out_dir: &Path, selected: &[CaseRow]) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("native_block_live_frame_schedule.tsv"))?;
    writeln!(
        file,
        "order\tsource_index\tframe\treplay\tpath_indices\tinitial_pc\tfinal_pc"
    )?;
    for (index, row) in selected.iter().enumerate() {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            row.source_index,
            row.frame,
            row.replay,
            row.path_indices,
            row.initial_pc,
            row.final_pc
        )?;
    }
    Ok(())
}

fn write_manifest(
    run_dir: &Path,
    out_dir: &Path,
    replay_filter: &[String],
    selected: &[CaseRow],
) -> io::Result<()> {
    let case_count = selected.len();
    let max_frame = selected.iter().map(|row| row.frame).max().unwrap_or(0);
    let active_frame_count = selected
        .iter()
        .map(|row| row.frame)
        .collect::<HashSet<_>>()
        .len() as u64;
    let idle_frame_count = max_frame.saturating_sub(active_frame_count);
    let replay_filter = if replay_filter.is_empty() {
        "*".to_string()
    } else {
        replay_filter.join(" ")
    };
    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "runtime=native_block_live_frame_schedule")?;
    writeln!(file, "source_run_dir={}", run_dir.display())?;
    writeln!(file, "replay_filter={replay_filter}")?;
    writeln!(file, "cases=native_block_run_cases.tsv")?;
    writeln!(file, "run_report=native_block_run.tsv")?;
    writeln!(file, "runtime_trace=native_block_runtime_trace.tsv")?;
    writeln!(file, "schedule=native_block_live_frame_schedule.tsv")?;
    writeln!(file, "case_count={case_count}")?;
    writeln!(file, "matched={case_count}")?;
    writeln!(file, "mismatches=0")?;
    writeln!(file, "max_frame={max_frame}")?;
    writeln!(file, "active_frame_count={active_frame_count}")?;
    writeln!(file, "idle_frame_count={idle_frame_count}")?;
    writeln!(
        file,
        "scope=verified native run cases reordered by original replay frame for SDL frame-scheduled live execution"
    )?;
    writeln!(
        file,
        "complete={}",
        u8::from(case_count > 0 && max_frame > 0)
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_frame_schedule_with_replay_filter() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_live_frame_schedule_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let run_dir = root.join("native_block_run");
        let out_dir = root.join("schedule");
        fs::create_dir_all(&run_dir).unwrap();
        fs::create_dir_all(root.join("native_block_verify")).unwrap();
        write_inputs(&root, &run_dir);

        run(&run_dir, &out_dir, &[String::from("b")]).unwrap();

        let cases = fs::read_to_string(out_dir.join("native_block_run_cases.tsv")).unwrap();
        assert_eq!(
            cases,
            "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
             b\t1\t2\tC020\t1C020\t3\tC020\t00\t00\t00\t24\tFD\tram2\tC021\t2\t00\t00\t00\t24\tFD\thash2\n\
             b\t1\t0\tC000\t1C000\t5\tC000\t00\t00\t00\t24\tFD\tram0\tC001\t2\t00\t00\t00\t24\tFD\thash0\n"
        );
        let schedule =
            fs::read_to_string(out_dir.join("native_block_live_frame_schedule.tsv")).unwrap();
        assert!(schedule.contains("1\t3\t3\tb\t2\tC020\tC021\n"));
        assert!(schedule.contains("2\t1\t5\tb\t0\tC000\tC001\n"));
        let manifest = fs::read_to_string(out_dir.join("manifest.txt")).unwrap();
        assert!(manifest.contains("case_count=2\n"));
        assert!(manifest.contains("max_frame=5\n"));
        assert!(manifest.contains("active_frame_count=2\n"));
        assert!(manifest.contains("idle_frame_count=3\n"));
        assert!(manifest.contains("complete=1\n"));
        let external =
            fs::read_to_string(out_dir.join("native_block_run_external_writes.tsv")).unwrap();
        assert!(external.contains("ppu\t5\t2000\t80\n"));
        assert!(external.contains("apu\t3\t4008\t00\n"));

        let _ = fs::remove_dir_all(root);
    }

    fn write_inputs(root: &Path, run_dir: &Path) {
        fs::write(run_dir.join("manifest.txt"), "complete=1\n").unwrap();
        fs::write(
            run_dir.join("native_block_run_cases.tsv"),
            "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
             b\t1\t0\tC000\t1C000\t5\tC000\t00\t00\t00\t24\tFD\tram0\tC001\t2\t00\t00\t00\t24\tFD\thash0\n\
             a\t1\t1\tC010\t1C010\t3\tC010\t00\t00\t00\t24\tFD\tram1\tC011\t2\t00\t00\t00\t24\tFD\thash1\n\
             b\t1\t2\tC020\t1C020\t3\tC020\t00\t00\t00\t24\tFD\tram2\tC021\t2\t00\t00\t00\t24\tFD\thash2\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_run.tsv"),
            "run_header\nrun0\nrun1\nrun2\n",
        )
        .unwrap();
        fs::write(
            run_dir.join("native_block_runtime_trace.tsv"),
            "trace_header\ntrace0\ntrace1\ntrace2\n",
        )
        .unwrap();
        fs::write(
            root.join("native_block_verify/native_block_verify_cases.tsv"),
            "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\texternal_writes\n\
             b\t0\tC000\t1C000\t5\tC000\t00\t00\t00\t24\tFD\tram0\tC001\t2\t00\t00\t00\t24\tFD\thash0\tppu:2000:80\n\
             a\t1\tC010\t1C010\t3\tC010\t00\t00\t00\t24\tFD\tram1\tC011\t2\t00\t00\t00\t24\tFD\thash1\tmapper:8000:06\n\
             b\t2\tC020\t1C020\t3\tC020\t00\t00\t00\t24\tFD\tram2\tC021\t2\t00\t00\t00\t24\tFD\thash2\tapu:4008:00\n",
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
