use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn compile(path: &Path, source: &str) {
    let source_path = path.with_extension("rs");
    fs::write(&source_path, source).unwrap();
    let output = Command::new("rustc")
        .arg("--edition=2021")
        .arg(&source_path)
        .arg("-o")
        .arg(path)
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "failed to launch rustc for {}: {err}",
                source_path.display()
            )
        });
    if !output.status.success() {
        panic!(
            "rustc failed for {}\nstdout:\n{}\nstderr:\n{}",
            source_path.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn compile_noop(path: &Path) {
    compile(path, "fn main() {}\n");
}

pub fn unique_temp_dir(name: &str) -> PathBuf {
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
