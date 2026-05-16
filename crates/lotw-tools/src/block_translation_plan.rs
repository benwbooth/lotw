use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct BlockCandidate {
    hit_count: u64,
    stop_reason: String,
    control_flow_ops: u64,
    terminator_opcode: String,
}

#[derive(Debug, Clone)]
struct PlanRow {
    replay: String,
    id: String,
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    stop_reason: String,
    status: String,
    hit_count: u64,
    steps: String,
    writes: String,
    ppu_writes: String,
    apu_writes: String,
    mapper_writes: String,
    state_applied: String,
    final_ram_sha256: String,
    class: String,
    priority: u64,
}

#[derive(Debug, Default)]
struct ReplaySummary {
    replay: String,
    block_count: u64,
    straight_line: u64,
    call_or_jump: u64,
    return_or_interrupt: u64,
    branch: u64,
    loop_or_step_limit: u64,
    unsupported_or_invalid: u64,
    final_ram_hash_rows: u64,
    hit_count_total: u64,
}

#[derive(Debug, Default)]
struct OpcodeSummary {
    blocks: u64,
    hit_count: u64,
    straight_line: u64,
    call_or_jump: u64,
    return_or_interrupt: u64,
    branch: u64,
    loop_or_step_limit: u64,
    unsupported_or_invalid: u64,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    replays: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if replays.is_empty() {
        return Err("block_translation_plan: at least one replay is required".into());
    }

    fs::create_dir_all(out_dir)?;
    let mut rows = Vec::new();
    let mut summaries = Vec::new();

    for replay in replays {
        let blocks = build_dir
            .join("blocks")
            .join(replay)
            .join("block_candidates.tsv");
        let exec = build_dir
            .join("block_exec")
            .join(replay)
            .join("block_exec.tsv");
        require_file(&blocks)?;
        require_file(&exec)?;

        let candidates = read_candidates(&blocks)?;
        let (mut replay_rows, summary) = read_exec_rows(&exec, replay, &candidates)?;
        rows.append(&mut replay_rows);
        summaries.push(summary);
    }

    write_plan(&out_dir.join("block_translation_plan.tsv"), &rows)?;
    write_summary(&out_dir.join("summary.tsv"), &summaries)?;
    write_opcode_summary(&out_dir.join("opcode_summary.tsv"), &rows)?;
    write_manifest(&out_dir.join("manifest.txt"), build_dir, replays)?;

    println!("block_translation_plan: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("block_translation_plan: missing input: {}", path.display()),
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

fn read_candidates(path: &Path) -> io::Result<HashMap<String, BlockCandidate>> {
    let text = fs::read_to_string(path)?;
    let mut candidates = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 11 {
            return invalid_tsv(path, line_no + 1, fields.len(), 11);
        }
        candidates.insert(
            fields[0].to_string(),
            BlockCandidate {
                hit_count: parse_u64(fields[4]),
                stop_reason: fields[7].to_string(),
                control_flow_ops: parse_u64(fields[9]),
                terminator_opcode: fields[10].to_string(),
            },
        );
    }
    Ok(candidates)
}

fn read_exec_rows(
    path: &Path,
    replay: &str,
    candidates: &HashMap<String, BlockCandidate>,
) -> io::Result<(Vec<PlanRow>, ReplaySummary)> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    let mut summary = ReplaySummary {
        replay: replay.to_string(),
        ..ReplaySummary::default()
    };

    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 22 {
            return invalid_tsv(path, line_no + 1, fields.len(), 22);
        }
        let candidate = candidates.get(fields[0]);
        let hit_count = candidate.map(|item| item.hit_count).unwrap_or(0);
        let stop_reason = candidate
            .map(|item| item.stop_reason.clone())
            .unwrap_or_default();
        let control_flow_ops = candidate.map(|item| item.control_flow_ops).unwrap_or(0);
        let terminator_opcode = candidate
            .map(|item| item.terminator_opcode.as_str())
            .unwrap_or("");
        let class = classify(fields[4], fields[5], control_flow_ops, terminator_opcode);
        summary.add(&class, hit_count, fields[21]);

        rows.push(PlanRow {
            replay: replay.to_string(),
            id: fields[0].to_string(),
            cpu_addr: fields[1].to_string(),
            prg_offset: fields[2].to_string(),
            bytes: fields[3].to_string(),
            first_opcode: fields[4].to_string(),
            stop_reason,
            status: fields[5].to_string(),
            hit_count,
            steps: fields[6].to_string(),
            writes: fields[10].to_string(),
            ppu_writes: fields[11].to_string(),
            apu_writes: fields[12].to_string(),
            mapper_writes: fields[13].to_string(),
            state_applied: fields[15].to_string(),
            final_ram_sha256: fields[21].to_string(),
            class,
            priority: hit_count,
        });
    }

    Ok((rows, summary))
}

fn classify(opcode: &str, status: &str, control_flow_ops: u64, terminator_opcode: &str) -> String {
    if status == "unsupported_opcode" || status == "invalid_block" {
        "unsupported_or_invalid"
    } else if status == "step_limit" {
        "loop_or_step_limit"
    } else if control_flow_ops > 0 && (terminator_opcode == "40" || terminator_opcode == "60") {
        "return_or_interrupt"
    } else if control_flow_ops > 0
        && (terminator_opcode == "4C" || terminator_opcode == "6C" || opcode == "20")
    {
        "call_or_jump"
    } else if control_flow_ops > 0 {
        "branch"
    } else {
        "straight_line"
    }
    .to_string()
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

impl ReplaySummary {
    fn add(&mut self, class: &str, hit_count: u64, final_hash: &str) {
        self.block_count += 1;
        self.hit_count_total += hit_count;
        match class {
            "straight_line" => self.straight_line += 1,
            "call_or_jump" => self.call_or_jump += 1,
            "return_or_interrupt" => self.return_or_interrupt += 1,
            "branch" => self.branch += 1,
            "loop_or_step_limit" => self.loop_or_step_limit += 1,
            _ => self.unsupported_or_invalid += 1,
        }
        if is_lower_hex_sha256(final_hash) {
            self.final_ram_hash_rows += 1;
        }
    }
}

impl OpcodeSummary {
    fn add(&mut self, row: &PlanRow) {
        self.blocks += 1;
        self.hit_count += row.hit_count;
        match row.class.as_str() {
            "straight_line" => self.straight_line += 1,
            "call_or_jump" => self.call_or_jump += 1,
            "return_or_interrupt" => self.return_or_interrupt += 1,
            "branch" => self.branch += 1,
            "loop_or_step_limit" => self.loop_or_step_limit += 1,
            _ => self.unsupported_or_invalid += 1,
        }
    }
}

fn write_plan(path: &Path, rows: &[PlanRow]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "replay\tid\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstop_reason\tstatus\thit_count\tsteps\twrites\tppu_writes\tapu_writes\tmapper_writes\tstate_applied\tfinal_ram_sha256\tclass\tpriority"
    )?;
    for row in rows {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.replay,
            row.id,
            row.cpu_addr,
            row.prg_offset,
            row.bytes,
            row.first_opcode,
            row.stop_reason,
            row.status,
            row.hit_count,
            row.steps,
            row.writes,
            row.ppu_writes,
            row.apu_writes,
            row.mapper_writes,
            row.state_applied,
            row.final_ram_sha256,
            row.class,
            row.priority
        )?;
    }
    Ok(())
}

fn write_summary(path: &Path, summaries: &[ReplaySummary]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "replay\tblock_count\tstraight_line\tcall_or_jump\treturn_or_interrupt\tbranch\tloop_or_step_limit\tunsupported_or_invalid\tfinal_ram_hash_rows\thit_count_total"
    )?;
    for summary in summaries {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            summary.replay,
            summary.block_count,
            summary.straight_line,
            summary.call_or_jump,
            summary.return_or_interrupt,
            summary.branch,
            summary.loop_or_step_limit,
            summary.unsupported_or_invalid,
            summary.final_ram_hash_rows,
            summary.hit_count_total
        )?;
    }
    Ok(())
}

fn write_opcode_summary(path: &Path, rows: &[PlanRow]) -> io::Result<()> {
    let mut summaries = BTreeMap::<String, OpcodeSummary>::new();
    for row in rows {
        summaries
            .entry(row.first_opcode.clone())
            .or_default()
            .add(row);
    }

    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "opcode\tblocks\thit_count\tstraight_line\tcall_or_jump\treturn_or_interrupt\tbranch\tloop_or_step_limit\tunsupported_or_invalid"
    )?;
    for (opcode, summary) in summaries {
        writeln!(
            file,
            "{opcode}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            summary.blocks,
            summary.hit_count,
            summary.straight_line,
            summary.call_or_jump,
            summary.return_or_interrupt,
            summary.branch,
            summary.loop_or_step_limit,
            summary.unsupported_or_invalid
        )?;
    }
    Ok(())
}

fn write_manifest(path: &Path, build_dir: &Path, replays: &[String]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "build_dir={}", build_dir.display())?;
    writeln!(file, "replays={}", replays.join(" "))?;
    writeln!(file, "plan=block_translation_plan.tsv")?;
    writeln!(file, "summary=summary.tsv")?;
    writeln!(file, "opcode_summary=opcode_summary.tsv")?;
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
            "lotw_tools_block_translation_plan_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn writes_block_translation_plan() {
        let root = temp_dir();
        let build = root.join("build");
        let out = root.join("plan");
        write(
            &build.join("blocks/smoke/block_candidates.tsv"),
            "id\tcpu_addr\tprg_offset\tbytes\thit_count\tfirst_frame\tfirst_opcode\tstop_reason\trole\tcontrol_flow_ops\tterminator_opcode\n0\t8000\t1C000\t2\t10\t1\tA9\tnext_trace_label\tsmoke\t0\t00\n1\t8002\t1C002\t3\t5\t2\t20\tnext_trace_label\tsmoke\t1\t00\n2\t8005\t1C005\t2\t4\t60\t60\tterminator_60\tsmoke\t1\t60\n3\t8007\t1C007\t2\t3\t61\tD0\tnext_trace_label\tsmoke\t1\t00\n4\t8009\t1C009\t2\t2\t62\tA5\tnext_trace_label\tsmoke\t2\t60\n5\t800B\t1C00B\t2\t1\t63\tA5\tnext_trace_label\tsmoke\t0\t00\n",
        );
        write(
            &build.join("block_exec/smoke/block_exec.tsv"),
            "id\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstatus\tsteps\tunsupported_opcode\tfinal_pc\tcycles\twrites\tppu_writes\tapu_writes\tmapper_writes\tunmapped_reads\tstate_applied\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256\n0\t8000\t1C000\t2\tA9\tleft_block\t1\t\t8002\t2\t0\t0\t0\t0\t0\t1\t01\t02\t03\t24\tFD\t0000000000000000000000000000000000000000000000000000000000000000\n1\t8002\t1C002\t3\t20\tleft_block\t1\t\t9000\t6\t2\t0\t0\t0\t0\t1\t01\t02\t03\t24\tFA\t1111111111111111111111111111111111111111111111111111111111111111\n2\t8005\t1C005\t2\t60\tleft_block\t1\t\t8123\t6\t0\t0\t0\t0\t0\t1\t01\t02\t03\t24\tFC\t2222222222222222222222222222222222222222222222222222222222222222\n3\t8007\t1C007\t2\tD0\tleft_block\t1\t\t8010\t2\t0\t0\t0\t0\t0\t1\t01\t02\t03\t24\tFD\t3333333333333333333333333333333333333333333333333333333333333333\n4\t8009\t1C009\t2\tA5\tleft_block\t2\t\t8011\t6\t0\t0\t0\t0\t0\t1\t01\t02\t03\t24\tFD\t4444444444444444444444444444444444444444444444444444444444444444\n5\t800B\t1C00B\t2\tA5\tstep_limit\t64\t\t800B\t192\t0\t0\t0\t0\t0\t1\t01\t02\t03\t24\tFD\t5555555555555555555555555555555555555555555555555555555555555555\n",
        );

        run(&build, &out, &[String::from("smoke")]).unwrap();

        let summary = fs::read_to_string(out.join("summary.tsv")).unwrap();
        assert!(summary.contains("smoke\t6\t1\t1\t2\t1\t1\t0\t6\t25\n"));
        let opcodes = fs::read_to_string(out.join("opcode_summary.tsv")).unwrap();
        assert!(opcodes.contains("A9\t1\t10\t1\t0\t0\t0\t0\t0\n"));
        let plan = fs::read_to_string(out.join("block_translation_plan.tsv")).unwrap();
        assert!(plan.contains("\tcall_or_jump\t5\n"));
        assert!(plan.contains("\treturn_or_interrupt\t4\n"));
        assert!(plan.contains("\treturn_or_interrupt\t2\n"));
        assert!(plan.contains("\tbranch\t3\n"));
        assert!(plan.contains("\tloop_or_step_limit\t1\n"));

        fs::remove_dir_all(root).unwrap();
    }
}
