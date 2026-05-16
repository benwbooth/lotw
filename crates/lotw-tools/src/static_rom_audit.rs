use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct ProofGroup {
    name: &'static str,
    blocks: PathBuf,
    summary: PathBuf,
    manifest: PathBuf,
}

#[derive(Debug, Default, Clone)]
struct EntryInfo {
    shape: String,
    recommended: String,
}

#[derive(Debug, Default, Clone)]
struct ProofInfo {
    verified: bool,
    reasons: Vec<String>,
    seen: HashSet<String>,
}

#[derive(Debug, Clone)]
struct FrontierRow {
    rank: String,
    priority: String,
    bank_kind: String,
    bank: String,
    cpu_addr: String,
    prg_offset: String,
    label: String,
    first_opcode: String,
    mnemonic: String,
    known_opcode: String,
    mapped_in_edges: String,
    reachable_in_edges: String,
    reachable_from: String,
    static_shape: String,
    recommended_next_step: String,
    synthetic_verified: u64,
    native_static_reason: String,
    file: String,
}

#[derive(Debug, Default)]
struct AuditValues {
    label_count: String,
    known_label_count: String,
    data_label_count: String,
    covered_label_count: String,
    uncovered_label_count: String,
    uncovered_known_count: String,
    instruction_count: String,
    edge_count: String,
    reachable_label_count: String,
    reachable_uncovered_count: String,
    reachable_uncovered_known_count: String,
    entry_candidate_count: String,
    entry_leaf_count: String,
    entry_control_count: String,
    entry_indirect_count: String,
    entry_data_count: String,
    leaf_selected_count: String,
    leaf_case_count: String,
    leaf_ram_writes: String,
    leaf_external_writes: String,
    leaf_unmapped_reads: String,
    leaf_skipped_call_like: String,
    leaf_skipped_missing_byte_count: String,
    leaf_skipped_unsupported: String,
    leaf_skipped_loop_like: String,
    static_handoff_verified_count: u64,
    static_branch_verified_count: u64,
    static_jsr_verified_count: u64,
    static_return_verified_count: u64,
    static_verified_block_count: u64,
    frontier_count: u64,
    frontier_verified_count: u64,
    frontier_unverified_count: u64,
    frontier_unqueued_count: u64,
    frontier_unverified_leaf: u64,
    frontier_unverified_calls: u64,
    frontier_unverified_control: u64,
    frontier_unverified_straight: u64,
    frontier_unverified_unqueued: u64,
    frontier_unverified_other: u64,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    top_limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if top_limit == 0 {
        return Err("static_rom_audit: top-limit must be a positive integer".into());
    }

    let paths = InputPaths::new(build_dir);
    paths.require_required()?;
    let optional = paths.present_optional_proofs()?;
    fs::create_dir_all(out_dir)?;

    let static_summary = read_key_values(&paths.static_summary)?;
    let entry_summary = read_key_values(&paths.entry_summary)?;
    let leaf_summary = read_key_values(&paths.leaf_summary)?;
    let entry_info = read_entry_plan(&paths.entry_plan)?;

    let mut proof_files = vec![paths.leaf_blocks.clone()];
    proof_files.extend(optional.iter().map(|group| group.blocks.clone()));
    let proof_info = read_proofs(&proof_files)?;

    let mut frontier = build_frontier(&paths.reachable_gaps, &entry_info, &proof_info, top_limit)?;
    let mut values = collect_values(
        &static_summary,
        &entry_summary,
        &leaf_summary,
        &paths,
        &proof_files,
    )?;
    summarize_frontier(&frontier, &mut values);

    write_frontier(&out_dir.join("static_rom_frontier.tsv"), &frontier)?;
    write_metrics(&out_dir.join("static_rom_audit.tsv"), &values)?;
    write_summary(
        &out_dir.join("static_rom_audit_summary.txt"),
        &paths,
        top_limit,
        &values,
    )?;
    write_manifest(&out_dir.join("manifest.txt"))?;

    // Keep the vector mutable in case future callers want sorted post-processing.
    frontier.clear();
    println!("static_rom_audit: wrote {}", out_dir.display());
    Ok(())
}

#[derive(Debug)]
struct InputPaths {
    static_dir: PathBuf,
    entry_dir: PathBuf,
    leaf_dir: PathBuf,
    static_summary: PathBuf,
    reachable_gaps: PathBuf,
    entry_summary: PathBuf,
    entry_plan: PathBuf,
    leaf_summary: PathBuf,
    leaf_blocks: PathBuf,
    optional: Vec<ProofGroup>,
}

impl InputPaths {
    fn new(build_dir: &Path) -> Self {
        let static_dir = build_dir.join("static_cfg");
        let entry_dir = build_dir.join("static_entry_plan");
        let leaf_dir = build_dir.join("static_leaf_verify");
        Self {
            static_summary: static_dir.join("static_cfg_summary.txt"),
            reachable_gaps: static_dir.join("coverage_gap_reachable.tsv"),
            entry_summary: entry_dir.join("static_entry_plan_summary.txt"),
            entry_plan: entry_dir.join("static_entry_plan.tsv"),
            leaf_summary: leaf_dir.join("static_leaf_verify_summary.txt"),
            leaf_blocks: leaf_dir.join("static_leaf_native_blocks.tsv"),
            optional: vec![
                ProofGroup {
                    name: "static handoff",
                    blocks: build_dir
                        .join("static_handoff_verify")
                        .join("static_handoff_native_blocks.tsv"),
                    summary: build_dir
                        .join("static_handoff_verify")
                        .join("static_handoff_verify_summary.txt"),
                    manifest: build_dir
                        .join("static_handoff_verify")
                        .join("native_verify")
                        .join("manifest.txt"),
                },
                ProofGroup {
                    name: "static branch",
                    blocks: build_dir
                        .join("static_branch_verify")
                        .join("static_branch_native_blocks.tsv"),
                    summary: build_dir
                        .join("static_branch_verify")
                        .join("static_branch_verify_summary.txt"),
                    manifest: build_dir
                        .join("static_branch_verify")
                        .join("native_verify")
                        .join("manifest.txt"),
                },
                ProofGroup {
                    name: "static JSR",
                    blocks: build_dir
                        .join("static_jsr_verify")
                        .join("static_jsr_native_blocks.tsv"),
                    summary: build_dir
                        .join("static_jsr_verify")
                        .join("static_jsr_verify_summary.txt"),
                    manifest: build_dir
                        .join("static_jsr_verify")
                        .join("native_verify")
                        .join("manifest.txt"),
                },
                ProofGroup {
                    name: "static return",
                    blocks: build_dir
                        .join("static_return_verify")
                        .join("static_return_native_blocks.tsv"),
                    summary: build_dir
                        .join("static_return_verify")
                        .join("static_return_verify_summary.txt"),
                    manifest: build_dir
                        .join("static_return_verify")
                        .join("native_verify")
                        .join("manifest.txt"),
                },
            ],
            static_dir,
            entry_dir,
            leaf_dir,
        }
    }

    fn require_required(&self) -> io::Result<()> {
        for path in [
            &self.static_summary,
            &self.reachable_gaps,
            &self.entry_summary,
            &self.entry_plan,
            &self.leaf_summary,
            &self.leaf_blocks,
        ] {
            require_file(path)?;
        }
        Ok(())
    }

    fn present_optional_proofs(&self) -> Result<Vec<ProofGroup>, Box<dyn std::error::Error>> {
        let mut groups = Vec::new();
        for group in &self.optional {
            let any_present =
                group.blocks.exists() || group.summary.exists() || group.manifest.exists();
            if !any_present {
                continue;
            }
            for path in [&group.blocks, &group.summary, &group.manifest] {
                if !path.is_file() {
                    return Err(format!(
                        "static_rom_audit: incomplete {} proof input: {}",
                        group.name,
                        path.display()
                    )
                    .into());
                }
            }
            let summary = read_key_values(&group.summary)?;
            let manifest = read_key_values(&group.manifest)?;
            require_key_value(&summary, "complete", "1", &group.summary)?;
            require_key_value(&manifest, "complete", "1", &group.manifest)?;
            require_key_value(&manifest, "mismatches", "0", &group.manifest)?;
            require_key_value(&manifest, "external_write_mismatches", "0", &group.manifest)?;
            groups.push(group.clone());
        }
        Ok(groups)
    }
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("static_rom_audit: missing input: {}", path.display()),
        ))
    }
}

fn read_key_values(path: &Path) -> io::Result<BTreeMap<String, String>> {
    let text = fs::read_to_string(path)?;
    let mut values = BTreeMap::new();
    for line in text.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.to_string(), value.to_string());
        }
    }
    Ok(values)
}

fn require_value(
    values: &BTreeMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    values
        .get(key)
        .cloned()
        .ok_or_else(|| format!("static_rom_audit: missing {key} in {}", path.display()).into())
}

fn require_key_value(
    values: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = require_value(values, key, path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "static_rom_audit: expected {key}={expected} in {}, got {actual}",
            path.display()
        )
        .into())
    }
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn invalid_tsv<T>(path: &Path, line_no: usize, actual: usize, expected: usize) -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "{}:{line_no} has {actual} fields, expected at least {expected}",
            path.display()
        ),
    ))
}

fn read_entry_plan(path: &Path) -> io::Result<HashMap<String, EntryInfo>> {
    let text = fs::read_to_string(path)?;
    let mut entries = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 21 {
            return invalid_tsv(path, line_no + 1, fields.len(), 21);
        }
        entries.insert(
            fields[6].to_uppercase(),
            EntryInfo {
                shape: fields[19].to_string(),
                recommended: fields[20].to_string(),
            },
        );
    }
    Ok(entries)
}

fn read_proofs(paths: &[PathBuf]) -> io::Result<HashMap<String, ProofInfo>> {
    let mut proofs = HashMap::<String, ProofInfo>::new();
    for path in paths {
        let text = fs::read_to_string(path)?;
        for (line_no, line) in text.lines().enumerate().skip(1) {
            let fields = split_tsv(line);
            if fields.len() < 15 {
                return invalid_tsv(path, line_no + 1, fields.len(), 15);
            }
            let key = fields[2].to_uppercase();
            let reason = fields[14].to_string();
            let info = proofs.entry(key).or_default();
            info.verified = true;
            if info.seen.insert(reason.clone()) {
                info.reasons.push(reason);
            }
        }
    }
    Ok(proofs)
}

fn build_frontier(
    reachable_gaps: &Path,
    entries: &HashMap<String, EntryInfo>,
    proofs: &HashMap<String, ProofInfo>,
    top_limit: usize,
) -> io::Result<Vec<FrontierRow>> {
    let text = fs::read_to_string(reachable_gaps)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        if rows.len() >= top_limit {
            break;
        }
        let fields = split_tsv(line);
        if fields.len() < 14 {
            return invalid_tsv(reachable_gaps, line_no + 1, fields.len(), 14);
        }
        let key = fields[5].to_uppercase();
        let entry = entries.get(&key);
        let proof = proofs.get(&key);
        rows.push(FrontierRow {
            rank: fields[0].to_string(),
            priority: fields[1].to_string(),
            bank_kind: fields[2].to_string(),
            bank: fields[3].to_string(),
            cpu_addr: fields[4].to_string(),
            prg_offset: fields[5].to_string(),
            label: fields[6].to_string(),
            first_opcode: fields[7].to_string(),
            mnemonic: fields[8].to_string(),
            known_opcode: fields[9].to_string(),
            mapped_in_edges: fields[10].to_string(),
            reachable_in_edges: fields[11].to_string(),
            reachable_from: fields[12].to_string(),
            static_shape: entry
                .map(|entry| entry.shape.clone())
                .unwrap_or_else(|| "not_in_current_entry_plan".to_string()),
            recommended_next_step: entry
                .map(|entry| entry.recommended.clone())
                .unwrap_or_else(|| "expand_static_entry_plan_or_confirm_code".to_string()),
            synthetic_verified: u64::from(proof.map(|proof| proof.verified).unwrap_or(false)),
            native_static_reason: proof
                .map(|proof| proof.reasons.join(","))
                .unwrap_or_default(),
            file: fields[13].to_string(),
        });
    }
    Ok(rows)
}

fn collect_values(
    static_summary: &BTreeMap<String, String>,
    entry_summary: &BTreeMap<String, String>,
    leaf_summary: &BTreeMap<String, String>,
    paths: &InputPaths,
    proof_files: &[PathBuf],
) -> Result<AuditValues, Box<dyn std::error::Error>> {
    Ok(AuditValues {
        label_count: require_value(static_summary, "label_count", &paths.static_summary)?,
        known_label_count: require_value(
            static_summary,
            "known_opcode_label_count",
            &paths.static_summary,
        )?,
        data_label_count: require_value(
            static_summary,
            "data_or_unknown_label_count",
            &paths.static_summary,
        )?,
        covered_label_count: require_value(
            static_summary,
            "covered_label_count",
            &paths.static_summary,
        )?,
        uncovered_label_count: require_value(
            static_summary,
            "uncovered_label_count",
            &paths.static_summary,
        )?,
        uncovered_known_count: require_value(
            static_summary,
            "uncovered_known_opcode_label_count",
            &paths.static_summary,
        )?,
        instruction_count: require_value(
            static_summary,
            "instruction_count",
            &paths.static_summary,
        )?,
        edge_count: require_value(static_summary, "edge_count", &paths.static_summary)?,
        reachable_label_count: require_value(
            static_summary,
            "static_reachable_label_count",
            &paths.static_summary,
        )?,
        reachable_uncovered_count: require_value(
            static_summary,
            "static_reachable_uncovered_label_count",
            &paths.static_summary,
        )?,
        reachable_uncovered_known_count: require_value(
            static_summary,
            "static_reachable_uncovered_known_opcode_label_count",
            &paths.static_summary,
        )?,
        entry_candidate_count: require_value(
            entry_summary,
            "candidate_count",
            &paths.entry_summary,
        )?,
        entry_leaf_count: require_value(
            entry_summary,
            "leaf_return_or_interrupt_count",
            &paths.entry_summary,
        )?,
        entry_control_count: require_value(
            entry_summary,
            "control_flow_or_call_count",
            &paths.entry_summary,
        )?,
        entry_indirect_count: require_value(
            entry_summary,
            "unresolved_indirect_count",
            &paths.entry_summary,
        )?,
        entry_data_count: require_value(
            entry_summary,
            "data_or_unknown_count",
            &paths.entry_summary,
        )?,
        leaf_selected_count: require_value(leaf_summary, "selected_count", &paths.leaf_summary)?,
        leaf_case_count: require_value(leaf_summary, "synthetic_case_count", &paths.leaf_summary)?,
        leaf_ram_writes: require_value(leaf_summary, "ram_writes_total", &paths.leaf_summary)?,
        leaf_external_writes: require_value(
            leaf_summary,
            "external_writes_total",
            &paths.leaf_summary,
        )?,
        leaf_unmapped_reads: require_value(
            leaf_summary,
            "unmapped_reads_total",
            &paths.leaf_summary,
        )?,
        leaf_skipped_call_like: require_value(
            leaf_summary,
            "skipped_call_like",
            &paths.leaf_summary,
        )?,
        leaf_skipped_missing_byte_count: require_value(
            leaf_summary,
            "skipped_missing_byte_count",
            &paths.leaf_summary,
        )?,
        leaf_skipped_unsupported: require_value(
            leaf_summary,
            "skipped_unsupported",
            &paths.leaf_summary,
        )?,
        leaf_skipped_loop_like: leaf_summary
            .get("skipped_loop_like")
            .cloned()
            .unwrap_or_else(|| "0".to_string()),
        static_handoff_verified_count: count_tsv_rows(&paths.optional[0].blocks)?,
        static_branch_verified_count: count_tsv_rows(&paths.optional[1].blocks)?,
        static_jsr_verified_count: count_tsv_rows(&paths.optional[2].blocks)?,
        static_return_verified_count: count_tsv_rows(&paths.optional[3].blocks)?,
        static_verified_block_count: count_unique_prg_offsets(proof_files)?,
        ..AuditValues::default()
    })
}

fn count_tsv_rows(path: &Path) -> io::Result<u64> {
    if !path.is_file() {
        return Ok(0);
    }
    let text = fs::read_to_string(path)?;
    Ok(text.lines().skip(1).count() as u64)
}

fn count_unique_prg_offsets(paths: &[PathBuf]) -> io::Result<u64> {
    let mut seen = HashSet::<String>::new();
    for path in paths {
        let text = fs::read_to_string(path)?;
        for (line_no, line) in text.lines().enumerate().skip(1) {
            let fields = split_tsv(line);
            if fields.len() < 3 {
                return invalid_tsv(path, line_no + 1, fields.len(), 3);
            }
            seen.insert(fields[2].to_uppercase());
        }
    }
    Ok(seen.len() as u64)
}

fn summarize_frontier(rows: &[FrontierRow], values: &mut AuditValues) {
    values.frontier_count = rows.len() as u64;
    for row in rows {
        if row.synthetic_verified == 1 {
            values.frontier_verified_count += 1;
        } else {
            values.frontier_unverified_count += 1;
            match row.static_shape.as_str() {
                "leaf_return_or_interrupt" => values.frontier_unverified_leaf += 1,
                "calls_subroutine" => values.frontier_unverified_calls += 1,
                "control_flow" => values.frontier_unverified_control += 1,
                "straight_line_or_data" => values.frontier_unverified_straight += 1,
                "not_in_current_entry_plan" => values.frontier_unverified_unqueued += 1,
                _ => values.frontier_unverified_other += 1,
            }
        }
        if row.static_shape == "not_in_current_entry_plan" {
            values.frontier_unqueued_count += 1;
        }
    }
}

fn write_frontier(path: &Path, rows: &[FrontierRow]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tstatic_shape\trecommended_next_step\tsynthetic_verified\tnative_static_reason\tfile"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.rank,
            row.priority,
            row.bank_kind,
            row.bank,
            row.cpu_addr,
            row.prg_offset,
            row.label,
            row.first_opcode,
            row.mnemonic,
            row.known_opcode,
            row.mapped_in_edges,
            row.reachable_in_edges,
            row.reachable_from,
            row.static_shape,
            row.recommended_next_step,
            row.synthetic_verified,
            row.native_static_reason,
            row.file
        )?;
    }
    Ok(())
}

fn write_metrics(path: &Path, values: &AuditValues) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "metric\tvalue\tnote")?;
    metric(
        &mut file,
        "static_label_candidates",
        &values.label_count,
        "all mapper-aware labels decoded from PRG banks",
    )?;
    metric(
        &mut file,
        "known_opcode_label_candidates",
        &values.known_label_count,
        "labels whose first byte decodes as an opcode instead of data",
    )?;
    metric(
        &mut file,
        "data_or_unknown_label_candidates",
        &values.data_label_count,
        "labels kept uncertain instead of treated as proven code",
    )?;
    metric(
        &mut file,
        "replay_covered_labels",
        &values.covered_label_count,
        "labels covered by committed replay block starts",
    )?;
    metric(
        &mut file,
        "uncovered_labels",
        &values.uncovered_label_count,
        "static labels not covered by committed replay starts",
    )?;
    metric(
        &mut file,
        "uncovered_known_opcode_labels",
        &values.uncovered_known_count,
        "uncovered labels whose first byte decodes as code",
    )?;
    metric(
        &mut file,
        "static_instruction_rows",
        &values.instruction_count,
        "linear disassembly instruction rows considered",
    )?;
    metric(
        &mut file,
        "static_control_edges",
        &values.edge_count,
        "mapped and uncertain control-flow edges emitted",
    )?;
    metric(
        &mut file,
        "reachable_static_labels",
        &values.reachable_label_count,
        "labels reachable from vectors or dynamic roots through mapped static edges",
    )?;
    metric(
        &mut file,
        "reachable_uncovered_labels",
        &values.reachable_uncovered_count,
        "reachable labels still not covered by replay block starts",
    )?;
    metric(
        &mut file,
        "reachable_uncovered_known_opcode_labels",
        &values.reachable_uncovered_known_count,
        "reachable uncovered labels that look like code",
    )?;
    metric(
        &mut file,
        "current_entry_plan_candidates",
        &values.entry_candidate_count,
        "bounded frontier queued for replay or synthetic work",
    )?;
    metric(
        &mut file,
        "current_entry_plan_leaf_candidates",
        &values.entry_leaf_count,
        "queued leaf return/interrupt candidates",
    )?;
    metric(
        &mut file,
        "current_entry_plan_control_candidates",
        &values.entry_control_count,
        "queued control-flow or call candidates",
    )?;
    metric(
        &mut file,
        "current_entry_plan_indirect_candidates",
        &values.entry_indirect_count,
        "queued unresolved indirect candidates",
    )?;
    metric(
        &mut file,
        "current_entry_plan_data_candidates",
        &values.entry_data_count,
        "queued data/unknown candidates",
    )?;
    metric(
        &mut file,
        "synthetic_native_leaf_blocks",
        &values.leaf_selected_count,
        "static leaves already verified against synthetic oracle states",
    )?;
    metric(
        &mut file,
        "synthetic_native_leaf_cases",
        &values.leaf_case_count,
        "synthetic CPU/RAM cases verified for static leaves",
    )?;
    metric(
        &mut file,
        "synthetic_native_leaf_ram_writes",
        &values.leaf_ram_writes,
        "RAM writes admitted after final RAM hash verification",
    )?;
    metric(
        &mut file,
        "synthetic_native_leaf_external_writes",
        &values.leaf_external_writes,
        "PPU/APU/mapper writes admitted by the static leaf gate",
    )?;
    metric(
        &mut file,
        "synthetic_native_leaf_unmapped_reads",
        &values.leaf_unmapped_reads,
        "unmapped reads admitted by the static leaf gate",
    )?;
    metric(
        &mut file,
        "synthetic_native_handoff_blocks",
        &values.static_handoff_verified_count.to_string(),
        "straight-line handoff blocks already verified against synthetic oracle states",
    )?;
    metric(
        &mut file,
        "synthetic_native_branch_blocks",
        &values.static_branch_verified_count.to_string(),
        "branch blocks already verified against synthetic oracle states",
    )?;
    metric(
        &mut file,
        "synthetic_native_jsr_blocks",
        &values.static_jsr_verified_count.to_string(),
        "JSR handoff blocks already verified against synthetic oracle states",
    )?;
    metric(
        &mut file,
        "synthetic_native_return_blocks",
        &values.static_return_verified_count.to_string(),
        "return-prefix blocks already verified against synthetic oracle states",
    )?;
    metric(
        &mut file,
        "synthetic_native_unique_prg_offsets",
        &values.static_verified_block_count.to_string(),
        "unique PRG offsets covered by any static synthetic proof",
    )?;
    metric(
        &mut file,
        "static_leaf_skipped_call_like",
        &values.leaf_skipped_call_like,
        "leaf-shaped entries deferred because they include call edges",
    )?;
    metric(
        &mut file,
        "static_leaf_skipped_missing_byte_count",
        &values.leaf_skipped_missing_byte_count,
        "leaf entries skipped because byte count could not be bounded",
    )?;
    metric(
        &mut file,
        "static_leaf_skipped_unsupported",
        &values.leaf_skipped_unsupported,
        "leaf entries skipped because generated native opcode support is missing",
    )?;
    metric(
        &mut file,
        "static_leaf_skipped_loop_like",
        &values.leaf_skipped_loop_like,
        "leaf entries deferred because they contain in-block backward branches",
    )?;
    metric(
        &mut file,
        "frontier_rows",
        &values.frontier_count.to_string(),
        "top reachable uncovered candidates written to static_rom_frontier.tsv",
    )?;
    metric(
        &mut file,
        "frontier_synthetic_verified",
        &values.frontier_verified_count.to_string(),
        "frontier rows already covered by static synthetic native verification",
    )?;
    metric(
        &mut file,
        "frontier_unverified",
        &values.frontier_unverified_count.to_string(),
        "frontier rows that still need replay routing, handoff states, or opcode support",
    )?;
    metric(
        &mut file,
        "frontier_unverified_leaf",
        &values.frontier_unverified_leaf.to_string(),
        "unverified frontier leaf rows, usually call-like leaves deferred from simple leaf proof",
    )?;
    metric(
        &mut file,
        "frontier_unverified_calls",
        &values.frontier_unverified_calls.to_string(),
        "unverified frontier rows that call subroutines",
    )?;
    metric(
        &mut file,
        "frontier_unverified_control",
        &values.frontier_unverified_control.to_string(),
        "unverified frontier rows with branch or jump control flow",
    )?;
    metric(
        &mut file,
        "frontier_unverified_straight",
        &values.frontier_unverified_straight.to_string(),
        "unverified frontier straight-line rows needing handoff states",
    )?;
    metric(
        &mut file,
        "frontier_unverified_unqueued",
        &values.frontier_unverified_unqueued.to_string(),
        "unverified frontier rows outside the bounded current entry queue",
    )?;
    metric(
        &mut file,
        "frontier_unverified_other",
        &values.frontier_unverified_other.to_string(),
        "unverified frontier rows in other static shapes",
    )?;
    metric(
        &mut file,
        "frontier_not_in_current_entry_plan",
        &values.frontier_unqueued_count.to_string(),
        "frontier rows outside the bounded current entry queue",
    )?;
    Ok(())
}

fn metric(file: &mut fs::File, name: &str, value: &str, note: &str) -> io::Result<()> {
    writeln!(file, "{name}\t{value}\t{note}")
}

fn write_summary(
    path: &Path,
    paths: &InputPaths,
    top_limit: usize,
    values: &AuditValues,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=static_rom_audit")?;
    writeln!(file, "static_cfg_dir={}", paths.static_dir.display())?;
    writeln!(file, "static_entry_plan_dir={}", paths.entry_dir.display())?;
    writeln!(file, "static_leaf_verify_dir={}", paths.leaf_dir.display())?;
    writeln!(file, "top_limit={top_limit}")?;
    writeln!(file, "label_count={}", values.label_count)?;
    writeln!(
        file,
        "known_opcode_label_count={}",
        values.known_label_count
    )?;
    writeln!(
        file,
        "data_or_unknown_label_count={}",
        values.data_label_count
    )?;
    writeln!(file, "covered_label_count={}", values.covered_label_count)?;
    writeln!(
        file,
        "uncovered_label_count={}",
        values.uncovered_label_count
    )?;
    writeln!(
        file,
        "uncovered_known_opcode_label_count={}",
        values.uncovered_known_count
    )?;
    writeln!(
        file,
        "static_reachable_label_count={}",
        values.reachable_label_count
    )?;
    writeln!(
        file,
        "static_reachable_uncovered_label_count={}",
        values.reachable_uncovered_count
    )?;
    writeln!(
        file,
        "static_reachable_uncovered_known_opcode_label_count={}",
        values.reachable_uncovered_known_count
    )?;
    writeln!(
        file,
        "static_entry_candidate_count={}",
        values.entry_candidate_count
    )?;
    writeln!(
        file,
        "static_leaf_selected_count={}",
        values.leaf_selected_count
    )?;
    writeln!(
        file,
        "static_leaf_synthetic_case_count={}",
        values.leaf_case_count
    )?;
    writeln!(
        file,
        "static_leaf_ram_writes_total={}",
        values.leaf_ram_writes
    )?;
    writeln!(
        file,
        "static_leaf_external_writes_total={}",
        values.leaf_external_writes
    )?;
    writeln!(
        file,
        "static_leaf_unmapped_reads_total={}",
        values.leaf_unmapped_reads
    )?;
    writeln!(
        file,
        "static_handoff_verified_count={}",
        values.static_handoff_verified_count
    )?;
    writeln!(
        file,
        "static_branch_verified_count={}",
        values.static_branch_verified_count
    )?;
    writeln!(
        file,
        "static_jsr_verified_count={}",
        values.static_jsr_verified_count
    )?;
    writeln!(
        file,
        "static_return_verified_count={}",
        values.static_return_verified_count
    )?;
    writeln!(
        file,
        "static_verified_unique_prg_offsets={}",
        values.static_verified_block_count
    )?;
    writeln!(
        file,
        "static_leaf_skipped_call_like={}",
        values.leaf_skipped_call_like
    )?;
    writeln!(
        file,
        "static_leaf_skipped_missing_byte_count={}",
        values.leaf_skipped_missing_byte_count
    )?;
    writeln!(
        file,
        "static_leaf_skipped_unsupported={}",
        values.leaf_skipped_unsupported
    )?;
    writeln!(
        file,
        "static_leaf_skipped_loop_like={}",
        values.leaf_skipped_loop_like
    )?;
    writeln!(file, "frontier_count={}", values.frontier_count)?;
    writeln!(
        file,
        "frontier_synthetic_verified={}",
        values.frontier_verified_count
    )?;
    writeln!(
        file,
        "frontier_unverified={}",
        values.frontier_unverified_count
    )?;
    writeln!(
        file,
        "frontier_unverified_leaf={}",
        values.frontier_unverified_leaf
    )?;
    writeln!(
        file,
        "frontier_unverified_calls={}",
        values.frontier_unverified_calls
    )?;
    writeln!(
        file,
        "frontier_unverified_control={}",
        values.frontier_unverified_control
    )?;
    writeln!(
        file,
        "frontier_unverified_straight={}",
        values.frontier_unverified_straight
    )?;
    writeln!(
        file,
        "frontier_unverified_unqueued={}",
        values.frontier_unverified_unqueued
    )?;
    writeln!(
        file,
        "frontier_unverified_other={}",
        values.frontier_unverified_other
    )?;
    writeln!(
        file,
        "frontier_not_in_current_entry_plan={}",
        values.frontier_unqueued_count
    )?;
    writeln!(file, "metrics=static_rom_audit.tsv")?;
    writeln!(file, "frontier=static_rom_frontier.tsv")?;
    writeln!(file, "analysis_kind=conservative_static_frontier")?;
    writeln!(file, "behavior_proven_by_static_analysis=0")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(path: &Path) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "runtime=static_rom_audit")?;
    writeln!(file, "summary=static_rom_audit_summary.txt")?;
    writeln!(file, "metrics=static_rom_audit.tsv")?;
    writeln!(file, "frontier=static_rom_frontier.tsv")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_static_rom_audit() {
        let root = std::env::temp_dir().join(format!(
            "lotw_static_rom_audit_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let build = root.join("build");
        let out = root.join("out");
        fs::create_dir_all(build.join("static_cfg")).unwrap();
        fs::create_dir_all(build.join("static_entry_plan")).unwrap();
        fs::create_dir_all(build.join("static_leaf_verify")).unwrap();
        fs::create_dir_all(build.join("static_handoff_verify/native_verify")).unwrap();
        fs::write(
            build.join("static_cfg/static_cfg_summary.txt"),
            "label_count=5\nknown_opcode_label_count=5\ndata_or_unknown_label_count=0\ncovered_label_count=1\nuncovered_label_count=4\nuncovered_known_opcode_label_count=4\ninstruction_count=9\nedge_count=6\ncomplete=1\nstatic_reachable_label_count=4\nstatic_reachable_uncovered_label_count=3\nstatic_reachable_uncovered_known_opcode_label_count=3\n",
        )
        .unwrap();
        fs::write(
            build.join("static_cfg/coverage_gap_reachable.tsv"),
            "rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tfile\n\
             1\t2000\tfixed\t1\tC010\t04010\tL_C010\t60\tRTS\t1\t2\t2\tdynamic\tfixture.asm\n\
             2\t1000\tfixed\t1\tC020\t04020\tL_C020\tA9\tLDA\t1\t1\t1\tvector_reset\tfixture.asm\n\
             3\t900\tfixed\t1\tC030\t04030\tL_C030\tA5\tLDA\t1\t1\t1\tvector_reset\tfixture.asm\n",
        )
        .unwrap();
        fs::write(
            build.join("static_entry_plan/static_entry_plan_summary.txt"),
            "candidate_count=3\nleaf_return_or_interrupt_count=1\ncontrol_flow_or_call_count=2\nunresolved_indirect_count=0\ndata_or_unknown_count=0\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            build.join("static_entry_plan/static_entry_plan.tsv"),
            "rank\tgap_rank\tpriority\tbank_kind\tbank\tcpu_addr\tprg_offset\tlabel\tfirst_opcode\tmnemonic\tknown_opcode\tmapped_in_edges\treachable_in_edges\treachable_from\tsource_edge_count\tsource_labels\tsource_instruction_cpu_addrs\tsource_edge_types\toutgoing_edge_types\tstatic_shape\trecommended_next_step\tfile\n\
             1\t1\t2000\tfixed\t1\tC010\t04010\tL_C010\t60\tRTS\t1\t2\t2\tdynamic\t1\tL_C000\tC001\tcall_return\tstop\tleaf_return_or_interrupt\tadd_synthetic_first_hit_state_then_native_leaf_test\tfixture.asm\n\
             2\t2\t1000\tfixed\t1\tC020\t04020\tL_C020\tA9\tLDA\t1\t1\t1\tvector_reset\t1\tL_C000\tC003\tbranch_target\t\tcontrol_flow\ttarget_replay_or_synthetic_handoff_state\tfixture.asm\n\
             3\t3\t900\tfixed\t1\tC030\t04030\tL_C030\tA5\tLDA\t1\t1\t1\tvector_reset\t1\tL_C000\tC004\tcall_target\t\tstraight_line_or_data\ttarget_replay_or_synthetic_handoff_state\tfixture.asm\n",
        )
        .unwrap();
        fs::write(
            build.join("static_leaf_verify/static_leaf_verify_summary.txt"),
            "selected_count=1\nsynthetic_case_count=4\nram_writes_total=0\nexternal_writes_total=0\nunmapped_reads_total=0\nskipped_call_like=0\nskipped_missing_byte_count=0\nskipped_unsupported=0\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            build.join("static_leaf_verify/static_leaf_native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n\
             1\tC010\t04010\t1\t60\t0\tstatic_entry_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_leaf_synthetic\n",
        )
        .unwrap();
        fs::write(
            build.join("static_handoff_verify/static_handoff_verify_summary.txt"),
            "runtime=static_handoff_verify\nselected_count=1\nsynthetic_case_count=4\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            build.join("static_handoff_verify/native_verify/manifest.txt"),
            "case_count=4\nmatched=4\nmismatches=0\nexternal_write_mismatches=0\ncomplete=1\n",
        )
        .unwrap();
        fs::write(
            build.join("static_handoff_verify/static_handoff_native_blocks.tsv"),
            "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason\n\
             1\tC020\t04020\t2\tA9\t0\tstatic_handoff_plan\t4\t0\t0\t0\t0\t0\t4\tstatic_handoff_linear\n",
        )
        .unwrap();

        run(&build, &out, 3).unwrap();

        let summary = fs::read_to_string(out.join("static_rom_audit_summary.txt")).unwrap();
        assert!(summary.contains("frontier_count=3\n"));
        assert!(summary.contains("frontier_synthetic_verified=2\n"));
        assert!(summary.contains("frontier_unverified=1\n"));
        assert!(summary.contains("static_verified_unique_prg_offsets=2\n"));
        let frontier = fs::read_to_string(out.join("static_rom_frontier.tsv")).unwrap();
        assert!(frontier.contains("1\t2000\tfixed\t1\tC010\t04010\tL_C010\t60\tRTS\t1\t2\t2\tdynamic\tleaf_return_or_interrupt\tadd_synthetic_first_hit_state_then_native_leaf_test\t1\tstatic_leaf_synthetic\tfixture.asm\n"));
        assert!(frontier.contains("3\t900\tfixed\t1\tC030\t04030\tL_C030\tA5\tLDA\t1\t1\t1\tvector_reset\tstraight_line_or_data\ttarget_replay_or_synthetic_handoff_state\t0\t\tfixture.asm\n"));
        let _ = fs::remove_dir_all(root);
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
