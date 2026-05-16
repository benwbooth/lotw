use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(
    run_cases: &Path,
    verify_cases: &Path,
    out: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    require_file(run_cases)?;
    require_file(verify_cases)?;
    write_external_writes(run_cases, verify_cases, out)?;
    println!("native_block_run_external_writes: wrote {}", out.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "native_block_run_external_writes: missing input: {}",
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

fn row_key(replay: &str, index: &str) -> String {
    format!("{replay}\t{index}")
}

fn write_external_writes(
    run_cases: &Path,
    verify_cases: &Path,
    out: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut external = HashMap::new();
    let verify_text = fs::read_to_string(verify_cases)?;
    for (line_no, line) in verify_text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 2 {
            return invalid_tsv(verify_cases, line_no + 1, fields.len(), 2)?;
        }
        external.insert(
            row_key(fields[0], fields[1]),
            fields.get(20).copied().unwrap_or("").to_string(),
        );
    }

    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let run_text = fs::read_to_string(run_cases)?;
    let mut file = fs::File::create(out)?;
    writeln!(file, "kind\tframe\taddr\tvalue")?;
    for (line_no, line) in run_text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 6 {
            return invalid_tsv(run_cases, line_no + 1, fields.len(), 6)?;
        }
        let replay = fields[0];
        let frame = fields[5];
        for index in fields[2].split(',') {
            let writes = external
                .get(&row_key(replay, index))
                .map_or("", String::as_str);
            if writes.is_empty() {
                continue;
            }
            for write in writes.split(',').filter(|value| !value.is_empty()) {
                let write_fields = write.split(':').collect::<Vec<_>>();
                if write_fields.len() != 3 {
                    return Err(format!(
                        "native_block_run_external_writes: malformed row: {write}"
                    )
                    .into());
                }
                writeln!(
                    file,
                    "{}\t{}\t{}\t{}",
                    write_fields[0],
                    frame,
                    write_fields[1].to_ascii_uppercase(),
                    write_fields[2].to_ascii_uppercase()
                )?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_external_write_sidecar() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_run_external_writes_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&root).unwrap();
        let verify_cases = root.join("native_block_verify_cases.tsv");
        let run_cases = root.join("native_block_run_cases.tsv");
        let actual = root.join("out/native_block_run_external_writes.tsv");

        fs::write(
            &verify_cases,
            "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\texternal_writes\n\
             title\t3\tAE6F\t1AE6F\t5\tAE6F\t00\t00\t00\t00\tFD\t00\tAE76\t9\tA0\t00\t00\t80\tFD\t0000000000000000000000000000000000000000000000000000000000000000\tppu:2000:a0\n\
             title\t4\tAE76\t1AE76\t5\tAE76\tA0\t00\t00\t80\tFD\t00\tAE83\t17\t00\t00\t00\t02\tFD\t1111111111111111111111111111111111111111111111111111111111111111\tppu:2001:00\n\
             title\t9\tF8EC\t1F8EC\t6\tF8EC\t00\t00\t00\t00\tFD\t00\tF8EF\t6\t00\t00\t00\t00\tFD\t2222222222222222222222222222222222222222222222222222222222222222\tapu:400c:30\n\
             title\t10\tF000\t1F000\t7\tF000\t00\t00\t00\t00\tFD\t00\tF001\t2\t00\t00\t00\t00\tFD\t3333333333333333333333333333333333333333333333333333333333333333\n",
        )
        .unwrap();
        fs::write(
            &run_cases,
            "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
             title\t3\t3,4,10\tAE6F,AE76,F000\t1AE6F,1AE76,1F000\t5\tAE6F\t00\t00\t00\t00\tFD\t00\tF001\t28\t00\t00\t00\t00\tFD\t3333333333333333333333333333333333333333333333333333333333333333\n\
             title\t1\t9\tF8EC\t1F8EC\t6\tF8EC\t00\t00\t00\t00\tFD\t00\tF8EF\t6\t00\t00\t00\t00\tFD\t2222222222222222222222222222222222222222222222222222222222222222\n",
        )
        .unwrap();

        run(&run_cases, &verify_cases, &actual).unwrap();
        let output = fs::read_to_string(&actual).unwrap();
        assert_eq!(
            output,
            "kind\tframe\taddr\tvalue\n\
             ppu\t5\t2000\tA0\n\
             ppu\t5\t2001\t00\n\
             apu\t6\t400C\t30\n"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_malformed_external_write_rows() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_run_external_writes_bad_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&root).unwrap();
        let verify_cases = root.join("native_block_verify_cases.tsv");
        let run_cases = root.join("native_block_run_cases.tsv");
        let actual = root.join("native_block_run_external_writes.tsv");

        fs::write(
            &verify_cases,
            "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\texternal_writes\n\
             title\t3\tAE6F\t1AE6F\t5\tAE6F\t00\t00\t00\t00\tFD\t00\tAE76\t9\tA0\t00\t00\t80\tFD\t0000000000000000000000000000000000000000000000000000000000000000\tppu:2000\n",
        )
        .unwrap();
        fs::write(
            &run_cases,
            "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\n\
             title\t1\t3\tAE6F\t1AE6F\t5\n",
        )
        .unwrap();

        let err = run(&run_cases, &verify_cases, &actual).unwrap_err();
        assert!(err.to_string().contains("malformed row"));

        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
