//! Tokenizer for inline markdown strings.
//!
//! Parses delimiter-based markdown syntax (`**`, `*`, `_`, `__`, `~~`, `` ` ``)
//! into a recursive [`MdToken`] tree. Unclosed delimiters are treated as plain
//! text, this module never errors.

/// A parsed inline markdown token.
///
/// Each variant carries its own content as an owned `String`, allowing the
/// renderer to process tokens independently without a separate open/close pass.
/// Produced by [`tokenize`] and consumed by [`render`].
#[derive(Debug, PartialEq)]
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

fn tokens_to_text(tokens: Vec<MdToken>, delim: &str) -> String {
    let mut s = String::from(delim);
    for tok in tokens {
        match tok {
            MdToken::Text(t) => s.push_str(&t),
            MdToken::Bold(children) => {
                s.push_str(&tokens_to_text(children, "**"));
            }
            MdToken::Italic(children) => {
                s.push_str(&tokens_to_text(children, "*"));
            }
            MdToken::Underline(children) => {
                s.push_str(&tokens_to_text(children, "__"));
            }
            MdToken::Strikethrough(children) => {
                s.push_str(&tokens_to_text(children, "~~"));
            }
            MdToken::Code(t) => {
                s.push('`');
                s.push_str(&t);
            }
        }
    }
    s
}

fn tokenize_inner(input: &str, pos: &mut usize, stop_at: Option<&str>) -> (Vec<MdToken>, bool) {
    let mut tokens = Vec::new();
    let mut text_buf = String::new();

    while *pos < input.len() {
        if let Some(stop) = stop_at {
            if input[*pos..].starts_with(stop) {
                *pos += stop.len();
                if !text_buf.is_empty() {
                    tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                }
                return (tokens, true);
            }
        }

        if input[*pos..].starts_with("**") {
            *pos += 2;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let (inner, found) = tokenize_inner(input, pos, Some("**"));
            if found {
                tokens.push(MdToken::Bold(inner));
            } else {
                text_buf.push_str(&tokens_to_text(inner, "**"));
            }
        } else if input[*pos..].starts_with('*') {
            *pos += 1;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let (inner, found) = tokenize_inner(input, pos, Some("*"));
            if found {
                tokens.push(MdToken::Italic(inner));
            } else {
                text_buf.push_str(&tokens_to_text(inner, "*"));
            }
        } else if input[*pos..].starts_with("__") {
            *pos += 2;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let (inner, found) = tokenize_inner(input, pos, Some("__"));
            if found {
                tokens.push(MdToken::Underline(inner));
            } else {
                text_buf.push_str(&tokens_to_text(inner, "__"));
            }
        } else if input[*pos..].starts_with('_') {
            *pos += 1;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let (inner, found) = tokenize_inner(input, pos, Some("_"));
            if found {
                tokens.push(MdToken::Italic(inner));
            } else {
                text_buf.push_str(&tokens_to_text(inner, "_"));
            }
        } else if input[*pos..].starts_with("~~") {
            *pos += 2;
            if !text_buf.is_empty() {
                tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
            }
            let (inner, found) = tokenize_inner(input, pos, Some("~~"));
            if found {
                tokens.push(MdToken::Strikethrough(inner));
            } else {
                text_buf.push_str(&tokens_to_text(inner, "~~"));
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

    (tokens, false)
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
    let (tokens, _) = tokenize_inner(input, &mut pos, None);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- tokenize ---

    #[test]
    fn test_tokenize_empty_string() {
        assert!(tokenize("").is_empty());
    }

    #[test]
    fn test_tokenize_plain_text() {
        assert_eq!(tokenize("hello"), vec![MdToken::Text("hello".into())]);
    }

    #[test]
    fn test_tokenize_single_bold() {
        assert_eq!(
            tokenize("**bold**"),
            vec![MdToken::Bold(vec![MdToken::Text("bold".into())])]
        );
    }

    #[test]
    fn test_tokenize_single_italic_star() {
        assert_eq!(
            tokenize("*italic*"),
            vec![MdToken::Italic(vec![MdToken::Text("italic".into())])]
        );
    }

    #[test]
    fn test_tokenize_single_italic_underscore() {
        assert_eq!(
            tokenize("_italic_"),
            vec![MdToken::Italic(vec![MdToken::Text("italic".into())])]
        );
    }

    #[test]
    fn test_tokenize_underline() {
        assert_eq!(
            tokenize("__underline__"),
            vec![MdToken::Underline(vec![MdToken::Text("underline".into())])]
        );
    }

    #[test]
    fn test_tokenize_strikethrough() {
        assert_eq!(
            tokenize("~~strike~~"),
            vec![MdToken::Strikethrough(vec![MdToken::Text("strike".into())])]
        );
    }

    #[test]
    fn test_tokenize_inline_code() {
        assert_eq!(tokenize("`code`"), vec![MdToken::Code("code".into())]);
    }

    #[test]
    fn test_tokenize_nested_bold_containing_italic() {
        assert_eq!(
            tokenize("**bold *italic* end**"),
            vec![MdToken::Bold(vec![
                MdToken::Text("bold ".into()),
                MdToken::Italic(vec![MdToken::Text("italic".into())]),
                MdToken::Text(" end".into()),
            ])]
        );
    }

    #[test]
    fn test_tokenize_unclosed_bold_treated_as_text() {
        assert_eq!(tokenize("**oops"), vec![MdToken::Text("**oops".into())]);
    }

    #[test]
    fn test_tokenize_unclosed_italic_treated_as_text() {
        assert_eq!(tokenize("*oops"), vec![MdToken::Text("*oops".into())]);
    }

    #[test]
    fn test_tokenize_unclosed_code_treated_as_text() {
        assert_eq!(tokenize("`oops"), vec![MdToken::Text("`oops".into())]);
    }

    #[test]
    fn test_tokenize_mixed_plain_and_styled() {
        assert_eq!(
            tokenize("hello **world** bye"),
            vec![
                MdToken::Text("hello ".into()),
                MdToken::Bold(vec![MdToken::Text("world".into())]),
                MdToken::Text(" bye".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_multiple_consecutive_spans() {
        assert_eq!(
            tokenize("**bold***italic*"),
            vec![
                MdToken::Bold(vec![MdToken::Text("bold".into())]),
                MdToken::Italic(vec![MdToken::Text("italic".into())]),
            ]
        );
    }
}
