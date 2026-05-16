use crate::{native_block_runtime_trace_verify, runtime_native_trace_verify};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(
    lotw: &Path,
    cases: &Path,
    verify_command: &Path,
    out_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in [lotw, cases, verify_command] {
        require_file(path)?;
    }

    let run_dir = out_dir.join("run");
    let trace_dir = out_dir.join("trace");
    let verify_dir = out_dir.join("verify");
    let trace_verify_dir = out_dir.join("trace_verify");
    let log = out_dir.join("runtime_native_blocks.log");
    let expected_external = cases
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("native_block_run_external_writes.tsv");

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    run_lotw(lotw, cases, &run_dir, &trace_dir, &log)?;
    require_key_value(
        &read_key_values(&run_dir.join("manifest.txt"))?,
        "runtime",
        "lotw_runtime_native_blocks",
        &run_dir.join("manifest.txt"),
    )?;

    native_block_runtime_trace_verify::run(&run_dir, &verify_dir)?;
    require_key_value(
        &read_key_values(&verify_dir.join("native_block_runtime_trace_verify.txt"))?,
        "complete",
        "1",
        &verify_dir.join("native_block_runtime_trace_verify.txt"),
    )?;

    let expected_external = if has_data_rows(&expected_external)? {
        Some(expected_external.as_path())
    } else {
        None
    };
    runtime_native_trace_verify::run(
        &run_dir,
        &trace_dir,
        &trace_verify_dir,
        "lotw_runtime_native_blocks",
        expected_external,
    )?;
    require_key_value(
        &read_key_values(&trace_verify_dir.join("runtime_native_trace_verify.txt"))?,
        "complete",
        "1",
        &trace_verify_dir.join("runtime_native_trace_verify.txt"),
    )?;

    println!("runtime_native_blocks: wrote {}", run_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("runtime_native_blocks: missing input: {}", path.display()),
        ))
    }
}

fn run_lotw(
    lotw: &Path,
    cases: &Path,
    run_dir: &Path,
    trace_dir: &Path,
    log: &Path,
) -> io::Result<()> {
    let log_file = fs::File::create(log)?;
    let log_stderr = log_file.try_clone()?;
    let status = Command::new(lotw)
        .arg("--native-run-cases")
        .arg(cases)
        .arg("--native-run-dir")
        .arg(run_dir)
        .arg("--native-trace-dir")
        .arg(trace_dir)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(log_stderr))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "runtime_native_blocks: lotw failed: {status}"
        )))
    }
}

fn has_data_rows(path: &Path) -> io::Result<bool> {
    if !path.is_file() {
        return Ok(false);
    }
    Ok(fs::read_to_string(path)?
        .lines()
        .skip(1)
        .any(|line| !line.is_empty()))
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
    let actual = values
        .get(key)
        .ok_or_else(|| format!("runtime_native_blocks: missing {key} in {}", path.display()))?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "runtime_native_blocks: {} expected {key}={expected}, got {actual}",
            path.display()
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::{compile, compile_noop, unique_temp_dir};
    use lotw_port::sha256;

    #[test]
    fn runs_native_blocks_and_rust_verifiers() {
        let root = unique_temp_dir("runtime-native-blocks");
        let out = root.join("out");
        let cases = root.join("native_block_run_cases.tsv");
        let verify_command = root.join("native_block_runtime_trace_verify");
        let lotw = root.join("fake_lotw");
        let zero_ram = "00".repeat(2048);
        let zero_sha = sha256::digest_hex(&vec![0; 2048]);

        fs::write(
            &cases,
            format!(
                "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n\
smoke\t2\t0,1\tC000,C001\t1C000,1C001\t7\tC000\t00\t00\t00\t24\tFD\t{zero_ram}\tC010\t11\t01\t02\t03\t24\tF8\t{zero_sha}\n\
smoke\t3\t0,1,2\tC010,C011,C012\t1C010,1C011,1C012\t8\tC010\t01\t02\t03\t24\tF8\t{zero_ram}\tC020\t17\t04\t05\t06\t25\tF7\t{zero_sha}\n"
            ),
        )
        .unwrap();
        compile_noop(&verify_command);
        write_fake_lotw(&lotw, &zero_ram, &zero_sha);

        run(&lotw, &cases, &verify_command, &out).unwrap();

        let verify =
            fs::read_to_string(out.join("verify/native_block_runtime_trace_verify.txt")).unwrap();
        assert!(verify.contains("complete=1\n"));
        let trace_verify =
            fs::read_to_string(out.join("trace_verify/runtime_native_trace_verify.txt")).unwrap();
        assert!(trace_verify.contains("complete=1\n"));
        assert!(trace_verify.contains("manifest_case_count=2\n"));
        assert!(trace_verify.contains("trace_rows=2\n"));
    }

    fn write_fake_lotw(path: &Path, zero_ram: &str, zero_sha: &str) {
        let source = r#"
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut run_dir = None;
    let mut trace_dir = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--native-run-dir" => {
                run_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--native-trace-dir" => {
                trace_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            _ => i += 1,
        }
    }
    let run_dir = run_dir.expect("missing run dir");
    let trace_dir = trace_dir.expect("missing trace dir");
    fs::create_dir_all(&run_dir).unwrap();
    fs::create_dir_all(&trace_dir).unwrap();
    fs::write(
        run_dir.join("manifest.txt"),
        "runtime=lotw_runtime_native_blocks\ncases=/tmp/native_block_run_cases.tsv\nrun_report=native_block_run.tsv\nruntime_trace=native_block_runtime_trace.tsv\ncase_count=2\nmatched=2\nmismatches=0\ncomplete=1\n",
    )
    .unwrap();
    fs::write(
        run_dir.join("native_block_run.tsv"),
        "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tdispatch_match\tpath_match\tmetadata_match\tregister_match\tcycles_match\tram_match\tmatch\tfinal_pc\tcycles\nsmoke\t2\t2\t0,1\t1\t1\t1\t1\t1\t1\t1\tC010\t11\nsmoke\t3\t3\t0,1,2\t1\t1\t1\t1\t1\t1\t1\tC020\t17\n",
    )
    .unwrap();
    fs::write(
        run_dir.join("native_block_runtime_trace.tsv"),
        "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tfirst_frame\tinitial_pc\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\toracle_final_pc\toracle_cycles\toracle_final_a\toracle_final_x\toracle_final_y\toracle_final_p\toracle_final_s\toracle_final_ram_sha256\tstate_match\nsmoke\t2\t2\t0,1\t7\tC000\tC010\t11\t01\t02\t03\t24\tF8\t{ZERO_SHA}\tC010\t11\t01\t02\t03\t24\tF8\t{ZERO_SHA}\t1\nsmoke\t3\t3\t0,1,2\t8\tC010\tC020\t17\t04\t05\t06\t25\tF7\t{ZERO_SHA}\tC020\t17\t04\t05\t06\t25\tF7\t{ZERO_SHA}\t1\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_trace_summary.txt"),
        "runtime=lotw_runtime_native_blocks\nframes=8\nmapper_write_count=0\napu_write_count=0\nppu_write_count=0\nppu_vram_write_count=0\noam_dma_count=0\nlabel_state_count=2\ncomplete=1\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_label_states.tsv"),
        "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\nC010\t1C010\t7\tC010\t01\t02\t03\t24\tF8\t{ZERO_RAM}\nC020\t1C020\t8\tC020\t04\t05\t06\t25\tF7\t{ZERO_RAM}\n",
    )
    .unwrap();
    fs::write(trace_dir.join("port_mapper_writes.tsv"), "frame\taddr\tvalue\tstate\n").unwrap();
    fs::write(trace_dir.join("port_apu_writes.tsv"), "frame\tcycle\taddr\tvalue\n").unwrap();
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
"#
        .replace("{ZERO_RAM}", zero_ram)
        .replace("{ZERO_SHA}", zero_sha);
        compile(path, &source);
    }
}
