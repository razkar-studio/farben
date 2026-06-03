//! Format string splitter for compile-time farben template expansion.
//!
//! Splits a farben markup format string (e.g. `"[red]{name} joined[/]"`) into
//! an alternating sequence of [`Piece`]s: pre-rendered static ANSI segments and
//! raw `{...}` format argument specs. The proc macro then emits a block that
//! writes each piece directly, bypassing the runtime parser entirely.

/// A single piece of a split format string.
pub enum Piece {
    /// A static segment with both its ANSI-rendered and plain-text forms
    /// pre-computed at compile time. Empty segments are omitted by the caller.
    Static { ansi: String, plain: String },
    /// A raw `{...}` format argument spec, passed through verbatim.
    /// Examples: `""` (bare `{}`), `"name"`, `"0"`, `":.2"`, `"name:.2"`.
    Arg(String),
}

/// Splits a farben markup format string into alternating [`Piece`]s.
///
/// Scans `input` for `{...}` format placeholders (respecting `{{`/`}}` escapes),
/// renders each static segment between them through the farben pipeline at
/// compile time, and returns the resulting sequence. If `bleed` is `false`,
/// a trailing `\x1b[0m` reset is appended to the last static segment's ANSI
/// form (matching the behaviour of `cformat!`). If `bleed` is `true`, no reset
/// is appended (matching `cformatb!`).
///
/// # Errors
///
/// Returns a `farben_core::errors::LexError` if any static segment contains
/// invalid farben markup.
pub fn split(input: &str, bleed: bool) -> Result<Vec<Piece>, farben_core::errors::LexError> {
    farben_core::clear_active_stack();

    let mut pieces: Vec<Piece> = Vec::new();
    let mut static_buf = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '{' => if let Some('{') = chars.peek() {
                chars.next();
                static_buf.push('{');
            } else {
                flush_static(&mut static_buf, &mut pieces, false)?;
                let mut spec = String::new();
                let mut closed = false;
                for inner in chars.by_ref() {
                    if inner == '}' {
                        closed = true;
                        break;
                    }
                    spec.push(inner);
                }
                if closed {
                    pieces.push(Piece::Arg(spec));
                } else {
                    static_buf.push('{');
                    static_buf.push_str(&spec);
                }
            },
            '}' => match chars.peek() {
                Some('}') => {
                    chars.next();
                    static_buf.push('}');
                }
                _ => static_buf.push('}'),
            },
            other => static_buf.push(other),
        }
    }

    flush_static(&mut static_buf, &mut pieces, !bleed)?;

    if !bleed {
        ensure_trailing_reset(&mut pieces);
    }

    Ok(pieces)
}

/// Renders `buf` through the farben pipeline and pushes a `Static` piece.
/// Does nothing if `buf` is empty. Clears `buf` afterwards.
fn flush_static(
    buf: &mut String,
    pieces: &mut Vec<Piece>,
    append_reset: bool,
) -> Result<(), farben_core::errors::LexError> {
    if buf.is_empty() && !append_reset {
        return Ok(());
    }

    let tokens = farben_core::lexer::tokenize(buf.as_str())?;
    let mut ansi = farben_core::parser::render_forced(tokens);
    if append_reset {
        ansi.push_str("\x1b[0m");
    }
    let plain = farben_core::strip::strip_ansi(&ansi);

    if !ansi.is_empty() || !plain.is_empty() {
        pieces.push(Piece::Static { ansi, plain });
    }

    buf.clear();
    Ok(())
}

/// Ensures there is a trailing `\x1b[0m` reset in the last static segment.
///
/// Called when `bleed` is false and the format string ends with an `Arg` piece
/// (i.e. there's no trailing static text to carry the reset). Appends a new
/// `Static { ansi: "\x1b[0m", plain: "" }` piece in that case.
fn ensure_trailing_reset(pieces: &mut Vec<Piece>) {
    let needs_reset = match pieces.last() {
        Some(Piece::Static { ansi, .. }) => !ansi.ends_with("\x1b[0m"),
        Some(Piece::Arg(_)) => true,
        None => false,
    };

    if needs_reset {
        pieces.push(Piece::Static {
            ansi: "\x1b[0m".to_string(),
            plain: String::new(),
        });
    }
}
