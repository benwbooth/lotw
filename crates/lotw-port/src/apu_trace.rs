use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApuTraceEvent {
    pub frame: u32,
    pub cycle: u64,
    pub cycle_known: bool,
    pub address: u16,
    pub value: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegisterStats {
    pub count: u32,
    pub first_frame: u32,
    pub last_frame: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApuTraceError {
    Empty,
    BadHeader { actual: String },
    BadTsv { line: usize, message: String },
}

impl fmt::Display for ApuTraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty APU trace"),
            Self::BadHeader { actual } => write!(f, "bad APU trace header: {actual}"),
            Self::BadTsv { line, message } => write!(f, "APU trace line {line}: {message}"),
        }
    }
}

impl std::error::Error for ApuTraceError {}

pub fn parse_apu_writes_tsv(text: &str) -> Result<Vec<ApuTraceEvent>, ApuTraceError> {
    let mut lines = text.lines();
    let header = lines.next().ok_or(ApuTraceError::Empty)?;
    if header != "frame\tcycle\taddr\tvalue" {
        return Err(ApuTraceError::BadHeader {
            actual: header.to_string(),
        });
    }

    let mut events = Vec::new();
    for (line_index, line) in lines.enumerate() {
        let line_no = line_index + 2;
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 {
            return Err(ApuTraceError::BadTsv {
                line: line_no,
                message: format!("expected 4 TSV fields, got {}", fields.len()),
            });
        }
        let address = parse_hex_u16(line_no, "addr", fields[2])?;
        if !is_audio_register(address) {
            continue;
        }
        let (cycle, cycle_known) = if fields[1] == "unknown" {
            (0, false)
        } else {
            (parse_dec_u64(line_no, "cycle", fields[1])?, true)
        };
        events.push(ApuTraceEvent {
            frame: parse_dec_u32(line_no, "frame", fields[0])?,
            cycle,
            cycle_known,
            address,
            value: parse_hex_u8(line_no, "value", fields[3])?,
        });
    }
    Ok(events)
}

pub fn collect_register_stats(events: &[ApuTraceEvent]) -> [RegisterStats; 0x18] {
    let mut stats: [RegisterStats; 0x18] = std::array::from_fn(|_| RegisterStats::default());
    for event in events {
        let stat = &mut stats[(event.address - 0x4000) as usize];
        if stat.count == 0 {
            stat.first_frame = event.frame;
        }
        stat.last_frame = event.frame;
        stat.count += 1;
    }
    stats
}

pub fn fallback_cycle_for_frame(frame: u32) -> u64 {
    (0..frame)
        .map(|index| if index & 1 == 0 { 29_780 } else { 29_781 })
        .sum()
}

pub fn is_audio_register(address: u16) -> bool {
    (0x4000..=0x4013).contains(&address) || matches!(address, 0x4015 | 0x4017)
}

fn parse_dec_u32(line: usize, field: &str, value: &str) -> Result<u32, ApuTraceError> {
    value.parse::<u32>().map_err(|err| ApuTraceError::BadTsv {
        line,
        message: format!("invalid {field} decimal value {value}: {err}"),
    })
}

fn parse_dec_u64(line: usize, field: &str, value: &str) -> Result<u64, ApuTraceError> {
    value.parse::<u64>().map_err(|err| ApuTraceError::BadTsv {
        line,
        message: format!("invalid {field} decimal value {value}: {err}"),
    })
}

fn parse_hex_u16(line: usize, field: &str, value: &str) -> Result<u16, ApuTraceError> {
    u16::from_str_radix(value, 16).map_err(|err| ApuTraceError::BadTsv {
        line,
        message: format!("invalid {field} hex value {value}: {err}"),
    })
}

fn parse_hex_u8(line: usize, field: &str, value: &str) -> Result<u8, ApuTraceError> {
    u8::from_str_radix(value, 16).map_err(|err| ApuTraceError::BadTsv {
        line,
        message: format!("invalid {field} hex value {value}: {err}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_apu_writes_and_filters_non_audio_registers() {
        let events = parse_apu_writes_tsv(concat!(
            "frame\tcycle\taddr\tvalue\n",
            "1\t10\t4000\t30\n",
            "2\t11\t4014\t02\n",
            "3\tunknown\t4015\t0F\n",
            "4\t12\t4017\t40\n",
        ))
        .unwrap();

        assert_eq!(events.len(), 3);
        assert_eq!(events[0].address, 0x4000);
        assert_eq!(events[0].cycle, 10);
        assert!(events[0].cycle_known);
        assert_eq!(events[1].address, 0x4015);
        assert_eq!(events[1].cycle, 0);
        assert!(!events[1].cycle_known);
        assert_eq!(events[2].address, 0x4017);

        let stats = collect_register_stats(&events);
        assert_eq!(stats[0].count, 1);
        assert_eq!(stats[0x15].first_frame, 3);
        assert_eq!(stats[0x17].last_frame, 4);
    }

    #[test]
    fn computes_c_fallback_cycles() {
        assert_eq!(fallback_cycle_for_frame(0), 0);
        assert_eq!(fallback_cycle_for_frame(1), 29_780);
        assert_eq!(fallback_cycle_for_frame(2), 59_561);
        assert_eq!(fallback_cycle_for_frame(3), 89_341);
    }
}
