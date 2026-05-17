use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const REQUIRED_TOP_LEVEL: &[&str] = &["version", "evidence_sources", "functions", "ram"];

const FUNCTION_REQUIRED_FIELDS: &[&str] = &[
    "label",
    "cpu_addr",
    "prg_offset",
    "proposed_name",
    "role",
    "confidence",
    "evidence",
    "callers",
    "reads",
    "writes",
    "trace_contexts",
    "constants",
    "notes",
];

const RAM_REQUIRED_FIELDS: &[&str] = &[
    "addr",
    "proposed_name",
    "width",
    "confidence",
    "evidence",
    "readers",
    "writers",
    "trace_contexts",
    "constants",
    "notes",
];

#[derive(Debug, Default, Clone, Copy)]
struct AuditReport {
    functions: usize,
    ram: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Other,
    Functions,
    Ram,
}

impl Section {
    fn name(self) -> &'static str {
        match self {
            Section::Other => "other",
            Section::Functions => "functions",
            Section::Ram => "ram",
        }
    }

    fn required_fields(self) -> &'static [&'static str] {
        match self {
            Section::Functions => FUNCTION_REQUIRED_FIELDS,
            Section::Ram => RAM_REQUIRED_FIELDS,
            Section::Other => &[],
        }
    }
}

#[derive(Debug, Clone)]
struct Entry {
    section: Section,
    start_line: usize,
    fields: BTreeSet<String>,
    proposed_name: Option<String>,
    confidence: Option<f64>,
    evidence_items: usize,
    active_field: Option<String>,
}

impl Entry {
    fn new(section: Section, start_line: usize) -> Self {
        Self {
            section,
            start_line,
            fields: BTreeSet::new(),
            proposed_name: None,
            confidence: None,
            evidence_items: 0,
            active_field: None,
        }
    }
}

pub fn run(symbols_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(symbols_path)?;
    let report = audit_text(&text).map_err(|err| {
        format!(
            "symbol_audit: {} failed validation: {err}",
            symbols_path.display()
        )
    })?;
    println!(
        "symbol_audit: validated {} functions and {} RAM entries in {}",
        report.functions,
        report.ram,
        symbols_path.display()
    );
    Ok(())
}

fn audit_text(text: &str) -> Result<AuditReport, String> {
    let mut top_level_keys = BTreeSet::new();
    let mut section = Section::Other;
    let mut current_entry = None;
    let mut report = AuditReport::default();

    for (index, line) in text.lines().enumerate() {
        let line_no = index + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if !line.starts_with(' ') {
            finish_entry(current_entry.take(), &mut report)?;
            if let Some((key, _value)) = parse_key_value(trimmed) {
                top_level_keys.insert(key.to_string());
                section = match key {
                    "functions" => Section::Functions,
                    "ram" => Section::Ram,
                    _ => Section::Other,
                };
            } else {
                section = Section::Other;
            }
            continue;
        }

        if matches!(section, Section::Functions | Section::Ram) && line.starts_with("  - ") {
            finish_entry(current_entry.take(), &mut report)?;
            let mut entry = Entry::new(section, line_no);
            parse_entry_field(trimmed.trim_start_matches("- "), &mut entry)?;
            current_entry = Some(entry);
            continue;
        }

        let Some(entry) = current_entry.as_mut() else {
            continue;
        };

        if entry.active_field.as_deref() == Some("evidence") && trimmed.starts_with("- source:") {
            entry.evidence_items += 1;
        }

        if line.starts_with("    ") && !line.starts_with("      ") {
            parse_entry_field(trimmed, entry)?;
        }
    }

    finish_entry(current_entry.take(), &mut report)?;

    for key in REQUIRED_TOP_LEVEL {
        if !top_level_keys.contains(*key) {
            return Err(format!("missing top-level key {key:?}"));
        }
    }
    if report.functions == 0 {
        return Err("functions must contain at least one entry".to_string());
    }
    if report.ram == 0 {
        return Err("ram must contain at least one entry".to_string());
    }

    Ok(report)
}

fn parse_entry_field(text: &str, entry: &mut Entry) -> Result<(), String> {
    let Some((key, value)) = parse_key_value(text) else {
        return Ok(());
    };
    entry.fields.insert(key.to_string());
    entry.active_field = Some(key.to_string());
    match key {
        "proposed_name" => {
            let normalized = value.trim().trim_matches('"').trim_matches('\'');
            if !normalized.is_empty() && normalized != "null" {
                entry.proposed_name = Some(normalized.to_string());
            }
        }
        "confidence" => {
            let confidence = value.trim().parse::<f64>().map_err(|err| {
                format!(
                    "{} entry at line {} has invalid confidence {:?}: {err}",
                    entry.section.name(),
                    entry.start_line,
                    value.trim()
                )
            })?;
            entry.confidence = Some(confidence);
        }
        _ => {}
    }
    Ok(())
}

fn finish_entry(entry: Option<Entry>, report: &mut AuditReport) -> Result<(), String> {
    let Some(entry) = entry else {
        return Ok(());
    };
    let required = entry.section.required_fields();
    let missing = required
        .iter()
        .filter(|field| !entry.fields.iter().any(|present| present == **field))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(format!(
            "{} entry at line {} is missing required field(s): {}",
            entry.section.name(),
            entry.start_line,
            missing.join(", ")
        ));
    }

    let confidence = entry.confidence.ok_or_else(|| {
        format!(
            "{} entry at line {} is missing parseable confidence",
            entry.section.name(),
            entry.start_line
        )
    })?;
    if !(0.0..=1.0).contains(&confidence) {
        return Err(format!(
            "{} entry at line {} has confidence outside 0.0..=1.0: {confidence}",
            entry.section.name(),
            entry.start_line
        ));
    }

    if entry.proposed_name.is_some() && entry.evidence_items == 0 {
        return Err(format!(
            "{} entry at line {} proposes a name without any evidence source item",
            entry.section.name(),
            entry.start_line
        ));
    }

    match entry.section {
        Section::Functions => report.functions += 1,
        Section::Ram => report.ram += 1,
        Section::Other => {}
    }
    Ok(())
}

fn parse_key_value(text: &str) -> Option<(&str, &str)> {
    let (key, value) = text.split_once(':')?;
    let key = key.trim();
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((key, value.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_exe::unique_temp_dir;
    use std::fs;

    const VALID_SYMBOLS: &str = r#"
version: 1
evidence_sources:
  remaining_units: build/whole_program_report/whole_program_remaining_units.tsv
functions:
  - label: L_8000
    cpu_addr: "8000"
    prg_offset: "1C000"
    proposed_name: null
    role: replay_covered_frontier
    confidence: 0.10
    evidence:
      - source: build/whole_program_report/whole_program_remaining_units.tsv
        detail: replay_count=1.
    callers: []
    reads: []
    writes: []
    trace_contexts: []
    constants: []
    notes: Needs trace context before naming.
ram:
  - addr: null
    proposed_name: null
    width: null
    confidence: 0.0
    evidence: []
    readers: []
    writers: []
    trace_contexts: []
    constants: []
    notes: Add entries after read/write aggregation exists.
"#;

    #[test]
    fn accepts_evidence_shaped_symbol_database() {
        let report = audit_text(VALID_SYMBOLS).unwrap();
        assert_eq!(report.functions, 1);
        assert_eq!(report.ram, 1);
    }

    #[test]
    fn rejects_missing_required_function_field() {
        let invalid = VALID_SYMBOLS.replace("    trace_contexts: []\n", "");
        let err = audit_text(&invalid).unwrap_err();
        assert!(err.contains("missing required field(s): trace_contexts"));
    }

    #[test]
    fn rejects_accepted_name_without_evidence_item() {
        let invalid = VALID_SYMBOLS
            .replace("    proposed_name: null\n", "    proposed_name: init_room\n")
            .replace(
                "    evidence:\n      - source: build/whole_program_report/whole_program_remaining_units.tsv\n        detail: replay_count=1.\n",
                "    evidence: []\n",
            );
        let err = audit_text(&invalid).unwrap_err();
        assert!(err.contains("proposes a name without any evidence source item"));
    }

    #[test]
    fn run_validates_file() {
        let root = unique_temp_dir("symbol-audit");
        fs::create_dir_all(&root).unwrap();
        let path = root.join("symbols.yaml");
        fs::write(&path, VALID_SYMBOLS).unwrap();

        run(&path).unwrap();

        fs::remove_dir_all(root).unwrap();
    }
}
