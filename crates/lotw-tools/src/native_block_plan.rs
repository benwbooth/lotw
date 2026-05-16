use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const STATIC_EXTERNAL_CPUS: &[&str] = &[
    "AE6F", "AE76", "AEB5", "B69F", "D41F", "FA54", "FB82", "FD9C",
];
const PINNED_CPUS: &[&str] = &[
    "AE6F", "AE76", "AEB5", "AF05", "B69F", "C3F1", "C409", "C46B", "C483", "D3C6", "D408", "D41F",
    "F8F0", "F96E", "FA54", "FA74", "FB1F", "FB82", "F8EC", "FD9C", "CAB6", "CABE", "CC97", "C1C7",
    "E11E", "E5CA", "E5CD", "E6B3", "E6B6",
];

#[derive(Debug)]
struct TranslationRow {
    replay: String,
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    status: String,
    hit_count: u64,
    writes: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
    state_applied: u64,
    final_ram_sha256: String,
    class: String,
}

#[derive(Debug, Default)]
struct CandidateAggregate {
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    replay_order: Vec<String>,
    replay_seen: HashSet<String>,
    hit_count_total: u64,
    observations: u64,
    writes_total: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
    final_hashes: HashSet<String>,
}

#[derive(Debug, Clone)]
struct Candidate {
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    replay_count: u64,
    replays: String,
    observations: u64,
    hit_count_total: u64,
    writes_total: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
    final_ram_hash_count: u64,
    reason: String,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if limit == 0 {
        return Err("native_block_plan: limit must be a positive integer".into());
    }

    let plan = build_dir
        .join("block_translation_plan")
        .join("block_translation_plan.tsv");
    require_file(&plan)?;
    fs::create_dir_all(out_dir)?;

    let candidates = candidates_from_rows(&read_translation_rows(&plan)?);
    let selected = select_candidates(&candidates, limit);

    write_native_blocks(&out_dir.join("native_blocks.tsv"), &selected)?;
    write_opcode_summary(&out_dir.join("native_opcode_summary.tsv"), &selected)?;
    write_summary(
        &out_dir.join("native_block_summary.tsv"),
        &candidates,
        &selected,
        limit,
    )?;
    write_manifest(&out_dir.join("manifest.txt"), build_dir)?;

    println!("native_block_plan: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("native_block_plan: missing input: {}", path.display()),
        ))
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

fn parse_u64(value: &str) -> u64 {
    value.parse::<u64>().unwrap_or(0)
}

fn read_translation_rows(path: &Path) -> io::Result<Vec<TranslationRow>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 18 {
            return invalid_tsv(path, line_no + 1, fields.len(), 18);
        }
        rows.push(TranslationRow {
            replay: fields[0].to_string(),
            cpu_addr: fields[2].to_string(),
            prg_offset: fields[3].to_string(),
            bytes: fields[4].to_string(),
            first_opcode: fields[5].to_string(),
            status: fields[7].to_string(),
            hit_count: parse_u64(fields[8]),
            writes: parse_u64(fields[10]),
            ppu_writes: parse_u64(fields[11]),
            apu_writes: parse_u64(fields[12]),
            mapper_writes: parse_u64(fields[13]),
            state_applied: parse_u64(fields[14]),
            final_ram_sha256: fields[15].to_string(),
            class: fields[16].to_string(),
        });
    }
    Ok(rows)
}

fn candidates_from_rows(rows: &[TranslationRow]) -> Vec<Candidate> {
    let mut order = Vec::new();
    let mut aggregates = HashMap::<String, CandidateAggregate>::new();
    for row in rows {
        if !is_native_subset(row)
            || row.status != "left_block"
            || row.state_applied != 1
            || !is_lower_hex_sha256(&row.final_ram_sha256)
        {
            continue;
        }

        let key = format!(
            "{}\t{}\t{}\t{}",
            row.cpu_addr, row.prg_offset, row.bytes, row.first_opcode
        );
        if !aggregates.contains_key(&key) {
            order.push(key.clone());
        }
        let entry = aggregates.entry(key).or_insert_with(|| CandidateAggregate {
            cpu_addr: row.cpu_addr.clone(),
            prg_offset: row.prg_offset.clone(),
            bytes: row.bytes.clone(),
            first_opcode: row.first_opcode.clone(),
            ..CandidateAggregate::default()
        });
        entry.hit_count_total += row.hit_count;
        entry.observations += 1;
        entry.writes_total += row.writes;
        entry.ppu_writes += row.ppu_writes;
        entry.apu_writes += row.apu_writes;
        entry.mapper_writes += row.mapper_writes;
        if entry.replay_seen.insert(row.replay.clone()) {
            entry.replay_order.push(row.replay.clone());
        }
        entry.final_hashes.insert(row.final_ram_sha256.clone());
    }

    let mut candidates = order
        .into_iter()
        .filter_map(|key| aggregates.remove(&key))
        .filter(|item| {
            is_static_external_cpu(&item.cpu_addr)
                || (item.ppu_writes == 0 && item.apu_writes == 0 && item.mapper_writes == 0)
        })
        .map(|item| Candidate {
            reason: reason_for(&item),
            replay_count: item.replay_order.len() as u64,
            replays: item.replay_order.join(","),
            final_ram_hash_count: item.final_hashes.len() as u64,
            cpu_addr: item.cpu_addr,
            prg_offset: item.prg_offset,
            bytes: item.bytes,
            first_opcode: item.first_opcode,
            observations: item.observations,
            hit_count_total: item.hit_count_total,
            writes_total: item.writes_total,
            ppu_writes: item.ppu_writes,
            apu_writes: item.apu_writes,
            mapper_writes: item.mapper_writes,
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|lhs, rhs| {
        rhs.hit_count_total
            .cmp(&lhs.hit_count_total)
            .then_with(|| lhs.cpu_addr.cmp(&rhs.cpu_addr))
    });
    candidates
}

fn is_native_subset(row: &TranslationRow) -> bool {
    if row.class == "straight_line" {
        return true;
    }
    if row.class == "return_or_interrupt"
        && (row.first_opcode == "60"
            || matches!(
                row.cpu_addr.as_str(),
                "C1C7"
                    | "C1D8"
                    | "C1EB"
                    | "CABE"
                    | "CC97"
                    | "FCB7"
                    | "D408"
                    | "D415"
                    | "B6ED"
                    | "CD5D"
                    | "E6B6"
                    | "F8EC"
                    | "F96E"
                    | "FA74"
            )
            || is_static_external_cpu(&row.cpu_addr))
    {
        return true;
    }
    matches!(
        row.cpu_addr.as_str(),
        "AF05"
            | "B6D0"
            | "C3F1"
            | "C409"
            | "C46B"
            | "C483"
            | "CAB6"
            | "CC90"
            | "D3C6"
            | "D40F"
            | "D8B6"
            | "D8CB"
            | "E11E"
            | "E5CA"
            | "E5CD"
            | "E5F3"
            | "E6B3"
            | "F8F0"
            | "FB1F"
    ) || is_static_external_cpu(&row.cpu_addr)
}

fn is_static_external_cpu(cpu: &str) -> bool {
    STATIC_EXTERNAL_CPUS.contains(&cpu)
}

fn is_pinned_cpu(cpu: &str) -> bool {
    PINNED_CPUS.contains(&cpu)
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn reason_for(item: &CandidateAggregate) -> String {
    match item.cpu_addr.as_str() {
        "AE6F" => "ppu_control_external_write_leaf",
        "AE76" => "ppu_mask_zero_external_write",
        "AEB5" => "ppu_mask_external_write_leaf",
        "AF05" => "dialog_setup_jsr_handoff",
        "B69F" => "ppu_control_stack_restore_leaf",
        "B6D0" => "nibble_loop_body",
        "B6ED" => "branch_return",
        "C1C7" => "coordinate_pack_return",
        "C1D8" => "jsr_target_return",
        "C1EB" => "jsr_target_oam_shadow_return",
        "C3F1" => "sprite_nibble_shift_loop_entry",
        "C409" => "sprite_nibble_shift_loop_tail",
        "C46B" => "sprite_nibble_shift_return_entry",
        "C483" => "sprite_nibble_shift_return_tail",
        "CAB6" => "coordinate_clamp_branch",
        "CABE" => "coordinate_clamp_jsr_handoff",
        "CC90" => "stack_restore_loop",
        "CC97" => "wait_return",
        "CD5D" => "adc_absx_loop_return",
        "D3C6" => "ppu_status_vblank_wait_exit",
        "D408" => "timer_tick_return",
        "D40F" => "timer_branch",
        "D415" => "timer_loop_return",
        "D41F" => "mapper_bank_restore_loop",
        "D8B6" => "coordinate_adjust_entry",
        "D8CB" => "coordinate_adjust_finish",
        "E11E" => "jsr_cab6_handoff",
        "E5CA" => "jsr_coordinate_adjust_entry",
        "E5CD" => "coordinate_gate_handoff",
        "E5F3" => "jsr_handoff",
        "E6B3" => "jsr_coordinate_pack_handoff",
        "E6B6" => "jsr_return_rts",
        "F8F0" => "pulse_channel_gate",
        "F96E" => "pulse_sustain_gate_return",
        "F8EC" => "jsr_fd9c_handoff",
        "FA74" => "triangle_gate_return",
        "FA54" => "apu_triangle_linear_external_write_leaf",
        "FB82" => "apu_noise_volume_external_write_leaf",
        "FB1F" => "noise_channel_gate",
        "FCB7" => "accumulate_loop_return",
        "FD9C" => "mapper_bank_external_write",
        _ if item.first_opcode == "60" => "rts_return",
        _ if item.writes_total == 0 => "no_writes",
        _ => "ram_only_writes",
    }
    .to_string()
}

fn select_candidates(candidates: &[Candidate], limit: usize) -> Vec<Candidate> {
    let mut regular = Vec::new();
    let mut pinned = Vec::new();
    for candidate in candidates {
        if is_pinned_cpu(&candidate.cpu_addr) {
            pinned.push(candidate.clone());
        } else {
            regular.push(candidate.clone());
        }
    }

    let max_regular = limit.saturating_sub(pinned.len());
    let mut selected = Vec::new();
    for candidate in regular.into_iter().take(max_regular) {
        if selected.len() < limit {
            selected.push(candidate);
        }
    }
    for candidate in pinned {
        if selected.len() < limit {
            selected.push(candidate);
        }
    }
    selected
}

fn write_native_blocks(path: &Path, selected: &[Candidate]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason"
    )?;
    for (index, item) in selected.iter().enumerate() {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            item.cpu_addr,
            item.prg_offset,
            item.bytes,
            item.first_opcode,
            item.replay_count,
            item.replays,
            item.observations,
            item.hit_count_total,
            item.writes_total,
            item.ppu_writes,
            item.apu_writes,
            item.mapper_writes,
            item.final_ram_hash_count,
            item.reason
        )?;
    }
    Ok(())
}

fn write_opcode_summary(path: &Path, selected: &[Candidate]) -> io::Result<()> {
    let mut opcodes = BTreeMap::<String, (u64, u64)>::new();
    for item in selected {
        let entry = opcodes.entry(item.first_opcode.clone()).or_default();
        entry.0 += 1;
        entry.1 += item.hit_count_total;
    }
    let mut file = fs::File::create(path)?;
    writeln!(file, "first_opcode\tblocks\thit_count_total")?;
    for (opcode, (blocks, hits)) in opcodes {
        writeln!(file, "{opcode}\t{blocks}\t{hits}")?;
    }
    Ok(())
}

fn write_summary(
    path: &Path,
    candidates: &[Candidate],
    selected: &[Candidate],
    limit: usize,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "candidate_count={}", candidates.len())?;
    writeln!(file, "selected_count={}", selected.len())?;
    writeln!(
        file,
        "candidate_hit_count={}",
        candidates
            .iter()
            .map(|candidate| candidate.hit_count_total)
            .sum::<u64>()
    )?;
    writeln!(
        file,
        "selected_hit_count={}",
        selected
            .iter()
            .map(|candidate| candidate.hit_count_total)
            .sum::<u64>()
    )?;
    writeln!(file, "limit={limit}")?;
    writeln!(file, "selection=straight_line_rts_af05_b6d0_c3f1_c409_c46b_c483_cab6_cabe_cc97_c1c7_c1d8_c1eb_d3c6_d408_d8b6_d8cb_e11e_e5ca_e5cd_e6b3_e6b6_f8f0_f96e_f8ec_fa74_fb1f_fcb7_b6ed_cc90_cd5d_d40f_d415_or_e5f3_left_block_plus_static_external_writes")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(path: &Path, build_dir: &Path) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "build_dir={}", build_dir.display())?;
    writeln!(
        file,
        "source=block_translation_plan/block_translation_plan.tsv"
    )?;
    writeln!(file, "native_blocks=native_blocks.tsv")?;
    writeln!(file, "native_opcode_summary=native_opcode_summary.tsv")?;
    writeln!(file, "native_block_summary=native_block_summary.tsv")?;
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
            "lotw_tools_native_block_plan_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_native_block_plan() {
        let root = temp_dir();
        let build = root.join("build");
        let out = root.join("plan");
        write(
            &build.join("block_translation_plan/block_translation_plan.tsv"),
            "replay\tid\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstop_reason\tstatus\thit_count\tsteps\twrites\tppu_writes\tapu_writes\tmapper_writes\tstate_applied\tfinal_ram_sha256\tclass\tpriority\ntitle\t1\tB000\t1B000\t2\tA9\tnext_trace_label\tleft_block\t10\t1\t0\t0\t0\t0\t1\t0000000000000000000000000000000000000000000000000000000000000000\tstraight_line\t10\nwalk\t2\tB000\t1B000\t2\tA9\tnext_trace_label\tleft_block\t5\t1\t0\t0\t0\t0\t1\t1111111111111111111111111111111111111111111111111111111111111111\tstraight_line\t5\ntitle\t3\tC000\t1C000\t3\t85\tnext_trace_label\tleft_block\t20\t2\t1\t0\t0\t0\t1\t2222222222222222222222222222222222222222222222222222222222222222\tstraight_line\t20\ntitle\t4\tD000\t1D000\t2\tD0\tnext_trace_label\tleft_block\t30\t1\t0\t0\t0\t0\t1\t3333333333333333333333333333333333333333333333333333333333333333\tbranch\t30\ntitle\t5\tE000\t1E000\t2\tA2\tnext_trace_label\tstep_limit\t40\t64\t0\t0\t0\t0\t1\t4444444444444444444444444444444444444444444444444444444444444444\tstraight_line\t40\nwalk\t6\tF000\t1F000\t1\t18\tnext_trace_label\tleft_block\t7\t1\t0\t0\t0\t0\t1\t5555555555555555555555555555555555555555555555555555555555555555\tstraight_line\t7\n",
        );

        run(&build, &out, 2).unwrap();

        let summary = fs::read_to_string(out.join("native_block_summary.tsv")).unwrap();
        assert!(summary.contains("candidate_count=3\n"));
        assert!(summary.contains("selected_count=2\n"));
        assert!(summary.contains("candidate_hit_count=42\n"));
        assert!(summary.contains("selected_hit_count=35\n"));
        let blocks = fs::read_to_string(out.join("native_blocks.tsv")).unwrap();
        assert!(blocks
            .contains("1\tC000\t1C000\t3\t85\t1\ttitle\t1\t20\t1\t0\t0\t0\t1\tram_only_writes\n"));
        assert!(blocks
            .contains("2\tB000\t1B000\t2\tA9\t2\ttitle,walk\t2\t15\t0\t0\t0\t0\t2\tno_writes\n"));
        let opcodes = fs::read_to_string(out.join("native_opcode_summary.tsv")).unwrap();
        assert!(opcodes.contains("85\t1\t20\n"));
        assert!(opcodes.contains("A9\t1\t15\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
