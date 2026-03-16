/// A parsed inline markdown token.
///
/// Each variant carries its own content as an owned `String`, allowing the
/// renderer to process tokens independently without a separate open/close pass.
/// Produced by [`tokenize`] and consumed by [`render`].
pub enum MdToken {
    /// A run of plain text with no markdown formatting.
    Text(String),
    /// A bold span, delimited by `**...**`.
    Bold(String),
    /// An italic span, delimited by `*...*` or `_..._`.
    Italic(String),
    /// An inline code span, delimited by `` `...` ``.
    Code(String),
    /// A strikethrough span, delimited by `~~...~~`.
    Strikethrough(String),
    /// An underlined span, delimited by `__...__`.
    Underline(String),
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
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut text_buf = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next();
                    let mut content = String::new();
                    let mut found = false;
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'*') {
                            chars.next();
                            found = true;
                            break;
                        }
                        content.push(c);
                    }
                    if found {
                        if !text_buf.is_empty() {
                            tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                        }
                        tokens.push(MdToken::Bold(content));
                    } else {
                        text_buf.push_str("**");
                        text_buf.push_str(&content);
                    }
                } else {
                    let mut content = String::new();
                    let mut found = false;
                    while let Some(c) = chars.next() {
                        if c == '*' {
                            found = true;
                            break;
                        }
                        content.push(c);
                    }
                    if found {
                        if !text_buf.is_empty() {
                            tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                        }
                        tokens.push(MdToken::Italic(content));
                    } else {
                        text_buf.push('*');
                        text_buf.push_str(&content);
                    }
                }
            }
            '_' => {
                if chars.peek() == Some(&'_') {
                    chars.next();
                    let mut content = String::new();
                    let mut found = false;
                    while let Some(c) = chars.next() {
                        if c == '_' && chars.peek() == Some(&'_') {
                            chars.next();
                            found = true;
                            break;
                        }
                        content.push(c);
                    }
                    if found {
                        if !text_buf.is_empty() {
                            tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                        }
                        tokens.push(MdToken::Underline(content));
                    } else {
                        text_buf.push_str("__");
                        text_buf.push_str(&content);
                    }
                } else {
                    let mut content = String::new();
                    let mut found = false;
                    while let Some(c) = chars.next() {
                        if c == '_' {
                            found = true;
                            break;
                        }
                        content.push(c);
                    }
                    if found {
                        if !text_buf.is_empty() {
                            tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                        }
                        tokens.push(MdToken::Italic(content));
                    } else {
                        text_buf.push('_');
                        text_buf.push_str(&content);
                    }
                }
            }
            '~' => {
                if chars.peek() == Some(&'~') {
                    chars.next();
                    let mut content = String::new();
                    let mut found = false;
                    while let Some(c) = chars.next() {
                        if c == '~' && chars.peek() == Some(&'~') {
                            chars.next();
                            found = true;
                            break;
                        }
                        content.push(c);
                    }
                    if found {
                        if !text_buf.is_empty() {
                            tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                        }
                        tokens.push(MdToken::Strikethrough(content));
                    } else {
                        text_buf.push_str("~~");
                        text_buf.push_str(&content);
                    }
                } else {
                    text_buf.push('~');
                }
            }
            '`' => {
                let mut content = String::new();
                let mut found = false;
                while let Some(c) = chars.next() {
                    if c == '`' {
                        found = true;
                        break;
                    }
                    content.push(c);
                }
                if found {
                    if !text_buf.is_empty() {
                        tokens.push(MdToken::Text(std::mem::take(&mut text_buf)));
                    }
                    tokens.push(MdToken::Code(content));
                } else {
                    text_buf.push('`');
                    text_buf.push_str(&content);
                }
            }
            _ => text_buf.push(ch),
        }
    }

    if !text_buf.is_empty() {
        tokens.push(MdToken::Text(text_buf));
    }

    tokens
}
