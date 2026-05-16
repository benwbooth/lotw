use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const REPORT_HEADER: &str = "replay\tnative_index\tcpu_addr\tprg_offset\tblock_first_frame\tfinal_pc\tref_prg_offset\tref_frame\thas_reference\tordered_reference_hit\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tref_a\tref_x\tref_y\tref_p\tref_s\tregister_match\tram_match\tstate_match\tstatus\tref_source\tref_hit_ordinal";

#[derive(Debug, Clone)]
struct FinalState {
    replay: String,
    native_index: String,
    cpu_addr: String,
    prg_offset: String,
    first_frame: String,
    final_pc: String,
    a: String,
    x: String,
    y: String,
    p: String,
    s: String,
    ram: String,
}

#[derive(Debug, Clone)]
struct RefState {
    prg_offset: String,
    frame: String,
    a: String,
    x: String,
    y: String,
    p: String,
    s: String,
    ram: String,
}

#[derive(Debug, Clone)]
struct HitState {
    prg_offset: String,
    ordinal: String,
    frame: String,
    a: String,
    x: String,
    y: String,
    p: String,
    s: String,
    ram: String,
}

#[derive(Debug, Default)]
struct Summary {
    case_count: u64,
    has_reference: u64,
    ordered_reference_hit: u64,
    register_match: u64,
    ram_match: u64,
    state_match: u64,
    status_count: HashMap<String, u64>,
}

#[derive(Debug)]
struct TransitionRow {
    replay: String,
    native_index: String,
    cpu_addr: String,
    prg_offset: String,
    block_first_frame: String,
    final_pc: String,
    ref_prg_offset: String,
    ref_frame: String,
    has_reference: bool,
    ordered_reference_hit: bool,
    final_a: String,
    final_x: String,
    final_y: String,
    final_p: String,
    final_s: String,
    ref_a: String,
    ref_x: String,
    ref_y: String,
    ref_p: String,
    ref_s: String,
    register_match: bool,
    ram_match: bool,
    state_match: bool,
    status: String,
    ref_source: String,
    ref_hit_ordinal: String,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    replays: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if replays.is_empty() {
        return Err("native_block_transition: at least one replay is required".into());
    }

    let final_states = build_dir.join("native_block_verify/native_block_final_states.tsv");
    require_file(&final_states)?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let report = out_dir.join("native_block_transition.tsv");
    let summary_path = out_dir.join("native_block_transition_summary.tsv");
    let manifest = out_dir.join("manifest.txt");
    let final_rows = read_final_states(&final_states)?;
    let mut summary = Summary::default();

    let mut report_file = fs::File::create(&report)?;
    writeln!(report_file, "{REPORT_HEADER}")?;
    for replay in replays {
        let states_path = build_dir.join(format!("trace/{replay}/label_states.tsv"));
        require_file(&states_path)?;
        let hits_path = build_dir.join(format!("trace/{replay}/label_state_hits.tsv"));
        let first_hits = read_label_states(&states_path)?;
        let later_hits = if hits_path.is_file() {
            read_label_state_hits(&hits_path)?
        } else {
            HashMap::new()
        };

        for final_state in final_rows.iter().filter(|row| row.replay == *replay) {
            let row = transition_row(final_state, &first_hits, &later_hits)?;
            summary.add(&row);
            writeln!(report_file, "{}", row.to_tsv())?;
        }
    }

    if summary.case_count == 0 {
        return Err("native_block_transition: no cases generated".into());
    }

    write_summary(&summary_path, &summary)?;
    write_manifest(&manifest, &final_states, &summary)?;

    println!("native_block_transition: wrote {}", out_dir.display());
    Ok(())
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("native_block_transition: missing input: {}", path.display()),
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

fn read_final_states(path: &Path) -> io::Result<Vec<FinalState>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 13 {
            return invalid_tsv(path, line_no + 1, fields.len(), 13);
        }
        rows.push(FinalState {
            replay: fields[0].to_string(),
            native_index: fields[1].to_string(),
            cpu_addr: fields[2].to_string(),
            prg_offset: fields[3].to_string(),
            first_frame: fields[4].to_string(),
            final_pc: fields[5].to_string(),
            a: fields[6].to_string(),
            x: fields[7].to_string(),
            y: fields[8].to_string(),
            p: fields[9].to_string(),
            s: fields[10].to_string(),
            ram: fields[12].to_string(),
        });
    }
    Ok(rows)
}

fn read_label_states(path: &Path) -> io::Result<HashMap<String, RefState>> {
    let text = fs::read_to_string(path)?;
    let mut states = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 10 {
            return invalid_tsv(path, line_no + 1, fields.len(), 10);
        }
        states.insert(
            fields[0].to_string(),
            RefState {
                prg_offset: fields[1].to_string(),
                frame: fields[2].to_string(),
                a: fields[4].to_string(),
                x: fields[5].to_string(),
                y: fields[6].to_string(),
                p: fields[7].to_string(),
                s: fields[8].to_string(),
                ram: fields[9].to_string(),
            },
        );
    }
    Ok(states)
}

fn read_label_state_hits(path: &Path) -> io::Result<HashMap<String, Vec<HitState>>> {
    let text = fs::read_to_string(path)?;
    let mut hits: HashMap<String, Vec<HitState>> = HashMap::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 11 {
            return invalid_tsv(path, line_no + 1, fields.len(), 11);
        }
        hits.entry(fields[0].to_string())
            .or_default()
            .push(HitState {
                prg_offset: fields[1].to_string(),
                ordinal: fields[2].to_string(),
                frame: fields[3].to_string(),
                a: fields[5].to_string(),
                x: fields[6].to_string(),
                y: fields[7].to_string(),
                p: fields[8].to_string(),
                s: fields[9].to_string(),
                ram: fields[10].to_string(),
            });
    }
    Ok(hits)
}

fn transition_row(
    final_state: &FinalState,
    first_hits: &HashMap<String, RefState>,
    later_hits: &HashMap<String, Vec<HitState>>,
) -> Result<TransitionRow, Box<dyn std::error::Error>> {
    let mut row = TransitionRow::missing(final_state);
    let first_hit = first_hits.get(&final_state.final_pc);
    let hits = later_hits
        .get(&final_state.final_pc)
        .map(Vec::as_slice)
        .unwrap_or(&[]);

    if first_hit.is_none() && hits.is_empty() {
        return Ok(row);
    }

    row.has_reference = true;
    if let Some(reference) = first_hit {
        row.apply_first_hit(final_state, reference)?;
    } else if let Some(hit) = hits.first() {
        row.apply_later_hit(final_state, hit, "label_state_hits", false)?;
    }

    if row.ordered_reference_hit && row.state_match {
        row.status = "matched_ordered".to_string();
        return Ok(row);
    }

    for hit in hits {
        if hit_ordered_match(final_state, hit)? {
            row.apply_later_hit(final_state, hit, "label_state_hits", true)?;
            row.status = "matched_ordered_later_hit".to_string();
            return Ok(row);
        }
    }

    if !row.ordered_reference_hit && row.state_match {
        row.status = "matched_stale_first_hit".to_string();
    } else if !row.ordered_reference_hit {
        row.status = "stale_first_hit".to_string();
    } else {
        row.status = "mismatch".to_string();
    }
    Ok(row)
}

fn hit_ordered_match(
    final_state: &FinalState,
    hit: &HitState,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(frame_ordered(&hit.frame, &final_state.first_frame)?
        && final_state.a == hit.a
        && final_state.x == hit.x
        && final_state.y == hit.y
        && final_state.p == hit.p
        && final_state.s == hit.s
        && final_state.ram == hit.ram)
}

fn frame_ordered(frame: &str, first_frame: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let frame = frame.parse::<i64>()?;
    let first_frame = first_frame.parse::<i64>()?;
    Ok(frame >= first_frame)
}

impl TransitionRow {
    fn missing(final_state: &FinalState) -> Self {
        Self {
            replay: final_state.replay.clone(),
            native_index: final_state.native_index.clone(),
            cpu_addr: final_state.cpu_addr.clone(),
            prg_offset: final_state.prg_offset.clone(),
            block_first_frame: final_state.first_frame.clone(),
            final_pc: final_state.final_pc.clone(),
            ref_prg_offset: "-".to_string(),
            ref_frame: "-".to_string(),
            has_reference: false,
            ordered_reference_hit: false,
            final_a: final_state.a.clone(),
            final_x: final_state.x.clone(),
            final_y: final_state.y.clone(),
            final_p: final_state.p.clone(),
            final_s: final_state.s.clone(),
            ref_a: "-".to_string(),
            ref_x: "-".to_string(),
            ref_y: "-".to_string(),
            ref_p: "-".to_string(),
            ref_s: "-".to_string(),
            register_match: false,
            ram_match: false,
            state_match: false,
            status: "missing_reference".to_string(),
            ref_source: "-".to_string(),
            ref_hit_ordinal: "-".to_string(),
        }
    }

    fn apply_first_hit(
        &mut self,
        final_state: &FinalState,
        reference: &RefState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.ref_prg_offset = reference.prg_offset.clone();
        self.ref_frame = reference.frame.clone();
        self.ref_a = reference.a.clone();
        self.ref_x = reference.x.clone();
        self.ref_y = reference.y.clone();
        self.ref_p = reference.p.clone();
        self.ref_s = reference.s.clone();
        self.ref_source = "first_hit".to_string();
        self.ref_hit_ordinal = "-".to_string();
        self.ordered_reference_hit = frame_ordered(&reference.frame, &final_state.first_frame)?;
        self.register_match = final_state.a == reference.a
            && final_state.x == reference.x
            && final_state.y == reference.y
            && final_state.p == reference.p
            && final_state.s == reference.s;
        self.ram_match = final_state.ram == reference.ram;
        self.state_match = self.register_match && self.ram_match;
        Ok(())
    }

    fn apply_later_hit(
        &mut self,
        final_state: &FinalState,
        hit: &HitState,
        source: &str,
        force_match: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.ref_prg_offset = hit.prg_offset.clone();
        self.ref_frame = hit.frame.clone();
        self.ref_a = hit.a.clone();
        self.ref_x = hit.x.clone();
        self.ref_y = hit.y.clone();
        self.ref_p = hit.p.clone();
        self.ref_s = hit.s.clone();
        self.ref_source = source.to_string();
        self.ref_hit_ordinal = hit.ordinal.clone();
        self.ordered_reference_hit = frame_ordered(&hit.frame, &final_state.first_frame)?;
        self.register_match = final_state.a == hit.a
            && final_state.x == hit.x
            && final_state.y == hit.y
            && final_state.p == hit.p
            && final_state.s == hit.s;
        self.ram_match = final_state.ram == hit.ram;
        self.state_match = self.register_match && self.ram_match;
        if force_match {
            self.ordered_reference_hit = true;
            self.register_match = true;
            self.ram_match = true;
            self.state_match = true;
        }
        Ok(())
    }

    fn to_tsv(&self) -> String {
        [
            self.replay.clone(),
            self.native_index.clone(),
            self.cpu_addr.clone(),
            self.prg_offset.clone(),
            self.block_first_frame.clone(),
            self.final_pc.clone(),
            self.ref_prg_offset.clone(),
            self.ref_frame.clone(),
            bit(self.has_reference).to_string(),
            bit(self.ordered_reference_hit).to_string(),
            self.final_a.clone(),
            self.final_x.clone(),
            self.final_y.clone(),
            self.final_p.clone(),
            self.final_s.clone(),
            self.ref_a.clone(),
            self.ref_x.clone(),
            self.ref_y.clone(),
            self.ref_p.clone(),
            self.ref_s.clone(),
            bit(self.register_match).to_string(),
            bit(self.ram_match).to_string(),
            bit(self.state_match).to_string(),
            self.status.clone(),
            self.ref_source.clone(),
            self.ref_hit_ordinal.clone(),
        ]
        .join("\t")
    }
}

impl Summary {
    fn add(&mut self, row: &TransitionRow) {
        self.case_count += 1;
        self.has_reference += bit(row.has_reference);
        self.ordered_reference_hit += bit(row.ordered_reference_hit);
        self.register_match += bit(row.register_match);
        self.ram_match += bit(row.ram_match);
        self.state_match += bit(row.state_match);
        *self.status_count.entry(row.status.clone()).or_insert(0) += 1;
    }

    fn status(&self, status: &str) -> u64 {
        self.status_count.get(status).copied().unwrap_or(0)
    }
}

fn bit(value: bool) -> u64 {
    u64::from(value)
}

fn write_summary(path: &Path, summary: &Summary) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "case_count={}", summary.case_count)?;
    writeln!(file, "has_reference={}", summary.has_reference)?;
    writeln!(
        file,
        "ordered_reference_hit={}",
        summary.ordered_reference_hit
    )?;
    writeln!(file, "register_match={}", summary.register_match)?;
    writeln!(file, "ram_match={}", summary.ram_match)?;
    writeln!(file, "state_match={}", summary.state_match)?;
    writeln!(
        file,
        "matched_ordered={}",
        summary.status("matched_ordered")
    )?;
    writeln!(
        file,
        "matched_ordered_later_hit={}",
        summary.status("matched_ordered_later_hit")
    )?;
    writeln!(
        file,
        "matched_stale_first_hit={}",
        summary.status("matched_stale_first_hit")
    )?;
    writeln!(
        file,
        "stale_first_hit={}",
        summary.status("stale_first_hit")
    )?;
    writeln!(file, "mismatch={}", summary.status("mismatch"))?;
    writeln!(
        file,
        "missing_reference={}",
        summary.status("missing_reference")
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_manifest(manifest: &Path, final_states: &Path, summary: &Summary) -> io::Result<()> {
    let mut file = fs::File::create(manifest)?;
    writeln!(file, "final_states={}", final_states.display())?;
    writeln!(file, "transition_report=native_block_transition.tsv")?;
    writeln!(
        file,
        "transition_summary=native_block_transition_summary.tsv"
    )?;
    writeln!(file, "case_count={}", summary.case_count)?;
    writeln!(file, "has_reference={}", summary.has_reference)?;
    writeln!(
        file,
        "ordered_reference_hit={}",
        summary.ordered_reference_hit
    )?;
    writeln!(file, "register_match={}", summary.register_match)?;
    writeln!(file, "ram_match={}", summary.ram_match)?;
    writeln!(file, "state_match={}", summary.state_match)?;
    writeln!(
        file,
        "matched_ordered={}",
        summary.status("matched_ordered")
    )?;
    writeln!(
        file,
        "matched_ordered_later_hit={}",
        summary.status("matched_ordered_later_hit")
    )?;
    writeln!(
        file,
        "matched_stale_first_hit={}",
        summary.status("matched_stale_first_hit")
    )?;
    writeln!(
        file,
        "stale_first_hit={}",
        summary.status("stale_first_hit")
    )?;
    writeln!(file, "mismatch={}", summary.status("mismatch"))?;
    writeln!(
        file,
        "missing_reference={}",
        summary.status("missing_reference")
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_transition_report_and_summary() {
        let root = std::env::temp_dir().join(format!(
            "lotw_native_block_transition_test_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        let build_dir = root.join("build");
        let out_dir = root.join("transition");
        fs::create_dir_all(build_dir.join("native_block_verify")).unwrap();
        fs::create_dir_all(build_dir.join("trace/title")).unwrap();

        write_smoke_inputs(&build_dir);
        run(&build_dir, &out_dir, &[String::from("title")]).unwrap();

        let summary =
            fs::read_to_string(out_dir.join("native_block_transition_summary.tsv")).unwrap();
        assert!(summary.contains("case_count=5\n"));
        assert!(summary.contains("has_reference=4\n"));
        assert!(summary.contains("ordered_reference_hit=3\n"));
        assert!(summary.contains("state_match=3\n"));
        assert!(summary.contains("matched_ordered=1\n"));
        assert!(summary.contains("matched_ordered_later_hit=1\n"));
        assert!(summary.contains("matched_stale_first_hit=1\n"));
        assert!(summary.contains("mismatch=1\n"));
        assert!(summary.contains("missing_reference=1\n"));

        let report = fs::read_to_string(out_dir.join("native_block_transition.tsv")).unwrap();
        assert!(report.contains(
            "title\t0\tC000\t1C000\t10\tC002\t1C002\t12\t1\t1\t01\t02\t03\t24\tFA\t01\t02\t03\t24\tFA\t1\t1\t1\tmatched_ordered\tfirst_hit\t-\n"
        ));
        assert!(report.contains(
            "title\t2\tC020\t1C020\t30\tC022\t1C022\t5\t1\t0\t07\t08\t09\t26\tFC\t07\t08\t09\t26\tFC\t1\t1\t1\tmatched_stale_first_hit\tfirst_hit\t-\n"
        ));
        assert!(report.contains(
            "title\t3\tC030\t1C030\t40\tC099\t-\t-\t0\t0\t07\t08\t09\t26\tFC\t-\t-\t-\t-\t-\t0\t0\t0\tmissing_reference\t-\t-\n"
        ));
        assert!(report.contains(
            "title\t4\tC040\t1C040\t40\tC042\t1C042\t45\t1\t1\t0A\t0B\t0C\t27\tFD\t0A\t0B\t0C\t27\tFD\t1\t1\t1\tmatched_ordered_later_hit\tlabel_state_hits\t2\n"
        ));

        let _ = fs::remove_dir_all(root);
    }

    fn write_smoke_inputs(build_dir: &Path) {
        fs::write(
            build_dir.join("native_block_verify/native_block_final_states.tsv"),
            "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tcycles\tram_0000_07ff\tfinal_ram_sha256\n\
             title\t0\tC000\t1C000\t10\tC002\t01\t02\t03\t24\tFA\t4\tAABB\taaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\
             title\t1\tC010\t1C010\t20\tC012\t04\t05\t06\t25\tFB\t7\tCCDD\tbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n\
             title\t2\tC020\t1C020\t30\tC022\t07\t08\t09\t26\tFC\t8\tEEFF\tcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\n\
             title\t3\tC030\t1C030\t40\tC099\t07\t08\t09\t26\tFC\t8\tEEFF\tdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\n\
             title\t4\tC040\t1C040\t40\tC042\t0A\t0B\t0C\t27\tFD\t9\t1122\teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\n",
        )
        .unwrap();
        fs::write(
            build_dir.join("trace/title/label_states.tsv"),
            "cpu_addr\tprg_offset\tfirst_frame\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\n\
             C002\t1C002\t12\tC002\t01\t02\t03\t24\tFA\tAABB\n\
             C012\t1C012\t22\tC012\t04\t05\t00\t25\tFB\tCCDD\n\
             C022\t1C022\t5\tC022\t07\t08\t09\t26\tFC\tEEFF\n\
             C042\t1C042\t5\tC042\t00\t00\t00\t20\tFD\t0000\n",
        )
        .unwrap();
        fs::write(
            build_dir.join("trace/title/label_state_hits.tsv"),
            "cpu_addr\tprg_offset\thit_ordinal\tframe\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\n\
             C042\t1C042\t1\t5\tC042\t00\t00\t00\t20\tFD\t0000\n\
             C042\t1C042\t2\t45\tC042\t0A\t0B\t0C\t27\tFD\t1122\n",
        )
        .unwrap();
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
