//! Compile-time music DSL for Legacy of the Wizard. Each note (`c4q`, `fs5e`)
//! is parsed and validated here and emitted as an exact `lotw::audio::Tok`, so a
//! malformed note is a compile error and the result is byte-exact.
//!
//! Macros:
//!   * `ser![ items ]`                  — a `Vec<Tok>` (a reusable section)
//!   * `pulse1![..] pulse2![..] triangle![..] noise![..]` — a tagged `Channel`
//!   * `song!{ ch, ch, ch, ch }`        — a `Song` from four channels
//!   * `param!(duty=0x0b, volume=0xff)` — command tokens
//!   * `raw!(0x9f, e)`                  — a note with an un-nameable pitch byte
//!
//! An item is a note (`c4q`, `c4 30`), a rest (`rq`, `r 30`), `|`/`end`, or any
//! expression evaluating to something iterable of `Tok` (a section variable,
//! `param!(..)`, `ser![..]`, ...), which is spliced in.

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Spacing, TokenStream as TS, TokenTree};
use quote::quote;

/// note_idx -> name (idx 5 is the unused gap; idx 0..=12 = C..B).
const NOTE_NAMES: [&str; 13] = ["c", "cs", "d", "ds", "e", "", "f", "fs", "g", "gs", "a", "as", "b"];
const DURS: &[(&str, u8)] = &[
    ("w", 96), ("hdd", 84), ("hd", 72), ("h", 48), ("qdd", 42), ("qd", 36),
    ("q", 24), ("edd", 21), ("ed", 18), ("e", 12), ("id", 9), ("i", 6), ("t", 3),
];
const BASE_OCTAVE: i32 = 2;

// --- entry points ---

#[proc_macro]
pub fn ser(input: TokenStream) -> TokenStream {
    build(input.into()).unwrap_or_else(syn_err).into()
}

#[proc_macro]
pub fn param(input: TokenStream) -> TokenStream {
    build_params(input.into()).unwrap_or_else(syn_err).into()
}

#[proc_macro]
pub fn raw(input: TokenStream) -> TokenStream {
    build_raw(input.into()).unwrap_or_else(syn_err).into()
}

macro_rules! channel_macro {
    ($name:ident, $variant:ident) => {
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            match build(input.into()) {
                Ok(stream) => quote! {
                    ::lotw::audio::Channel { id: ::lotw::audio::ChannelId::$variant, stream: #stream }
                }
                .into(),
                Err(e) => syn_err(e).into(),
            }
        }
    };
}
channel_macro!(pulse1, Pulse1);
channel_macro!(pulse2, Pulse2);
channel_macro!(triangle, Triangle);
channel_macro!(noise, Noise);

#[proc_macro]
pub fn song(input: TokenStream) -> TokenStream {
    let chans: Vec<TS> = split_items(input.into()).into_iter().map(|i| i.into_iter().collect()).collect();
    quote! { ::lotw::audio::Song::from_channels(::std::vec![ #(#chans),* ]) }.into()
}

// --- core: build a Vec<Tok> from a list of items ---

fn build(input: TS) -> Result<TS, String> {
    let mut body = TS::new();
    for item in split_items(input) {
        body.extend(emit_item(&item)?);
    }
    Ok(quote! {{
        let mut __s: ::std::vec::Vec<::lotw::audio::Tok> = ::std::vec::Vec::new();
        #body
        __s
    }})
}

fn emit_item(item: &[TokenTree]) -> Result<TS, String> {
    // `|` -> end of stream.
    if let [TokenTree::Punct(p)] = item {
        if p.as_char() == '|' {
            return Ok(quote! { __s.push(::lotw::audio::Tok::End); });
        }
    }
    match item {
        // A lone ident: a clean note/rest, or a section variable to splice.
        [TokenTree::Ident(id)] => {
            let s = id.to_string();
            if s == "end" {
                return Ok(quote! { __s.push(::lotw::audio::Tok::End); });
            }
            if let Some(tok) = parse_event(&s) {
                return Ok(push_event(tok));
            }
            // Otherwise it's a section variable: splice its tokens.
            Ok(splice(&item.iter().cloned().collect::<TS>()))
        }
        // `note <dur>` / `r <dur>` where <dur> is `30` (ticks), `q~i` (tied
        // letters), or a spaced single letter. Only when the head is a note/`r`
        // and the suffix parses as a duration; otherwise it's an expression.
        [TokenTree::Ident(id), rest @ ..] if !rest.is_empty() && parse_dur_tokens(rest).is_some() => {
            let name = id.to_string();
            let dur = parse_dur_tokens(rest).unwrap();
            if dur & 0x80 != 0 {
                return Err(format!("duration {dur} >= 128"));
            }
            if name == "r" {
                Ok(push_event(Event::Rest(dur)))
            } else if let Some(pitch) = pitch_byte(&name) {
                Ok(push_event(Event::Note(dur, pitch)))
            } else {
                // Head isn't a note (e.g. `intro.clone()` got here by accident) — splice.
                Ok(splice(&item.iter().cloned().collect::<TS>()))
            }
        }
        // Anything else is an expression (param!(..), ser![..], a call) -> splice.
        _ => Ok(splice(&item.iter().cloned().collect::<TS>())),
    }
}

fn splice(expr: &TS) -> TS {
    quote! { __s.extend((#expr).iter().copied()); }
}

enum Event {
    Note(u8, u8),
    Rest(u8),
}

fn push_event(e: Event) -> TS {
    match e {
        Event::Note(dur, pitch) => quote! { __s.push(::lotw::audio::Tok::Note { dur: #dur, pitch: #pitch }); },
        Event::Rest(dur) => quote! { __s.push(::lotw::audio::Tok::Rest { dur: #dur }); },
    }
}

/// Parse a single joined event ident: `c4q`, `fs5e`, `rhd`, `rq`.
fn parse_event(s: &str) -> Option<Event> {
    if let Some(rest) = s.strip_prefix('r') {
        // Could be a rest (`rq`) — but only if the remainder is a duration.
        if let Some(d) = dur_value(rest) {
            return Some(Event::Rest(d));
        }
    }
    let (idx, rest) = split_name(s)?;
    let split = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    let (oct_s, dur_s) = rest.split_at(split);
    let octave: i32 = oct_s.parse().ok()?;
    let nibble = octave - BASE_OCTAVE;
    if !(0..=15).contains(&nibble) {
        return None;
    }
    let dur = dur_value(dur_s)?;
    Some(Event::Note(dur, ((nibble as u8) << 4) | idx))
}

/// Pitch byte for a bare `name+octave` (no duration).
fn pitch_byte(s: &str) -> Option<u8> {
    let (idx, oct_s) = split_name(s)?;
    let octave: i32 = oct_s.parse().ok()?;
    let nibble = octave - BASE_OCTAVE;
    (0..=15).contains(&nibble).then(|| ((nibble as u8) << 4) | idx)
}

fn split_name(tok: &str) -> Option<(u8, &str)> {
    if tok.is_empty() || !tok.as_bytes()[0].is_ascii_alphabetic() {
        return None;
    }
    let (name, rest) = match tok.get(..2).filter(|s| NOTE_NAMES.contains(s)) {
        Some(n) => (n, &tok[2..]),
        None => (&tok[..1], &tok[1..]),
    };
    let idx = NOTE_NAMES.iter().position(|n| *n == name && !n.is_empty())?;
    Some((idx as u8, rest))
}

fn dur_value(s: &str) -> Option<u8> {
    DURS.iter().find(|(k, _)| *k == s).map(|(_, t)| *t)
}

/// Parse a duration written as separate tokens: `[Literal(30)]` (ticks), or a
/// `~`-tied chain of letters (`q ~ i`), or a single spaced letter (`q`).
fn parse_dur_tokens(toks: &[TokenTree]) -> Option<u8> {
    if let [TokenTree::Literal(l)] = toks {
        return parse_u8(&l.to_string());
    }
    let mut sum: u16 = 0;
    let mut expect_dur = true;
    for tt in toks {
        match tt {
            TokenTree::Ident(id) if expect_dur => {
                sum += dur_value(&id.to_string())? as u16;
                expect_dur = false;
            }
            TokenTree::Punct(p) if !expect_dur && p.as_char() == '~' => expect_dur = true,
            _ => return None,
        }
    }
    if expect_dur {
        return None; // empty or trailing `~`
    }
    (sum <= 0xFF).then_some(sum as u8)
}

fn parse_u8(s: &str) -> Option<u8> {
    let v = if let Some(h) = s.strip_prefix("0x") {
        u16::from_str_radix(h, 16).ok()?
    } else {
        s.parse::<u16>().ok()?
    };
    (v <= 0xFF).then_some(v as u8)
}

// --- param! and raw! ---

fn build_params(input: TS) -> Result<TS, String> {
    let names = ["duty", "volume", "flags", "pitch", "sweep"];
    let mut pushes = TS::new();
    for item in split_items(input) {
        // ident = value
        let s: Vec<TokenTree> = item;
        let (key, val) = parse_assign(&s)?;
        let id = names.iter().position(|n| *n == key).map(|i| i as u8).or_else(|| key.strip_prefix("cmd").and_then(|d| d.parse().ok())).ok_or_else(|| format!("unknown command {key:?}"))?;
        let arg = parse_u8(&val).ok_or_else(|| format!("bad command arg {val:?}"))?;
        pushes.extend(quote! { __s.push(::lotw::audio::Tok::Cmd { id: #id, arg: #arg }); });
    }
    Ok(quote! {{
        let mut __s: ::std::vec::Vec<::lotw::audio::Tok> = ::std::vec::Vec::new();
        #pushes
        __s
    }})
}

fn build_raw(input: TS) -> Result<TS, String> {
    // raw(pitch, dur)
    let toks: Vec<TokenTree> = input.into_iter().collect();
    let parts: Vec<String> = split_on_comma(&toks).iter().map(|p| p.iter().map(token_text).collect()).collect();
    let [pitch, dur] = parts.as_slice() else {
        return Err("raw! expects (pitch, dur)".into());
    };
    let pitch = parse_u8(pitch).ok_or_else(|| format!("bad raw pitch {pitch:?}"))?;
    let dur = dur_value(dur).or_else(|| parse_u8(dur)).ok_or_else(|| format!("bad raw dur {dur:?}"))?;
    if dur & 0x80 != 0 {
        return Err(format!("duration {dur} >= 128"));
    }
    Ok(quote! { ::std::vec![::lotw::audio::Tok::Note { dur: #dur, pitch: #pitch }] })
}

fn parse_assign(item: &[TokenTree]) -> Result<(String, String), String> {
    // ident = value (value may be a literal or `0x..`).
    let key = match item.first() {
        Some(TokenTree::Ident(id)) => id.to_string(),
        _ => return Err("expected `name = value`".into()),
    };
    let eq = matches!(item.get(1), Some(TokenTree::Punct(p)) if p.as_char() == '=');
    if !eq {
        return Err(format!("expected `=` after {key}"));
    }
    let val: String = item[2..].iter().map(token_text).collect();
    Ok((key, val))
}

// --- token-stream helpers ---

/// Split a token stream into items separated by top-level `,`.
fn split_items(ts: TS) -> Vec<Vec<TokenTree>> {
    let mut items = Vec::new();
    let mut cur = Vec::new();
    for tt in ts {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == ',' && p.spacing() == Spacing::Alone => {
                if !cur.is_empty() {
                    items.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(tt),
        }
    }
    if !cur.is_empty() {
        items.push(cur);
    }
    items
}

fn split_on_comma(item: &[TokenTree]) -> Vec<Vec<TokenTree>> {
    let mut out = Vec::new();
    let mut cur = Vec::new();
    for tt in item {
        match tt {
            TokenTree::Punct(p) if p.as_char() == ',' => {
                out.push(std::mem::take(&mut cur));
            }
            _ => cur.push(tt.clone()),
        }
    }
    out.push(cur);
    out
}

fn token_text(tt: &TokenTree) -> String {
    match tt {
        TokenTree::Group(g) if g.delimiter() == Delimiter::None => g.stream().into_iter().map(|t| token_text(&t)).collect(),
        other => other.to_string(),
    }
}

fn syn_err(msg: String) -> TS {
    quote! { compile_error!(#msg) }
}
