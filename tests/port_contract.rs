use std::{fs, path::Path};

#[test]
fn repository_has_no_c_or_cpp_runtime_sources() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut bad = Vec::new();
    visit(root, root, &mut bad);
    assert!(bad.is_empty(), "leftover C/C++ files:\n{}", bad.join("\n"));
}

#[test]
fn player_does_not_tick_music_outside_vblank() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let play_rs = fs::read_to_string(root.join("src/bin/play.rs")).unwrap();
    assert!(
        !play_rs.contains("sound_tick"),
        "play.rs must not call sound_tick directly; vblank_commit_tail owns the per-frame music tick"
    );
}

fn visit(root: &Path, path: &Path, bad: &mut Vec<String>) {
    let rel = path.strip_prefix(root).unwrap_or(path);
    if rel
        .components()
        .any(|part| matches!(part.as_os_str().to_str(), Some(".git" | "target" | "build")))
    {
        return;
    }
    let Ok(meta) = fs::metadata(path) else {
        return;
    };
    if meta.is_dir() {
        let mut entries = fs::read_dir(path)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        entries.sort();
        for entry in entries {
            visit(root, &entry, bad);
        }
        return;
    }
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return;
    };
    let forbidden_name = name == "CMakeLists.txt";
    let forbidden_ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext, "c" | "cc" | "cpp" | "h" | "hpp"));
    if forbidden_name || forbidden_ext {
        bad.push(rel.display().to_string());
    }
}
