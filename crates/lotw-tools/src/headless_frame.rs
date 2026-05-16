use lotw_port::rom::InesRom;
use lotw_port::runtime;
use lotw_port::sha256;
use lotw_port::video;
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
    let boot = runtime::render_boot_frame(&rom);
    let ppm = video::frame_ppm(&boot.frame);
    let frame_sha256 = sha256::digest_hex(&ppm);

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    fs::write(out_dir.join("frame.ppm"), ppm)?;
    write_manifest(
        &out_dir.join("manifest.txt"),
        rom_path,
        &actual_sha256,
        expected_sha256,
        &frame_sha256,
        boot.frame.width,
        boot.frame.height,
        boot.page,
        boot.page_count,
    )?;

    println!("headless_frame: wrote {}", out_dir.display());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_manifest(
    path: &Path,
    rom_path: &Path,
    actual_sha256: &str,
    expected_sha256: Option<&str>,
    frame_sha256: &str,
    width: usize,
    height: usize,
    page: usize,
    page_count: usize,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=rust_headless_frame")?;
    writeln!(file, "rom={}", rom_path.display())?;
    writeln!(file, "sha256={actual_sha256}")?;
    if let Some(expected) = expected_sha256 {
        writeln!(file, "expected_sha256={}", expected.to_ascii_lowercase())?;
        writeln!(file, "sha256_match=1")?;
    }
    writeln!(file, "frame=frame.ppm")?;
    writeln!(file, "frame_sha256={frame_sha256}")?;
    writeln!(file, "width={width}")?;
    writeln!(file, "height={height}")?;
    writeln!(file, "page={page}")?;
    writeln!(file, "page_count={page_count}")?;
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
            "lotw_tools_headless_frame_test_{}_{}",
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
    fn writes_headless_frame_manifest_and_ppm() {
        let root = temp_dir();
        let rom = root.join("fixture.nes");
        let out = root.join("out");
        let bytes = ines_fixture();
        let expected = sha256::digest_hex(&bytes);
        fs::create_dir_all(&root).unwrap();
        fs::write(&rom, bytes).unwrap();

        run(&rom, &out, Some(&expected)).unwrap();

        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        let ppm = fs::read(out.join("frame.ppm")).unwrap();
        assert!(manifest.contains("runtime=rust_headless_frame\n"));
        assert!(manifest.contains("frame=frame.ppm\n"));
        assert!(manifest.contains("sha256_match=1\n"));
        assert!(manifest.contains("width=256\n"));
        assert!(manifest.contains("height=240\n"));
        assert!(manifest.contains("page=0\n"));
        assert!(ppm.starts_with(b"P6\n256 240\n255\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
