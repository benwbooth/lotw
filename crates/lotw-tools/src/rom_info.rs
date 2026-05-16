use lotw_port::rom::{InesRom, Mirroring, RomFormat};
use lotw_port::sha256;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(
    rom_path: &Path,
    out_dir: &Path,
    expected_sha256: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read(rom_path)?;
    let actual_sha256 = sha256::digest_hex(&bytes);

    if let Some(expected) = expected_sha256 {
        if !actual_sha256.eq_ignore_ascii_case(expected) {
            return Err(format!(
                "ROM hash mismatch: got {actual_sha256}, expected {}",
                expected.to_ascii_lowercase()
            )
            .into());
        }
    }

    let rom = InesRom::parse(&bytes)?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    write_manifest(
        &out_dir.join("manifest.txt"),
        rom_path,
        &actual_sha256,
        expected_sha256,
        &rom,
    )?;

    println!("rom_info: wrote {}", out_dir.display());
    Ok(())
}

fn write_manifest(
    path: &Path,
    rom_path: &Path,
    actual_sha256: &str,
    expected_sha256: Option<&str>,
    rom: &InesRom,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    let header = rom.header();
    writeln!(file, "runtime=rust_rom_info")?;
    writeln!(file, "rom={}", rom_path.display())?;
    writeln!(file, "sha256={actual_sha256}")?;
    if let Some(expected) = expected_sha256 {
        writeln!(file, "expected_sha256={}", expected.to_ascii_lowercase())?;
        writeln!(file, "sha256_match=1")?;
    }
    writeln!(file, "format={}", format_name(header.format))?;
    writeln!(file, "mapper={}", header.mapper)?;
    writeln!(file, "submapper={}", header.submapper)?;
    writeln!(file, "prg_size={}", header.prg_rom_size)?;
    writeln!(file, "chr_size={}", header.chr_rom_size)?;
    writeln!(file, "prg_16k_banks={}", header.prg_rom_size / 0x4000)?;
    writeln!(file, "chr_8k_banks={}", header.chr_rom_size / 0x2000)?;
    writeln!(file, "mirroring={}", mirroring_name(header.mirroring))?;
    writeln!(file, "has_trainer={}", u8::from(header.has_trainer))?;
    writeln!(
        file,
        "battery_backed_ram={}",
        u8::from(header.battery_backed_ram)
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn mirroring_name(mirroring: Mirroring) -> &'static str {
    match mirroring {
        Mirroring::Horizontal => "horizontal",
        Mirroring::Vertical => "vertical",
        Mirroring::FourScreen => "four_screen",
    }
}

fn format_name(format: RomFormat) -> &'static str {
    match format {
        RomFormat::INes => "iNES",
        RomFormat::Nes2 => "NES 2.0",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_rom_info_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + 0x8000 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 0x41;
        bytes
    }

    #[test]
    fn writes_rom_manifest() {
        let root = temp_dir();
        let rom = root.join("fixture.nes");
        let out = root.join("out");
        let bytes = ines_fixture();
        let expected = sha256::digest_hex(&bytes);
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom, bytes).unwrap();

        run(&rom, &out, Some(&expected)).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("runtime=rust_rom_info\n"));
        assert!(manifest.contains(&format!("sha256={expected}\n")));
        assert!(manifest.contains("sha256_match=1\n"));
        assert!(manifest.contains("mapper=4\n"));
        assert!(manifest.contains("prg_size=32768\n"));
        assert!(manifest.contains("chr_size=8192\n"));
        assert!(manifest.contains("mirroring=vertical\n"));
        assert!(manifest.contains("complete=1\n"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_hash_mismatch() {
        let root = temp_dir();
        let rom = root.join("fixture.nes");
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom, ines_fixture()).unwrap();

        let err = run(&rom, &root.join("out"), Some(&"0".repeat(64))).unwrap_err();
        assert!(err.to_string().contains("ROM hash mismatch"));

        fs::remove_dir_all(root).unwrap();
    }
}
