mod apu_trace;
mod block_exec;
mod block_exec_verify;
mod block_translation_plan;
mod blocks;
mod chr_preview;
mod coverage_report;
mod decomp_worklist;
mod disasm;
mod external_block_plan;
mod fceux_capture;
mod goal_runner;
mod goal_status;
mod headless_frame;
mod native_block_live_frame_schedule;
mod native_block_live_order;
mod native_block_plan;
mod native_block_run_external_writes;
mod native_block_run_maximal;
mod native_block_runtime_trace_verify;
mod native_block_static_merge;
mod native_block_transition;
mod port_capture;
mod progress_report;
mod reference_hash_report;
mod replay_dump;
mod replay_fixture_smoke;
mod replay_smoke;
mod rom_extract;
mod rom_info;
mod runtime_arch_smoke;
mod runtime_audio_trace;
mod runtime_native_blocks;
mod runtime_native_external_trace_cases;
mod runtime_native_live;
mod runtime_native_live_frame;
mod runtime_native_live_frame_ppu;
mod runtime_native_static_blocks;
mod runtime_native_trace_verify;
mod rust_port_capture;
mod rust_ppu_render_compare;
mod semantic_match_report;
mod smoke_assert;
mod source_audit;
mod static_branch_verify;
mod static_cfg_gap;
mod static_entry_plan;
mod static_handoff_plan;
mod static_handoff_verify;
mod static_jsr_verify;
mod static_proof_accumulate;
mod static_rom_audit;
mod symbol_audit;
mod trace_compare;
mod whole_program_report;

#[cfg(test)]
mod test_exe;

use std::env;
use std::path::Path;

fn usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  {program} apu-trace <apu_writes.tsv> <out-dir>");
    eprintln!("  {program} block-exec-verify <block-exec-dir> <out-dir>");
    eprintln!("  {program} block-exec <rom.nes> <block_candidates.tsv> <out-dir> [max_steps] [label_states.tsv]");
    eprintln!("  {program} block-exec --case-states <rom.nes> <block_state_cases.tsv> <out-dir> [max_steps]");
    eprintln!("  {program} block-translation-plan <build-dir> <out-dir> <replay>...");
    eprintln!("  {program} blocks <rom.nes> <executed_labels.tsv> <out-dir> [expected-sha256]");
    eprintln!("  {program} chr-preview <rom.nes> <out-dir> [expected-sha256]");
    eprintln!("  {program} coverage-report <build-dir> <out-dir> <replay>...");
    eprintln!("  {program} decomp-worklist <build-dir> <out-dir>");
    eprintln!("  {program} disasm <rom.nes> <out-dir> [expected-sha256]");
    eprintln!("  {program} external-block-plan <build-dir> <out-dir> <replay>...");
    eprintln!("  {program} goal <command>");
    eprintln!("  {program} goal-status <repo-root> <build-dir>");
    eprintln!("  {program} headless-frame <rom.nes> <out-dir> [expected-sha256]");
    eprintln!(
        "  {program} native-block-live-frame-schedule <native-block-run-dir> <out-dir> [replay]..."
    );
    eprintln!("  {program} native-block-live-order <native-block-run-dir> <source-port-trace-dir> <out-dir>");
    eprintln!("  {program} native-block-plan <build-dir> <out-dir> [limit]");
    eprintln!("  {program} native-block-run-external-writes <native-block-run-cases.tsv> <native-block-verify-cases.tsv> <out.tsv>");
    eprintln!("  {program} native-block-run-maximal <native-block-run-dir> <out-dir>");
    eprintln!("  {program} native-block-runtime-trace-verify <native-block-run-dir> <out-dir>");
    eprintln!("  {program} native-block-transition <build-dir> <out-dir> <replay>...");
    eprintln!("  {program} native-block-static-merge <build-dir> <out-dir>");
    eprintln!(
        "  {program} port-capture <lotw> <lotw-replay-dump> <rom.nes> <out-dir> <replay> <frame>"
    );
    eprintln!("  {program} progress-report <build-dir> <out-dir>");
    eprintln!("  {program} reference-hash-report <reference-capture-root> <out-dir> [replay]...");
    eprintln!("  {program} reference-capture <rom.nes> <out-dir> [replay] [frames]");
    eprintln!("  {program} replay-fixture-smoke <replay>...");
    eprintln!("  {program} replay-smoke <rom.nes> <out-dir>");
    eprintln!("  {program} replay-dump <replay> <out-dir> [frame-limit]");
    eprintln!("  {program} rom-extract <rom.nes> <out-dir> [expected-sha256]");
    eprintln!("  {program} rom-info <rom.nes> <out-dir> [expected-sha256]");
    eprintln!("  {program} rust-ppu-render-compare <rom.nes> <trace-dir> <c-ppu-frame.ppm> <out-dir> <frame>");
    eprintln!(
        "  {program} rust-ppu-render-compare-summary <compare-root> <out-dir> <name:frame>..."
    );
    eprintln!("  {program} rust-port-capture-report <capture-dir> <rom.nes> <replay> <frame>");
    eprintln!("  {program} smoke-assert <nul-delimited-assertion-file>");
    eprintln!(
        "  {program} runtime-audio-trace <lotw> <apu_writes.tsv> <out-dir> <frames> [rom.nes]"
    );
    eprintln!("  {program} runtime-arch-smoke <lotw-executable>");
    eprintln!("  {program} runtime-native-blocks <lotw> <native_block_run_cases.tsv> <native-block-runtime-trace-verify-command> <out-dir>");
    eprintln!(
        "  {program} runtime-native-external-trace-cases <native-block-verify-cases.tsv> <out-dir>"
    );
    eprintln!("  {program} runtime-native-live <lotw> <native-block-run-dir> <runtime-native-trace-verify-command> <out-dir> <rom.nes>");
    eprintln!("  {program} runtime-native-live-frame <lotw> <frame-scheduled-run-dir> <runtime-native-trace-verify-command> <out-dir> <rom.nes> [replay lotw-replay-dump]");
    eprintln!("  {program} runtime-native-live-frame-ppu <lotw> <lotw-ppu-trace-render> <lotw-ppm-compare> <runtime-native-live-frame-replays-dir> <out-dir> <rom.nes>");
    eprintln!("  {program} runtime-native-static-blocks <lotw> <native-block-static-verify-dir> <runtime-native-blocks-command> <native-block-runtime-trace-verify-command> <out-dir>");
    eprintln!("  {program} runtime-native-trace-verify <native-block-run-dir> <port-trace-dir> <out-dir> [expected-runtime] [expected-external-writes.tsv]");
    eprintln!("  {program} semantic-match-report <build-dir> <out-dir>");
    eprintln!("  {program} source-audit <repo-root>");
    eprintln!("  {program} static-branch-verify <build-dir> <out-dir> <rom.nes> [limit]");
    eprintln!("  {program} static-cfg-gap <build-dir> <out-dir> <replay>...");
    eprintln!("  {program} static-entry-plan <build-dir> <out-dir> [limit]");
    eprintln!("  {program} static-handoff-plan <build-dir> <out-dir> [limit]");
    eprintln!("  {program} static-handoff-verify <build-dir> <out-dir> <rom.nes> [limit]");
    eprintln!("  {program} static-jsr-verify <build-dir> <out-dir> <rom.nes> [limit]");
    eprintln!("  {program} static-proof-accumulate <leaf|handoff|branch|jsr|return> <old-dir> <new-dir> <out-dir>");
    eprintln!("  {program} static-rom-audit <build-dir> <out-dir> [top-limit]");
    eprintln!("  {program} symbol-audit <symbols.yaml>");
    eprintln!("  {program} trace-capture <rom.nes> <out-dir> <labels.txt> [replay] [frames]");
    eprintln!("  {program} trace-compare <reference-trace-dir> <port-capture-dir> <out-dir>");
    eprintln!("  {program} whole-program-report <build-dir> <out-dir>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(String::as_str).unwrap_or("lotw-tools");

    let result = match args.get(1).map(String::as_str) {
        Some("apu-trace") if args.len() == 4 => {
            apu_trace::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("block-exec") if args.len() >= 5 => block_exec::run_cli(&args[2..]),
        Some("block-exec-verify") if args.len() == 4 => {
            block_exec_verify::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("block-translation-plan") if args.len() >= 5 => {
            block_translation_plan::run(Path::new(&args[2]), Path::new(&args[3]), &args[4..])
        }
        Some("blocks") if args.len() == 5 || args.len() == 6 => blocks::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            args.get(5).map(String::as_str),
        ),
        Some("chr-preview") if args.len() == 4 || args.len() == 5 => chr_preview::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            args.get(4).map(String::as_str),
        ),
        Some("coverage-report") if args.len() >= 5 => {
            coverage_report::run(Path::new(&args[2]), Path::new(&args[3]), &args[4..])
        }
        Some("decomp-worklist") if args.len() == 4 => {
            decomp_worklist::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("disasm") if args.len() == 4 || args.len() == 5 => disasm::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            args.get(4).map(String::as_str),
        ),
        Some("external-block-plan") if args.len() >= 5 => {
            external_block_plan::run(Path::new(&args[2]), Path::new(&args[3]), &args[4..])
        }
        Some("goal") if args.len() >= 2 => goal_runner::run(&args[2..]),
        Some("goal-status") if args.len() == 4 => {
            goal_status::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("headless-frame") if args.len() == 4 || args.len() == 5 => headless_frame::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            args.get(4).map(String::as_str),
        ),
        Some("native-block-live-frame-schedule") if args.len() >= 4 => {
            native_block_live_frame_schedule::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                &args[4..],
            )
        }
        Some("native-block-live-order") if args.len() == 5 => native_block_live_order::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
        ),
        Some("native-block-plan") if args.len() == 4 || args.len() == 5 => {
            let limit = match args.get(4) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => limit,
                    Err(err) => {
                        eprintln!("lotw-tools: invalid native block limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => 24,
            };
            native_block_plan::run(Path::new(&args[2]), Path::new(&args[3]), limit)
        }
        Some("native-block-run-external-writes") if args.len() == 5 => {
            native_block_run_external_writes::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
            )
        }
        Some("native-block-run-maximal") if args.len() == 4 => {
            native_block_run_maximal::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("native-block-runtime-trace-verify") if args.len() == 4 => {
            native_block_runtime_trace_verify::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("native-block-transition") if args.len() >= 5 => {
            native_block_transition::run(Path::new(&args[2]), Path::new(&args[3]), &args[4..])
        }
        Some("native-block-static-merge") if args.len() == 4 => {
            native_block_static_merge::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("port-capture") if args.len() == 8 => port_capture::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
            Path::new(&args[6]),
            &args[7],
        ),
        Some("progress-report") if args.len() == 4 => {
            progress_report::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("reference-hash-report") if args.len() >= 4 => {
            reference_hash_report::run(Path::new(&args[2]), Path::new(&args[3]), &args[4..])
        }
        Some("reference-capture") if args.len() >= 4 && args.len() <= 6 => {
            fceux_capture::reference_run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                args.get(4).map(Path::new),
                args.get(5).map(String::as_str),
            )
        }
        Some("replay-smoke") if args.len() == 4 => {
            replay_smoke::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("replay-fixture-smoke") if args.len() >= 3 => replay_fixture_smoke::run(&args[2..]),
        Some("replay-dump") if args.len() == 4 || args.len() == 5 => {
            let frame_limit = match args.get(4) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => Some(limit),
                    Err(err) => {
                        eprintln!("lotw-tools: invalid frame limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => None,
            };
            replay_dump::run(Path::new(&args[2]), Path::new(&args[3]), frame_limit)
        }
        Some("rom-info") if args.len() == 4 || args.len() == 5 => rom_info::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            args.get(4).map(String::as_str),
        ),
        Some("rom-extract") if args.len() == 4 || args.len() == 5 => rom_extract::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            args.get(4).map(String::as_str),
        ),
        Some("rust-ppu-render-compare") if args.len() == 7 => rust_ppu_render_compare::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
            &args[6],
        ),
        Some("rust-ppu-render-compare-summary") if args.len() >= 5 => {
            rust_ppu_render_compare::run_summary(
                Path::new(&args[2]),
                Path::new(&args[3]),
                &args[4..],
            )
        }
        Some("rust-port-capture-report") if args.len() == 6 => rust_port_capture::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            &args[5],
        ),
        Some("runtime-audio-trace") if args.len() == 6 || args.len() == 7 => {
            runtime_audio_trace::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                &args[5],
                args.get(6).map(Path::new),
            )
        }
        Some("runtime-arch-smoke") if args.len() == 3 => {
            runtime_arch_smoke::run(Path::new(&args[2]))
        }
        Some("runtime-native-blocks") if args.len() == 6 => runtime_native_blocks::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
        ),
        Some("runtime-native-external-trace-cases") if args.len() == 4 => {
            runtime_native_external_trace_cases::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("runtime-native-live") if args.len() == 7 => runtime_native_live::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
            Path::new(&args[6]),
        ),
        Some("runtime-native-live-frame") if args.len() == 7 || args.len() == 9 => {
            runtime_native_live_frame::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                Path::new(&args[6]),
                args.get(7).map(Path::new),
                args.get(8).map(Path::new),
            )
        }
        Some("runtime-native-live-frame-ppu") if args.len() == 8 => {
            runtime_native_live_frame_ppu::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                Path::new(&args[6]),
                Path::new(&args[7]),
            )
        }
        Some("runtime-native-static-blocks") if args.len() == 7 => {
            runtime_native_static_blocks::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                Path::new(&args[6]),
            )
        }
        Some("runtime-native-trace-verify")
            if args.len() == 5 || args.len() == 6 || args.len() == 7 =>
        {
            runtime_native_trace_verify::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                args.get(5)
                    .map(String::as_str)
                    .unwrap_or("lotw_runtime_native_blocks"),
                args.get(6).map(Path::new),
            )
        }
        Some("semantic-match-report") if args.len() == 4 => {
            semantic_match_report::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        Some("smoke-assert") if args.len() == 3 => smoke_assert::run(Path::new(&args[2])),
        Some("source-audit") if args.len() == 3 => source_audit::run(Path::new(&args[2])),
        Some("static-cfg-gap") if args.len() >= 5 => {
            static_cfg_gap::run(Path::new(&args[2]), Path::new(&args[3]), &args[4..])
        }
        Some("static-entry-plan") if args.len() == 4 || args.len() == 5 => {
            let limit = match args.get(4) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => limit,
                    Err(err) => {
                        eprintln!("lotw-tools: invalid static entry plan limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => 32,
            };
            static_entry_plan::run(Path::new(&args[2]), Path::new(&args[3]), limit)
        }
        Some("static-handoff-plan") if args.len() == 4 || args.len() == 5 => {
            let limit = match args.get(4) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => limit,
                    Err(err) => {
                        eprintln!("lotw-tools: invalid static handoff plan limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => 64,
            };
            static_handoff_plan::run(Path::new(&args[2]), Path::new(&args[3]), limit)
        }
        Some("static-branch-verify") if args.len() == 5 || args.len() == 6 => {
            let limit = match args.get(5) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => limit,
                    Err(err) => {
                        eprintln!("lotw-tools: invalid static branch verify limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => 64,
            };
            static_branch_verify::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                limit,
            )
        }
        Some("static-handoff-verify") if args.len() == 5 || args.len() == 6 => {
            let limit = match args.get(5) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => limit,
                    Err(err) => {
                        eprintln!("lotw-tools: invalid static handoff verify limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => 64,
            };
            static_handoff_verify::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                limit,
            )
        }
        Some("static-jsr-verify") if args.len() == 5 || args.len() == 6 => {
            let limit = match args.get(5) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => limit,
                    Err(err) => {
                        eprintln!("lotw-tools: invalid static JSR verify limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => 64,
            };
            static_jsr_verify::run(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                limit,
            )
        }
        Some("static-proof-accumulate") if args.len() == 6 => static_proof_accumulate::run(
            &args[2],
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
        ),
        Some("static-rom-audit") if args.len() == 4 || args.len() == 5 => {
            let top_limit = match args.get(4) {
                Some(value) => match value.parse::<usize>() {
                    Ok(limit) => limit,
                    Err(err) => {
                        eprintln!("lotw-tools: invalid static ROM audit top limit: {err}");
                        std::process::exit(2);
                    }
                },
                None => 64,
            };
            static_rom_audit::run(Path::new(&args[2]), Path::new(&args[3]), top_limit)
        }
        Some("symbol-audit") if args.len() == 3 => symbol_audit::run(Path::new(&args[2])),
        Some("trace-capture") if args.len() >= 5 && args.len() <= 7 => fceux_capture::trace_run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            args.get(5).map(Path::new),
            args.get(6).map(String::as_str),
        ),
        Some("trace-compare") if args.len() == 5 => trace_compare::run(
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
        ),
        Some("whole-program-report") if args.len() == 4 => {
            whole_program_report::run(Path::new(&args[2]), Path::new(&args[3]))
        }
        _ => {
            usage(program);
            std::process::exit(2);
        }
    };

    if let Err(err) = result {
        eprintln!("lotw-tools: {err}");
        std::process::exit(1);
    }
}
