use crate::disasm;
use lotw_port::rom::InesRom;
use lotw_port::sha256;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const BANK_16K: usize = 0x4000;
const WINDOW_BYTES: usize = 0x80;

#[derive(Debug, Default, Clone)]
struct OffsetEvidence {
    executed_bytes: u64,
    verified_bytes: u64,
    label_count: u64,
    covered_labels: u64,
    static_reachable_labels: u64,
    known_opcode_labels: u64,
    data_or_unknown_labels: u64,
    mapped_in_edges: u64,
    reachable_in_edges: u64,
    mapped_out_edges: u64,
}

#[derive(Debug, Clone)]
struct DecodeStats {
    start_skip: usize,
    instructions: u64,
    valid_bytes: u64,
    invalid_bytes: u64,
    branch_count: u64,
    local_branch_count: u64,
    jsr_count: u64,
    jump_count: u64,
    return_count: u64,
    brk_count: u64,
    ppu_apu_mapper_refs: u64,
    zero_page_refs: u64,
    score: i64,
}

#[derive(Debug, Clone)]
struct TextureStats {
    entropy: f64,
    unique_bytes: usize,
    longest_run: usize,
    pointer_pairs: u64,
    pointer_pair_percent: f64,
}

#[derive(Debug, Clone)]
struct Window {
    index: usize,
    start: usize,
    end: usize,
    class_name: String,
    confidence: u8,
    reason: String,
    evidence: OffsetEvidence,
    decode: DecodeStats,
    texture: TextureStats,
}

#[derive(Debug, Clone)]
struct Segment {
    start: usize,
    end: usize,
    class_name: String,
    min_confidence: u8,
    windows: usize,
}

pub fn run(
    rom_path: &Path,
    build_dir: &Path,
    out_dir: &Path,
    expected_sha256: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rom_bytes = fs::read(rom_path)?;
    let actual_sha256 = sha256::digest_hex(&rom_bytes);
    if let Some(expected) = expected_sha256 {
        if !actual_sha256.eq_ignore_ascii_case(expected) {
            return Err(format!(
                "ai_rom_map: ROM hash mismatch: got {actual_sha256}, expected {}",
                expected.to_ascii_lowercase()
            )
            .into());
        }
    }

    let rom = InesRom::parse(&rom_bytes)?;
    let prg = rom.prg_rom();
    if prg.is_empty() {
        return Err("ai_rom_map: ROM has no PRG bytes".into());
    }

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    let evidence = read_evidence(build_dir, prg.len())?;
    let windows = classify_windows(prg, &evidence);
    let segments = merge_segments(&windows);

    write_windows(&out_dir.join("windows.tsv"), &windows)?;
    write_segments(&out_dir.join("segments.tsv"), &segments, prg.len())?;
    write_annotated(&out_dir.join("annotated.asm"), prg, &windows)?;
    write_manifest(
        &out_dir.join("manifest.txt"),
        build_dir,
        &actual_sha256,
        &rom,
        &windows,
        &segments,
    )?;

    println!("ai_rom_map: wrote {}", out_dir.display());
    Ok(())
}

fn read_evidence(build_dir: &Path, prg_len: usize) -> io::Result<Vec<OffsetEvidence>> {
    let mut evidence = vec![OffsetEvidence::default(); prg_len];
    read_static_reachable_labels(
        &build_dir.join("static_cfg/static_reachable_labels.tsv"),
        &mut evidence,
    )?;
    read_block_spans(
        &build_dir.join("block_translation_plan/block_translation_plan.tsv"),
        &mut evidence,
        EvidenceSpanKind::Executed,
    )?;
    read_block_spans(
        &build_dir.join("native_block_plan_static/native_blocks.tsv"),
        &mut evidence,
        EvidenceSpanKind::Verified,
    )?;
    Ok(evidence)
}

fn read_static_reachable_labels(path: &Path, evidence: &mut [OffsetEvidence]) -> io::Result<()> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(err),
    };
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Ok(());
    };
    let columns = header_columns(header);
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        let Some(offset) = column(&fields, &columns, "prg_offset").and_then(parse_hex_usize) else {
            continue;
        };
        let Some(slot) = evidence.get_mut(offset) else {
            continue;
        };
        slot.label_count += 1;
        if column(&fields, &columns, "covered") == Some("1") {
            slot.covered_labels += 1;
        }
        if column(&fields, &columns, "static_reachable") == Some("1") {
            slot.static_reachable_labels += 1;
        }
        match column(&fields, &columns, "mnemonic") {
            Some(".db") | None => slot.data_or_unknown_labels += 1,
            Some(_) => slot.known_opcode_labels += 1,
        }
        slot.mapped_in_edges += column(&fields, &columns, "mapped_in_edges")
            .and_then(parse_u64)
            .unwrap_or(0);
        slot.reachable_in_edges += column(&fields, &columns, "reachable_in_edges")
            .and_then(parse_u64)
            .unwrap_or(0);
        slot.mapped_out_edges += column(&fields, &columns, "mapped_out_edges")
            .and_then(parse_u64)
            .unwrap_or(0);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum EvidenceSpanKind {
    Executed,
    Verified,
}

fn read_block_spans(
    path: &Path,
    evidence: &mut [OffsetEvidence],
    kind: EvidenceSpanKind,
) -> io::Result<()> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(err),
    };
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Ok(());
    };
    let columns = header_columns(header);
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        let Some(start) = column(&fields, &columns, "prg_offset").and_then(parse_hex_usize) else {
            continue;
        };
        let len = column(&fields, &columns, "bytes")
            .and_then(parse_usize)
            .unwrap_or(1)
            .max(1);
        let end = start.saturating_add(len).min(evidence.len());
        for slot in evidence.iter_mut().take(end).skip(start) {
            match kind {
                EvidenceSpanKind::Executed => slot.executed_bytes = 1,
                EvidenceSpanKind::Verified => slot.verified_bytes = 1,
            }
        }
    }
    Ok(())
}

fn classify_windows(prg: &[u8], evidence: &[OffsetEvidence]) -> Vec<Window> {
    let mut windows = Vec::new();
    for (index, start) in (0..prg.len()).step_by(WINDOW_BYTES).enumerate() {
        let end = start.saturating_add(WINDOW_BYTES).min(prg.len());
        let bytes = &prg[start..end];
        let evidence_sum = sum_evidence(&evidence[start..end]);
        let decode = best_decode(prg, start, end);
        let texture = texture_stats(bytes);
        let (class_name, confidence, reason) =
            classify_window(bytes, &evidence_sum, &decode, &texture);
        windows.push(Window {
            index,
            start,
            end,
            class_name,
            confidence,
            reason,
            evidence: evidence_sum,
            decode,
            texture,
        });
    }
    windows
}

fn sum_evidence(items: &[OffsetEvidence]) -> OffsetEvidence {
    let mut out = OffsetEvidence::default();
    for item in items {
        out.executed_bytes += item.executed_bytes;
        out.verified_bytes += item.verified_bytes;
        out.label_count += item.label_count;
        out.covered_labels += item.covered_labels;
        out.static_reachable_labels += item.static_reachable_labels;
        out.known_opcode_labels += item.known_opcode_labels;
        out.data_or_unknown_labels += item.data_or_unknown_labels;
        out.mapped_in_edges += item.mapped_in_edges;
        out.reachable_in_edges += item.reachable_in_edges;
        out.mapped_out_edges += item.mapped_out_edges;
    }
    out
}

fn classify_window(
    bytes: &[u8],
    evidence: &OffsetEvidence,
    decode: &DecodeStats,
    texture: &TextureStats,
) -> (String, u8, String) {
    let len = bytes.len().max(1) as f64;
    let valid_ratio = decode.valid_bytes as f64 / len;
    let invalid_ratio = decode.invalid_bytes as f64 / len;
    let executed_ratio = evidence.executed_bytes as f64 / len;
    let verified_ratio = evidence.verified_bytes as f64 / len;
    let has_control_shape =
        decode.branch_count + decode.jsr_count + decode.jump_count + decode.return_count > 0;
    let has_memory_shape =
        decode.zero_page_refs + decode.ppu_apu_mapper_refs >= 6 || decode.ppu_apu_mapper_refs > 0;
    let padding_like =
        texture.longest_run >= 32 || (texture.unique_bytes <= 2 && texture.entropy < 0.75);
    let pointer_like = texture.pointer_pairs >= 12 && texture.pointer_pair_percent >= 35.0;

    if evidence.executed_bytes > 0 {
        return (
            "definite_code".to_string(),
            confidence_from_ratio(92, executed_ratio),
            format!(
                "trace/native block evidence covers {} bytes; valid_decode={:.1}%",
                evidence.executed_bytes,
                valid_ratio * 100.0
            ),
        );
    }
    if evidence.verified_bytes > 0 {
        return (
            "verified_code".to_string(),
            confidence_from_ratio(88, verified_ratio),
            format!(
                "static/native proof evidence covers {} bytes; valid_decode={:.1}%",
                evidence.verified_bytes,
                valid_ratio * 100.0
            ),
        );
    }
    if evidence.static_reachable_labels > 0 && valid_ratio >= 0.70 {
        return (
            "probable_code".to_string(),
            82,
            format!(
                "{} static reachable labels, {} reachable inbound edges, control_shape={}",
                evidence.static_reachable_labels,
                evidence.reachable_in_edges,
                u8::from(has_control_shape)
            ),
        );
    }
    if evidence.covered_labels > 0 || evidence.reachable_in_edges > 0 {
        return (
            "probable_code".to_string(),
            76,
            format!(
                "label graph evidence: covered_labels={} reachable_in_edges={}",
                evidence.covered_labels, evidence.reachable_in_edges
            ),
        );
    }
    if padding_like {
        return (
            "padding_or_fill".to_string(),
            90,
            format!(
                "low entropy {:.2}, {} unique bytes, longest repeated run {}",
                texture.entropy, texture.unique_bytes, texture.longest_run
            ),
        );
    }
    if pointer_like {
        return (
            "probable_pointer_table".to_string(),
            78,
            format!(
                "{} little-endian PRG-looking address pairs ({:.1}% of pairs)",
                texture.pointer_pairs, texture.pointer_pair_percent
            ),
        );
    }
    if valid_ratio >= 0.88 && decode.score >= 80 && (has_control_shape || has_memory_shape) {
        return (
            "ai_probable_code".to_string(),
            70,
            format!(
                "6502 texture score {}, valid_decode={:.1}%, control_ops={}, memory_refs={}",
                decode.score,
                valid_ratio * 100.0,
                decode.branch_count + decode.jsr_count + decode.jump_count + decode.return_count,
                decode.zero_page_refs + decode.ppu_apu_mapper_refs
            ),
        );
    }
    if valid_ratio >= 0.92 && decode.score >= 45 && !pointer_like {
        return (
            "ai_possible_code".to_string(),
            58,
            format!(
                "mostly legal 6502 stream, score {}, valid_decode={:.1}%, weak external evidence",
                decode.score,
                valid_ratio * 100.0
            ),
        );
    }
    if invalid_ratio >= 0.18 || evidence.data_or_unknown_labels > evidence.known_opcode_labels {
        return (
            "probable_data".to_string(),
            68,
            format!(
                "invalid_decode={:.1}%, data_or_unknown_labels={}, known_opcode_labels={}",
                invalid_ratio * 100.0,
                evidence.data_or_unknown_labels,
                evidence.known_opcode_labels
            ),
        );
    }

    (
        "ambiguous_code_or_data".to_string(),
        42,
        format!(
            "valid_decode={:.1}%, score {}, entropy {:.2}; no trace/static proof evidence",
            valid_ratio * 100.0,
            decode.score,
            texture.entropy
        ),
    )
}

fn confidence_from_ratio(base: u8, ratio: f64) -> u8 {
    (f64::from(base) + ratio.clamp(0.0, 1.0) * 8.0)
        .round()
        .min(99.0) as u8
}

fn best_decode(prg: &[u8], start: usize, end: usize) -> DecodeStats {
    let max_skip = (end - start).min(15);
    let mut best = decode_from(prg, start, end, 0);
    for skip in 1..=max_skip {
        let candidate = decode_from(prg, start + skip, end, skip);
        if candidate.score > best.score {
            best = candidate;
        }
    }
    best
}

fn decode_from(prg: &[u8], cursor_start: usize, end: usize, start_skip: usize) -> DecodeStats {
    let mut stats = DecodeStats {
        start_skip,
        instructions: 0,
        valid_bytes: 0,
        invalid_bytes: start_skip as u64,
        branch_count: 0,
        local_branch_count: 0,
        jsr_count: 0,
        jump_count: 0,
        return_count: 0,
        brk_count: 0,
        ppu_apu_mapper_refs: 0,
        zero_page_refs: 0,
        score: -(start_skip as i64 * 2),
    };
    let mut cursor = cursor_start;
    while cursor < end {
        let opcode = prg[cursor];
        let op = disasm::op_info(opcode);
        let mut len = op.len;
        if cursor + len > end {
            len = 1;
        }
        let bytes = &prg[cursor..cursor + len];
        if op.mnemonic.is_some() && len == op.len {
            stats.instructions += 1;
            stats.valid_bytes += len as u64;
            stats.score += 2 * len as i64;
            if matches!(
                op.mode,
                disasm::AddrMode::Zp | disasm::AddrMode::Zpx | disasm::AddrMode::Zpy
            ) {
                stats.zero_page_refs += 1;
                stats.score += 2;
            }
            if references_hardware_or_prg(op, bytes) {
                stats.ppu_apu_mapper_refs += 1;
                stats.score += 6;
            }
            if op.is_branch && len >= 2 {
                stats.branch_count += 1;
                stats.score += 5;
                if branch_target_in_window(cursor, bytes[1], start_of_window(cursor), end) {
                    stats.local_branch_count += 1;
                    stats.score += 3;
                }
            }
            if op.is_jsr {
                stats.jsr_count += 1;
                stats.score += 8;
            }
            if op.is_jump {
                stats.jump_count += 1;
                stats.score += 5;
            }
            match opcode {
                0x60 | 0x40 => {
                    stats.return_count += 1;
                    stats.score += 4;
                }
                0x00 => {
                    stats.brk_count += 1;
                    stats.score -= 8;
                }
                0xEA => stats.score -= 1,
                _ => {}
            }
        } else {
            stats.invalid_bytes += 1;
            stats.score -= 8;
        }
        cursor += len.max(1);
    }
    if stats.instructions > 0 && stats.return_count * 5 > stats.instructions {
        stats.score -= (stats.return_count * 4) as i64;
    }
    if stats.brk_count * 4 > stats.instructions.max(1) {
        stats.score -= (stats.brk_count * 5) as i64;
    }
    stats
}

fn start_of_window(offset: usize) -> usize {
    offset - (offset % WINDOW_BYTES)
}

fn branch_target_in_window(cursor: usize, rel: u8, window_start: usize, window_end: usize) -> bool {
    let target = cursor as isize + 2 + isize::from(rel as i8);
    target >= window_start as isize && target < window_end as isize
}

fn references_hardware_or_prg(op: disasm::OpInfo, bytes: &[u8]) -> bool {
    if bytes.len() < 3 {
        return false;
    }
    if !matches!(
        op.mode,
        disasm::AddrMode::Abs
            | disasm::AddrMode::Absx
            | disasm::AddrMode::Absy
            | disasm::AddrMode::Ind
    ) {
        return false;
    }
    let addr = u16::from(bytes[1]) | (u16::from(bytes[2]) << 8);
    matches!(addr, 0x2000..=0x2007 | 0x4000..=0x4017 | 0x8000..=0xFFFF)
}

fn texture_stats(bytes: &[u8]) -> TextureStats {
    let mut counts = [0u64; 256];
    let mut longest_run = 0usize;
    let mut current_run = 0usize;
    let mut previous = None;
    for &byte in bytes {
        counts[usize::from(byte)] += 1;
        if previous == Some(byte) {
            current_run += 1;
        } else {
            current_run = 1;
            previous = Some(byte);
        }
        longest_run = longest_run.max(current_run);
    }
    let len = bytes.len().max(1) as f64;
    let mut entropy = 0.0;
    let mut unique_bytes = 0usize;
    for count in counts {
        if count == 0 {
            continue;
        }
        unique_bytes += 1;
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }

    let pair_count = bytes.len() / 2;
    let mut pointer_pairs = 0u64;
    for pair in bytes.chunks_exact(2) {
        let value = u16::from(pair[0]) | (u16::from(pair[1]) << 8);
        if (0x8000..=0xFFFF).contains(&value) {
            pointer_pairs += 1;
        }
    }
    let pointer_pair_percent = if pair_count == 0 {
        0.0
    } else {
        pointer_pairs as f64 * 100.0 / pair_count as f64
    };

    TextureStats {
        entropy,
        unique_bytes,
        longest_run,
        pointer_pairs,
        pointer_pair_percent,
    }
}

fn merge_segments(windows: &[Window]) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();
    for window in windows {
        if let Some(last) = segments.last_mut() {
            if last.class_name == window.class_name && last.end == window.start {
                last.end = window.end;
                last.min_confidence = last.min_confidence.min(window.confidence);
                last.windows += 1;
                continue;
            }
        }
        segments.push(Segment {
            start: window.start,
            end: window.end,
            class_name: window.class_name.clone(),
            min_confidence: window.confidence,
            windows: 1,
        });
    }
    segments
}

fn write_windows(path: &Path, windows: &[Window]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "index\tprg_start\tprg_end\tbank\tcpu_start\tcpu_end_exclusive\tclass\tconfidence\treason\texecuted_bytes\tverified_bytes\tstatic_reachable_labels\tknown_opcode_labels\tdata_or_unknown_labels\treachable_in_edges\tdecode_start_skip\tdecode_score\tvalid_bytes\tinvalid_bytes\tinstructions\tbranches\tlocal_branches\tjsr\tjumps\treturns\tbrk\thardware_or_prg_refs\tzero_page_refs\tentropy\tunique_bytes\tlongest_run\tpointer_pairs\tpointer_pair_percent"
    )?;
    for window in windows {
        writeln!(
            file,
            "{}\t{:05X}\t{:05X}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{}\t{}\t{}\t{:.1}",
            window.index,
            window.start,
            window.end,
            bank_text(window.start),
            cpu_text(window.start),
            cpu_text(window.end),
            window.class_name,
            window.confidence,
            sanitize_tsv(&window.reason),
            window.evidence.executed_bytes,
            window.evidence.verified_bytes,
            window.evidence.static_reachable_labels,
            window.evidence.known_opcode_labels,
            window.evidence.data_or_unknown_labels,
            window.evidence.reachable_in_edges,
            window.decode.start_skip,
            window.decode.score,
            window.decode.valid_bytes,
            window.decode.invalid_bytes,
            window.decode.instructions,
            window.decode.branch_count,
            window.decode.local_branch_count,
            window.decode.jsr_count,
            window.decode.jump_count,
            window.decode.return_count,
            window.decode.brk_count,
            window.decode.ppu_apu_mapper_refs,
            window.decode.zero_page_refs,
            window.texture.entropy,
            window.texture.unique_bytes,
            window.texture.longest_run,
            window.texture.pointer_pairs,
            window.texture.pointer_pair_percent,
        )?;
    }
    Ok(())
}

fn write_segments(path: &Path, segments: &[Segment], prg_len: usize) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "prg_start\tprg_end\tbytes\tbank_start\tbank_end\tcpu_start\tcpu_end_exclusive\tclass\tmin_confidence\twindows"
    )?;
    for segment in segments {
        writeln!(
            file,
            "{:05X}\t{:05X}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            segment.start,
            segment.end,
            segment.end - segment.start,
            bank_text(segment.start),
            bank_text(segment.end.saturating_sub(1).min(prg_len.saturating_sub(1))),
            cpu_text(segment.start),
            cpu_text(segment.end),
            segment.class_name,
            segment.min_confidence,
            segment.windows,
        )?;
    }
    Ok(())
}

fn write_annotated(path: &Path, prg: &[u8], windows: &[Window]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    writeln!(
        file,
        "; Experimental one-pass AI-style ROM texture map. Generated from local ROM bytes."
    )?;
    writeln!(
        file,
        "; Classes are guesses unless backed by trace/static/native evidence."
    )?;
    for window in windows {
        writeln!(file)?;
        writeln!(
            file,
            "; window={} prg=${:05X}-${:05X} bank={} cpu={}..{} class={} confidence={} reason={}",
            window.index,
            window.start,
            window.end,
            bank_text(window.start),
            cpu_text(window.start),
            cpu_text(window.end),
            window.class_name,
            window.confidence,
            window.reason
        )?;
        let bytes = &prg[window.start..window.end];
        if is_code_like(&window.class_name) {
            write_window_disasm(
                &mut file,
                prg,
                window.start,
                bytes,
                window.decode.start_skip,
            )?;
        } else {
            write_data_lines(&mut file, window.start, bytes)?;
        }
    }
    Ok(())
}

fn is_code_like(class_name: &str) -> bool {
    matches!(
        class_name,
        "definite_code"
            | "verified_code"
            | "probable_code"
            | "ai_probable_code"
            | "ai_possible_code"
    )
}

fn write_window_disasm(
    file: &mut fs::File,
    prg: &[u8],
    window_start: usize,
    bytes: &[u8],
    start_skip: usize,
) -> io::Result<()> {
    if start_skip > 0 {
        write_data_lines(file, window_start, &bytes[..start_skip])?;
    }
    let mut local = start_skip;
    while local < bytes.len() {
        let prg_offset = window_start + local;
        let opcode = prg[prg_offset];
        let op = disasm::op_info(opcode);
        let mut len = op.len;
        if local + len > bytes.len() {
            len = 1;
        }
        let instruction_bytes = &bytes[local..local + len];
        let byte_text = instruction_bytes
            .iter()
            .map(|value| format!("{value:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        let cpu = cpu_addr(prg_offset);
        if let Some(mnemonic) = op.mnemonic {
            if len == op.len {
                let operand = disasm::format_operand(op, prg, prg_offset, cpu);
                writeln!(
                    file,
                    "  ; ${prg_offset:05X} {}  {byte_text:<8}  {mnemonic:<3} {operand}",
                    cpu_text(prg_offset)
                )?;
            } else {
                writeln!(
                    file,
                    "  ; ${prg_offset:05X} {}  {byte_text:<8}  .db ${opcode:02X} ; truncated instruction",
                    cpu_text(prg_offset)
                )?;
            }
        } else {
            writeln!(
                file,
                "  ; ${prg_offset:05X} {}  {byte_text:<8}  .db ${opcode:02X}",
                cpu_text(prg_offset)
            )?;
        }
        local += len.max(1);
    }
    Ok(())
}

fn write_data_lines(file: &mut fs::File, window_start: usize, bytes: &[u8]) -> io::Result<()> {
    for (line_index, chunk) in bytes.chunks(16).enumerate() {
        let prg_offset = window_start + line_index * 16;
        let byte_text = chunk
            .iter()
            .map(|value| format!("${value:02X}"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            file,
            "  ; ${prg_offset:05X} {}  .db {byte_text}",
            cpu_text(prg_offset)
        )?;
    }
    Ok(())
}

fn write_manifest(
    path: &Path,
    build_dir: &Path,
    sha256_hex: &str,
    rom: &InesRom,
    windows: &[Window],
    segments: &[Segment],
) -> io::Result<()> {
    let mut classes = BTreeMap::<String, u64>::new();
    let mut class_bytes = BTreeMap::<String, u64>::new();
    for window in windows {
        *classes.entry(window.class_name.clone()).or_default() += 1;
        *class_bytes.entry(window.class_name.clone()).or_default() +=
            (window.end - window.start) as u64;
    }
    let header = rom.header();
    let mut file = fs::File::create(path)?;
    writeln!(file, "sha256={sha256_hex}")?;
    writeln!(file, "mapper={}", header.mapper)?;
    writeln!(file, "prg_size={}", header.prg_rom_size)?;
    writeln!(file, "chr_size={}", header.chr_rom_size)?;
    writeln!(file, "window_bytes={WINDOW_BYTES}")?;
    writeln!(file, "window_count={}", windows.len())?;
    writeln!(file, "segment_count={}", segments.len())?;
    writeln!(file, "build_dir={}", build_dir.display())?;
    writeln!(file, "windows=windows.tsv")?;
    writeln!(file, "segments=segments.tsv")?;
    writeln!(file, "annotated=annotated.asm")?;
    for (class_name, count) in classes {
        writeln!(file, "class_windows_{class_name}={count}")?;
    }
    for (class_name, bytes) in class_bytes {
        writeln!(file, "class_bytes_{class_name}={bytes}")?;
    }
    writeln!(
        file,
        "note=experimental texture classifier; trace/static/native evidence wins over local byte-shape guesses"
    )?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn header_columns(header: &str) -> HashMap<String, usize> {
    header
        .split('\t')
        .enumerate()
        .map(|(index, name)| (name.to_string(), index))
        .collect()
}

fn column<'a>(
    fields: &'a [&'a str],
    columns: &HashMap<String, usize>,
    name: &str,
) -> Option<&'a str> {
    fields.get(*columns.get(name)?).copied()
}

fn parse_hex_usize(value: &str) -> Option<usize> {
    usize::from_str_radix(value.trim_start_matches("0x"), 16).ok()
}

fn parse_usize(value: &str) -> Option<usize> {
    value.parse().ok()
}

fn parse_u64(value: &str) -> Option<u64> {
    value.parse().ok()
}

fn bank_text(prg_offset: usize) -> String {
    format!("{:02}", prg_offset / BANK_16K)
}

fn cpu_addr(prg_offset: usize) -> u16 {
    let bank_offset = prg_offset % BANK_16K;
    let bank = prg_offset / BANK_16K;
    if bank == 7 {
        0xC000 + bank_offset as u16
    } else {
        0x8000 + bank_offset as u16
    }
}

fn cpu_text(prg_offset: usize) -> String {
    format!("${:04X}", cpu_addr(prg_offset))
}

fn sanitize_tsv(value: &str) -> String {
    value.replace(['\t', '\n', '\r'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "lotw_tools_ai_rom_map_test_{}_{}",
            std::process::id(),
            nanos
        ))
    }

    fn ines_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 16 + BANK_16K * 2 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 0x40;
        let prg = 16;

        for base in (0..0x80).step_by(16) {
            let off = prg + base;
            bytes[off] = 0xA9;
            bytes[off + 1] = base as u8;
            bytes[off + 2] = 0x85;
            bytes[off + 3] = 0x10;
            bytes[off + 4] = 0xA2;
            bytes[off + 5] = 0x04;
            bytes[off + 6] = 0xCA;
            bytes[off + 7] = 0xD0;
            bytes[off + 8] = 0xFD;
            bytes[off + 9] = 0x8D;
            bytes[off + 10] = 0x00;
            bytes[off + 11] = 0x20;
            bytes[off + 12] = 0x20;
            bytes[off + 13] = 0x00;
            bytes[off + 14] = 0x80;
            bytes[off + 15] = 0x60;
        }

        for pair in 0..64 {
            let value = 0x8000u16 + pair as u16 * 4;
            let off = prg + 0x100 + pair * 2;
            bytes[off] = value as u8;
            bytes[off + 1] = (value >> 8) as u8;
        }
        bytes[prg + 0x200..prg + 0x280].fill(0xFF);

        let vectors = prg + BANK_16K * 2 - 6;
        bytes[vectors] = 0x00;
        bytes[vectors + 1] = 0xC0;
        bytes[vectors + 2] = 0x00;
        bytes[vectors + 3] = 0xC0;
        bytes[vectors + 4] = 0x00;
        bytes[vectors + 5] = 0xC0;
        bytes
    }

    #[test]
    fn writes_experimental_windows_and_segments() {
        let root = temp_dir();
        let build = root.join("build");
        let out = root.join("out");
        fs::create_dir_all(build.join("block_translation_plan")).unwrap();
        fs::write(
            build.join("block_translation_plan/block_translation_plan.tsv"),
            "replay\tid\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstop_reason\tstatus\thit_count\tsteps\twrites\tppu_writes\tapu_writes\tmapper_writes\tstate_applied\tfinal_ram_sha256\tclass\tpriority\n\
fixture\t0\t8000\t00000\t128\tA9\twindow\tleft_block\t1\t1\t0\t0\t0\t0\t1\t00\tstraight_line\t1\n",
        )
        .unwrap();
        let rom = root.join("fixture.nes");
        fs::write(&rom, ines_fixture()).unwrap();

        run(&rom, &build, &out, None).unwrap();

        let windows = fs::read_to_string(out.join("windows.tsv")).unwrap();
        assert!(windows.contains("definite_code"));
        assert!(windows.contains("probable_pointer_table"));
        assert!(windows.contains("padding_or_fill"));
        let manifest = fs::read_to_string(out.join("manifest.txt")).unwrap();
        assert!(manifest.contains("complete=1\n"));
        assert!(out.join("annotated.asm").is_file());

        fs::remove_dir_all(root).unwrap();
    }
}
