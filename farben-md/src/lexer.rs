use std::{iter::Peekable, str::Chars};

/// A parsed inline markdown token.
///
/// Each variant carries its own content as an owned `String`, allowing the
/// renderer to process tokens independently without a separate open/close pass.
/// Produced by [`tokenize`] and consumed by [`render`].
pub enum MdToken {
    /// A run of plain text with no markdown formatting.
    Text(String),
    /// A bold span, delimited by `**...**`.
    Bold(Vec<MdToken>),
    /// An italic span, delimited by `*...*` or `_..._`.
    Italic(Vec<MdToken>),
    /// An inline code span, delimited by `` `...` ``.
    Code(String),
    /// A strikethrough span, delimited by `~~...~~`.
    Strikethrough(Vec<MdToken>),
    /// An underlined span, delimited by `__...__`.
    Underline(Vec<MdToken>),
}

fn tokenize_inner(input: &str, pos: &mut usize, stop_at: Option<&str>) -> Vec<MdToken> {
    let mut tokens = Vec::new();
    let mut text_buf = String::new();

    while *pos < input.len() {
        if let Some(stop) = stop_at {
            if input[*pos..].starts_with(stop) {
                *pos += stop.len();
                break;
            }
        }

        if input[*pos..].starts_with("**") {
            *pos += 2;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let inner = tokenize_inner(input, pos, Some("**"));
            if !inner.is_empty() {
                tokens.push(MdToken::Bold(inner));
            } else {
                text_buf.push_str("**");
            }
        } else if input[*pos..].starts_with('*') {
            *pos += 1;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let inner = tokenize_inner(input, pos, Some("*"));
            if !inner.is_empty() {
                tokens.push(MdToken::Italic(inner));
            } else {
                text_buf.push('*');
            }
        } else if input[*pos..].starts_with("__") {
            *pos += 2;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let inner = tokenize_inner(input, pos, Some("__"));
            if !inner.is_empty() {
                tokens.push(MdToken::Underline(inner));
            } else {
                text_buf.push_str("__");
            }
        } else if input[*pos..].starts_with('_') {
            *pos += 1;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let inner = tokenize_inner(input, pos, Some("_"));
            if !inner.is_empty() {
                tokens.push(MdToken::Italic(inner));
            } else {
                text_buf.push('_');
            }
        } else if input[*pos..].starts_with("~~") {
            *pos += 2;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let inner = tokenize_inner(input, pos, Some("~~"));
            if !inner.is_empty() {
                tokens.push(MdToken::Strikethrough(inner));
            } else {
                text_buf.push_str("~~");
            }
        } else if input[*pos..].starts_with('`') {
            *pos += 1;
            let start = *pos;
            let mut found = false;
            while *pos < input.len() {
                if input[*pos..].starts_with('`') {
                    found = true;
                    break;
                }
                *pos += input[*pos..].chars().next().map_or(1, |c| c.len_utf8());
            }
            if found {
                let content = input[start..*pos].to_string();
                *pos += 1;
                if !text_buf.is_empty() {
                    tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                }
                tokens.push(MdToken::Code(content));
            } else {
                text_buf.push('`');
                text_buf.push_str(&input[start..*pos]);
            }
        } else {
            let ch = input[*pos..].chars().next().unwrap();
            text_buf.push(ch);
            *pos += ch.len_utf8();
        }
    }

    if !text_buf.is_empty() {
        tokens.push(MdToken::Text(text_buf));
    }

    tokens
}

/// Tokenizes a markdown string into a sequence of [`MdToken`]s.
///
/// Parses inline markdown delimiters (`**`, `*`, `_`, `__`, `~~`, `` ` ``)
/// into their corresponding token types. Unclosed delimiters are treated as
/// plain text rather than errors — this function always succeeds.
///
/// # Example
///
/// ```
/// use farben_md::lexer::{tokenize, MdToken};
///
/// let tokens = tokenize("**bold** and *italic*");
/// assert!(matches!(tokens[0], MdToken::Bold(_)));
/// ```
pub fn tokenize(input: &str) -> Vec<MdToken> {
    let mut pos = 0;
    tokenize_inner(input, &mut pos, None)
}
