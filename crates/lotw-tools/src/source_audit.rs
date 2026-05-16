use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

const REQUIRED_IGNORES: &[&str] = &[
    "build/",
    "generated/",
    "*.nes",
    "*.fds",
    "*.nsf",
    "*.sav",
    "*.srm",
    "*.zip",
    "*.7z",
];

const FORBIDDEN_EXTENSIONS: &[&str] = &[
    "nes", "fds", "nsf", "sav", "srm", "zip", "7z", "c", "cc", "cpp", "cxx", "h", "hpp", "cmake",
];

const FORBIDDEN_BASENAMES: &[&str] = &[
    "apu_register_counts.tsv",
    "apu_writes.tsv",
    "audio_summary.txt",
    "background_frame.ppm",
    "block_candidates.tsv",
    "block_exec.tsv",
    "block_exec_verify.txt",
    "block_external_writes.tsv",
    "block_state_exec.tsv",
    "block_state_external_writes.tsv",
    "block_translation_plan.tsv",
    "capture_manifest.txt",
    "CMakeLists.txt",
    "chr.bin",
    "chr_tiles.png",
    "chr_tiles.ppm",
    "coverage_summary.tsv",
    "expected_input_trace.tsv",
    "expected_native_external_writes.tsv",
    "executed_labels.tsv",
    "external_block_plan.tsv",
    "external_block_summary.txt",
    "external_write_site_summary.tsv",
    "frame_compare.txt",
    "frame_hashes.sha256",
    "label_state_hits.tsv",
    "label_states.tsv",
    "lotw_rom_info.h",
    "manifest.txt",
    "mapper_writes.tsv",
    "native_block_chain.tsv",
    "native_block_chain_cases.tsv",
    "native_block_codegen.tsv",
    "native_block_external_run_cases.tsv",
    "native_block_external_verify_cases.tsv",
    "native_block_final_states.tsv",
    "native_block_hit_final_states.tsv",
    "native_block_hit_state_cases.tsv",
    "native_block_hit_verify_cases.tsv",
    "native_block_live_frame_schedule.tsv",
    "native_block_live_order.tsv",
    "native_block_run.tsv",
    "native_block_run_cases.tsv",
    "native_block_run_external_writes.tsv",
    "native_block_run_maximal.tsv",
    "native_block_run_maximal_summary.tsv",
    "native_block_runtime_trace.tsv",
    "native_block_runtime_trace_verify.txt",
    "native_block_sequence.tsv",
    "native_block_sequence_cases.tsv",
    "native_block_static_merge.tsv",
    "native_block_static_merge_summary.txt",
    "native_block_static_verify_cases.tsv",
    "native_block_static_verify_summary.txt",
    "native_block_summary.tsv",
    "native_block_transition.tsv",
    "native_block_transition_summary.tsv",
    "native_block_verify.tsv",
    "native_block_verify_cases.tsv",
    "native_blocks.tsv",
    "native_opcode_summary.tsv",
    "oam_dma.tsv",
    "opcode_summary.tsv",
    "port_apu_writes.tsv",
    "port_input_trace.tsv",
    "port_label_states.tsv",
    "port_mapper_writes.tsv",
    "port_oam_dma.tsv",
    "port_ppu_vram_writes.tsv",
    "port_ppu_writes.tsv",
    "port_run.log",
    "port_summary.txt",
    "port_trace_hashes.sha256",
    "port_trace_summary.txt",
    "progress_summary.txt",
    "ppu_background_hashes.sha256",
    "ppu_render_compare.txt",
    "ppu_render_diff.ppm",
    "ppu_render_hashes.sha256",
    "ppu_render_pixels.txt",
    "ppu_state_summary.txt",
    "ppu_vram_writes.tsv",
    "ppu_writes.tsv",
    "prg.bin",
    "ram_hashes.sha256",
    "reference_frame_hashes.tsv",
    "reference_inputs.tsv",
    "reference_ram_hashes.tsv",
    "reference_summary.txt",
    "replay_block_verify.tsv",
    "replay_frame_compare.tsv",
    "replay_ppu_render_compare.tsv",
    "replay_runtime_native_live_frame.tsv",
    "replay_runtime_native_live_frame_maximal.tsv",
    "replay_runtime_native_live_frame_ppu.tsv",
    "replay_runtime_ppu_seed.tsv",
    "replay_runtime_trace_seed.tsv",
    "replay_smoke_summary.txt",
    "replay_trace_compare.tsv",
    "replay_trace_roundtrip.tsv",
    "runtime_audio.log",
    "runtime_native_blocks.log",
    "runtime_native_external_trace.log",
    "runtime_native_live.log",
    "runtime_native_live_frame.log",
    "runtime_native_live_frame_audio_summary.txt",
    "runtime_native_live_frame_ppu.log",
    "runtime_native_live_frame_ppu_compare.txt",
    "runtime_native_live_frame_ppu_diff.ppm",
    "runtime_native_live_frame_ppu_pixels.txt",
    "runtime_native_live_frame_summary.txt",
    "runtime_native_live_summary.txt",
    "runtime_native_trace_verify.txt",
    "runtime_ppu_seed_compare.txt",
    "runtime_ppu_seed_diff.ppm",
    "runtime_ppu_seed_pixels.txt",
    "semantic_match_summary.txt",
    "semantic_matched_units.tsv",
    "static_branch_native_blocks.tsv",
    "static_branch_outcomes.tsv",
    "static_branch_skipped.tsv",
    "static_branch_state_cases.tsv",
    "static_branch_targets.tsv",
    "static_branch_verify_summary.txt",
    "static_frontier_match_status.tsv",
    "static_handoff_native_blocks.tsv",
    "static_handoff_plan.tsv",
    "static_handoff_plan_summary.txt",
    "static_handoff_skipped.tsv",
    "static_handoff_state_cases.tsv",
    "static_handoff_verify_summary.txt",
    "static_jsr_native_blocks.tsv",
    "static_jsr_outcomes.tsv",
    "static_jsr_skipped.tsv",
    "static_jsr_state_cases.tsv",
    "static_jsr_targets.tsv",
    "static_jsr_verify_summary.txt",
    "static_leaf_native_blocks.tsv",
    "static_leaf_skipped.tsv",
    "static_leaf_state_cases.tsv",
    "static_leaf_verify_summary.txt",
    "static_return_native_blocks.tsv",
    "static_return_skipped.tsv",
    "static_return_state_cases.tsv",
    "static_return_verify_summary.txt",
    "static_rom_audit.tsv",
    "static_rom_audit_summary.txt",
    "static_rom_frontier.tsv",
    "trace_compare.txt",
    "trace_hashes.sha256",
    "trace_summary.txt",
    "unsupported_opcodes.tsv",
];

pub fn run(repo_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !repo_root.join(".git").is_dir() {
        return Err(format!(
            "source_audit: not a git repository: {}",
            repo_root.display()
        )
        .into());
    }

    let gitignore = repo_root.join(".gitignore");
    if !gitignore.is_file() {
        return Err("source_audit: missing .gitignore".into());
    }

    let ignore_lines = fs::read_to_string(&gitignore)?
        .lines()
        .map(str::to_string)
        .collect::<HashSet<_>>();
    for entry in REQUIRED_IGNORES {
        if !ignore_lines.contains(*entry) {
            return Err(
                format!("source_audit: .gitignore is missing required entry: {entry}").into(),
            );
        }
    }

    let tracked = tracked_files(repo_root)?;
    let bad_files = tracked
        .iter()
        .filter(|path| is_forbidden_tracked_path(path))
        .cloned()
        .collect::<Vec<_>>();

    if !bad_files.is_empty() {
        eprintln!("source_audit: forbidden generated or ROM-derived files are tracked:");
        for path in bad_files {
            eprintln!("{path}");
        }
        return Err(
            "source_audit: tracked files include generated or ROM-derived artifacts".into(),
        );
    }

    println!("source_audit: tracked files are clean");
    Ok(())
}

fn tracked_files(repo_root: &Path) -> io::Result<Vec<String>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("ls-files")
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "source_audit: git ls-files failed with {}",
            output.status
        )));
    }

    let stdout = String::from_utf8(output.stdout).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("source_audit: git ls-files was not UTF-8: {err}"),
        )
    })?;
    Ok(stdout.lines().map(str::to_string).collect())
}

fn is_forbidden_tracked_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    if normalized
        .split('/')
        .any(|component| component == "build" || component == "generated")
    {
        return true;
    }

    let basename = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    if FORBIDDEN_BASENAMES.contains(&basename) {
        return true;
    }

    if let Some(extension) = basename.rsplit_once('.').map(|(_, ext)| ext.to_lowercase()) {
        if FORBIDDEN_EXTENSIONS.contains(&extension.as_str()) {
            return true;
        }
    }

    has_digit_prefixed_ppm(basename, "frame_")
        || basename.starts_with("port_frame_") && basename.ends_with(".ppm")
        || basename.starts_with("background_frame_") && basename.ends_with(".ppm")
        || basename.starts_with("ppu_frame_") && basename.ends_with(".ppm")
        || basename.starts_with("runtime_ppu_frame_") && basename.ends_with(".ppm")
        || basename.starts_with("ram_") && basename.ends_with(".bin")
        || (basename.starts_with("lotw_generated_")
            && (basename.ends_with(".c") || basename.ends_with(".h")))
}

fn has_digit_prefixed_ppm(basename: &str, prefix: &str) -> bool {
    basename.strip_prefix(prefix).is_some_and(|rest| {
        rest.ends_with(".ppm") && rest.starts_with(|c: char| c.is_ascii_digit())
    })
}
