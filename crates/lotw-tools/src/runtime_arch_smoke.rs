use std::io;
use std::path::Path;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn run(lotw: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !is_executable(lotw) {
        return Err(format!(
            "runtime_arch_smoke: executable not found: {}",
            lotw.display()
        )
        .into());
    }

    let output = Command::new("nm")
        .arg("-a")
        .arg(lotw)
        .output()
        .map_err(|err| {
            if err.kind() == io::ErrorKind::NotFound {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "runtime_arch_smoke: missing required tool: nm",
                )
            } else {
                err
            }
        })?;

    let symbols = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).into_owned()
    } else {
        String::new()
    };

    let bad_symbols = symbols
        .lines()
        .filter(|line| line.contains("lotw_cpu6502") || line.contains("cpu6502"))
        .collect::<Vec<_>>();

    if !bad_symbols.is_empty() {
        eprintln!("runtime_arch_smoke: runtime links tooling CPU symbols");
        for symbol in bad_symbols {
            eprintln!("{symbol}");
        }
        return Err("runtime_arch_smoke: runtime links tooling CPU symbols".into());
    }

    println!(
        "runtime_arch_smoke: no tooling CPU symbols in {}",
        lotw.display()
    );
    Ok(())
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    path.is_file()
        && path
            .metadata()
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
