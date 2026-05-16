use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(repo_root: &Path, build_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Repository: {}", repo_root.display());
    println!("Build dir:  {}", build_dir.display());

    match source_rom(build_dir) {
        Some(path) => println!("ROM input:  {}", path.display()),
        None => println!("ROM input:  not found"),
    }

    for tool in ["cargo", "rustc", "rustfmt", "clippy", "7z", "fceux"] {
        match find_tool(tool) {
            Some(path) => println!("{tool}:       {}", path.display()),
            None => println!("{tool}:       missing"),
        }
    }

    Ok(())
}

fn source_rom(build_dir: &Path) -> Option<PathBuf> {
    if let Some(path) = env::var_os("LOTW_ROM").filter(|value| !value.is_empty()) {
        return Some(PathBuf::from(path));
    }

    let default_archive =
        Path::new("/mnt/roms/emudeck/Emulation/roms/nes/Legacy of the Wizard (USA).zip");
    if default_archive.is_file() {
        return Some(default_archive.to_path_buf());
    }

    let default_nes = build_dir.join("rom").join("Legacy of the Wizard (USA).nes");
    if default_nes.is_file() {
        return Some(default_nes);
    }

    None
}

fn find_tool(tool: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    for dir in env::split_paths(&path) {
        let candidate = dir.join(tool);
        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }
    None
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_rom_prefers_env_path() {
        let previous = env::var_os("LOTW_ROM");
        env::set_var("LOTW_ROM", "/tmp/custom-lotw.nes");
        let actual = source_rom(Path::new("/tmp/build")).expect("env path");
        match previous {
            Some(value) => env::set_var("LOTW_ROM", value),
            None => env::remove_var("LOTW_ROM"),
        }
        assert_eq!(actual, PathBuf::from("/tmp/custom-lotw.nes"));
    }
}
