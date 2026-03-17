//! Token tree renderer.
//!
//! Converts a [`MdToken`] tree produced by [`crate::lexer::tokenize`] into a
//! final ANSI-escaped string. Nested spans are handled via an active style stack
//! that re-emits surviving styles after each reset.

use crate::lexer::MdToken;
use farben_core::ansi::{Color, Ground, NamedColor, color_to_ansi, emphasis_to_ansi};
use farben_core::lexer::EmphasisType;

/// Renders a slice of [`MdToken`]s into an ANSI-escaped string.
///
/// Nested spans are handled correctly — styles are re-emitted after each
/// reset so that active styles are preserved across span boundaries.
/// Appends a final `\x1b[0m` reset to clean up after all output.
///
/// # Example
///
/// ```
/// use farben_md::{lexer::tokenize, renderer::render};
///
/// let tokens = tokenize("**bold** and *italic*");
/// let output = render(&tokens);
/// assert!(output.contains("\x1b[1m"));
/// ```
pub fn render(tokens: &[MdToken]) -> String {
    let mut out = String::new();
    let mut active: Vec<EmphasisType> = Vec::new();
    render_inner(tokens, &mut active, &mut out);
    out.push_str("\x1b[0m");
    out
}

fn render_inner(tokens: &[MdToken], active: &mut Vec<EmphasisType>, out: &mut String) {
    for token in tokens {
        match token {
            MdToken::Text(text) => out.push_str(text),
            MdToken::Code(text) => {
                out.push_str("\x1b[0m");
                out.push_str(&emphasis_to_ansi(&EmphasisType::Bold));
                out.push_str(&color_to_ansi(
                    &Color::Named(NamedColor::BrightWhite),
                    Ground::Foreground,
                ));
                out.push_str(&color_to_ansi(
                    &Color::Named(NamedColor::BrightBlack),
                    Ground::Background,
                ));
                out.push_str(text);
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Bold(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Bold));
                active.push(EmphasisType::Bold);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Italic(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Italic));
                active.push(EmphasisType::Italic);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Strikethrough(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Strikethrough));
                active.push(EmphasisType::Strikethrough);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Underline(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Underline));
                active.push(EmphasisType::Underline);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::MdToken;

    // --- render ---

    #[test]
    fn test_render_empty_tokens_returns_reset() {
        assert_eq!(render(&[]), "\x1b[0m");
    }

    #[test]
    fn test_render_plain_text_passes_through() {
        let tokens = vec![MdToken::Text("hello".into())];
        assert_eq!(render(&tokens), "hello\x1b[0m");
    }

    #[test]
    fn test_render_appends_trailing_reset() {
        let tokens = vec![MdToken::Text("x".into())];
        assert!(render(&tokens).ends_with("\x1b[0m"));
    }

    #[test]
    fn test_render_bold() {
        let tokens = vec![MdToken::Bold(vec![MdToken::Text("bold".into())])];
        let out = render(&tokens);
        assert!(out.starts_with("\x1b[1m"));
        assert!(out.contains("bold"));
    }

    #[test]
    fn test_render_italic() {
        let tokens = vec![MdToken::Italic(vec![MdToken::Text("italic".into())])];
        let out = render(&tokens);
        assert!(out.starts_with("\x1b[3m"));
        assert!(out.contains("italic"));
    }

    #[test]
    fn test_render_underline() {
        let tokens = vec![MdToken::Underline(vec![MdToken::Text("under".into())])];
        let out = render(&tokens);
        assert!(out.starts_with("\x1b[4m"));
        assert!(out.contains("under"));
    }

    #[test]
    fn test_render_strikethrough() {
        let tokens = vec![MdToken::Strikethrough(vec![MdToken::Text("strike".into())])];
        let out = render(&tokens);
        assert!(out.starts_with("\x1b[9m"));
        assert!(out.contains("strike"));
    }

    #[test]
    fn test_render_inline_code() {
        let tokens = vec![MdToken::Code("code".into())];
        let out = render(&tokens);
        assert!(out.contains("\x1b[1m"));
        assert!(out.contains("\x1b[97m")); // BrightWhite fg
        assert!(out.contains("\x1b[100m")); // BrightBlack bg
        assert!(out.contains("code"));
    }

    #[test]
    fn test_render_multiple_spans_in_order() {
        let tokens = vec![
            MdToken::Bold(vec![MdToken::Text("a".into())]),
            MdToken::Italic(vec![MdToken::Text("b".into())]),
        ];
        let out = render(&tokens);
        let bold_pos = out.find("\x1b[1m").unwrap();
        let italic_pos = out.find("\x1b[3m").unwrap();
        assert!(bold_pos < italic_pos);
    }

    #[test]
    fn test_render_nested_bold_italic_re_emits_bold() {
        let tokens = vec![MdToken::Bold(vec![
            MdToken::Text("a".into()),
            MdToken::Italic(vec![MdToken::Text("b".into())]),
            MdToken::Text("c".into()),
        ])];
        let out = render(&tokens);
        // bold should appear at least twice -- opening and re-emit after italic
        assert!(out.matches("\x1b[1m").count() >= 2);
        assert!(out.contains("a"));
        assert!(out.contains("b"));
        assert!(out.contains("c"));
    }
}

// Skipped (side effects): none: render() is a pure function.
