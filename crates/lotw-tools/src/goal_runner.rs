use crate::{
    apu_trace, block_exec, block_exec_verify, block_translation_plan, blocks, chr_preview,
    coverage_report, decomp_worklist, disasm, external_block_plan, fceux_capture, goal_status,
    headless_frame, native_block_plan, native_block_run_maximal, native_block_runtime_trace_verify,
    native_block_static_merge, native_block_transition, progress_report, reference_hash_report,
    replay_dump, replay_smoke, rom_extract, rom_info, rust_port_capture, semantic_match_report,
    source_audit, static_cfg_gap, static_entry_plan, static_handoff_plan, static_handoff_verify,
    static_proof_accumulate, static_rom_audit, symbol_audit, trace_compare, whole_program_report,
};
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXPECTED_ROM_SHA256: &str =
    "079f648d669966357fe4414a986573eacd7ecadf5c4f289c288427b8c5f491f1";
const DEFAULT_ARCHIVE: &str = "/mnt/roms/emudeck/Emulation/roms/nes/Legacy of the Wizard (USA).zip";
const DEFAULT_ROM_NAME: &str = "Legacy of the Wizard (USA).nes";

const REPLAYS: &[&str] = &[
    "title_idle",
    "start_game",
    "gameplay_walk",
    "gameplay_climb",
    "pochi_move",
    "password_prompt",
    "room_transition",
    "outside_walk",
    "door_return",
];

const TRACE_SPECS: &[(&str, &str, &str)] = &[
    ("title_idle", "LOTW_TRACE_FRAMES", "180"),
    ("start_game", "LOTW_GAMEPLAY_TRACE_FRAMES", "420"),
    ("gameplay_walk", "LOTW_WALK_TRACE_FRAMES", "732"),
    ("gameplay_climb", "LOTW_CLIMB_TRACE_FRAMES", "960"),
    ("pochi_move", "LOTW_POCHI_TRACE_FRAMES", "906"),
    ("password_prompt", "LOTW_PASSWORD_TRACE_FRAMES", "804"),
    ("room_transition", "LOTW_ROOM_TRACE_FRAMES", "1261"),
    ("outside_walk", "LOTW_OUTSIDE_TRACE_FRAMES", "1831"),
    ("door_return", "LOTW_DOOR_TRACE_FRAMES", "1711"),
];

#[derive(Debug, Clone, Copy)]
enum StaticVerifier {
    Leaf,
    Handoff,
    Branch,
    Jsr,
    Return,
}

impl StaticVerifier {
    fn kind(self) -> &'static str {
        match self {
            Self::Leaf => "leaf",
            Self::Handoff => "handoff",
            Self::Branch => "branch",
            Self::Jsr => "jsr",
            Self::Return => "return",
        }
    }

    fn summary_name(self) -> &'static str {
        match self {
            Self::Leaf => "static_leaf_verify_summary.txt",
            Self::Handoff => "static_handoff_verify_summary.txt",
            Self::Branch => "static_branch_verify_summary.txt",
            Self::Jsr => "static_jsr_verify_summary.txt",
            Self::Return => "static_return_verify_summary.txt",
        }
    }
}

struct GoalContext {
    repo_root: PathBuf,
    build_dir: PathBuf,
    rom_cache: PathBuf,
    default_archive: PathBuf,
    default_nes: PathBuf,
}

pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = GoalContext::new()?;
    match args.first().map(String::as_str) {
        None | Some("") | Some("help") | Some("-h") | Some("--help") => {
            usage();
            Ok(())
        }
        Some("status") => goal_status::run(&ctx.repo_root, &ctx.build_dir),
        Some("source-audit") => source_audit::run(&ctx.repo_root),
        Some("symbol-audit") => symbol_audit::run(&ctx.repo_root.join("symbols.yaml")),
        Some("rom") => {
            println!("{}", ctx.ensure_rom()?.display());
            Ok(())
        }
        Some("rust-rom") => ctx.cmd_rust_rom(),
        Some("rust-chr-preview") => ctx.cmd_rust_chr_preview(),
        Some("rust-headless-frame") => ctx.cmd_rust_headless_frame(),
        Some("rust-runtime") => ctx.cmd_rust_runtime(),
        Some("rust-runtime-window") => ctx.cmd_rust_runtime_window(),
        Some("rust-runtime-audio-trace") => ctx.cmd_rust_runtime_audio_trace(),
        Some("rust-port-capture") => ctx.cmd_rust_port_capture(),
        Some("rust-trace-compare") => ctx.cmd_rust_trace_compare(),
        Some("rust-replay-dump") => ctx.cmd_rust_replay_dump(),
        Some("run") => ctx.cmd_rust_runtime_window(),
        Some("configure") => ctx.cmd_configure_rust(),
        Some("build") => ctx.cmd_build_rust(),
        Some("test") => ctx.cmd_test_rust(),
        Some("clean") => remove_path(&ctx.build_dir).map_err(Into::into),
        Some("reference") => ctx.cmd_reference_current(),
        Some("disasm") => ctx.cmd_disasm(),
        Some("trace") => ctx.cmd_trace_current(),
        Some("audio-trace") => ctx.cmd_audio_trace(),
        Some("blocks") => ctx.cmd_blocks_current(),
        Some("block-exec") => ctx.cmd_block_exec_current(),
        Some("block-verify") => ctx.cmd_block_verify_current(),
        Some("coverage") => ctx.cmd_coverage(),
        Some("coverage-report") => ctx.cmd_coverage_report(),
        Some("block-translation-plan") => ctx.cmd_block_translation_plan(),
        Some("external-block-plan") => ctx.cmd_external_block_plan(),
        Some("static-cfg") => ctx.cmd_static_cfg(),
        Some("static-entry-plan") => ctx.cmd_static_entry_plan(),
        Some("static-leaf-verify") => ctx.cmd_static_leaf_verify(),
        Some("static-rom-audit") => ctx.cmd_static_rom_audit(),
        Some("static-handoff-plan") => ctx.cmd_static_handoff_plan(),
        Some("static-handoff-verify") => ctx.cmd_static_handoff_verify(false, false),
        Some("static-branch-verify") => ctx.cmd_static_branch_verify(false, false),
        Some("static-jsr-verify") => ctx.cmd_static_jsr_verify(false, false),
        Some("static-return-verify") => ctx.cmd_static_return_verify(false, false),
        Some("native-block-plan") => ctx.cmd_native_block_plan(false),
        Some("native-block-codegen") => ctx.cmd_native_block_codegen(),
        Some("native-block-static-merge") => ctx.cmd_native_block_static_merge(),
        Some("native-block-codegen-static") => ctx.cmd_native_block_codegen_static(),
        Some("native-block-static-verify") => ctx.cmd_native_block_static_verify(),
        Some("native-block-verify") => ctx.cmd_native_block_verify(),
        Some("native-block-hit-verify") => ctx.cmd_native_block_hit_verify(),
        Some("native-block-trace") => ctx.cmd_native_block_trace(),
        Some("native-block-transition") => ctx.cmd_native_block_transition(),
        Some("native-block-sequence") => ctx.cmd_native_block_sequence(),
        Some("native-block-chain") => ctx.cmd_native_block_chain(),
        Some("native-block-run") => ctx.cmd_native_block_run(),
        Some("native-block-run-maximal") => ctx.cmd_native_block_run_maximal(),
        Some("native-block-runtime-trace") => ctx.cmd_native_block_runtime_trace(),
        Some("replay-smoke") => ctx.cmd_replay_smoke(),
        Some("reference-hash-harness") => ctx.cmd_reference_hash_harness(),
        Some("progress") => ctx.cmd_progress(),
        Some("decomp-worklist") | Some("worklist") => ctx.cmd_decomp_worklist(),
        Some("semantic-match-report") => ctx.cmd_semantic_match_report(),
        Some("whole-program-report") => ctx.cmd_whole_program_report(),
        Some("replay-dump") => ctx.cmd_replay_dump(),
        Some("extract") => ctx.cmd_extract(),
        Some(other) => Err(format!("goal: unknown command {other:?}").into()),
    }
}

fn usage() {
    eprintln!("Usage: cargo run --manifest-path Cargo.toml -p lotw-tools -- goal <command>");
    eprintln!();
    eprintln!("Rust-orchestrated commands:");
    eprintln!("  status source-audit rom rust-rom rust-chr-preview rust-headless-frame");
    eprintln!("  rust-runtime rust-runtime-window run configure build test clean");
    eprintln!("  extract reference rust-port-capture");
    eprintln!("  rust-trace-compare");
    eprintln!("  disasm trace blocks block-exec coverage coverage-report");
    eprintln!("  block-translation-plan static-cfg static-entry-plan static-leaf-verify");
    eprintln!("  static-rom-audit static-handoff-plan static-handoff-verify");
    eprintln!("  static-branch-verify static-jsr-verify static-return-verify");
    eprintln!("  native-block-plan native-block-static-merge native-block-codegen-static");
    eprintln!("  native-block-static-verify semantic-match-report whole-program-report");
    eprintln!("  native-block-verify native-block-hit-verify native-block-transition");
    eprintln!("  native-block-sequence native-block-chain native-block-run");
    eprintln!("  native-block-run-maximal native-block-runtime-trace replay-smoke");
    eprintln!("  rust-replay-dump reference-hash-harness progress audio-trace block-verify");
    eprintln!("  symbol-audit decomp-worklist worklist");
    eprintln!();
    eprintln!("Rust-only migration is the target; remaining C callers are migration debt.");
}

impl GoalContext {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let repo_root = manifest_dir
            .parent()
            .and_then(Path::parent)
            .map(Path::to_path_buf)
            .ok_or("goal: could not determine repository root")?;
        let build_dir = env::var_os("LOTW_BUILD_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| repo_root.join("build"));
        let rom_cache = build_dir.join("rom");
        let default_nes = rom_cache.join(DEFAULT_ROM_NAME);
        Ok(Self {
            repo_root,
            build_dir,
            rom_cache,
            default_archive: PathBuf::from(DEFAULT_ARCHIVE),
            default_nes,
        })
    }

    fn source_rom(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(path) = env::var_os("LOTW_ROM") {
            return Ok(PathBuf::from(path));
        }
        if self.default_archive.is_file() {
            return Ok(self.default_archive.clone());
        }
        if self.default_nes.is_file() {
            return Ok(self.default_nes.clone());
        }
        Err("goal: could not find ROM; set LOTW_ROM or place the archive in /mnt/roms".into())
    }

    fn ensure_rom(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let source = self.source_rom()?;
        fs::create_dir_all(&self.rom_cache)?;

        match source
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("nes") => Ok(source),
            Some("zip" | "7z") => {
                if !self.default_nes.is_file() {
                    let out_arg = format!("-o{}", self.rom_cache.display());
                    run_status(
                        Command::new("7z")
                            .arg("x")
                            .arg(&source)
                            .arg(out_arg)
                            .arg("-y"),
                        "7z ROM extraction",
                    )?;
                }
                Ok(self.default_nes.clone())
            }
            _ => Err(format!("goal: unsupported ROM input: {}", source.display()).into()),
        }
    }

    fn replay_name(&self) -> String {
        env::var("LOTW_REPLAY_NAME").unwrap_or_else(|_| "title_idle".to_string())
    }

    fn replay_path(&self, name: &str) -> PathBuf {
        env::var_os("LOTW_REPLAY")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                self.repo_root
                    .join("fixtures")
                    .join("reference")
                    .join(format!("{name}.replay"))
            })
    }

    fn replay_names(&self) -> Vec<String> {
        REPLAYS.iter().map(|name| (*name).to_string()).collect()
    }

    fn cmd_rust_rom(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        rom_info::run(
            &rom,
            &self.build_dir.join("rust_rom"),
            Some(EXPECTED_ROM_SHA256),
        )
    }

    fn cmd_rust_chr_preview(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        chr_preview::run(
            &rom,
            &self.build_dir.join("rust_chr_preview"),
            Some(EXPECTED_ROM_SHA256),
        )
    }

    fn cmd_rust_headless_frame(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        headless_frame::run(
            &rom,
            &self.build_dir.join("rust_headless_frame"),
            Some(EXPECTED_ROM_SHA256),
        )
    }

    fn cmd_rust_runtime(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        let name = self.replay_name();
        let out_dir = self.build_dir.join("rust_runtime").join(&name);
        let input_trace = self
            .build_dir
            .join("rust_runtime")
            .join(format!("{name}_input_trace.tsv"));
        let replay = self.replay_path(&name);
        let frames = env::var("LOTW_RUST_RUNTIME_FRAMES").unwrap_or_else(|_| "180".to_string());
        self.run_runtime(
            false,
            &[
                OsString::from("--headless"),
                OsString::from("--out-dir"),
                out_dir.as_os_str().to_os_string(),
                OsString::from("--dump-input-trace"),
                input_trace.as_os_str().to_os_string(),
                OsString::from("--dump-trace-dir"),
                out_dir.as_os_str().to_os_string(),
                OsString::from("--replay"),
                replay.as_os_str().to_os_string(),
                OsString::from("--frames"),
                OsString::from(frames),
                OsString::from("--expected-sha256"),
                OsString::from(EXPECTED_ROM_SHA256),
                rom.as_os_str().to_os_string(),
            ],
        )
    }

    fn cmd_rust_runtime_audio_trace(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        let name = self.replay_name();
        let frames = env::var("LOTW_RUST_RUNTIME_AUDIO_TRACE_FRAMES")
            .or_else(|_| env::var("LOTW_TRACE_FRAMES"))
            .unwrap_or_else(|_| "180".to_string());
        self.cmd_trace_for(&name, &frames)?;

        let out_dir = self.build_dir.join("rust_runtime_audio_trace").join(&name);
        let summary = out_dir.join("audio_summary.txt");
        self.run_runtime(
            false,
            &[
                OsString::from("--headless"),
                OsString::from("--out-dir"),
                out_dir.as_os_str().to_os_string(),
                OsString::from("--apu-trace"),
                self.build_dir
                    .join("trace")
                    .join(&name)
                    .join("apu_writes.tsv")
                    .as_os_str()
                    .to_os_string(),
                OsString::from("--dump-audio-summary"),
                summary.as_os_str().to_os_string(),
                OsString::from("--frames"),
                OsString::from(frames),
                OsString::from("--expected-sha256"),
                OsString::from(EXPECTED_ROM_SHA256),
                rom.as_os_str().to_os_string(),
            ],
        )?;

        let values = read_key_values(&summary)?;
        require_key_value(&values, "runtime", "rust_native_port_headless", &summary)?;
        require_key_value(&values, "backend", "rust_audio_stub", &summary)?;
        require_key_value(&values, "complete", "1", &summary)?;
        let events = required_value(&values, "apu_trace_events", &summary)?;
        if events != required_value(&values, "apu_trace_events_played", &summary)?
            || events != required_value(&values, "audio_apu_write_count", &summary)?
        {
            return Err(format!(
                "rust-runtime-audio-trace: expected all APU events to be played in {}",
                summary.display()
            )
            .into());
        }
        println!("rust-runtime-audio-trace: wrote {}", summary.display());
        Ok(())
    }

    fn cmd_rust_port_capture(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        let frame = env::var("LOTW_RUST_PORT_CAPTURE_FRAME").unwrap_or_else(|_| "180".to_string());
        self.cmd_rust_port_capture_for(&name, &frame)
    }

    fn cmd_rust_port_capture_for(
        &self,
        name: &str,
        frame: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        let replay = self.replay_path(name);
        let out_dir = self.build_dir.join("rust_port_capture").join(name);
        let frame_file = format!(
            "port_frame_{:06}.ppm",
            parse_positive_usize(frame, "frame")?
        );

        self.run_runtime(
            false,
            &[
                OsString::from("--headless"),
                OsString::from("--out-dir"),
                out_dir.as_os_str().to_os_string(),
                OsString::from("--dump-frame"),
                out_dir.join(frame_file).as_os_str().to_os_string(),
                OsString::from("--dump-input-trace"),
                out_dir
                    .join("port_input_trace.tsv")
                    .as_os_str()
                    .to_os_string(),
                OsString::from("--dump-trace-dir"),
                out_dir.as_os_str().to_os_string(),
                OsString::from("--replay"),
                replay.as_os_str().to_os_string(),
                OsString::from("--frames"),
                OsString::from(frame),
                OsString::from("--expected-sha256"),
                OsString::from(EXPECTED_ROM_SHA256),
                rom.as_os_str().to_os_string(),
            ],
        )?;
        rust_port_capture::run(&out_dir, &rom, &replay, frame)
    }

    fn cmd_rust_trace_compare(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        let frames = env::var("LOTW_RUST_TRACE_COMPARE_FRAMES")
            .or_else(|_| env::var("LOTW_TRACE_COMPARE_FRAMES"))
            .or_else(|_| env::var("LOTW_TRACE_FRAMES"))
            .unwrap_or_else(|_| "180".to_string());
        self.cmd_trace_for(&name, &frames)?;
        self.cmd_rust_port_capture_for(&name, &frames)?;
        trace_compare::run(
            &self.build_dir.join("trace").join(&name),
            &self.build_dir.join("rust_port_capture").join(&name),
            &self.build_dir.join("rust_trace_compare").join(&name),
        )
    }

    fn cmd_rust_replay_dump(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        let limit = Some(env_usize("LOTW_RUST_REPLAY_FRAME_LIMIT", 32)?);
        replay_dump::run(
            &self.replay_path(&name),
            &self.build_dir.join("rust_replay").join(&name),
            limit,
        )
    }

    fn cmd_rust_runtime_window(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        let frames =
            env::var("LOTW_RUST_RUNTIME_WINDOW_FRAMES").unwrap_or_else(|_| "1".to_string());
        let scale = env::var("LOTW_RUST_RUNTIME_SCALE").unwrap_or_else(|_| "3".to_string());
        self.run_runtime(
            true,
            &[
                OsString::from("--window"),
                OsString::from("--frames"),
                OsString::from(frames),
                OsString::from("--scale"),
                OsString::from(scale),
                OsString::from("--expected-sha256"),
                OsString::from(EXPECTED_ROM_SHA256),
                rom.as_os_str().to_os_string(),
            ],
        )
    }

    fn run_runtime(&self, sdl: bool, args: &[OsString]) -> Result<(), Box<dyn std::error::Error>> {
        let mut command = Command::new("cargo");
        command
            .current_dir(&self.repo_root)
            .arg("run")
            .arg("--quiet")
            .arg("--manifest-path")
            .arg(self.repo_root.join("Cargo.toml"))
            .arg("-p")
            .arg("lotw-runtime");
        if sdl {
            command.arg("--features").arg("sdl");
        }
        command.arg("--").args(args);
        run_status(&mut command, "lotw-runtime")
    }

    fn cmd_configure_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("configure: Cargo workspace has no configure step");
        Ok(())
    }

    fn cmd_build_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        run_status(
            Command::new("cargo")
                .current_dir(&self.repo_root)
                .arg("build")
                .arg("--workspace"),
            "cargo build --workspace",
        )
    }

    fn cmd_test_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        source_audit::run(&self.repo_root)?;
        symbol_audit::run(&self.repo_root.join("symbols.yaml"))?;
        run_status(
            Command::new("cargo")
                .current_dir(&self.repo_root)
                .arg("fmt")
                .arg("--all")
                .arg("--check"),
            "cargo fmt --all --check",
        )?;
        run_status(
            Command::new("cargo")
                .current_dir(&self.repo_root)
                .arg("test")
                .arg("--workspace"),
            "cargo test --workspace",
        )?;
        run_status(
            Command::new("cargo")
                .current_dir(&self.repo_root)
                .arg("clippy")
                .arg("--workspace")
                .arg("--")
                .arg("-D")
                .arg("warnings"),
            "cargo clippy --workspace",
        )?;
        run_status(
            Command::new("cargo")
                .current_dir(&self.repo_root)
                .arg("clippy")
                .arg("-p")
                .arg("lotw-runtime")
                .arg("--features")
                .arg("sdl")
                .arg("--")
                .arg("-D")
                .arg("warnings"),
            "cargo clippy -p lotw-runtime --features sdl",
        )
    }

    fn cmd_disasm(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        disasm::run(
            &rom,
            &self.build_dir.join("disasm"),
            Some(EXPECTED_ROM_SHA256),
        )
    }

    fn cmd_extract(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        rom_extract::run(
            &rom,
            &self.build_dir.join("generated"),
            Some(EXPECTED_ROM_SHA256),
        )
    }

    fn cmd_reference_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        let name = self.replay_name();
        let frames = env::var("LOTW_REFERENCE_FRAMES").unwrap_or_else(|_| "1,60,120,180".into());
        fceux_capture::reference_run(
            &rom,
            &self.build_dir.join("reference").join(&name),
            Some(&self.replay_path(&name)),
            Some(&frames),
        )
    }

    fn cmd_trace_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        let frames = env::var("LOTW_TRACE_FRAMES").unwrap_or_else(|_| "180".to_string());
        self.cmd_trace_for(&name, &frames)
    }

    fn cmd_trace_for(&self, name: &str, frames: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        self.cmd_disasm()?;
        fceux_capture::trace_run(
            &rom,
            &self.build_dir.join("trace").join(name),
            &self.build_dir.join("disasm").join("labels.txt"),
            Some(&self.replay_path(name)),
            Some(frames),
        )
    }

    fn cmd_audio_trace(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        self.cmd_trace_current()?;
        apu_trace::run(
            &self
                .build_dir
                .join("trace")
                .join(&name)
                .join("apu_writes.tsv"),
            &self.build_dir.join("audio_trace").join(&name),
        )
    }

    fn cmd_blocks_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        let frames = env::var("LOTW_TRACE_FRAMES").unwrap_or_else(|_| "180".to_string());
        self.cmd_blocks_for(&name, &frames)
    }

    fn cmd_blocks_for(&self, name: &str, frames: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        self.cmd_trace_for(name, frames)?;
        blocks::run(
            &rom,
            &self
                .build_dir
                .join("trace")
                .join(name)
                .join("executed_labels.tsv"),
            &self.build_dir.join("blocks").join(name),
            Some(EXPECTED_ROM_SHA256),
        )
    }

    fn cmd_block_exec_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        let frames = env::var("LOTW_TRACE_FRAMES").unwrap_or_else(|_| "180".to_string());
        self.cmd_block_exec_for(&name, &frames)
    }

    fn cmd_block_exec_for(
        &self,
        name: &str,
        frames: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        self.cmd_blocks_for(name, frames)?;
        let max_steps = env::var("LOTW_BLOCK_EXEC_STEPS").unwrap_or_else(|_| "64".to_string());
        block_exec::run(
            &rom,
            &self
                .build_dir
                .join("blocks")
                .join(name)
                .join("block_candidates.tsv"),
            &self.build_dir.join("block_exec").join(name),
            max_steps.parse().unwrap_or(64).max(1),
            Some(
                &self
                    .build_dir
                    .join("trace")
                    .join(name)
                    .join("label_states.tsv"),
            ),
        )
    }

    fn cmd_block_verify_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        self.cmd_block_exec_current()?;
        block_exec_verify::run(
            &self.build_dir.join("block_exec").join(&name),
            &self.build_dir.join("block_verify").join(&name),
        )
    }

    fn cmd_coverage(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (name, env_key, default_frames) in TRACE_SPECS {
            let frames = env::var(env_key).unwrap_or_else(|_| default_frames.to_string());
            self.cmd_block_exec_for(name, &frames)?;
        }
        self.cmd_coverage_report()
    }

    fn cmd_coverage_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        coverage_report::run(
            &self.build_dir,
            &self.build_dir.join("coverage"),
            &REPLAYS
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>(),
        )
    }

    fn cmd_external_block_plan(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_block_translation_plan()?;
        external_block_plan::run(
            &self.build_dir,
            &self.build_dir.join("external_block_plan"),
            &self.replay_names(),
        )
    }

    fn cmd_block_translation_plan(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_coverage()?;
        block_translation_plan::run(
            &self.build_dir,
            &self.build_dir.join("block_translation_plan"),
            &REPLAYS
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>(),
        )
    }

    fn cmd_static_cfg(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_block_translation_plan()?;
        static_cfg_gap::run(
            &self.build_dir,
            &self.build_dir.join("static_cfg"),
            &REPLAYS
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>(),
        )
    }

    fn cmd_static_entry_plan(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_static_cfg()?;
        static_entry_plan::run(
            &self.build_dir,
            &self.build_dir.join("static_entry_plan"),
            env_usize("LOTW_STATIC_ENTRY_PLAN_LIMIT", 2048)?,
        )
    }

    fn cmd_static_leaf_verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_static_entry_plan()?;
        let rom = self.ensure_rom()?;
        self.run_static_proof_verifier(
            StaticVerifier::Leaf,
            &self.build_dir.join("static_leaf_verify"),
            &rom,
            env_usize("LOTW_STATIC_LEAF_LIMIT", 256)?,
            false,
        )
    }

    fn cmd_static_rom_audit(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_static_leaf_verify()?;
        static_rom_audit::run(
            &self.build_dir,
            &self.build_dir.join("static_rom_audit"),
            env_usize("LOTW_STATIC_ROM_AUDIT_LIMIT", 2048)?,
        )
    }

    fn cmd_static_handoff_plan(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_static_rom_audit()?;
        static_handoff_plan::run(
            &self.build_dir,
            &self.build_dir.join("static_handoff_plan"),
            env_usize("LOTW_STATIC_HANDOFF_PLAN_LIMIT", 512)?,
        )
    }

    fn static_handoff_plan_ready(&self) -> bool {
        let manifest = self.build_dir.join("static_handoff_plan/manifest.txt");
        read_key_values(&manifest)
            .ok()
            .and_then(|values| values.get("complete").cloned())
            .as_deref()
            == Some("1")
    }

    fn ensure_static_handoff_plan(&self) -> Result<(), Box<dyn std::error::Error>> {
        if env::var("LOTW_STATIC_HANDOFF_PLAN_READY").unwrap_or_default() == "1"
            || self.static_handoff_plan_ready()
        {
            Ok(())
        } else {
            self.cmd_static_handoff_plan()
        }
    }

    fn cmd_static_handoff_verify(
        &self,
        best_effort: bool,
        plan_ready: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !plan_ready {
            self.ensure_static_handoff_plan()?;
        }
        let rom = self.ensure_rom()?;
        self.run_static_proof_verifier(
            StaticVerifier::Handoff,
            &self.build_dir.join("static_handoff_verify"),
            &rom,
            env_usize("LOTW_STATIC_HANDOFF_LIMIT", 64)?,
            best_effort,
        )
    }

    fn cmd_static_branch_verify(
        &self,
        best_effort: bool,
        plan_ready: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !plan_ready {
            self.ensure_static_handoff_plan()?;
        }
        let rom = self.ensure_rom()?;
        self.run_static_proof_verifier(
            StaticVerifier::Branch,
            &self.build_dir.join("static_branch_verify"),
            &rom,
            env_usize("LOTW_STATIC_BRANCH_LIMIT", 64)?,
            best_effort,
        )
    }

    fn cmd_static_jsr_verify(
        &self,
        best_effort: bool,
        plan_ready: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !plan_ready {
            self.ensure_static_handoff_plan()?;
        }
        let rom = self.ensure_rom()?;
        self.run_static_proof_verifier(
            StaticVerifier::Jsr,
            &self.build_dir.join("static_jsr_verify"),
            &rom,
            env_usize("LOTW_STATIC_JSR_LIMIT", 64)?,
            best_effort,
        )
    }

    fn cmd_static_return_verify(
        &self,
        best_effort: bool,
        plan_ready: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !plan_ready {
            self.ensure_static_handoff_plan()?;
        }
        let rom = self.ensure_rom()?;
        self.run_static_proof_verifier(
            StaticVerifier::Return,
            &self.build_dir.join("static_return_verify"),
            &rom,
            env_usize("LOTW_STATIC_RETURN_LIMIT", 64)?,
            best_effort,
        )
    }

    fn cmd_native_block_plan(&self, plan_ready: bool) -> Result<(), Box<dyn std::error::Error>> {
        if !plan_ready && env::var("LOTW_BLOCK_TRANSLATION_PLAN_READY").unwrap_or_default() != "1" {
            self.cmd_block_translation_plan()?;
        }
        native_block_plan::run(
            &self.build_dir,
            &self.build_dir.join("native_block_plan"),
            env_usize("LOTW_NATIVE_BLOCK_LIMIT", 136)?,
        )
    }

    fn cmd_native_block_codegen(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_plan(false)?;
        rust_rewrite_pending("native-block-codegen")
    }

    fn cmd_native_block_static_merge(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_static_handoff_plan()?;
        self.cmd_native_block_plan(true)?;
        self.cmd_static_handoff_verify(true, true)?;
        self.cmd_static_branch_verify(true, true)?;
        self.cmd_static_jsr_verify(true, true)?;
        self.cmd_static_return_verify(true, true)?;
        native_block_static_merge::run(
            &self.build_dir,
            &self.build_dir.join("native_block_plan_static"),
        )
    }

    fn cmd_native_block_codegen_static(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_static_merge()?;
        rust_rewrite_pending("native-block-codegen-static")
    }

    fn cmd_native_block_static_verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_codegen_static()?;
        rust_rewrite_pending("native-block-static-verify")
    }

    fn cmd_native_block_verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_codegen()?;
        rust_rewrite_pending("native-block-verify")
    }

    fn cmd_native_block_hit_verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_codegen()?;
        rust_rewrite_pending("native-block-hit-verify")
    }

    fn cmd_native_block_trace(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_verify()?;
        let manifest = self.build_dir.join("native_block_verify/manifest.txt");
        let values = read_key_values(&manifest)?;
        require_key_value(
            &values,
            "final_states",
            "native_block_final_states.tsv",
            &manifest,
        )
    }

    fn cmd_native_block_transition(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_trace()?;
        native_block_transition::run(
            &self.build_dir,
            &self.build_dir.join("native_block_transition"),
            &self.replay_names(),
        )
    }

    fn cmd_native_block_sequence(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_transition()?;
        self.cmd_native_block_hit_verify()?;
        rust_rewrite_pending("native-block-sequence")
    }

    fn cmd_native_block_chain(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_sequence()?;
        rust_rewrite_pending("native-block-chain")
    }

    fn cmd_native_block_run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_chain()?;
        rust_rewrite_pending("native-block-run")
    }

    fn cmd_native_block_run_maximal(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_run()?;
        native_block_run_maximal::run(
            &self.build_dir.join("native_block_run"),
            &self.build_dir.join("native_block_run_maximal"),
        )
    }

    fn cmd_native_block_runtime_trace(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_run()?;
        native_block_runtime_trace_verify::run(
            &self.build_dir.join("native_block_run"),
            &self.build_dir.join("native_block_runtime_trace"),
        )
    }

    fn cmd_replay_smoke(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rom = self.ensure_rom()?;
        replay_smoke::run(&rom, &self.build_dir.join("replay_smoke"))
    }

    fn cmd_reference_hash_harness(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_replay_smoke()?;
        reference_hash_report::run(
            &self.build_dir.join("replay_smoke"),
            &self.build_dir.join("reference_hash_harness"),
            &self.replay_names(),
        )
    }

    fn cmd_replay_dump(&self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.replay_name();
        replay_dump::run(
            &self.replay_path(&name),
            &self.build_dir.join("replay_dump").join(name),
            env::var("LOTW_REPLAY_DUMP_FRAME_LIMIT")
                .ok()
                .map(|value| parse_positive_usize(&value, "LOTW_REPLAY_DUMP_FRAME_LIMIT"))
                .transpose()?,
        )
    }

    fn cmd_semantic_match_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_native_block_static_verify()?;
        semantic_match_report::run(
            &self.build_dir,
            &self.build_dir.join("semantic_match_report"),
        )
    }

    fn cmd_whole_program_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_semantic_match_report()?;
        whole_program_report::run(
            &self.build_dir,
            &self.build_dir.join("whole_program_report"),
        )
    }

    fn cmd_progress(&self) -> Result<(), Box<dyn std::error::Error>> {
        progress_report::run(&self.build_dir, &self.build_dir.join("progress"))
    }

    fn cmd_decomp_worklist(&self) -> Result<(), Box<dyn std::error::Error>> {
        decomp_worklist::run(&self.build_dir, &self.build_dir.join("decomp_worklist"))
    }

    fn run_static_proof_verifier(
        &self,
        verifier: StaticVerifier,
        canonical_out: &Path,
        rom: &Path,
        limit: usize,
        best_effort: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if canonical_out.join("manifest.txt").is_file()
            && canonical_out.join(verifier.summary_name()).is_file()
        {
            let batch_out = canonical_out.with_extension("batch");
            let merged_out = canonical_out.with_extension("merged");
            remove_path(&batch_out)?;
            remove_path(&merged_out)?;

            match self.invoke_static_verifier(verifier, &batch_out, rom, limit) {
                Ok(()) => {}
                Err(err) if best_effort => {
                    remove_path(&batch_out)?;
                    remove_path(&merged_out)?;
                    eprintln!("lotw-tools: {err}");
                    eprintln!(
                        "static-{}-verify: batch failed; keeping existing proof ledger at {}",
                        verifier.kind(),
                        canonical_out.display()
                    );
                    return Ok(());
                }
                Err(err) => return Err(err),
            }

            static_proof_accumulate::run(verifier.kind(), canonical_out, &batch_out, &merged_out)?;
            remove_path(canonical_out)?;
            fs::rename(&merged_out, canonical_out)?;
            remove_path(&batch_out)?;
        } else {
            self.invoke_static_verifier(verifier, canonical_out, rom, limit)?;
        }
        Ok(())
    }

    fn invoke_static_verifier(
        &self,
        verifier: StaticVerifier,
        out_dir: &Path,
        rom: &Path,
        limit: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (out_dir, rom, limit);
        match verifier {
            StaticVerifier::Handoff => {
                static_handoff_verify::run(&self.build_dir, out_dir, rom, limit)
            }
            StaticVerifier::Leaf => rust_rewrite_pending("static-leaf-verify"),
            StaticVerifier::Branch => rust_rewrite_pending("static-branch-verify"),
            StaticVerifier::Jsr => rust_rewrite_pending("static-jsr-verify"),
            StaticVerifier::Return => rust_rewrite_pending("static-return-verify"),
        }
    }
}

fn rust_rewrite_pending(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err(format!(
        "goal: {command} is disabled until its native block codegen/verifier path is rewritten in Rust"
    )
    .into())
}

fn env_usize(name: &str, default: usize) -> Result<usize, Box<dyn std::error::Error>> {
    match env::var(name) {
        Ok(value) => value
            .parse::<usize>()
            .map_err(|err| format!("goal: invalid {name}={value:?}: {err}").into()),
        Err(_) => Ok(default),
    }
}

fn parse_positive_usize(value: &str, name: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let parsed = value
        .parse::<usize>()
        .map_err(|err| format!("goal: invalid {name}={value:?}: {err}"))?;
    if parsed == 0 {
        return Err(format!("goal: {name} must be positive").into());
    }
    Ok(parsed)
}

fn read_key_values(path: &Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    let mut values = HashMap::new();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.to_string(), value.to_string());
        }
    }
    Ok(values)
}

fn required_value<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("goal: missing {key} in {}", path.display()).into())
}

fn require_key_value(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = required_value(values, key, path)?;
    if actual != expected {
        return Err(format!(
            "goal: {} expected {key}={expected}, got {actual}",
            path.display()
        )
        .into());
    }
    Ok(())
}

fn run_status(command: &mut Command, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("goal: {label} failed with {status}").into())
    }
}

fn remove_path(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}
