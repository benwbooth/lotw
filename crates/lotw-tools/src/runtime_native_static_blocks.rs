use crate::runtime_native_blocks;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const RUN_CASE_HEADER: &str = "replay\texpected_steps\tpath_indices\tpath_cpu_addrs\tpath_prg_offsets\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256";

pub fn run(
    lotw: &Path,
    static_verify_dir: &Path,
    runtime_native_blocks_command: &Path,
    native_runtime_verify: &Path,
    out_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_cases = static_verify_dir.join("native_block_static_verify_cases.tsv");
    let static_manifest = static_verify_dir.join("manifest.txt");
    let static_summary = static_verify_dir.join("native_block_static_verify_summary.txt");
    for path in [
        lotw,
        &source_cases,
        &static_manifest,
        &static_summary,
        runtime_native_blocks_command,
        native_runtime_verify,
    ] {
        require_file(path)?;
    }
    require_key_value(
        &read_key_values(&static_manifest)?,
        "complete",
        "1",
        &static_manifest,
    )?;
    require_key_value(
        &read_key_values(&static_manifest)?,
        "mismatches",
        "0",
        &static_manifest,
    )?;
    require_key_value(
        &read_key_values(&static_summary)?,
        "complete",
        "1",
        &static_summary,
    )?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let run_cases = out_dir.join("native_block_run_cases.tsv");
    let case_count = write_run_cases(&source_cases, &run_cases)?;
    if case_count == 0 {
        return Err("runtime_native_static_blocks: no cases generated".into());
    }

    let runtime_dir = out_dir.join("runtime");
    runtime_native_blocks::run(lotw, &run_cases, native_runtime_verify, &runtime_dir)?;
    require_key_value(
        &read_key_values(&runtime_dir.join("run/manifest.txt"))?,
        "complete",
        "1",
        &runtime_dir.join("run/manifest.txt"),
    )?;
    require_key_value(
        &read_key_values(&runtime_dir.join("verify/native_block_runtime_trace_verify.txt"))?,
        "complete",
        "1",
        &runtime_dir.join("verify/native_block_runtime_trace_verify.txt"),
    )?;
    require_key_value(
        &read_key_values(&runtime_dir.join("trace_verify/runtime_native_trace_verify.txt"))?,
        "complete",
        "1",
        &runtime_dir.join("trace_verify/runtime_native_trace_verify.txt"),
    )?;

    write_summary(out_dir, &source_cases, case_count)?;
    write_manifest(out_dir, static_verify_dir)?;
    println!("runtime_native_static_blocks: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "runtime_native_static_blocks: missing input: {}",
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

fn require_key_value(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = values.get(key).ok_or_else(|| {
        format!(
            "runtime_native_static_blocks: missing {key} in {}",
            path.display()
        )
    })?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "runtime_native_static_blocks: {} expected {key}={expected}, got {actual}",
            path.display()
        )
        .into())
    }
}

fn write_run_cases(source: &Path, out: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(source)?;
    let mut file = fs::File::create(out)?;
    writeln!(file, "{RUN_CASE_HEADER}")?;
    let mut count = 0;
    for (line_no, line) in text.lines().enumerate().skip(1) {
        if line.is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 20 {
            return Err(format!(
                "{}:{} has {} fields, expected at least 20",
                source.display(),
                line_no + 1,
                fields.len()
            )
            .into());
        }
        writeln!(
            file,
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
        count += 1;
    }
    Ok(count)
}

fn write_summary(out_dir: &Path, source_cases: &Path, case_count: u64) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("runtime_native_static_blocks_summary.txt"))?;
    writeln!(file, "runtime=runtime_native_static_blocks")?;
    writeln!(file, "source_cases={}", source_cases.display())?;
    writeln!(file, "run_cases=native_block_run_cases.tsv")?;
    writeln!(file, "runtime_dir=runtime")?;
    writeln!(file, "case_count={case_count}")?;
    writeln!(
        file,
        "runtime_manifest={}",
        out_dir.join("runtime/run/manifest.txt").display()
    )?;
    writeln!(
        file,
        "runtime_verify={}",
        out_dir
            .join("runtime/verify/native_block_runtime_trace_verify.txt")
            .display()
    )?;
    writeln!(
        file,
        "trace_verify={}",
        out_dir
            .join("runtime/trace_verify/runtime_native_trace_verify.txt")
            .display()
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(out_dir: &Path, static_verify_dir: &Path) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "runtime=runtime_native_static_blocks")?;
    writeln!(file, "static_verify_dir={}", static_verify_dir.display())?;
    writeln!(file, "cases=native_block_run_cases.tsv")?;
    writeln!(file, "summary=runtime_native_static_blocks_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::{compile, compile_noop, unique_temp_dir};
    use lotw_port::sha256;

    #[test]
    fn converts_static_cases_and_runs_native_blocks() {
        let root = unique_temp_dir("runtime-native-static-blocks");
        let static_dir = root.join("static_verify");
        let out = root.join("out");
        let lotw = root.join("fake_lotw");
        let runtime_blocks = root.join("runtime_native_blocks");
        let native_verify = root.join("native_block_runtime_trace_verify");
        fs::create_dir_all(&static_dir).unwrap();
        compile_noop(&runtime_blocks);
        compile_noop(&native_verify);

        let zero_ram = "00".repeat(2048);
        let zero_sha = sha256::digest_hex(&vec![0u8; 2048]);
        write_static_verify(&static_dir, &zero_ram, &zero_sha);
        write_fake_lotw(&lotw, &zero_ram, &zero_sha);

        run(&lotw, &static_dir, &runtime_blocks, &native_verify, &out).unwrap();

        let summary =
            fs::read_to_string(out.join("runtime_native_static_blocks_summary.txt")).unwrap();
        assert!(summary.contains("case_count=2\n"));
        assert!(summary.contains("complete=1\n"));
        let run_cases = fs::read_to_string(out.join("native_block_run_cases.tsv")).unwrap();
        assert!(run_cases.contains("\nsmoke\t1\t0\tC000\t1C000\t5\tC000\t"));
        let trace_verify =
            fs::read_to_string(out.join("runtime/trace_verify/runtime_native_trace_verify.txt"))
                .unwrap();
        assert!(trace_verify.contains("complete=1\n"));
    }

    fn write_static_verify(static_dir: &Path, zero_ram: &str, zero_sha: &str) {
        fs::write(
            static_dir.join("manifest.txt"),
            "cases=native_block_static_verify_cases.tsv\ncase_count=2\nmatched=2\nmismatches=0\nexternal_write_matched=2\nexternal_write_mismatches=0\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            static_dir.join("native_block_static_verify_summary.txt"),
            "runtime=native_block_static_verify\ncases=native_block_static_verify_cases.tsv\ncase_count=2\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            static_dir.join("native_block_static_verify_cases.tsv"),
            format!(
                "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\texternal_writes\n\
smoke\t0\tC000\t1C000\t5\tC000\t00\t01\t02\t24\tFD\t{zero_ram}\tC001\t2\t10\t01\t02\t24\tFD\t{zero_sha}\n\
smoke\t1\tC001\t1C001\t6\tC001\t10\t01\t02\t24\tFD\t{zero_ram}\tC002\t3\t20\t01\t02\t24\tFD\t{zero_sha}\n"
            ),
        )
        .unwrap();
    }

    fn write_fake_lotw(path: &Path, zero_ram: &str, zero_sha: &str) {
        let source = r#"
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut cases = String::new();
    let mut run_dir = None;
    let mut trace_dir = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--native-run-cases" => {
                cases = args[i + 1].clone();
                i += 2;
            }
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
        format!(
            "runtime=lotw_runtime_native_blocks\ncases={cases}\nrun_report=native_block_run.tsv\nruntime_trace=native_block_runtime_trace.tsv\ncase_count=2\nmatched=2\nmismatches=0\ncomplete=1\n"
        ),
    )
    .unwrap();
    fs::write(
        run_dir.join("native_block_run.tsv"),
        "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tdispatch_match\tpath_match\tmetadata_match\tregister_match\tcycles_match\tram_match\tmatch\tfinal_pc\tcycles\nsmoke\t1\t1\t0\t1\t1\t1\t1\t1\t1\t1\tC001\t2\nsmoke\t1\t1\t1\t1\t1\t1\t1\t1\t1\t1\tC002\t3\n",
    )
    .unwrap();
    fs::write(
        run_dir.join("native_block_runtime_trace.tsv"),
        "replay\texpected_steps\texecuted_steps\texecuted_path_indices\tfirst_frame\tinitial_pc\tfinal_pc\tcycles\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\toracle_final_pc\toracle_cycles\toracle_final_a\toracle_final_x\toracle_final_y\toracle_final_p\toracle_final_s\toracle_final_ram_sha256\tstate_match\nsmoke\t1\t1\t0\t5\tC000\tC001\t2\t10\t01\t02\t24\tFD\t{ZERO_SHA}\tC001\t2\t10\t01\t02\t24\tFD\t{ZERO_SHA}\t1\nsmoke\t1\t1\t1\t6\tC001\tC002\t3\t20\t01\t02\t24\tFD\t{ZERO_SHA}\tC002\t3\t20\t01\t02\t24\tFD\t{ZERO_SHA}\t1\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_trace_summary.txt"),
        "runtime=lotw_runtime_native_blocks\nframes=6\nmapper_write_count=0\napu_write_count=0\nppu_write_count=0\nppu_vram_write_count=0\noam_dma_count=0\nlabel_state_count=2\ncomplete=1\n",
    )
    .unwrap();
    fs::write(
        trace_dir.join("port_label_states.tsv"),
        "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\nC001\t1C000\t5\tC001\t10\t01\t02\t24\tFD\t{ZERO_RAM}\nC002\t1C001\t6\tC002\t20\t01\t02\t24\tFD\t{ZERO_RAM}\n",
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
