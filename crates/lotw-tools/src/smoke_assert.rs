use std::fs;
use std::path::{Path, PathBuf};

pub fn run(spec_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read(spec_path)?;
    let fields = split_nul_fields(&bytes)?;
    let mut index = 0usize;
    let mut checked = 0usize;

    while index < fields.len() {
        let kind = &fields[index];
        index += 1;
        match kind.as_str() {
            "line" => {
                let path = take_field(&fields, &mut index, kind)?;
                let expected = take_field(&fields, &mut index, kind)?;
                assert_line(Path::new(path), expected)?;
                checked += 1;
            }
            "prefix" => {
                let path = take_field(&fields, &mut index, kind)?;
                let expected = take_field(&fields, &mut index, kind)?;
                assert_prefix(Path::new(path), expected)?;
                checked += 1;
            }
            "prefix-any" => {
                let path = take_field(&fields, &mut index, kind)?;
                let count = take_field(&fields, &mut index, kind)?
                    .parse::<usize>()
                    .map_err(|err| format!("smoke_assert: invalid prefix-any count: {err}"))?;
                let mut prefixes = Vec::with_capacity(count);
                for _ in 0..count {
                    prefixes.push(take_field(&fields, &mut index, kind)?.to_string());
                }
                assert_prefix_any(Path::new(path), &prefixes)?;
                checked += 1;
            }
            "contains" => {
                let path = take_field(&fields, &mut index, kind)?;
                let expected = take_field(&fields, &mut index, kind)?;
                assert_contains(Path::new(path), expected)?;
                checked += 1;
            }
            "absent" => {
                let path = take_field(&fields, &mut index, kind)?;
                let forbidden = take_field(&fields, &mut index, kind)?;
                assert_absent(Path::new(path), forbidden)?;
                checked += 1;
            }
            "bytes-eq" => {
                let expected = PathBuf::from(take_field(&fields, &mut index, kind)?);
                let actual = PathBuf::from(take_field(&fields, &mut index, kind)?);
                assert_bytes_eq(&expected, &actual)?;
                checked += 1;
            }
            other => {
                return Err(format!("smoke_assert: unknown assertion kind: {other}").into());
            }
        }
    }

    println!("smoke_assert: checked={checked}");
    Ok(())
}

fn split_nul_fields(bytes: &[u8]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    if bytes.last() != Some(&0) {
        return Err("smoke_assert: assertion file must end with a NUL byte".into());
    }
    bytes[..bytes.len() - 1]
        .split(|byte| *byte == 0)
        .map(|field| {
            String::from_utf8(field.to_vec()).map_err(|err| {
                format!("smoke_assert: assertion field was not valid UTF-8: {err}").into()
            })
        })
        .collect()
}

fn take_field<'a>(
    fields: &'a [String],
    index: &mut usize,
    kind: &str,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    let field = fields
        .get(*index)
        .ok_or_else(|| format!("smoke_assert: truncated {kind} assertion"))?;
    *index += 1;
    Ok(field)
}

fn assert_line(path: &Path, expected: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if text.lines().any(|line| line == expected) {
        return Ok(());
    }
    Err(format!(
        "smoke_assert: {} did not contain expected line: {expected}",
        path.display()
    )
    .into())
}

fn assert_prefix(path: &Path, expected: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if text.lines().any(|line| line.starts_with(expected)) {
        return Ok(());
    }
    Err(format!(
        "smoke_assert: {} did not contain line prefix: {expected}",
        path.display()
    )
    .into())
}

fn assert_prefix_any(path: &Path, prefixes: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if text
        .lines()
        .any(|line| prefixes.iter().any(|prefix| line.starts_with(prefix)))
    {
        return Ok(());
    }
    Err(format!(
        "smoke_assert: {} did not contain any accepted line prefix: {}",
        path.display(),
        prefixes.join(" | ")
    )
    .into())
}

fn assert_contains(path: &Path, expected: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if text.contains(expected) {
        return Ok(());
    }
    Err(format!(
        "smoke_assert: {} did not contain expected text: {expected}",
        path.display()
    )
    .into())
}

fn assert_absent(path: &Path, forbidden: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if !text.contains(forbidden) {
        return Ok(());
    }
    Err(format!(
        "smoke_assert: {} contained forbidden text: {forbidden}",
        path.display()
    )
    .into())
}

fn assert_bytes_eq(expected: &Path, actual: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let expected_bytes = fs::read(expected)?;
    let actual_bytes = fs::read(actual)?;
    if expected_bytes == actual_bytes {
        return Ok(());
    }
    Err(format!(
        "smoke_assert: byte mismatch between {} and {}",
        expected.display(),
        actual.display()
    )
    .into())
}
