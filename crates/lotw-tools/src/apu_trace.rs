use lotw_port::apu_trace::{
    collect_register_stats, parse_apu_writes_tsv, ApuTraceEvent, RegisterStats,
};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(input: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    require_file(input, "missing APU trace")?;
    remove_path(out_dir)?;
    fs::create_dir_all(out_dir)?;

    let events = read_events(input)?;
    let stats = collect_register_stats(&events);
    write_manifest(out_dir, input, events.len(), &stats)?;
    write_counts(out_dir, &stats)?;
    write_generated_c(out_dir, &events)?;

    println!("apu_trace: wrote {}", out_dir.display());
    println!("apu_trace: {} APU register writes", events.len());
    Ok(())
}

fn read_events(path: &Path) -> Result<Vec<ApuTraceEvent>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    parse_apu_writes_tsv(&text)
        .map_err(|err| format!("apu_trace: {}: {err}", path.display()).into())
}

fn write_manifest(
    out_dir: &Path,
    input: &Path,
    event_count: usize,
    stats: &[RegisterStats; 0x18],
) -> io::Result<()> {
    let touched = stats.iter().filter(|stat| stat.count != 0).count();
    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "input={}", input.display())?;
    writeln!(file, "event_count={event_count}")?;
    writeln!(file, "registers_touched={touched}")?;
    writeln!(file, "scope=2a03 apu register write event stream")?;
    writeln!(file, "runtime=rust_apu_trace")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_counts(out_dir: &Path, stats: &[RegisterStats; 0x18]) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("apu_register_counts.tsv"))?;
    writeln!(file, "addr\tcount\tfirst_frame\tlast_frame")?;
    for (index, stat) in stats.iter().enumerate() {
        if stat.count != 0 {
            writeln!(
                file,
                "{:04X}\t{}\t{}\t{}",
                0x4000 + index,
                stat.count,
                stat.first_frame,
                stat.last_frame
            )?;
        }
    }
    Ok(())
}

fn write_generated_c(out_dir: &Path, events: &[ApuTraceEvent]) -> io::Result<()> {
    let mut header = fs::File::create(out_dir.join("lotw_generated_apu_events.h"))?;
    let mut source = fs::File::create(out_dir.join("lotw_generated_apu_events.c"))?;

    writeln!(header, "#ifndef LOTW_GENERATED_APU_EVENTS_H")?;
    writeln!(header, "#define LOTW_GENERATED_APU_EVENTS_H")?;
    writeln!(header)?;
    writeln!(header, "#include <stddef.h>")?;
    writeln!(header, "#include <stdint.h>")?;
    writeln!(header)?;
    writeln!(header, "typedef struct LotwGeneratedApuEvent {{")?;
    writeln!(header, "  uint32_t frame;")?;
    writeln!(header, "  uint64_t cycle;")?;
    writeln!(header, "  uint16_t address;")?;
    writeln!(header, "  uint8_t value;")?;
    writeln!(header, "  uint8_t cycle_known;")?;
    writeln!(header, "}} LotwGeneratedApuEvent;")?;
    writeln!(header)?;
    writeln!(
        header,
        "extern const size_t lotw_generated_apu_event_count;"
    )?;
    writeln!(
        header,
        "extern const LotwGeneratedApuEvent lotw_generated_apu_events[];"
    )?;
    writeln!(header)?;
    writeln!(header, "#endif")?;

    writeln!(source, "#include \"lotw_generated_apu_events.h\"")?;
    writeln!(source)?;
    writeln!(
        source,
        "const LotwGeneratedApuEvent lotw_generated_apu_events[] = {{"
    )?;
    for event in events {
        writeln!(
            source,
            "  {{ {}, {}, 0x{:04X}, 0x{:02X}, {} }},",
            event.frame,
            event.cycle,
            event.address,
            event.value,
            usize::from(event.cycle_known)
        )?;
    }
    writeln!(source, "}};")?;
    writeln!(source)?;
    writeln!(
        source,
        "const size_t lotw_generated_apu_event_count = sizeof(lotw_generated_apu_events) / sizeof(lotw_generated_apu_events[0]);"
    )?;
    Ok(())
}

fn require_file(path: &Path, message: &str) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("apu_trace: {message}: {}", path.display()),
        ))
    }
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
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_apu_trace_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_apu_event_metadata_and_filters_non_audio_registers() {
        let root = temp_dir();
        let input = root.join("apu_writes.tsv");
        let out = root.join("out");
        fs::create_dir_all(&root).unwrap();
        fs::write(
            &input,
            concat!(
                "frame\tcycle\taddr\tvalue\n",
                "1\t10\t4000\t30\n",
                "2\t11\t4014\t02\n",
                "3\tunknown\t4015\t0F\n",
                "4\t12\t4017\t40\n",
            ),
        )
        .unwrap();

        run(&input, &out).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("event_count=3\n"));
        assert!(manifest.contains("registers_touched=3\n"));
        assert!(manifest.contains("runtime=rust_apu_trace\n"));
        assert!(manifest.contains("complete=1\n"));

        let counts = fs::read_to_string(out.join("apu_register_counts.tsv")).unwrap();
        assert!(counts.contains("4000\t1\t1\t1\n"));
        assert!(!counts.contains("4014"));
        assert!(counts.contains("4015\t1\t3\t3\n"));
        assert!(counts.contains("4017\t1\t4\t4\n"));

        let generated = fs::read_to_string(out.join("lotw_generated_apu_events.c")).unwrap();
        assert!(generated.contains("  { 1, 10, 0x4000, 0x30, 1 },\n"));
        assert!(generated.contains("  { 3, 0, 0x4015, 0x0F, 0 },\n"));
        assert!(generated.contains("  { 4, 12, 0x4017, 0x40, 1 },\n"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_bad_header() {
        let root = temp_dir();
        let input = root.join("apu_writes.tsv");
        fs::create_dir_all(&root).unwrap();
        fs::write(&input, "bad\n").unwrap();

        let err = run(&input, &root.join("out")).unwrap_err();
        assert!(err.to_string().contains("bad APU trace header"));

        fs::remove_dir_all(root).unwrap();
    }
}
