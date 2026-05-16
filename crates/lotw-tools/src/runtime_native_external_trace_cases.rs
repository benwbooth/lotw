use std::fs;
use std::io::{self, Write};
use std::path::Path;

const SOURCE_HEADER: &str = "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\texternal_writes";
const RUN_HEADER: &str = "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256";

#[derive(Debug, Default)]
struct CaseStats {
    case_count: u64,
    expected_external_rows: u64,
}

pub fn run(source_cases: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    require_file(source_cases)?;
    fs::create_dir_all(out_dir)?;

    let run_cases = out_dir.join("native_block_external_run_cases.tsv");
    let expected_external = out_dir.join("expected_native_external_writes.tsv");
    let stats_path = out_dir.join("runtime_native_external_trace_cases.txt");
    let stats = write_cases(source_cases, &run_cases, &expected_external)?;
    if stats.case_count == 0 {
        return Err("runtime_native_external_trace_cases: no cases generated".into());
    }
    write_stats(&stats_path, &stats)?;

    println!(
        "runtime_native_external_trace_cases: wrote {}",
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
                "runtime_native_external_trace_cases: missing input: {}",
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

fn write_cases(
    source_cases: &Path,
    run_cases: &Path,
    expected_external: &Path,
) -> Result<CaseStats, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(source_cases)?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    if header != SOURCE_HEADER {
        return Err(format!(
            "runtime_native_external_trace_cases: bad header in {}",
            source_cases.display()
        )
        .into());
    }

    let mut run_file = fs::File::create(run_cases)?;
    let mut external_file = fs::File::create(expected_external)?;
    writeln!(run_file, "{RUN_HEADER}")?;
    writeln!(external_file, "kind\tframe\taddr\tvalue")?;

    let mut stats = CaseStats::default();
    for (line_no, line) in lines.enumerate() {
        let fields = split_tsv(line);
        if fields.len() < 21 {
            return invalid_tsv(source_cases, line_no + 2, fields.len(), 21)?;
        }
        if fields[20].is_empty() {
            continue;
        }
        stats.case_count += 1;
        writeln!(
            run_file,
            "{}\t1\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            fields[0],
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
            fields[6],
            fields[7],
            fields[8],
            fields[9],
            fields[10],
            fields[11],
            fields[12],
            fields[13],
            fields[14],
            fields[15],
            fields[16],
            fields[17],
            fields[18],
            fields[19]
        )?;
        for token in fields[20].split(',').filter(|token| !token.is_empty()) {
            let parts = token.split(':').collect::<Vec<_>>();
            if parts.len() != 3 {
                return Err(format!(
                    "runtime_native_external_trace_cases: malformed external write row: {token}"
                )
                .into());
            }
            stats.expected_external_rows += 1;
            writeln!(
                external_file,
                "{}\t{}\t{}\t{}",
                parts[0], fields[4], parts[1], parts[2]
            )?;
        }
    }
    Ok(stats)
}

fn write_stats(path: &Path, stats: &CaseStats) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "case_count={}", stats.case_count)?;
    writeln!(
        file,
        "expected_external_rows={}",
        stats.expected_external_rows
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_external_trace_cases() {
        let root = std::env::temp_dir().join(format!(
            "lotw_runtime_native_external_trace_cases_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&root).unwrap();
        let source_cases = root.join("native_block_verify_cases.tsv");
        let out_dir = root.join("out");
        fs::write(
            &source_cases,
            format!(
                "{SOURCE_HEADER}\n\
                 smoke\t0\tC000\t1C000\t7\tC000\t00\t00\t00\t24\tFD\tram0\tC010\t5\t80\t00\t00\tA4\tFD\thash0\tppu:2000:80\n\
                 smoke\t1\tC010\t1C010\t8\tC010\t80\t00\t00\tA4\tFD\tram1\tC020\t9\t00\t00\t00\t26\tFD\thash1\tapu:4008:00,mapper:8000:06\n\
                 smoke\t2\tC020\t1C020\t9\tC020\t00\t00\t00\t26\tFD\tram2\tC030\t4\t00\t00\t00\t26\tFD\thash2\t\n"
            ),
        )
        .unwrap();

        run(&source_cases, &out_dir).unwrap();
        let cases =
            fs::read_to_string(out_dir.join("native_block_external_run_cases.tsv")).unwrap();
        assert_eq!(
            cases,
            format!(
                "{RUN_HEADER}\n\
                 smoke\t1\t0\tC000\t1C000\t7\tC000\t00\t00\t00\t24\tFD\tram0\tC010\t5\t80\t00\t00\tA4\tFD\thash0\n\
                 smoke\t1\t1\tC010\t1C010\t8\tC010\t80\t00\t00\tA4\tFD\tram1\tC020\t9\t00\t00\t00\t26\tFD\thash1\n"
            )
        );
        let expected =
            fs::read_to_string(out_dir.join("expected_native_external_writes.tsv")).unwrap();
        assert_eq!(
            expected,
            "kind\tframe\taddr\tvalue\n\
             ppu\t7\t2000\t80\n\
             apu\t8\t4008\t00\n\
             mapper\t8\t8000\t06\n"
        );
        let stats =
            fs::read_to_string(out_dir.join("runtime_native_external_trace_cases.txt")).unwrap();
        assert!(stats.contains("case_count=2\n"));
        assert!(stats.contains("expected_external_rows=3\n"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_malformed_external_write() {
        let root = std::env::temp_dir().join(format!(
            "lotw_runtime_native_external_trace_cases_bad_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&root).unwrap();
        let source_cases = root.join("native_block_verify_cases.tsv");
        let out_dir = root.join("out");
        fs::write(
            &source_cases,
            format!(
                "{SOURCE_HEADER}\n\
                 smoke\t0\tC000\t1C000\t7\tC000\t00\t00\t00\t24\tFD\tram0\tC010\t5\t80\t00\t00\tA4\tFD\thash0\tppu:2000\n"
            ),
        )
        .unwrap();
        let err = run(&source_cases, &out_dir).unwrap_err();
        assert!(err.to_string().contains("malformed external write"));
        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
