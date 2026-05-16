use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct TranslationRow {
    replay: String,
    id: String,
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    hit_count: u64,
    writes: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
}

#[derive(Debug, Default, Clone)]
struct ExternalRows {
    sequence: Vec<String>,
    shape: Vec<String>,
    row_count: u64,
    bad_header: bool,
}

#[derive(Debug, Clone)]
struct Observation {
    replay: String,
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    hit_count: u64,
    writes: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
    external_rows: u64,
    shape: String,
    sequence: String,
    mismatch: u64,
}

#[derive(Debug, Default)]
struct BlockAggregate {
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    replay_order: Vec<String>,
    replay_seen: HashSet<String>,
    observations: u64,
    hit_count_total: u64,
    writes_total: u64,
    ppu_writes: u64,
    apu_writes: u64,
    mapper_writes: u64,
    external_write_rows: u64,
    mismatch_count: u64,
    shapes: Vec<String>,
    shape_seen: HashSet<String>,
    sequences: Vec<String>,
    sequence_seen: HashSet<String>,
}

#[derive(Debug, Clone)]
struct BlockOutput {
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
    external_write_rows: u64,
    shape_count: u64,
    sequence_count: u64,
    class: String,
    mismatch_count: u64,
    example_shape: String,
    example_sequence: String,
}

#[derive(Debug, Clone)]
struct SiteRow {
    kind: String,
    addr: String,
    replay: String,
    id: String,
    cpu_addr: String,
    prg_offset: String,
    bytes: String,
    first_opcode: String,
    hit_count: u64,
}

#[derive(Debug, Default)]
struct SiteAggregate {
    kind: String,
    addr: String,
    writes: u64,
    blocks: HashSet<String>,
    observations: HashSet<String>,
    hit_count_total: u64,
}

#[derive(Debug, Default)]
struct Summary {
    external_block_count: u64,
    static_sequence_count: u64,
    dynamic_values_count: u64,
    dynamic_shape_count: u64,
    external_write_rows: u64,
    mismatch_count: u64,
}

pub fn run(
    build_dir: &Path,
    out_dir: &Path,
    replays: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if replays.is_empty() {
        return Err("external_block_plan: at least one replay is required".into());
    }

    let translation_plan = build_dir
        .join("block_translation_plan")
        .join("block_translation_plan.tsv");
    require_file(&translation_plan)?;
    let translations = read_translation_plan(&translation_plan)?;

    fs::create_dir_all(out_dir)?;
    let mut observations = Vec::new();
    let mut site_rows = Vec::new();
    for replay in replays {
        let external_tsv = build_dir
            .join("block_exec")
            .join(replay)
            .join("block_external_writes.tsv");
        require_file(&external_tsv)?;
        let external = read_external_rows(&external_tsv)?;
        for row in translations.iter().filter(|row| row.replay == *replay) {
            let expected_external = row.ppu_writes + row.apu_writes + row.mapper_writes;
            if expected_external == 0 {
                continue;
            }
            let rows = external
                .by_id
                .get(&row.id)
                .cloned()
                .unwrap_or_else(|| ExternalRows {
                    bad_header: external.bad_header,
                    ..ExternalRows::default()
                });
            observations.push(make_observation(replay, row, expected_external, &rows));
        }
        site_rows.extend(make_site_rows(replay, &translations, &external));
    }

    let block_outputs = aggregate_blocks(&observations);
    let site_outputs = aggregate_sites(&site_rows);
    let summary = summarize_blocks(&block_outputs);

    write_external_blocks(&out_dir.join("external_block_plan.tsv"), &block_outputs)?;
    write_site_summary(
        &out_dir.join("external_write_site_summary.tsv"),
        &site_outputs,
    )?;
    write_summary(&out_dir.join("external_block_summary.txt"), &summary)?;
    write_manifest(&out_dir.join("manifest.txt"), build_dir, replays, &summary)?;

    if summary.mismatch_count != 0 {
        return Err("external_block_plan: row-count mismatches remain".into());
    }

    println!("external_block_plan: wrote {}", out_dir.display());
    Ok(())
}

struct ExternalFile {
    bad_header: bool,
    by_id: HashMap<String, ExternalRows>,
    rows: Vec<(String, String, String, String)>,
}

fn require_file(path: &Path) -> io::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("external_block_plan: missing input: {}", path.display()),
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

fn read_translation_plan(path: &Path) -> io::Result<Vec<TranslationRow>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 18 {
            return invalid_tsv(path, line_no + 1, fields.len(), 18);
        }
        rows.push(TranslationRow {
            replay: fields[0].to_string(),
            id: fields[1].to_string(),
            cpu_addr: fields[2].to_string(),
            prg_offset: fields[3].to_string(),
            bytes: fields[4].to_string(),
            first_opcode: fields[5].to_string(),
            hit_count: parse_u64(fields[8]),
            writes: parse_u64(fields[10]),
            ppu_writes: parse_u64(fields[11]),
            apu_writes: parse_u64(fields[12]),
            mapper_writes: parse_u64(fields[13]),
        });
    }
    Ok(rows)
}

fn read_external_rows(path: &Path) -> io::Result<ExternalFile> {
    const EXPECTED_HEADER: &str =
        "id\tcpu_addr\tprg_offset\twrite_index\texternal_index\tkind\taddr\tvalue";
    let text = fs::read_to_string(path)?;
    let mut by_id = HashMap::<String, ExternalRows>::new();
    let mut rows = Vec::new();
    let mut bad_header = false;
    for (line_no, line) in text.lines().enumerate() {
        if line_no == 0 {
            bad_header = line != EXPECTED_HEADER;
            continue;
        }
        let fields = split_tsv(line);
        if fields.len() < 8 {
            return invalid_tsv(path, line_no + 1, fields.len(), 8);
        }
        let token = format!("{}:{}:{}", fields[5], fields[6], fields[7]);
        let shape = format!("{}:{}", fields[5], fields[6]);
        let entry = by_id.entry(fields[0].to_string()).or_default();
        entry.sequence.push(token);
        entry.shape.push(shape);
        entry.row_count += 1;
        entry.bad_header = bad_header;
        rows.push((
            fields[0].to_string(),
            fields[5].to_string(),
            fields[6].to_string(),
            fields[7].to_string(),
        ));
    }
    Ok(ExternalFile {
        bad_header,
        by_id,
        rows,
    })
}

fn make_observation(
    replay: &str,
    row: &TranslationRow,
    expected_external: u64,
    external: &ExternalRows,
) -> Observation {
    let shape = if external.shape.is_empty() {
        "none".to_string()
    } else {
        external.shape.join(",")
    };
    let sequence = if external.sequence.is_empty() {
        "none".to_string()
    } else {
        external.sequence.join(",")
    };
    Observation {
        replay: replay.to_string(),
        cpu_addr: row.cpu_addr.clone(),
        prg_offset: row.prg_offset.clone(),
        bytes: row.bytes.clone(),
        first_opcode: row.first_opcode.clone(),
        hit_count: row.hit_count,
        writes: row.writes,
        ppu_writes: row.ppu_writes,
        apu_writes: row.apu_writes,
        mapper_writes: row.mapper_writes,
        external_rows: external.row_count,
        shape,
        sequence,
        mismatch: u64::from(external.row_count != expected_external || external.bad_header),
    }
}

fn make_site_rows(
    replay: &str,
    translations: &[TranslationRow],
    external: &ExternalFile,
) -> Vec<SiteRow> {
    let rows_by_id = translations
        .iter()
        .filter(|row| row.replay == replay)
        .map(|row| (row.id.as_str(), row))
        .collect::<HashMap<_, _>>();
    let mut rows = Vec::new();
    for (id, kind, addr, _value) in &external.rows {
        let row = rows_by_id.get(id.as_str());
        rows.push(SiteRow {
            kind: kind.clone(),
            addr: addr.clone(),
            replay: replay.to_string(),
            id: id.clone(),
            cpu_addr: row.map(|item| item.cpu_addr.clone()).unwrap_or_default(),
            prg_offset: row.map(|item| item.prg_offset.clone()).unwrap_or_default(),
            bytes: row.map(|item| item.bytes.clone()).unwrap_or_default(),
            first_opcode: row
                .map(|item| item.first_opcode.clone())
                .unwrap_or_default(),
            hit_count: row.map(|item| item.hit_count).unwrap_or(0),
        });
    }
    rows
}

fn aggregate_blocks(observations: &[Observation]) -> Vec<BlockOutput> {
    let mut order = Vec::new();
    let mut aggregates = HashMap::<String, BlockAggregate>::new();
    for obs in observations {
        let key = format!(
            "{}\t{}\t{}\t{}",
            obs.cpu_addr, obs.prg_offset, obs.bytes, obs.first_opcode
        );
        if !aggregates.contains_key(&key) {
            order.push(key.clone());
        }
        let entry = aggregates.entry(key).or_insert_with(|| BlockAggregate {
            cpu_addr: obs.cpu_addr.clone(),
            prg_offset: obs.prg_offset.clone(),
            bytes: obs.bytes.clone(),
            first_opcode: obs.first_opcode.clone(),
            ..BlockAggregate::default()
        });
        entry.observations += 1;
        entry.hit_count_total += obs.hit_count;
        entry.writes_total += obs.writes;
        entry.ppu_writes += obs.ppu_writes;
        entry.apu_writes += obs.apu_writes;
        entry.mapper_writes += obs.mapper_writes;
        entry.external_write_rows += obs.external_rows;
        entry.mismatch_count += obs.mismatch;
        if entry.replay_seen.insert(obs.replay.clone()) {
            entry.replay_order.push(obs.replay.clone());
        }
        if entry.shape_seen.insert(obs.shape.clone()) {
            entry.shapes.push(obs.shape.clone());
        }
        if entry.sequence_seen.insert(obs.sequence.clone()) {
            entry.sequences.push(obs.sequence.clone());
        }
    }

    let mut outputs = order
        .into_iter()
        .filter_map(|key| aggregates.remove(&key))
        .map(|item| {
            let shape_count = item.shapes.len() as u64;
            let sequence_count = item.sequences.len() as u64;
            let class = if shape_count == 1 && sequence_count == 1 {
                "static_sequence"
            } else if shape_count == 1 {
                "dynamic_values"
            } else {
                "dynamic_shape"
            };
            BlockOutput {
                cpu_addr: item.cpu_addr,
                prg_offset: item.prg_offset,
                bytes: item.bytes,
                first_opcode: item.first_opcode,
                replay_count: item.replay_order.len() as u64,
                replays: item.replay_order.join(","),
                observations: item.observations,
                hit_count_total: item.hit_count_total,
                writes_total: item.writes_total,
                ppu_writes: item.ppu_writes,
                apu_writes: item.apu_writes,
                mapper_writes: item.mapper_writes,
                external_write_rows: item.external_write_rows,
                shape_count,
                sequence_count,
                class: class.to_string(),
                mismatch_count: item.mismatch_count,
                example_shape: item.shapes.first().cloned().unwrap_or_default(),
                example_sequence: item.sequences.first().cloned().unwrap_or_default(),
            }
        })
        .collect::<Vec<_>>();

    outputs.sort_by(|lhs, rhs| {
        rhs.external_write_rows
            .cmp(&lhs.external_write_rows)
            .then_with(|| rhs.hit_count_total.cmp(&lhs.hit_count_total))
            .then_with(|| lhs.cpu_addr.cmp(&rhs.cpu_addr))
    });
    outputs
}

fn aggregate_sites(rows: &[SiteRow]) -> Vec<SiteAggregate> {
    let mut sites = BTreeMap::<String, SiteAggregate>::new();
    for row in rows {
        let key = format!("{}\t{}", row.kind, row.addr);
        let entry = sites.entry(key).or_insert_with(|| SiteAggregate {
            kind: row.kind.clone(),
            addr: row.addr.clone(),
            ..SiteAggregate::default()
        });
        entry.writes += 1;
        entry.blocks.insert(format!(
            "{}\t{}\t{}\t{}",
            row.cpu_addr, row.prg_offset, row.bytes, row.first_opcode
        ));
        let observation_key = format!("{}\t{}", row.replay, row.id);
        if entry.observations.insert(observation_key) {
            entry.hit_count_total += row.hit_count;
        }
    }
    let mut outputs = sites.into_values().collect::<Vec<_>>();
    outputs.sort_by(|lhs, rhs| {
        rhs.writes
            .cmp(&lhs.writes)
            .then_with(|| lhs.kind.cmp(&rhs.kind))
            .then_with(|| lhs.addr.cmp(&rhs.addr))
    });
    outputs
}

fn summarize_blocks(blocks: &[BlockOutput]) -> Summary {
    let mut summary = Summary {
        external_block_count: blocks.len() as u64,
        ..Summary::default()
    };
    for block in blocks {
        summary.external_write_rows += block.external_write_rows;
        summary.mismatch_count += block.mismatch_count;
        match block.class.as_str() {
            "static_sequence" => summary.static_sequence_count += 1,
            "dynamic_values" => summary.dynamic_values_count += 1,
            "dynamic_shape" => summary.dynamic_shape_count += 1,
            _ => {}
        }
    }
    summary
}

fn write_external_blocks(path: &Path, blocks: &[BlockOutput]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "rank\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\treplay_count\treplays\tobservations\thit_count_total\twrites_total\tppu_writes\tapu_writes\tmapper_writes\texternal_write_rows\tshape_count\tsequence_count\tclass\tmismatch_count\texample_shape\texample_sequence"
    )?;
    for (index, block) in blocks.iter().enumerate() {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            index + 1,
            block.cpu_addr,
            block.prg_offset,
            block.bytes,
            block.first_opcode,
            block.replay_count,
            block.replays,
            block.observations,
            block.hit_count_total,
            block.writes_total,
            block.ppu_writes,
            block.apu_writes,
            block.mapper_writes,
            block.external_write_rows,
            block.shape_count,
            block.sequence_count,
            block.class,
            block.mismatch_count,
            block.example_shape,
            block.example_sequence
        )?;
    }
    Ok(())
}

fn write_site_summary(path: &Path, sites: &[SiteAggregate]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "kind\taddr\tblocks\tobservations\twrites\thit_count_total"
    )?;
    for site in sites {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}",
            site.kind,
            site.addr,
            site.blocks.len(),
            site.observations.len(),
            site.writes,
            site.hit_count_total
        )?;
    }
    Ok(())
}

fn write_summary(path: &Path, summary: &Summary) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    write_summary_values(&mut file, summary)
}

fn write_summary_values(mut file: impl Write, summary: &Summary) -> io::Result<()> {
    writeln!(
        file,
        "external_block_count={}",
        summary.external_block_count
    )?;
    writeln!(
        file,
        "static_sequence_count={}",
        summary.static_sequence_count
    )?;
    writeln!(
        file,
        "dynamic_values_count={}",
        summary.dynamic_values_count
    )?;
    writeln!(file, "dynamic_shape_count={}", summary.dynamic_shape_count)?;
    writeln!(file, "external_write_rows={}", summary.external_write_rows)?;
    writeln!(file, "mismatch_count={}", summary.mismatch_count)?;
    writeln!(file, "complete={}", u8::from(summary.mismatch_count == 0))?;
    Ok(())
}

fn write_manifest(
    path: &Path,
    build_dir: &Path,
    replays: &[String],
    summary: &Summary,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "build_dir={}", build_dir.display())?;
    writeln!(
        file,
        "source=block_translation_plan/block_translation_plan.tsv"
    )?;
    writeln!(file, "replays={}", replays.join(" "))?;
    writeln!(file, "external_blocks=external_block_plan.tsv")?;
    writeln!(
        file,
        "external_write_site_summary=external_write_site_summary.tsv"
    )?;
    writeln!(file, "external_block_summary=external_block_summary.txt")?;
    write_summary_values(&mut file, summary)
}
