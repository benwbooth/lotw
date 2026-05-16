use lotw_port::sha256;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const REQUIRED_REFERENCE: &[&str] = &[
    "trace_summary.txt",
    "mapper_writes.tsv",
    "apu_writes.tsv",
    "oam_dma.tsv",
    "ppu_writes.tsv",
    "ppu_vram_writes.tsv",
    "label_states.tsv",
];

const REQUIRED_PORT: &[&str] = &[
    "port_trace_summary.txt",
    "port_mapper_writes.tsv",
    "port_apu_writes.tsv",
    "port_oam_dma.tsv",
    "port_ppu_writes.tsv",
    "port_ppu_vram_writes.tsv",
    "port_label_states.tsv",
];

struct TraceFile<'a> {
    reference_count_key: &'a str,
    port_count_key: &'a str,
    reference_file: &'a str,
    port_file: &'a str,
    reference_hash_key: &'a str,
    port_hash_key: &'a str,
    match_key: &'a str,
}

struct CompareContext<'a> {
    reference_dir: &'a Path,
    port_dir: &'a Path,
    ref_summary: &'a HashMap<String, String>,
    port_summary: &'a HashMap<String, String>,
    ref_summary_path: &'a Path,
    port_summary_path: &'a Path,
}

const TRACE_FILES: &[TraceFile<'_>] = &[
    TraceFile {
        reference_count_key: "mapper_write_count",
        port_count_key: "mapper_write_count",
        reference_file: "mapper_writes.tsv",
        port_file: "port_mapper_writes.tsv",
        reference_hash_key: "reference_mapper_hash",
        port_hash_key: "port_mapper_hash",
        match_key: "mapper_match",
    },
    TraceFile {
        reference_count_key: "apu_write_count",
        port_count_key: "apu_write_count",
        reference_file: "apu_writes.tsv",
        port_file: "port_apu_writes.tsv",
        reference_hash_key: "reference_apu_hash",
        port_hash_key: "port_apu_hash",
        match_key: "apu_match",
    },
    TraceFile {
        reference_count_key: "oam_dma_count",
        port_count_key: "oam_dma_count",
        reference_file: "oam_dma.tsv",
        port_file: "port_oam_dma.tsv",
        reference_hash_key: "reference_oam_dma_hash",
        port_hash_key: "port_oam_dma_hash",
        match_key: "oam_dma_match",
    },
    TraceFile {
        reference_count_key: "ppu_write_count",
        port_count_key: "ppu_write_count",
        reference_file: "ppu_writes.tsv",
        port_file: "port_ppu_writes.tsv",
        reference_hash_key: "reference_ppu_hash",
        port_hash_key: "port_ppu_hash",
        match_key: "ppu_match",
    },
    TraceFile {
        reference_count_key: "ppu_vram_write_count",
        port_count_key: "ppu_vram_write_count",
        reference_file: "ppu_vram_writes.tsv",
        port_file: "port_ppu_vram_writes.tsv",
        reference_hash_key: "reference_ppu_vram_hash",
        port_hash_key: "port_ppu_vram_hash",
        match_key: "ppu_vram_match",
    },
    TraceFile {
        reference_count_key: "label_state_count",
        port_count_key: "label_state_count",
        reference_file: "label_states.tsv",
        port_file: "port_label_states.tsv",
        reference_hash_key: "reference_label_state_hash",
        port_hash_key: "port_label_state_hash",
        match_key: "label_state_match",
    },
];

pub fn run(
    reference_dir: &Path,
    port_dir: &Path,
    out_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in REQUIRED_REFERENCE {
        require_file(&reference_dir.join(file), "missing reference file")?;
    }
    for file in REQUIRED_PORT {
        require_file(&port_dir.join(file), "missing port file")?;
    }

    let ref_summary_path = reference_dir.join("trace_summary.txt");
    let port_summary_path = port_dir.join("port_trace_summary.txt");
    let ref_summary = read_key_values(&ref_summary_path)?;
    let port_summary = read_key_values(&port_summary_path)?;
    let port_runtime = required(&port_summary, "runtime", &port_summary_path)?;

    remove_path(out_dir)?;
    fs::create_dir_all(out_dir)?;

    let mut report = fs::File::create(out_dir.join("trace_compare.txt"))?;
    writeln!(report, "reference_trace={}", reference_dir.display())?;
    writeln!(report, "port_capture={}", port_dir.display())?;
    writeln!(report, "port_runtime={port_runtime}")?;
    writeln!(
        report,
        "reference_frames={}",
        required(&ref_summary, "frames", &ref_summary_path)?
    )?;
    writeln!(
        report,
        "port_frames={}",
        required(&port_summary, "frames", &port_summary_path)?
    )?;

    let context = CompareContext {
        reference_dir,
        port_dir,
        ref_summary: &ref_summary,
        port_summary: &port_summary,
        ref_summary_path: &ref_summary_path,
        port_summary_path: &port_summary_path,
    };

    for trace_file in TRACE_FILES {
        write_trace_file_report(&mut report, &context, trace_file)?;
    }

    writeln!(report, "expected_current_runtime={port_runtime}")?;
    writeln!(report, "complete=1")?;

    println!(
        "trace_compare: wrote {}",
        out_dir.join("trace_compare.txt").display()
    );
    Ok(())
}

fn write_trace_file_report(
    report: &mut fs::File,
    context: &CompareContext<'_>,
    trace_file: &TraceFile<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    let reference_path = context.reference_dir.join(trace_file.reference_file);
    let port_path = context.port_dir.join(trace_file.port_file);
    let reference_bytes = fs::read(&reference_path)?;
    let port_bytes = fs::read(&port_path)?;
    let reference_count_key = format!("reference_{}", trace_file.reference_count_key);
    let port_count_key = format!("port_{}", trace_file.port_count_key);

    writeln!(
        report,
        "{}={}",
        reference_count_key,
        required(
            context.ref_summary,
            trace_file.reference_count_key,
            context.ref_summary_path
        )?
    )?;
    writeln!(
        report,
        "{}={}",
        port_count_key,
        required(
            context.port_summary,
            trace_file.port_count_key,
            context.port_summary_path
        )?
    )?;
    writeln!(
        report,
        "{}={}",
        trace_file.reference_hash_key,
        sha256::digest_hex(&reference_bytes)
    )?;
    writeln!(
        report,
        "{}={}",
        trace_file.port_hash_key,
        sha256::digest_hex(&port_bytes)
    )?;
    writeln!(
        report,
        "{}={}",
        trace_file.match_key,
        u8::from(reference_bytes == port_bytes)
    )?;
    Ok(())
}

fn require_file(path: &Path, message: &str) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("trace_compare: {message}: {}", path.display()),
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
        .ok_or_else(|| format!("trace_compare: missing {key} in {}", path.display()).into())
}

fn remove_path(path: &Path) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path),
        Ok(_) => fs::remove_file(path),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn writes_hashes_and_match_flags_for_trace_pairs() {
        let root = unique_temp_dir("trace-compare");
        let reference = root.join("reference");
        let port = root.join("port");
        let out = root.join("out");
        write_reference(&reference);
        write_port(&port);

        run(&reference, &port, &out).unwrap();

        let report = fs::read_to_string(out.join("trace_compare.txt")).unwrap();
        assert!(report.contains(&format!("reference_trace={}\n", reference.display())));
        assert!(report.contains(&format!("port_capture={}\n", port.display())));
        assert!(report.contains("port_runtime=chr_preview\n"));
        assert!(report.contains("reference_frames=12\n"));
        assert!(report.contains("port_frames=12\n"));
        assert!(report.contains("reference_mapper_write_count=1\n"));
        assert!(report.contains("port_mapper_write_count=1\n"));
        assert!(report.contains(&format!(
            "reference_mapper_hash={}\n",
            sha256::digest_hex(b"frame\taddr\tvalue\tstate\n1\t8000\t01\t02\n")
        )));
        assert!(report.contains("mapper_match=1\n"));
        assert!(report.contains("apu_match=0\n"));
        assert!(report.contains("oam_dma_match=1\n"));
        assert!(report.contains("ppu_match=1\n"));
        assert!(report.contains("ppu_vram_match=1\n"));
        assert!(report.contains("label_state_match=1\n"));
        assert!(report.contains("expected_current_runtime=chr_preview\n"));
        assert!(report.contains("complete=1\n"));
    }

    fn write_reference(dir: &Path) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join("trace_summary.txt"),
            "runtime=fceux\nframes=12\nmapper_write_count=1\napu_write_count=1\noam_dma_count=1\nppu_write_count=1\nppu_vram_write_count=1\nlabel_state_count=1\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            dir.join("mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n1\t8000\t01\t02\n",
        )
        .unwrap();
        fs::write(
            dir.join("apu_writes.tsv"),
            "frame\tcycle\taddr\tvalue\n1\t2\t4000\t7f\n",
        )
        .unwrap();
        fs::write(
            dir.join("oam_dma.tsv"),
            "frame\tcycle\tpage\tbytes_0000_00ff\n1\t2\t02\tabcd\n",
        )
        .unwrap();
        fs::write(
            dir.join("ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n1\t2\t2000\tPPUCTRL\t80\n",
        )
        .unwrap();
        fs::write(
            dir.join("ppu_vram_writes.tsv"),
            "frame\tcycle\taddr\tregion\tvalue\n1\t2\t2000\tnt\t24\n",
        )
        .unwrap();
        fs::write(
            dir.join("label_states.tsv"),
            "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\nC000\t1C000\t1\tC000\t00\t00\t00\t24\tFD\t00\n",
        )
        .unwrap();
    }

    fn write_port(dir: &Path) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join("port_trace_summary.txt"),
            "runtime=chr_preview\nframes=12\nmapper_write_count=1\napu_write_count=1\noam_dma_count=1\nppu_write_count=1\nppu_vram_write_count=1\nlabel_state_count=1\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            dir.join("port_mapper_writes.tsv"),
            "frame\taddr\tvalue\tstate\n1\t8000\t01\t02\n",
        )
        .unwrap();
        fs::write(
            dir.join("port_apu_writes.tsv"),
            "frame\tcycle\taddr\tvalue\n1\t2\t4000\t80\n",
        )
        .unwrap();
        fs::write(
            dir.join("port_oam_dma.tsv"),
            "frame\tcycle\tpage\tbytes_0000_00ff\n1\t2\t02\tabcd\n",
        )
        .unwrap();
        fs::write(
            dir.join("port_ppu_writes.tsv"),
            "frame\tcycle\taddr\tregister\tvalue\n1\t2\t2000\tPPUCTRL\t80\n",
        )
        .unwrap();
        fs::write(
            dir.join("port_ppu_vram_writes.tsv"),
            "frame\tcycle\taddr\tregion\tvalue\n1\t2\t2000\tnt\t24\n",
        )
        .unwrap();
        fs::write(
            dir.join("port_label_states.tsv"),
            "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\nC000\t1C000\t1\tC000\t00\t00\t00\t24\tFD\t00\n",
        )
        .unwrap();
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "lotw-tools-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        path
    }
}
