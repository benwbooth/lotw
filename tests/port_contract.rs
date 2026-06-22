use std::{fs, path::Path};

#[test]
fn repository_has_no_c_or_cpp_runtime_sources() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut bad = Vec::new();
    visit(root, root, &mut bad);
    assert!(bad.is_empty(), "leftover C/C++ files:\n{}", bad.join("\n"));
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
