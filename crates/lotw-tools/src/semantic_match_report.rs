use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Default)]
struct MatchedStats {
    total: u64,
    replay: u64,
    static_leaf: u64,
    static_handoff: u64,
    static_branch: u64,
    static_jsr: u64,
    static_return: u64,
}

#[derive(Debug, Default)]
struct FrontierStats {
    total: u64,
    matched: u64,
    unverified: u64,
    linear: u64,
    branch: u64,
    call_like_leaf: u64,
    jsr: u64,
    unverified_linear: u64,
    unverified_branch: u64,
    unverified_call_like_leaf: u64,
    unverified_jsr: u64,
}

struct SemanticInputs {
    merged_blocks: std::path::PathBuf,
    merge_summary: std::path::PathBuf,
    static_verify_manifest: std::path::PathBuf,
    codegen_manifest: std::path::PathBuf,
    handoff_plan: std::path::PathBuf,
    static_handoff_blocks: std::path::PathBuf,
    static_branch_blocks: std::path::PathBuf,
    static_jsr_blocks: std::path::PathBuf,
    static_return_blocks: std::path::PathBuf,
}

pub fn run(build_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let inputs = SemanticInputs {
        merged_blocks: build_dir.join("native_block_plan_static/native_blocks.tsv"),
        merge_summary: build_dir
            .join("native_block_plan_static/native_block_static_merge_summary.txt"),
        static_verify_manifest: build_dir.join("native_block_static_verify/manifest.txt"),
        codegen_manifest: build_dir.join("native_block_codegen_static/manifest.txt"),
        handoff_plan: build_dir.join("static_handoff_plan/static_handoff_plan.tsv"),
        static_handoff_blocks: build_dir
            .join("static_handoff_verify/static_handoff_native_blocks.tsv"),
        static_branch_blocks: build_dir
            .join("static_branch_verify/static_branch_native_blocks.tsv"),
        static_jsr_blocks: build_dir.join("static_jsr_verify/static_jsr_native_blocks.tsv"),
        static_return_blocks: build_dir
            .join("static_return_verify/static_return_native_blocks.tsv"),
    };

    for path in [
        &inputs.merged_blocks,
        &inputs.merge_summary,
        &inputs.static_verify_manifest,
        &inputs.codegen_manifest,
        &inputs.handoff_plan,
        &inputs.static_handoff_blocks,
        &inputs.static_branch_blocks,
        &inputs.static_jsr_blocks,
        &inputs.static_return_blocks,
    ] {
        require_file(path)?;
    }

    let merge_kv = read_key_values(&inputs.merge_summary)?;
    let verify_kv = read_key_values(&inputs.static_verify_manifest)?;
    let codegen_kv = read_key_values(&inputs.codegen_manifest)?;

    ensure_eq(&merge_kv, "complete", "1", &inputs.merge_summary)?;
    ensure_eq(&verify_kv, "complete", "1", &inputs.static_verify_manifest)?;
    ensure_eq(
        &verify_kv,
        "mismatches",
        "0",
        &inputs.static_verify_manifest,
    )?;
    ensure_eq(
        &verify_kv,
        "external_write_mismatches",
        "0",
        &inputs.static_verify_manifest,
    )?;
    ensure_eq(&codegen_kv, "complete", "1", &inputs.codegen_manifest)?;
    ensure_eq(
        &codegen_kv,
        "unsupported_count",
        "0",
        &inputs.codegen_manifest,
    )?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let matched_units = out_dir.join("semantic_matched_units.tsv");
    let frontier_status = out_dir.join("static_frontier_match_status.tsv");
    let summary = out_dir.join("semantic_match_summary.txt");
    let manifest = out_dir.join("manifest.txt");

    let matched_stats = write_matched_units(&inputs.merged_blocks, &matched_units)?;
    let frontier_stats = write_frontier_status(
        &inputs.handoff_plan,
        &inputs.static_handoff_blocks,
        &inputs.static_branch_blocks,
        &inputs.static_jsr_blocks,
        &inputs.static_return_blocks,
        &frontier_status,
    )?;

    write_summary(
        &summary,
        &matched_stats,
        &frontier_stats,
        required(&verify_kv, "case_count", &inputs.static_verify_manifest)?,
        required(&codegen_kv, "block_count", &inputs.codegen_manifest)?,
    )?;
    write_manifest(&manifest, &inputs)?;

    println!("semantic_match_report: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("missing input: {}", path.display()),
        ))
    }
}

fn read_key_values(path: &Path) -> io::Result<HashMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut values = HashMap::new();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.to_string(), value.to_string());
        }
    }
    Ok(values)
}

fn required<'a>(
    values: &'a HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing {key} in {}", path.display()).into())
}

fn ensure_eq(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = required(values, key, path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{} has {key}={actual}, expected {expected}", path.display()).into())
    }
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn key(cpu_addr: &str, prg_offset: &str) -> String {
    format!(
        "{}\t{}",
        cpu_addr.to_ascii_uppercase(),
        prg_offset.to_ascii_uppercase()
    )
}

fn match_kind(reason: &str) -> &'static str {
    match reason {
        "static_verified_leaf" => "static_leaf",
        "static_verified_handoff" => "static_handoff",
        "static_verified_branch" => "static_branch",
        "static_verified_jsr" => "static_jsr",
        "static_verified_return" => "static_return",
        _ => "replay",
    }
}

fn add_match_count(stats: &mut MatchedStats, kind: &str) {
    stats.total += 1;
    match kind {
        "static_leaf" => stats.static_leaf += 1,
        "static_handoff" => stats.static_handoff += 1,
        "static_branch" => stats.static_branch += 1,
        "static_jsr" => stats.static_jsr += 1,
        "static_return" => stats.static_return += 1,
        _ => stats.replay += 1,
    }
}

fn write_matched_units(blocks_path: &Path, out_path: &Path) -> io::Result<MatchedStats> {
    let text = fs::read_to_string(blocks_path)?;
    let mut out = fs::File::create(out_path)?;
    writeln!(
        out,
        "output_rank\tmatch_kind\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\tstatus"
    )?;

    let mut stats = MatchedStats::default();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 15 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} has {} fields, expected at least 15",
                    blocks_path.display(),
                    line_no + 1,
                    fields.len()
                ),
            ));
        }

        let kind = match_kind(fields[14]);
        add_match_count(&mut stats, kind);
        writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\tsemantics_matched",
            fields[0],
            kind,
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[7],
            fields[8],
            fields[9],
            fields[10],
            fields[11],
            fields[12],
            fields[13],
            fields[14]
        )?;
    }
    Ok(stats)
}

fn read_block_keys(path: &Path) -> io::Result<HashSet<String>> {
    let text = fs::read_to_string(path)?;
    let mut keys = HashSet::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} has {} fields, expected at least 3",
                    path.display(),
                    line_no + 1,
                    fields.len()
                ),
            ));
        }
        keys.insert(key(fields[1], fields[2]));
    }
    Ok(keys)
}

fn add_action_count(stats: &mut FrontierStats, action: &str, matched: bool) {
    match action {
        "generate_linear_handoff_case" => {
            stats.linear += 1;
            if !matched {
                stats.unverified_linear += 1;
            }
        }
        "generate_branch_taken_and_fallthrough_cases" => {
            stats.branch += 1;
            if !matched {
                stats.unverified_branch += 1;
            }
        }
        "add_call_handoff_synthetic_cases" => {
            stats.call_like_leaf += 1;
            if !matched {
                stats.unverified_call_like_leaf += 1;
            }
        }
        "translate_callee_or_split_at_call_return" => {
            stats.jsr += 1;
            if !matched {
                stats.unverified_jsr += 1;
            }
        }
        _ => {}
    }
}

fn frontier_match_kind(
    action: &str,
    block_key: &str,
    handoff: &HashSet<String>,
    branch: &HashSet<String>,
    jsr: &HashSet<String>,
    returns: &HashSet<String>,
) -> Option<&'static str> {
    if handoff.contains(block_key) {
        Some("static_handoff")
    } else if action == "generate_branch_taken_and_fallthrough_cases" && branch.contains(block_key)
    {
        Some("static_branch")
    } else if (action == "add_call_handoff_synthetic_cases"
        || action == "translate_callee_or_split_at_call_return")
        && jsr.contains(block_key)
    {
        Some("static_jsr")
    } else if action == "add_call_handoff_synthetic_cases" && returns.contains(block_key) {
        Some("static_return")
    } else {
        None
    }
}

fn write_frontier_status(
    handoff_plan: &Path,
    handoff_blocks: &Path,
    branch_blocks: &Path,
    jsr_blocks: &Path,
    return_blocks: &Path,
    out_path: &Path,
) -> io::Result<FrontierStats> {
    let handoff = read_block_keys(handoff_blocks)?;
    let branch = read_block_keys(branch_blocks)?;
    let jsr = read_block_keys(jsr_blocks)?;
    let returns = read_block_keys(return_blocks)?;
    let text = fs::read_to_string(handoff_plan)?;

    let mut out = fs::File::create(out_path)?;
    writeln!(
        out,
        "plan_rank\tlabel\tcpu_addr\tprg_offset\tstatic_shape\tnext_action\tmatch_status\tmatched_kind\tblocking_reason"
    )?;

    let mut stats = FrontierStats::default();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 13 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} has {} fields, expected at least 13",
                    handoff_plan.display(),
                    line_no + 1,
                    fields.len()
                ),
            ));
        }

        let block_key = key(fields[5], fields[6]);
        let matched_kind =
            frontier_match_kind(fields[12], &block_key, &handoff, &branch, &jsr, &returns);
        let matched = matched_kind.is_some();
        stats.total += 1;
        if matched {
            stats.matched += 1;
        } else {
            stats.unverified += 1;
        }
        add_action_count(&mut stats, fields[12], matched);

        writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            fields[0],
            fields[7],
            fields[5],
            fields[6],
            fields[10],
            fields[12],
            if matched {
                "semantics_matched"
            } else {
                "unverified"
            },
            matched_kind.unwrap_or(""),
            fields[11]
        )?;
    }
    Ok(stats)
}

fn write_summary(
    path: &Path,
    matched: &MatchedStats,
    frontier: &FrontierStats,
    case_count: &str,
    native_block_count: &str,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "matched_total={}", matched.total)?;
    writeln!(file, "replay_matched={}", matched.replay)?;
    writeln!(file, "static_leaf_matched={}", matched.static_leaf)?;
    writeln!(file, "static_handoff_matched={}", matched.static_handoff)?;
    writeln!(file, "static_branch_matched={}", matched.static_branch)?;
    writeln!(file, "static_jsr_matched={}", matched.static_jsr)?;
    writeln!(file, "static_return_matched={}", matched.static_return)?;
    writeln!(file, "native_block_count={native_block_count}")?;
    writeln!(file, "static_case_count={case_count}")?;
    writeln!(file, "frontier_total={}", frontier.total)?;
    writeln!(file, "frontier_matched={}", frontier.matched)?;
    writeln!(file, "frontier_unverified={}", frontier.unverified)?;
    writeln!(file, "frontier_linear={}", frontier.linear)?;
    writeln!(file, "frontier_branch={}", frontier.branch)?;
    writeln!(file, "frontier_call_like_leaf={}", frontier.call_like_leaf)?;
    writeln!(file, "frontier_jsr={}", frontier.jsr)?;
    writeln!(file, "unverified_linear={}", frontier.unverified_linear)?;
    writeln!(file, "unverified_branch={}", frontier.unverified_branch)?;
    writeln!(
        file,
        "unverified_call_like_leaf={}",
        frontier.unverified_call_like_leaf
    )?;
    writeln!(file, "unverified_jsr={}", frontier.unverified_jsr)?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(path: &Path, inputs: &SemanticInputs) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=semantic_match_report")?;
    writeln!(file, "merged_blocks={}", inputs.merged_blocks.display())?;
    writeln!(file, "merge_summary={}", inputs.merge_summary.display())?;
    writeln!(
        file,
        "static_verify_manifest={}",
        inputs.static_verify_manifest.display()
    )?;
    writeln!(
        file,
        "codegen_manifest={}",
        inputs.codegen_manifest.display()
    )?;
    writeln!(file, "handoff_plan={}", inputs.handoff_plan.display())?;
    writeln!(
        file,
        "static_handoff_blocks={}",
        inputs.static_handoff_blocks.display()
    )?;
    writeln!(
        file,
        "static_branch_blocks={}",
        inputs.static_branch_blocks.display()
    )?;
    writeln!(
        file,
        "static_jsr_blocks={}",
        inputs.static_jsr_blocks.display()
    )?;
    writeln!(
        file,
        "static_return_blocks={}",
        inputs.static_return_blocks.display()
    )?;
    writeln!(file, "matched_units=semantic_matched_units.tsv")?;
    writeln!(file, "frontier_status=static_frontier_match_status.tsv")?;
    writeln!(file, "summary=semantic_match_summary.txt")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn write(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_semantic_report_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_semantic_match_report() {
        let root = temp_dir();
        let build = root.join("build");
        let out = root.join("semantic");

        write(
            &build.join("native_block_plan_static/native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n1\tFCB7\t1FCB7\t1\t18\t1\ttitle_idle\t1\t1\t0\t0\t0\t0\t1\treplay\n2\tF0E1\t1F0E1\t3\tA5\t0\tstatic_handoff_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_handoff\n3\tE333\t1E333\t6\tA5\t0\tstatic_handoff_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_branch\n4\tA489\t1E489\t5\tA9\t0\tstatic_handoff_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_handoff\n",
        );
        write(
            &build.join("native_block_plan_static/native_block_static_merge_summary.txt"),
            "complete=1\n",
        );
        write(
            &build.join("native_block_static_verify/manifest.txt"),
            "case_count=12\nmatched=12\nmismatches=0\nexternal_write_mismatches=0\ncomplete=1\n",
        );
        write(
            &build.join("native_block_codegen_static/manifest.txt"),
            "block_count=4\nunsupported_count=0\ncomplete=1\n",
        );
        write(
            &build.join("static_handoff_plan/static_handoff_plan.tsv"),
            "rank\tfrontier_rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tstatic_shape\tblocking_reason\tnext_action\tsource_edge_count\tsource_edge_types\toutgoing_edge_types\tmapped_in_edges\treachable_in_edges\treachable_from\tfile\n1\t1\t1\tfixed\t7\tF0E1\t1F0E1\tL_F0E1\tA5\tLDA\tstraight_line_or_data\tneeds_synthetic_handoff_state\tgenerate_linear_handoff_case\t1\tfallthrough\tfallthrough\t1\t1\tvector_reset\tfixed.asm\n2\t2\t1\tfixed\t7\tE333\t1E333\tL_E333\tA5\tLDA\tcontrol_flow\tneeds_branch_state\tgenerate_branch_taken_and_fallthrough_cases\t1\tbranch\tbranch\t1\t1\tvector_reset\tfixed.asm\n3\t3\t1\tfixed\t7\tF100\t1F100\tL_F100\t20\tJSR\tcontrol_flow\tneeds_call_state\tadd_call_handoff_synthetic_cases\t1\tcall\tcall\t1\t1\tvector_reset\tfixed.asm\n4\t4\t1\tfixed\t7\tA489\t1E489\tL_A489\tA9\tLDA\tcontrol_flow\tneeds_branch_state\tgenerate_branch_taken_and_fallthrough_cases\t1\tbranch_target\tjump_target\t1\t1\tvector_reset\tfixed.asm\n",
        );
        write(
            &build.join("static_handoff_verify/static_handoff_native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n1\tF0E1\t1F0E1\t3\tA5\t0\tstatic_handoff_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_handoff\n2\tA489\t1E489\t5\tA9\t0\tstatic_handoff_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_handoff\n",
        );
        write(
            &build.join("static_branch_verify/static_branch_native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n1\tE333\t1E333\t6\tA5\t0\tstatic_handoff_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_branch\n",
        );
        write(
            &build.join("static_jsr_verify/static_jsr_native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n",
        );
        write(
            &build.join("static_return_verify/static_return_native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n",
        );

        run(&build, &out).unwrap();

        let summary = fs::read_to_string(out.join("semantic_match_summary.txt")).unwrap();
        assert!(summary.contains("matched_total=4\n"));
        assert!(summary.contains("replay_matched=1\n"));
        assert!(summary.contains("static_handoff_matched=2\n"));
        assert!(summary.contains("static_branch_matched=1\n"));
        assert!(summary.contains("frontier_total=4\n"));
        assert!(summary.contains("frontier_matched=3\n"));
        assert!(summary.contains("frontier_unverified=1\n"));
        assert!(summary.contains("unverified_branch=0\n"));
        assert!(summary.contains("unverified_call_like_leaf=1\n"));

        let frontier = fs::read_to_string(out.join("static_frontier_match_status.tsv")).unwrap();
        assert!(frontier.contains(
            "1\tL_F0E1\tF0E1\t1F0E1\tstraight_line_or_data\tgenerate_linear_handoff_case\tsemantics_matched\tstatic_handoff\tneeds_synthetic_handoff_state\n"
        ));
        assert!(frontier.contains(
            "4\tL_A489\tA489\t1E489\tcontrol_flow\tgenerate_branch_taken_and_fallthrough_cases\tsemantics_matched\tstatic_handoff\tneeds_branch_state\n"
        ));
        assert!(frontier.contains(
            "3\tL_F100\tF100\t1F100\tcontrol_flow\tadd_call_handoff_synthetic_cases\tunverified\t\tneeds_call_state\n"
        ));

        let units = fs::read_to_string(out.join("semantic_matched_units.tsv")).unwrap();
        assert!(units.contains("2\tstatic_handoff\tF0E1\t1F0E1\t3\tA5\t4\t0\t0\t0\t0\t0\t4\tstatic_verified_handoff\tsemantics_matched\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
