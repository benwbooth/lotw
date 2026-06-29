//! The `env!` parameter-envelope macro for the LotW music DSL.
//!
//! `env!(param, <value> <note>…, <value> <note>…, …)` — each segment sets a
//! channel parameter, then plays one or more carrier notes, expanding to a
//! `Note::Seq` of `param(v), note…` pairs:
//!
//! ```ignore
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

#[proc_macro]
pub fn env(input: TokenStream) -> TokenStream {
    match build(input.into()) {
        Ok(ts) => ts.into(),
        Err(e) => quote! { compile_error!(#e) }.into(),
    }
}

fn build(input: TS) -> Result<TS, String> {
    let toks: Vec<TokenTree> = input.into_iter().collect();
    let mut groups = split_commas(&toks).into_iter();

    let param = match groups.next().and_then(|g| g.into_iter().next()) {
        Some(TokenTree::Ident(id)) => id.to_string(),
        _ => return Err("env! expects a parameter name first".into()),
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
        return Err("env! needs at least one segment".into());
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
