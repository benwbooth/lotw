use lotw_port::chr;
use lotw_port::png;
use lotw_port::rom::InesRom;
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
    let (preview, ppm) = chr::preview_ppm(rom.chr_rom())?;
    let png = png::encode_rgb(preview.width, preview.height, &preview.rgb)?;
    let preview_sha256 = sha256::digest_hex(&ppm);
    let png_sha256 = sha256::digest_hex(&png);

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    fs::write(out_dir.join("chr_tiles.ppm"), ppm)?;
    fs::write(out_dir.join("chr_tiles.png"), png)?;
    write_manifest(
        &out_dir.join("manifest.txt"),
        rom_path,
        &actual_sha256,
        expected_sha256,
        &preview_sha256,
        &png_sha256,
        preview.width,
        preview.height,
        preview.tile_count,
    )?;

    println!("chr_preview: wrote {}", out_dir.display());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_manifest(
    path: &Path,
    rom_path: &Path,
    actual_sha256: &str,
    expected_sha256: Option<&str>,
    preview_sha256: &str,
    png_sha256: &str,
    width: usize,
    height: usize,
    tile_count: usize,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=rust_chr_preview")?;
    writeln!(file, "rom={}", rom_path.display())?;
    writeln!(file, "sha256={actual_sha256}")?;
    if let Some(expected) = expected_sha256 {
        writeln!(file, "expected_sha256={}", expected.to_ascii_lowercase())?;
        writeln!(file, "sha256_match=1")?;
    }
    writeln!(file, "preview=chr_tiles.ppm")?;
    writeln!(file, "preview_sha256={preview_sha256}")?;
    writeln!(file, "png=chr_tiles.png")?;
    writeln!(file, "png_sha256={png_sha256}")?;
    writeln!(file, "width={width}")?;
    writeln!(file, "height={height}")?;
    writeln!(file, "tile_count={tile_count}")?;
    writeln!(file, "complete=1")?;
    Ok(())
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
            "lotw_tools_chr_preview_test_{}_{}",
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
        bytes[16 + 0x4000] = 0xff;
        bytes
    }

    #[test]
    fn writes_preview_manifest_and_ppm() {
        let root = temp_dir();
        let rom = root.join("fixture.nes");
        let out = root.join("out");
        let bytes = ines_fixture();
        let expected = sha256::digest_hex(&bytes);
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom, bytes).unwrap();

        run(&rom, &out, Some(&expected)).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        let ppm = fs::read(out.join("chr_tiles.ppm")).unwrap();
        let png = fs::read(out.join("chr_tiles.png")).unwrap();
        assert!(manifest.contains("runtime=rust_chr_preview\n"));
        assert!(manifest.contains("preview=chr_tiles.ppm\n"));
        assert!(manifest.contains("png=chr_tiles.png\n"));
        assert!(manifest.contains("sha256_match=1\n"));
        assert!(manifest.contains("width=512\n"));
        assert!(manifest.contains("height=64\n"));
        assert!(manifest.contains("tile_count=512\n"));
        assert!(ppm.starts_with(b"P6\n512 64\n255\n"));
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
