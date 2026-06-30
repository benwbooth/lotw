//! Proc macros for the LotW music DSL: `note_consts!` generates the documented
//! pitch×value note grid (rests, hits), and `env!`/`duty!`/… build parameter
//! envelopes.
//!
//! The `env!` parameter-envelope macro:
//!
//! `env!(param, <value> <note>…, <value> <note>…, …)` — each segment sets a
//! channel parameter, then plays one or more carrier notes, expanding to a
//! `Note::Seq` of `param(v), note…` pairs:
//!
//! ```text
//! env!(volume, 0 g4x, 252 fs4x, 250 f4x)        // per-note volume decay
//! env!(volume, 0 b5x as5x b5x a5x, 255 as5x …)  // value held across notes
//! env!(pitch,  8 e3x, +8 e3x, +8 e3x, 0 f3x)    // relative / absolute bend
//! ```
//!
//! A value is absolute (`252`), relative (`+8` / `-2` from the running value),
//! or a bare `+` / `-` to step by one. The running value is resolved at compile
//! time, so the macro emits a `const` `Note::Seq`.

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TS, TokenTree};
use quote::{format_ident, quote};

/// Parameter-envelope: `env!(param, <value> <note>…, <value> <note>…, …)`.
/// Each segment sets channel `param` (`volume`/`duty`/`pitch`/…) then plays its
/// carrier notes, so the value changes per segment. Values are absolute (`252`),
/// relative (`+8`/`-2`), or a bare `+`/`-` step. Expands to a `const Note::Seq`.
/// The per-parameter shorthands (`volume!`, `duty!`, …) drop the first arg.
#[proc_macro]
pub fn env(input: TokenStream) -> TokenStream {
    emit(build(input.into(), None))
}

// `duty!(<seg>, …)` / `volume!(…)` / … — the parameter is the macro name. These
// don't clash with the `duty(arg)`/`volume(arg)` note functions (macro vs value
// namespace).
macro_rules! param_macro {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        ///
        /// Shorthand for `env!` with this parameter fixed:
        /// `volume!(0 g4x, 252 fs4x)` ≡ `env!(volume, 0 g4x, 252 fs4x)`.
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            emit(build(input.into(), Some(stringify!($name))))
        }
    };
}
param_macro!(duty, "Duty/instrument envelope: step the pulse duty cycle and envelope instrument across a run of notes (the envelope form of the `duty` command).");
param_macro!(volume, "Volume envelope: step the channel volume across notes — for fades and accents (the envelope form of the `volume` command).");
param_macro!(flags, "Channel-flags envelope: step the raw duty/volume-flag byte across notes (the envelope form of the `flags` command).");
param_macro!(pitch, "Pitch-bend envelope: step the pitch offset across notes; supports relative `+`/`-` steps (the envelope form of the `pitch` command).");
param_macro!(sweep, "Sweep / drum-period envelope: step the sweep (pulse) or drum period (noise) across notes (the envelope form of the `sweep` command).");

fn emit(r: Result<TS, String>) -> TokenStream {
    match r {
        Ok(ts) => ts.into(),
        Err(e) => quote! { compile_error!(#e) }.into(),
    }
}

/// Build the envelope `Note::Seq`. `fixed` = the parameter when it's the macro
/// name (`volume!`…); `None` means the first argument is the parameter (`env!`).
fn build(input: TS, fixed: Option<&str>) -> Result<TS, String> {
    let toks: Vec<TokenTree> = input.into_iter().collect();
    let mut groups = split_commas(&toks);

    let param = match fixed {
        Some(p) => p.to_string(),
        None => {
            if groups.is_empty() {
                return Err("env! expects a parameter name first".into());
            }
            match groups.remove(0).into_iter().next() {
                Some(TokenTree::Ident(id)) => id.to_string(),
                _ => return Err("env! expects a parameter name first".into()),
            }
        }
    };
    let pident = format_ident!("{}", param);

    let mut elems: Vec<TS> = Vec::new();
    let mut running: i64 = 0;
    let mut any = false;
    for seg in groups {
        if seg.is_empty() {
            continue;
        }
        any = true;
        let (val, notes) = parse_segment(&seg, &mut running)?;
        let vlit = Literal::u8_unsuffixed(val);
        elems.push(quote! { ::lotw_music::note::#pident(#vlit) });
        for n in notes {
            let nid = format_ident!("{}", n);
            elems.push(quote! { #nid });
        }
    }
    if !any {
        return Err("envelope needs at least one segment".into());
    }
    Ok(quote! { ::lotw_music::Note::Seq(const { &[ #(#elems),* ] }) })
}

/// Parse a `<value> <note>…` segment, advancing the running parameter value.
fn parse_segment(seg: &[TokenTree], running: &mut i64) -> Result<(u8, Vec<String>), String> {
    let lit = |t: &TokenTree| -> Option<i64> {
        match t {
            TokenTree::Literal(l) => l.to_string().parse().ok(),
            _ => None,
        }
    };
    let notes_from = |from: usize| -> Result<Vec<String>, String> {
        let notes: Vec<String> = seg[from..]
            .iter()
            .map(|t| match t {
                TokenTree::Ident(id) => Ok(id.to_string()),
                other => Err(format!("env! carrier must be a note name, found `{other}`")),
            })
            .collect::<Result<_, _>>()?;
        if notes.is_empty() {
            return Err("env! segment needs at least one carrier note".into());
        }
        Ok(notes)
    };

    let notes = match &seg[0] {
        TokenTree::Literal(_) => {
            *running = lit(&seg[0]).ok_or("env! bad value")?;
            notes_from(1)?
        }
        TokenTree::Punct(p) if p.as_char() == '+' || p.as_char() == '-' => {
            let sign = if p.as_char() == '+' { 1 } else { -1 };
            match seg.get(1).and_then(lit) {
                Some(n) => {
                    *running += sign * n;
                    notes_from(2)?
                }
                None => {
                    *running += sign; // bare + / - steps by one
                    notes_from(1)?
                }
            }
        }
        other => return Err(format!("env! segment must start with a value, found `{other}`")),
    };
    if !(0..=255).contains(running) {
        return Err(format!("env! parameter value {running} out of range 0..=255"));
    }
    Ok((*running as u8, notes))
}

fn split_commas(toks: &[TokenTree]) -> Vec<Vec<TokenTree>> {
    let mut out = Vec::new();
    let mut cur = Vec::new();
    for t in toks {
        match t {
            TokenTree::Punct(p) if p.as_char() == ',' => out.push(std::mem::take(&mut cur)),
            _ => cur.push(t.clone()),
        }
    }
    out.push(cur);
    out
}

// ---- note grid -------------------------------------------------------------

// idx 0..=12 = C..B (idx 5 is the unused gap). The lower-case names build the
// const identifiers; the pretty names go in the generated docs.
const NOTE_NAMES: [&str; 13] = ["c", "cs", "d", "ds", "e", "", "f", "fs", "g", "gs", "a", "as", "b"];
const NOTE_PRETTY: [&str; 13] = ["C", "C♯", "D", "D♯", "E", "", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
// (suffix, num, den, human description): a value is num/den of a quarter note.
// The `N3` names are triplets (e3 = an eighth-note triplet = 1/3 of a quarter).
const VALS: &[(&str, u16, u16, &str)] = &[
    ("w", 4, 1, "whole note"),
    ("hdd", 7, 2, "half note, double-dotted"),
    ("hd", 3, 1, "half note, dotted"),
    ("h", 2, 1, "half note"),
    ("qdd", 7, 4, "quarter note, double-dotted"),
    ("qd", 3, 2, "quarter note, dotted"),
    ("q", 1, 1, "quarter note"),
    ("edd", 7, 8, "eighth note, double-dotted"),
    ("ed", 3, 4, "eighth note, dotted"),
    ("e", 1, 2, "eighth note"),
    ("id", 3, 8, "sixteenth note, dotted"),
    ("i", 1, 4, "sixteenth note"),
    ("td", 3, 16, "thirty-second note, dotted"),
    ("t", 1, 8, "thirty-second note"),
    ("x", 1, 16, "sixty-fourth note"),
    ("h3", 4, 3, "half-note triplet"),
    ("q3", 2, 3, "quarter-note triplet"),
    ("e3", 1, 3, "eighth-note triplet"),
    ("i3", 1, 6, "sixteenth-note triplet"),
    ("t3", 1, 12, "thirty-second-note triplet"),
];
const BASE_OCTAVE: u8 = 2;

/// Generate the documented note grid: every `<pitch><octave><value>` const
/// (`c3hd`, `as5e3`), every `r<value>` rest, and every `hit<value>` noise hit,
/// each carrying a one-line doc like `C octave 3 half note, dotted`. Pure note
/// math (no ROM data); invoked once in `lotw_music::note`.
#[proc_macro]
pub fn note_consts(_input: TokenStream) -> TokenStream {
    let mut out = TS::new();
    let lit = |n: u16| Literal::u16_unsuffixed(n);

    for nib in 0u8..=9u8 {
        let oct = nib + BASE_OCTAVE; // 2..=11 (u8: format_ident needs an unsigned type)
        for (idx, nm) in NOTE_NAMES.iter().enumerate() {
            if nm.is_empty() {
                continue;
            }
            let pitch = Literal::u8_unsuffixed((nib << 4) | idx as u8);
            let pretty = NOTE_PRETTY[idx];
            for (vn, num, den, desc) in VALS {
                let name = format_ident!("{}{}{}", nm, oct, vn);
                let doc = format!("{pretty} octave {oct} {desc}");
                let (num, den) = (lit(*num), lit(*den));
                out.extend(quote! {
                    #[doc = #doc]
                    pub const #name: crate::Note = crate::Note::Pitched { pitch: #pitch, val: crate::Val { num: #num, den: #den } };
                });
            }
        }
    }
    for (vn, num, den, desc) in VALS {
        let name = format_ident!("r{}", vn);
        let doc = format!("Rest — {desc}");
        let (num, den) = (lit(*num), lit(*den));
        out.extend(quote! {
            #[doc = #doc]
            pub const #name: crate::Note = crate::Note::Rest { val: crate::Val { num: #num, den: #den } };
        });
    }
    // Noise drum hits (no pitch; the drum sound is set by a command).
    for (vn, num, den, desc) in VALS {
        let name = format_ident!("hit{}", vn);
        let doc = format!("Noise drum hit — {desc}");
        let (num, den) = (lit(*num), lit(*den));
        out.extend(quote! {
            #[doc = #doc]
            pub const #name: crate::Note = crate::Note::Hit { val: crate::Val { num: #num, den: #den } };
        });
    }
    out.into()
}
