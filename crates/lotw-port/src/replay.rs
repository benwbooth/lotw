use std::fmt;

pub const BUTTON_UP: u16 = 1 << 0;
pub const BUTTON_DOWN: u16 = 1 << 1;
pub const BUTTON_LEFT: u16 = 1 << 2;
pub const BUTTON_RIGHT: u16 = 1 << 3;
pub const BUTTON_A: u16 = 1 << 4;
pub const BUTTON_B: u16 = 1 << 5;
pub const BUTTON_START: u16 = 1 << 6;
pub const BUTTON_SELECT: u16 = 1 << 7;

const BUTTON_ORDER: &[(u16, &str)] = &[
    (BUTTON_UP, "up"),
    (BUTTON_DOWN, "down"),
    (BUTTON_LEFT, "left"),
    (BUTTON_RIGHT, "right"),
    (BUTTON_A, "A"),
    (BUTTON_B, "B"),
    (BUTTON_START, "start"),
    (BUTTON_SELECT, "select"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    frames: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    UnknownDirective { line: usize },
    MissingFrameCount { line: usize },
    InvalidFrameCount { line: usize },
    UnknownButton { line: usize, name: String },
    FrameCountOverflow { line: usize },
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownDirective { line } => write!(f, "line {line}: unknown directive"),
            Self::MissingFrameCount { line } => write!(f, "line {line}: missing frame count"),
            Self::InvalidFrameCount { line } => write!(f, "line {line}: invalid frame count"),
            Self::UnknownButton { line, name } => write!(f, "line {line}: unknown button: {name}"),
            Self::FrameCountOverflow { line } => {
                write!(f, "line {line}: replay frame count overflow")
            }
        }
    }
}

impl std::error::Error for ReplayError {}

impl Replay {
    pub fn parse(text: &str) -> Result<Self, ReplayError> {
        let mut frames = Vec::new();

        for (line_index, raw_line) in text.lines().enumerate() {
            let line_no = line_index + 1;
            let line = raw_line
                .split_once('#')
                .map(|(left, _)| left)
                .unwrap_or(raw_line)
                .trim();
            if line.is_empty() {
                continue;
            }

            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("frame") => {}
                _ => return Err(ReplayError::UnknownDirective { line: line_no }),
            }

            let count = tokens
                .next()
                .ok_or(ReplayError::MissingFrameCount { line: line_no })?
                .parse::<usize>()
                .map_err(|_| ReplayError::InvalidFrameCount { line: line_no })?;
            if count == 0 {
                return Err(ReplayError::InvalidFrameCount { line: line_no });
            }

            let mut mask = 0u16;
            for token in tokens {
                let button = parse_button(token).ok_or_else(|| ReplayError::UnknownButton {
                    line: line_no,
                    name: token.to_string(),
                })?;
                mask |= button;
            }

            let new_len = frames
                .len()
                .checked_add(count)
                .ok_or(ReplayError::FrameCountOverflow { line: line_no })?;
            frames.resize(new_len, mask);
        }

        Ok(Self { frames })
    }

    pub fn frames(&self) -> &[u16] {
        &self.frames
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn buttons_at(&self, frame_index: usize) -> u16 {
        self.frames.get(frame_index).copied().unwrap_or(0)
    }

    pub fn stats(&self) -> ReplayStats {
        let mut pressed_frame_count = 0usize;
        let mut first_pressed_frame = 0usize;
        let mut last_pressed_frame = 0usize;

        for (index, mask) in self.frames.iter().enumerate() {
            if *mask != 0 {
                pressed_frame_count += 1;
                if first_pressed_frame == 0 {
                    first_pressed_frame = index + 1;
                }
                last_pressed_frame = index + 1;
            }
        }

        ReplayStats {
            frame_count: self.frame_count(),
            pressed_frame_count,
            first_pressed_frame,
            last_pressed_frame,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayStats {
    pub frame_count: usize,
    pub pressed_frame_count: usize,
    pub first_pressed_frame: usize,
    pub last_pressed_frame: usize,
}

pub fn parse_button(name: &str) -> Option<u16> {
    BUTTON_ORDER
        .iter()
        .find_map(|(mask, button_name)| (*button_name == name).then_some(*mask))
}

pub fn format_buttons(mask: u16) -> String {
    BUTTON_ORDER
        .iter()
        .filter_map(|(button, name)| (mask & button != 0).then_some(*name))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn input_trace_tsv(replay: Option<&Replay>, frame_limit: usize) -> String {
    let mut out = String::from("frame\tmask\tbuttons\n");
    for frame in 0..frame_limit {
        let mask = replay.map(|replay| replay.buttons_at(frame)).unwrap_or(0);
        out.push_str(&format!(
            "{}\t{mask:04X}\t{}\n",
            frame + 1,
            format_buttons(mask)
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_replay_frames_and_stats() {
        let replay = Replay::parse(
            "\
# comment
frame 2
frame 3 start right
frame 1 A # inline comment
",
        )
        .unwrap();

        assert_eq!(
            replay.frames(),
            &[
                0,
                0,
                BUTTON_START | BUTTON_RIGHT,
                BUTTON_START | BUTTON_RIGHT,
                BUTTON_START | BUTTON_RIGHT,
                BUTTON_A,
            ]
        );
        assert_eq!(
            replay.stats(),
            ReplayStats {
                frame_count: 6,
                pressed_frame_count: 4,
                first_pressed_frame: 3,
                last_pressed_frame: 6,
            }
        );
    }

    #[test]
    fn formats_buttons_in_c_parser_order() {
        assert_eq!(
            format_buttons(BUTTON_START | BUTTON_RIGHT | BUTTON_A),
            "right A start"
        );
        assert_eq!(format_buttons(0), "");
    }

    #[test]
    fn rejects_bad_button() {
        assert!(matches!(
            Replay::parse("frame 1 nope"),
            Err(ReplayError::UnknownButton { line: 1, .. })
        ));
    }

    #[test]
    fn writes_input_trace_tsv_with_blank_zero_button_names() {
        let replay = Replay::parse("frame 2\nframe 1 start\n").unwrap();

        assert_eq!(
            input_trace_tsv(Some(&replay), 4),
            "frame\tmask\tbuttons\n1\t0000\t\n2\t0000\t\n3\t0040\tstart\n4\t0000\t\n"
        );
        assert_eq!(
            input_trace_tsv(None, 1),
            "frame\tmask\tbuttons\n1\t0000\t\n"
        );
    }
}
