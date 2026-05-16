use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
enum ProofKind {
    Leaf,
    Handoff,
    Branch,
    Jsr,
    Return,
}

#[derive(Debug, Default)]
struct BlockMergeStats {
    old_block_count: u64,
    new_block_count: u64,
    old_kept: u64,
    new_kept: u64,
    old_duplicates: u64,
    new_duplicates: u64,
    selected_count: u64,
}

#[derive(Debug, Clone)]
struct MapEntry {
    source: Source,
    output_index: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    Old,
    New,
}

#[derive(Debug, Default)]
struct OracleStats {
    case_count: u64,
    left_block: u64,
    stopped: u64,
    unsupported_opcode: u64,
    step_limit: u64,
    invalid_block: u64,
    external_write_rows: u64,
    ram_writes_total: u64,
    ram_write_rows: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
    unmapped_reads: u64,
}

#[derive(Debug, Default)]
struct VerifyStats {
    cases: u64,
    matched: u64,
    mismatches: u64,
    external_matched: u64,
    external_mismatches: u64,
}

#[derive(Debug, Default)]
struct SkipReasonStats {
    call_like_leaf: u64,
    missing_byte_count: u64,
    unsupported: u64,
    loop_like_leaf: u64,
}

struct ProofNames {
    prefix: String,
    summary: String,
    blocks: String,
    cases: String,
    skipped: String,
}

struct IndexedMerge {
    old_file: PathBuf,
    new_file: PathBuf,
    out_file: PathBuf,
    columns: IndexedColumns,
}

#[derive(Debug, Clone, Copy)]
struct IndexedColumns {
    native_index: usize,
    cpu: usize,
    prg: usize,
}

pub fn run(
    kind: &str,
    old_dir: &Path,
    new_dir: &Path,
    out_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let kind = ProofKind::parse(kind)?;
    let names = kind.names();

    require_complete_dir(old_dir, &names)?;
    require_complete_dir(new_dir, &names)?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir.join("oracle"))?;
    fs::create_dir_all(out_dir.join("native_verify"))?;

    let map_file = out_dir.join("static_proof_index_map.tsv");
    let counts_file = out_dir.join("static_proof_accumulate_counts.txt");
    let mut index_map = HashMap::new();
    let stats = merge_blocks(old_dir, new_dir, out_dir, &names, &map_file, &mut index_map)?;
    write_counts(&counts_file, &stats)?;

    for spec in [
        indexed(
            old_dir,
            new_dir,
            out_dir,
            &names.cases,
            &names.cases,
            cols(2, 3, 4),
        ),
        indexed(
            old_dir,
            new_dir,
            out_dir,
            "oracle/block_state_exec.tsv",
            "oracle/block_state_exec.tsv",
            cols(2, 3, 4),
        ),
        indexed(
            old_dir,
            new_dir,
            out_dir,
            "oracle/block_state_external_writes.tsv",
            "oracle/block_state_external_writes.tsv",
            cols(2, 3, 4),
        ),
        indexed(
            old_dir,
            new_dir,
            out_dir,
            "native_verify/native_block_verify.tsv",
            "native_verify/native_block_verify.tsv",
            cols(2, 3, 4),
        ),
        indexed(
            old_dir,
            new_dir,
            out_dir,
            "native_verify/native_block_final_states.tsv",
            "native_verify/native_block_final_states.tsv",
            cols(2, 3, 4),
        ),
    ] {
        merge_indexed_tsv(&spec, &index_map)?;
    }

    let verified_keys = read_verified_block_keys(&out_dir.join(&names.blocks))?;
    merge_skipped_tsv(
        &old_dir.join(&names.skipped),
        &new_dir.join(&names.skipped),
        &out_dir.join(&names.skipped),
        &verified_keys,
    )?;
    merge_plain_tsv(
        &old_dir.join("oracle/unsupported_opcodes.tsv"),
        &new_dir.join("oracle/unsupported_opcodes.tsv"),
        &out_dir.join("oracle/unsupported_opcodes.tsv"),
    )?;

    if let (Some(old_cases), Some(new_cases)) = (
        first_native_verify_cases(old_dir)?,
        first_native_verify_cases(new_dir)?,
    ) {
        merge_indexed_tsv(
            &IndexedMerge {
                old_file: old_cases,
                new_file: new_cases,
                out_file: out_dir
                    .join("native_verify")
                    .join(format!("{}_native_verify_cases.tsv", names.prefix)),
                columns: cols(2, 3, 4),
            },
            &index_map,
        )?;
    }

    if matches!(kind, ProofKind::Branch | ProofKind::Jsr) {
        merge_indexed_tsv(
            &indexed(
                old_dir,
                new_dir,
                out_dir,
                &format!("{}_targets.tsv", names.prefix),
                &format!("{}_targets.tsv", names.prefix),
                cols(1, 4, 5),
            ),
            &index_map,
        )?;
        merge_indexed_tsv(
            &indexed(
                old_dir,
                new_dir,
                out_dir,
                &format!("{}_outcomes.tsv", names.prefix),
                &format!("{}_outcomes.tsv", names.prefix),
                cols(1, 4, 5),
            ),
            &index_map,
        )?;
    }

    let oracle_stats = summarize_oracle(&out_dir.join("oracle/block_state_exec.tsv"))?;
    let verify_stats = summarize_verify(&out_dir.join("native_verify/native_block_verify.tsv"))?;
    if oracle_stats.case_count == 0 || verify_stats.cases != oracle_stats.case_count {
        return Err("static_proof_accumulate: merged case count mismatch".into());
    }
    if oracle_stats.unsupported_opcode != 0
        || oracle_stats.step_limit != 0
        || verify_stats.mismatches != 0
        || verify_stats.external_mismatches != 0
    {
        return Err("static_proof_accumulate: merged proof failed semantic gates".into());
    }

    write_oracle_manifest(out_dir, &names, &oracle_stats)?;
    write_native_verify_manifest(out_dir, &names, &verify_stats)?;
    write_summary(out_dir, old_dir, new_dir, &names, &stats, &oracle_stats)?;
    write_manifest(
        out_dir,
        old_dir,
        new_dir,
        &names,
        &stats,
        oracle_stats.case_count,
    )?;

    println!("static_proof_accumulate: wrote {}", out_dir.display());
    Ok(())
}

impl ProofKind {
    fn parse(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match value {
            "leaf" => Ok(Self::Leaf),
            "handoff" => Ok(Self::Handoff),
            "branch" => Ok(Self::Branch),
            "jsr" => Ok(Self::Jsr),
            "return" => Ok(Self::Return),
            _ => Err(format!("static_proof_accumulate: unsupported kind: {value}").into()),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Leaf => "leaf",
            Self::Handoff => "handoff",
            Self::Branch => "branch",
            Self::Jsr => "jsr",
            Self::Return => "return",
        }
    }

    fn names(self) -> ProofNames {
        let prefix = format!("static_{}", self.as_str());
        ProofNames {
            summary: format!("{prefix}_verify_summary.txt"),
            blocks: format!("{prefix}_native_blocks.tsv"),
            cases: format!("{prefix}_state_cases.tsv"),
            skipped: format!("{prefix}_skipped.tsv"),
            prefix,
        }
    }
}

fn indexed(
    old_dir: &Path,
    new_dir: &Path,
    out_dir: &Path,
    input_name: &str,
    output_name: &str,
    columns: IndexedColumns,
) -> IndexedMerge {
    IndexedMerge {
        old_file: old_dir.join(input_name),
        new_file: new_dir.join(input_name),
        out_file: out_dir.join(output_name),
        columns,
    }
}

fn cols(native_index: usize, cpu: usize, prg: usize) -> IndexedColumns {
    IndexedColumns {
        native_index,
        cpu,
        prg,
    }
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

fn ensure_eq(
    values: &HashMap<String, String>,
    key: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual = values
        .get(key)
        .ok_or_else(|| format!("missing {key} in {}", path.display()))?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{} has {key}={actual}, expected {expected}", path.display()).into())
    }
}

fn require_complete_dir(dir: &Path, names: &ProofNames) -> Result<(), Box<dyn std::error::Error>> {
    let summary = dir.join(&names.summary);
    let manifest = dir.join("manifest.txt");
    let oracle_manifest = dir.join("oracle/manifest.txt");
    let native_manifest = dir.join("native_verify/manifest.txt");

    for path in [
        dir.join(&names.blocks),
        summary.clone(),
        manifest.clone(),
        dir.join("oracle/block_state_exec.tsv"),
        oracle_manifest.clone(),
        native_manifest.clone(),
    ] {
        require_file(&path)?;
    }

    let summary_kv = read_key_values(&summary)?;
    let manifest_kv = read_key_values(&manifest)?;
    let oracle_kv = read_key_values(&oracle_manifest)?;
    let native_kv = read_key_values(&native_manifest)?;
    ensure_eq(&summary_kv, "complete", "1", &summary)?;
    ensure_eq(&manifest_kv, "complete", "1", &manifest)?;
    ensure_eq(&oracle_kv, "complete", "1", &oracle_manifest)?;
    ensure_eq(&oracle_kv, "unsupported_opcode", "0", &oracle_manifest)?;
    ensure_eq(&oracle_kv, "step_limit", "0", &oracle_manifest)?;
    ensure_eq(&native_kv, "complete", "1", &native_manifest)?;
    ensure_eq(&native_kv, "mismatches", "0", &native_manifest)?;
    ensure_eq(
        &native_kv,
        "external_write_mismatches",
        "0",
        &native_manifest,
    )?;
    Ok(())
}

fn split_tsv(line: &str) -> Vec<String> {
    line.split('\t').map(str::to_string).collect()
}

fn key(cpu_addr: &str, prg_offset: &str) -> String {
    format!(
        "{}\t{}",
        cpu_addr.to_ascii_uppercase(),
        prg_offset.to_ascii_uppercase()
    )
}

fn source_name(source: Source) -> &'static str {
    match source {
        Source::Old => "old",
        Source::New => "new",
    }
}

fn merge_blocks(
    old_dir: &Path,
    new_dir: &Path,
    out_dir: &Path,
    names: &ProofNames,
    map_file: &Path,
    index_map: &mut HashMap<String, MapEntry>,
) -> io::Result<BlockMergeStats> {
    let mut out = fs::File::create(out_dir.join(&names.blocks))?;
    writeln!(
        out,
        "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\tfinal_ram_hash_count\treason"
    )?;
    let mut map = fs::File::create(map_file)?;
    writeln!(
        map,
        "source\tinput_native_index\tcpu_addr\tprg_offset\toutput_native_index\tstatus"
    )?;

    let mut seen = HashSet::new();
    let mut stats = BlockMergeStats::default();
    merge_block_source(
        Source::Old,
        &old_dir.join(&names.blocks),
        &mut out,
        &mut map,
        &mut seen,
        &mut stats,
        index_map,
    )?;
    merge_block_source(
        Source::New,
        &new_dir.join(&names.blocks),
        &mut out,
        &mut map,
        &mut seen,
        &mut stats,
        index_map,
    )?;
    Ok(stats)
}

fn merge_block_source(
    source: Source,
    path: &Path,
    out: &mut fs::File,
    map: &mut fs::File,
    seen: &mut HashSet<String>,
    stats: &mut BlockMergeStats,
    index_map: &mut HashMap<String, MapEntry>,
) -> io::Result<()> {
    let text = fs::read_to_string(path)?;
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let mut fields = split_tsv(line);
        if fields.len() < 15 {
            return invalid_tsv(path, line_no + 1, fields.len(), 15);
        }
        let block_key = key(&fields[1], &fields[2]);
        let input_index = fields[0].parse::<u64>().unwrap_or(0).saturating_sub(1);
        match source {
            Source::Old => stats.old_block_count += 1,
            Source::New => stats.new_block_count += 1,
        }
        if seen.contains(&block_key) {
            match source {
                Source::Old => stats.old_duplicates += 1,
                Source::New => stats.new_duplicates += 1,
            }
            writeln!(
                map,
                "{}\t{}\t{}\t{}\t\tduplicate_skipped",
                source_name(source),
                input_index,
                fields[1],
                fields[2]
            )?;
            continue;
        }

        seen.insert(block_key.clone());
        let output_index = stats.selected_count;
        stats.selected_count += 1;
        match source {
            Source::Old => stats.old_kept += 1,
            Source::New => stats.new_kept += 1,
        }
        fields[0] = stats.selected_count.to_string();
        writeln!(out, "{}", fields.join("\t"))?;
        writeln!(
            map,
            "{}\t{}\t{}\t{}\t{}\tkept",
            source_name(source),
            input_index,
            fields[1],
            fields[2],
            output_index
        )?;
        index_map.insert(
            block_key,
            MapEntry {
                source,
                output_index: output_index.to_string(),
            },
        );
    }
    Ok(())
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

fn write_counts(path: &Path, stats: &BlockMergeStats) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "old_block_count={}", stats.old_block_count)?;
    writeln!(file, "new_block_count={}", stats.new_block_count)?;
    writeln!(file, "old_kept={}", stats.old_kept)?;
    writeln!(file, "new_kept={}", stats.new_kept)?;
    writeln!(file, "old_duplicates={}", stats.old_duplicates)?;
    writeln!(file, "new_duplicates={}", stats.new_duplicates)?;
    writeln!(file, "selected_count={}", stats.selected_count)?;
    Ok(())
}

fn merge_indexed_tsv(spec: &IndexedMerge, index_map: &HashMap<String, MapEntry>) -> io::Result<()> {
    match (spec.old_file.is_file(), spec.new_file.is_file()) {
        (false, false) => return Ok(()),
        (true, true) => {}
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "static_proof_accumulate: cannot merge incomplete indexed TSV: {} / {}",
                    spec.old_file.display(),
                    spec.new_file.display()
                ),
            ));
        }
    }

    let mut out = fs::File::create(&spec.out_file)?;
    let mut printed_header = false;
    merge_indexed_source(
        Source::Old,
        &spec.old_file,
        &mut out,
        &mut printed_header,
        spec,
        index_map,
    )?;
    merge_indexed_source(
        Source::New,
        &spec.new_file,
        &mut out,
        &mut printed_header,
        spec,
        index_map,
    )?;
    Ok(())
}

fn merge_indexed_source(
    source: Source,
    path: &Path,
    out: &mut fs::File,
    printed_header: &mut bool,
    spec: &IndexedMerge,
    index_map: &HashMap<String, MapEntry>,
) -> io::Result<()> {
    let text = fs::read_to_string(path)?;
    let required_cols = spec
        .columns
        .native_index
        .max(spec.columns.cpu)
        .max(spec.columns.prg);
    for (line_no, line) in text.lines().enumerate() {
        if line_no == 0 {
            if !*printed_header {
                writeln!(out, "{line}")?;
                *printed_header = true;
            }
            continue;
        }
        let mut fields = split_tsv(line);
        if fields.len() < required_cols {
            return invalid_tsv(path, line_no + 1, fields.len(), required_cols);
        }
        let block_key = key(&fields[spec.columns.cpu - 1], &fields[spec.columns.prg - 1]);
        let Some(entry) = index_map.get(&block_key) else {
            continue;
        };
        if entry.source != source {
            continue;
        }
        fields[spec.columns.native_index - 1] = entry.output_index.clone();
        writeln!(out, "{}", fields.join("\t"))?;
    }
    Ok(())
}

fn merge_plain_tsv(old_file: &Path, new_file: &Path, out_file: &Path) -> io::Result<()> {
    match (old_file.is_file(), new_file.is_file()) {
        (false, false) => return Ok(()),
        (true, true) => {}
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "static_proof_accumulate: cannot merge incomplete TSV: {} / {}",
                    old_file.display(),
                    new_file.display()
                ),
            ));
        }
    }

    let mut out = fs::File::create(out_file)?;
    let mut printed_header = false;
    let mut seen = HashSet::new();
    for path in [old_file, new_file] {
        let text = fs::read_to_string(path)?;
        for (line_no, line) in text.lines().enumerate() {
            if line_no == 0 {
                if !printed_header {
                    writeln!(out, "{line}")?;
                    printed_header = true;
                }
                continue;
            }
            if seen.insert(line.to_string()) {
                writeln!(out, "{line}")?;
            }
        }
    }
    Ok(())
}

fn read_verified_block_keys(path: &Path) -> io::Result<HashSet<String>> {
    let text = fs::read_to_string(path)?;
    let mut keys = HashSet::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 3 {
            return invalid_tsv(path, line_no + 1, fields.len(), 3);
        }
        keys.insert(key(&fields[1], &fields[2]));
    }
    Ok(keys)
}

fn merge_skipped_tsv(
    old_file: &Path,
    new_file: &Path,
    out_file: &Path,
    verified_keys: &HashSet<String>,
) -> io::Result<()> {
    match (old_file.is_file(), new_file.is_file()) {
        (false, false) => return Ok(()),
        (true, true) => {}
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "static_proof_accumulate: cannot merge incomplete skipped TSV: {} / {}",
                    old_file.display(),
                    new_file.display()
                ),
            ));
        }
    }

    let mut out = fs::File::create(out_file)?;
    let mut printed_header = false;
    let mut row_index_by_key: HashMap<String, usize> = HashMap::new();
    let mut rows: Vec<(String, String)> = Vec::new();
    for path in [old_file, new_file] {
        let text = fs::read_to_string(path)?;
        for (line_no, line) in text.lines().enumerate() {
            if line_no == 0 {
                if !printed_header {
                    writeln!(out, "{line}")?;
                    printed_header = true;
                }
                continue;
            }
            let fields = split_tsv(line);
            if fields.len() >= 3 && verified_keys.contains(&key(&fields[1], &fields[2])) {
                continue;
            }
            let row_key = if fields.len() >= 3 {
                key(&fields[1], &fields[2])
            } else {
                format!("{}:{line_no}:{line}", path.display())
            };
            if let Some(index) = row_index_by_key.get(&row_key) {
                rows[*index].1 = line.to_string();
            } else {
                row_index_by_key.insert(row_key.clone(), rows.len());
                rows.push((row_key, line.to_string()));
            }
        }
    }
    for (_, line) in rows {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

fn first_native_verify_cases(dir: &Path) -> io::Result<Option<PathBuf>> {
    let native_verify = dir.join("native_verify");
    let mut matches = Vec::new();
    if !native_verify.is_dir() {
        return Ok(None);
    }
    for entry in fs::read_dir(native_verify)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.ends_with("_native_verify_cases.tsv") {
            matches.push(entry.path());
        }
    }
    matches.sort();
    Ok(matches.into_iter().next())
}

fn parse_u64_or_zero(value: Option<&String>) -> u64 {
    value.and_then(|item| item.parse::<u64>().ok()).unwrap_or(0)
}

fn summarize_oracle(path: &Path) -> io::Result<OracleStats> {
    let text = fs::read_to_string(path)?;
    let mut stats = OracleStats::default();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 24 {
            return invalid_tsv(path, line_no + 1, fields.len(), 24);
        }
        stats.case_count += 1;
        match fields[14].as_str() {
            "left_block" => stats.left_block += 1,
            "stopped" => stats.stopped += 1,
            "invalid_block" => stats.invalid_block += 1,
            "step_limit" => stats.step_limit += 1,
            _ => {}
        }
        if !fields[16].is_empty() {
            stats.unsupported_opcode += 1;
        }
        let writes = parse_u64_or_zero(fields.get(19));
        let ppu = parse_u64_or_zero(fields.get(20));
        let apu = parse_u64_or_zero(fields.get(21));
        let mapper = parse_u64_or_zero(fields.get(22));
        stats.ram_writes_total += writes;
        if writes > 0 {
            stats.ram_write_rows += 1;
        }
        if ppu + apu + mapper > 0 {
            stats.external_write_rows += 1;
        }
        stats.ppu_writes += ppu;
        stats.apu_writes += apu;
        stats.mapper_writes += mapper;
        stats.unmapped_reads += parse_u64_or_zero(fields.get(23));
    }
    Ok(stats)
}

fn summarize_verify(path: &Path) -> io::Result<VerifyStats> {
    let text = fs::read_to_string(path)?;
    let mut stats = VerifyStats::default();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 13 {
            return invalid_tsv(path, line_no + 1, fields.len(), 13);
        }
        stats.cases += 1;
        if fields[12] == "1" {
            stats.matched += 1;
        } else {
            stats.mismatches += 1;
        }
        if fields[9] == "1" {
            stats.external_matched += 1;
        } else {
            stats.external_mismatches += 1;
        }
    }
    Ok(stats)
}

fn write_oracle_manifest(
    out_dir: &Path,
    names: &ProofNames,
    stats: &OracleStats,
) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("oracle/manifest.txt"))?;
    writeln!(file, "cases={}", names.cases)?;
    writeln!(file, "case_count={}", stats.case_count)?;
    writeln!(file, "left_block={}", stats.left_block)?;
    writeln!(file, "stopped={}", stats.stopped)?;
    writeln!(file, "unsupported_opcode={}", stats.unsupported_opcode)?;
    writeln!(file, "step_limit={}", stats.step_limit)?;
    writeln!(file, "invalid_block={}", stats.invalid_block)?;
    writeln!(file, "external_write_rows={}", stats.external_write_rows)?;
    writeln!(file, "external_write_alloc_failed=0")?;
    writeln!(
        file,
        "scope=tooling-only accumulated semantic block execution from explicit states"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_native_verify_manifest(
    out_dir: &Path,
    names: &ProofNames,
    stats: &VerifyStats,
) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("native_verify/manifest.txt"))?;
    writeln!(
        file,
        "cases=native_verify/{}_native_verify_cases.tsv",
        names.prefix
    )?;
    writeln!(file, "final_states=native_block_final_states.tsv")?;
    writeln!(file, "case_count={}", stats.cases)?;
    writeln!(file, "matched={}", stats.matched)?;
    writeln!(file, "mismatches={}", stats.mismatches)?;
    writeln!(file, "external_write_matched={}", stats.external_matched)?;
    writeln!(
        file,
        "external_write_mismatches={}",
        stats.external_mismatches
    )?;
    writeln!(
        file,
        "scope=accumulated generated native block output, external writes, and final state versus block-exec oracle"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_summary(
    out_dir: &Path,
    old_dir: &Path,
    new_dir: &Path,
    names: &ProofNames,
    block_stats: &BlockMergeStats,
    oracle: &OracleStats,
) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join(&names.summary))?;
    writeln!(file, "runtime={}_verify_accumulated", names.prefix)?;
    writeln!(file, "kind={}", names.prefix.trim_start_matches("static_"))?;
    writeln!(file, "old_dir={}", old_dir.display())?;
    writeln!(file, "new_dir={}", new_dir.display())?;
    writeln!(file, "selected_count={}", block_stats.selected_count)?;
    writeln!(file, "synthetic_case_count={}", oracle.case_count)?;
    writeln!(file, "ram_write_rows={}", oracle.ram_write_rows)?;
    writeln!(file, "ram_writes_total={}", oracle.ram_writes_total)?;
    writeln!(
        file,
        "external_writes_total={}",
        oracle.ppu_writes + oracle.apu_writes + oracle.mapper_writes
    )?;
    writeln!(file, "unmapped_reads_total={}", oracle.unmapped_reads)?;
    writeln!(file, "native_blocks={}", names.blocks)?;
    writeln!(file, "cases={}", names.cases)?;
    writeln!(file, "skipped={}", names.skipped)?;
    if names.prefix == "static_leaf" {
        let skipped = summarize_skip_reasons(&out_dir.join(&names.skipped))?;
        writeln!(file, "skipped_call_like={}", skipped.call_like_leaf)?;
        writeln!(
            file,
            "skipped_missing_byte_count={}",
            skipped.missing_byte_count
        )?;
        writeln!(file, "skipped_unsupported={}", skipped.unsupported)?;
        writeln!(file, "skipped_loop_like={}", skipped.loop_like_leaf)?;
    }
    writeln!(file, "oracle=oracle/block_state_exec.tsv")?;
    writeln!(file, "native_verify=native_verify/native_block_verify.tsv")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn summarize_skip_reasons(path: &Path) -> io::Result<SkipReasonStats> {
    let text = fs::read_to_string(path)?;
    let mut stats = SkipReasonStats::default();
    for line in text.lines().skip(1) {
        let fields = split_tsv(line);
        let Some(reason) = fields.last() else {
            continue;
        };
        match reason.as_str() {
            "call_like_leaf_deferred" => stats.call_like_leaf += 1,
            "missing_byte_count" => stats.missing_byte_count += 1,
            "unsupported_native_opcode" => stats.unsupported += 1,
            "loop_like_leaf_deferred" => stats.loop_like_leaf += 1,
            _ => {}
        }
    }
    Ok(stats)
}

fn write_manifest(
    out_dir: &Path,
    old_dir: &Path,
    new_dir: &Path,
    names: &ProofNames,
    stats: &BlockMergeStats,
    case_count: u64,
) -> io::Result<()> {
    let mut file = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(file, "runtime={}_verify_accumulated", names.prefix)?;
    writeln!(file, "kind={}", names.prefix.trim_start_matches("static_"))?;
    writeln!(file, "old_dir={}", old_dir.display())?;
    writeln!(file, "new_dir={}", new_dir.display())?;
    writeln!(file, "native_blocks={}", names.blocks)?;
    writeln!(file, "cases={}", names.cases)?;
    writeln!(file, "skipped={}", names.skipped)?;
    writeln!(file, "oracle=oracle/block_state_exec.tsv")?;
    writeln!(file, "native_verify=native_verify/native_block_verify.tsv")?;
    writeln!(file, "summary={}", names.summary)?;
    writeln!(file, "selected_count={}", stats.selected_count)?;
    writeln!(file, "synthetic_case_count={case_count}")?;
    writeln!(file, "complete=1")?;
    Ok(())
}
