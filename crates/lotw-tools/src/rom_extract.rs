use lotw_port::chr;
use lotw_port::png;
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
    let (chr_preview, chr_ppm) = chr::preview_ppm(rom.chr_rom())?;
    let chr_png = png::encode_rgb(chr_preview.width, chr_preview.height, &chr_preview.rgb)?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;
    fs::write(out_dir.join("prg.bin"), rom.prg_rom())?;
    fs::write(out_dir.join("chr.bin"), rom.chr_rom())?;
    fs::write(out_dir.join("chr_tiles.ppm"), chr_ppm)?;
    fs::write(out_dir.join("chr_tiles.png"), chr_png)?;
    write_rom_info_header(&out_dir.join("lotw_rom_info.h"), &actual_sha256, &rom)?;
    write_manifest(&out_dir.join("manifest.txt"), &actual_sha256, &rom)?;

    println!("rom_extract: wrote {}", out_dir.display());
    Ok(())
}

fn write_manifest(path: &Path, sha256: &str, rom: &InesRom) -> io::Result<()> {
    let header = rom.header();
    let mut file = fs::File::create(path)?;
    writeln!(file, "sha256={sha256}")?;
    writeln!(file, "format={}", format_name(header.format))?;
    writeln!(file, "mapper={}", header.mapper)?;
    writeln!(file, "submapper={}", header.submapper)?;
    writeln!(file, "mirroring={}", mirroring_name(header.mirroring))?;
    writeln!(file, "battery={}", u8::from(header.battery_backed_ram))?;
    writeln!(file, "trainer={}", u8::from(header.has_trainer))?;
    writeln!(file, "prg_size={}", header.prg_rom_size)?;
    writeln!(file, "chr_size={}", header.chr_rom_size)?;
    writeln!(file, "chr_png=chr_tiles.png")?;
    writeln!(file, "runtime=rust_rom_extract")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_rom_info_header(path: &Path, sha256: &str, rom: &InesRom) -> io::Result<()> {
    let header = rom.header();
    let mut file = fs::File::create(path)?;
    writeln!(file, "#ifndef LOTW_GENERATED_ROM_INFO_H")?;
    writeln!(file, "#define LOTW_GENERATED_ROM_INFO_H")?;
    writeln!(file)?;
    writeln!(file, "#define LOTW_ROM_SHA256 \"{sha256}\"")?;
    writeln!(file, "#define LOTW_ROM_MAPPER {}", header.mapper)?;
    writeln!(file, "#define LOTW_ROM_SUBMAPPER {}", header.submapper)?;
    writeln!(file, "#define LOTW_ROM_PRG_SIZE {}", header.prg_rom_size)?;
    writeln!(file, "#define LOTW_ROM_CHR_SIZE {}", header.chr_rom_size)?;
    writeln!(file)?;
    writeln!(file, "#endif")?;
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
            "lotw_tools_rom_extract_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + 0x4000 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 1;
        bytes[5] = 1;
        bytes[6] = 0x40;
        bytes[16] = 0xaa;
        bytes[16 + 0x4000] = 0xbb;
        bytes
    }

    #[test]
    fn extracts_rom_artifacts() {
        let root = temp_dir();
        let rom = root.join("fixture.nes");
        let out = root.join("out");
        let bytes = ines_fixture();
        let expected = sha256::digest_hex(&bytes);
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom, bytes).unwrap();

        run(&rom, &out, Some(&expected)).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        let header = fs::read_to_string(out.join("lotw_rom_info.h")).unwrap();
        assert_eq!(fs::read(out.join("prg.bin")).unwrap().len(), 0x4000);
        assert_eq!(fs::read(out.join("chr.bin")).unwrap().len(), 0x2000);
        assert!(fs::read(out.join("chr_tiles.ppm"))
            .unwrap()
            .starts_with(b"P6\n512 64\n255\n"));
        assert!(fs::read(out.join("chr_tiles.png"))
            .unwrap()
            .starts_with(b"\x89PNG\r\n\x1a\n"));
        assert!(manifest.contains("runtime=rust_rom_extract\n"));
        assert!(manifest.contains("chr_png=chr_tiles.png\n"));
        assert!(manifest.contains("mapper=4\n"));
        assert!(manifest.contains("complete=1\n"));
        assert!(header.contains(&format!("#define LOTW_ROM_SHA256 \"{expected}\"\n")));
        assert!(header.contains("#define LOTW_ROM_PRG_SIZE 16384\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
