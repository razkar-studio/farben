//! Token stream renderer.
//!
//! Converts a sequence of [`Token`] values produced by the lexer into a final
//! ANSI-escaped string ready for terminal output. This module is the last stage
//! in the farben pipeline: tokenize with [`crate::lexer::tokenize`], then render
//! with [`render`].

use crate::{
    ansi::{write_color_ansi, write_emphasis_ansi},
    env::color_enabled,
    errors::LexError,
    lexer::{TagType, Token, parse_part, split_tag_parts},
    state::{active_stack, set_active_stack},
    strip::strip_markup,
};

/// Applies a single styling `TagType` to `result`, writing ANSI sequences and
/// tracking active styles in `active`.
fn apply_tag(tag: TagType, result: &mut String, active: &mut Vec<TagType>) {
    match tag {
        TagType::Prefix(s) => result.push_str(&s),
        TagType::Color { color, ground } => {
            #[cfg(feature = "lossy")]
            let color = crate::degrader::degrade(color);
            write_color_ansi(result, &color, ground);
            active.push(TagType::Color { color, ground });
        }
        TagType::Emphasis(e) => {
            write_emphasis_ansi(result, &e);
            active.push(TagType::Emphasis(e));
        }
        TagType::ResetAll => {
            result.push_str("\x1b[0m");
            active.clear();
        }
        TagType::ResetOne(r) => {
            result.push_str("\x1b[0m");
            active.retain(|x| !r.matches_tag(x));
            for a in &*active {
                match a {
                    TagType::Color { color, ground } => {
                        write_color_ansi(result, color, *ground);
                    }
                    TagType::Emphasis(e) => write_emphasis_ansi(result, e),
                    _ => {}
                }
            }
        }
    }
}

/// Renders a token stream into a raw ANSI-escaped string.
///
/// Text tokens are appended as-is. Tag tokens are converted to their corresponding
/// ANSI escape sequences. The active style stack persists across calls via thread-local state
/// callers using non-bleed semantics should call `clear_active_stack()` after their reset.
///
/// # Example
///
/// ```ignore
/// let tokens = tokenize("[red]hello[/]")?;
/// let output = render(tokens);
/// assert_eq!(output, "\x1b[31mhello\x1b[0m");
/// ```
#[must_use]
pub fn render(tokens: Vec<Token>) -> String {
    if !color_enabled() {
        return tokens
            .into_iter()
            .filter_map(|t| match t {
                Token::Text(s) => Some(s.into_owned()),
                Token::Tag(TagType::Prefix(s)) => Some(s),
                Token::Tag(_) => None,
            })
            .collect();
    }
    render_forced(tokens)
}

/// The same as [`render`], but bypasses the `color_enabled` check.
///
/// This means that this function renders directly without checking if color should be enabled.
#[must_use]
pub fn render_forced(tokens: Vec<Token>) -> String {
    let mut result = String::with_capacity(tokens.len() * 16);
    let mut active: Vec<TagType> = active_stack();
    for t in tokens {
        match t {
            Token::Text(s) => result.push_str(&s),
            Token::Tag(tag) => apply_tag(tag, &mut result, &mut active),
        }
    }
    set_active_stack(active);
    result
}

/// Single-pass render: parses and renders markup in one pass, avoiding intermediate `Vec<Token>`.
///
/// When colors are enabled, scans the input for tags and emits ANSI sequences directly.
/// When disabled, validates markup and strips all tags.
///
/// # Errors
///
/// Returns `LexError` on unclosed tags or malformed tag content.
/// Note: this is the optimized single-pass entry point used by the `farben` crate.
/// External consumers calling `render` + `tokenize` separately will also work, but
/// this function avoids the intermediate `Vec<Token>` allocation.
pub fn render_str(input: &str) -> Result<String, LexError> {
    if !color_enabled() {
        crate::lexer::tokenize(input)?; // validate (preserves existing error behavior)
        return Ok(strip_markup(input));
    }
    render_forced_str(input)
}

/// The same as `render_str`, but bypasses the `color_enabled` check.
fn render_forced_str(input: &str) -> Result<String, LexError> {
    let mut result = String::with_capacity(input.len() + input.len() / 4);
    let mut active: Vec<TagType> = active_stack();
    let mut tag_types = Vec::new();
    let bytes = input.as_bytes();
    let mut pos = 0;

    while pos < input.len() {
        // Find the next '[' or ']'
        let next = {
            let rest = &input[pos..];
            let open = rest.find('[');
            let close = rest.find(']');
            match (open, close) {
                (Some(o), Some(c)) if o <= c => Some((pos + o, b'[')),
                (Some(_) | None, Some(c)) => Some((pos + c, b']')),
                (Some(o), None) => Some((pos + o, b'[')),
                (None, None) => None,
            }
        };

        let Some((abs_pos, kind)) = next else {
            // No more brackets; flush remaining text.
            if pos < input.len() {
                result.push_str(&input[pos..]);
            }
            break;
        };

        // Flush text before the bracket.
        if abs_pos > pos {
            result.push_str(&input[pos..abs_pos]);
        }

        match kind {
            b']' => {
                if abs_pos + 1 < input.len() && bytes[abs_pos + 1] == b']' {
                    result.push(']');
                    pos = abs_pos + 2;
                } else {
                    result.push(']');
                    pos = abs_pos + 1;
                }
            }
            b'[' => {
                // ESC prefix: raw ANSI passthrough.
                if abs_pos > 0 && bytes[abs_pos - 1] == b'\x1b' {
                    result.push_str("\x1b[");
                    pos = abs_pos + 1;
                    continue;
                }

                // Double-bracket escape.
                if abs_pos + 1 < input.len() && bytes[abs_pos + 1] == b'[' {
                    result.push('[');
                    pos = abs_pos + 2;
                    continue;
                }

                // Find matching ']'.
                let tag_start = abs_pos + 1;
                let closing = input[tag_start..]
                    .find(']')
                    .ok_or(LexError::UnclosedTag(abs_pos))?;
                let raw_tag = &input[tag_start..tag_start + closing];

                // Parse tag parts and emit ANSI directly.
                tag_types.clear();
                for (offset, part) in split_tag_parts(raw_tag) {
                    let abs_off = tag_start + offset;
                    parse_part(part, abs_off, &mut tag_types)?;
                }
                for t in tag_types.drain(..) {
                    apply_tag(t, &mut result, &mut active);
                }

                pos = tag_start + closing + 1;
            }
            _ => unreachable!(),
        }
    }

    set_active_stack(active);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::{Color, Ground, NamedColor};
    use crate::env::color_enabled;
    use crate::lexer::{EmphasisType, ResetKind, TagType, Token};

    // --- render ---
    #[test]
    fn test_render_empty_token_list() {
        let result = render(vec![]);
        assert_eq!(result, "");
    }
    #[test]
    fn test_render_plain_text_token() {
        let result = render(vec![Token::Text("hello".into())]);
        assert_eq!(result, "hello");
    }
    #[test]
    fn test_render_named_color_tag() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::Color {
            color: Color::Named(NamedColor::Red),
            ground: Ground::Foreground,
        })]);
        assert_eq!(result, "\x1b[31m");
    }
    #[test]
    fn test_render_emphasis_tag_bold() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::Emphasis(EmphasisType::Bold))]);
        assert_eq!(result, "\x1b[1m");
    }
    #[test]
    fn test_render_reset_tag() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::ResetAll)]);
        assert_eq!(result, "\x1b[0m");
    }
    #[test]
    fn test_render_color_then_text() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![
            Token::Tag(TagType::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Foreground,
            }),
            Token::Text("hello".into()),
        ]);
        assert_eq!(result, "\x1b[31mhello");
    }
    #[test]
    fn test_render_color_text_reset() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![
            Token::Tag(TagType::Color {
                color: Color::Named(NamedColor::Green),
                ground: Ground::Foreground,
            }),
            Token::Text("go".into()),
            Token::Tag(TagType::ResetAll),
        ]);
        assert_eq!(result, "\x1b[32mgo\x1b[0m");
    }
    #[test]
    fn test_render_multiple_text_tokens() {
        let result = render(vec![Token::Text("foo".into()), Token::Text("bar".into())]);
        assert_eq!(result, "foobar");
    }
    #[test]
    fn test_render_ansi256_color_tag() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::Color {
            color: Color::Ansi256(21),
            ground: Ground::Foreground,
        })]);
        assert_eq!(result, "\x1b[38;5;21m");
    }
    #[test]
    fn test_render_rgb_color_tag() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::Color {
            color: Color::Rgb(255, 0, 0),
            ground: Ground::Foreground,
        })]);
        assert_eq!(result, "\x1b[38;2;255;0;0m");
    }
    #[test]
    fn test_render_does_not_append_trailing_reset() {
        let result = render(vec![Token::Text("plain".into())]);
        assert!(!result.ends_with("\x1b[0m"));
    }
    #[test]
    fn test_render_named_color_background() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::Color {
            color: Color::Named(NamedColor::Red),
            ground: Ground::Background,
        })]);
        assert_eq!(result, "\x1b[41m");
    }
    #[test]
    fn test_render_ansi256_background() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::Color {
            color: Color::Ansi256(21),
            ground: Ground::Background,
        })]);
        assert_eq!(result, "\x1b[48;5;21m");
    }
    #[test]
    fn test_render_rgb_background() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![Token::Tag(TagType::Color {
            color: Color::Rgb(255, 0, 0),
            ground: Ground::Background,
        })]);
        assert_eq!(result, "\x1b[48;2;255;0;0m");
    }
    #[test]
    fn test_render_fg_and_bg_together() {
        if !color_enabled() {
            return;
        }
        let result = render(vec![
            Token::Tag(TagType::Color {
                color: Color::Named(NamedColor::White),
                ground: Ground::Foreground,
            }),
            Token::Tag(TagType::Color {
                color: Color::Named(NamedColor::Blue),
                ground: Ground::Background,
            }),
            Token::Text("hello".into()),
        ]);
        assert_eq!(result, "\x1b[37m\x1b[44mhello");
    }

    // --- render with color disabled ---

    #[test]
    fn test_render_no_color_strips_tag_tokens() {
        if color_enabled() {
            return;
        }
        let result = render(vec![
            Token::Tag(TagType::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Foreground,
            }),
            Token::Text("hello".into()),
            Token::Tag(TagType::ResetAll),
        ]);
        assert_eq!(result, "hello");
    }
    #[test]
    fn test_render_no_color_preserves_text_and_prefix() {
        if color_enabled() {
            return;
        }
        let result = render(vec![
            Token::Tag(TagType::Prefix(">>".to_string())),
            Token::Text(" world".into()),
        ]);
        assert_eq!(result, ">> world");
    }
    #[test]
    fn test_render_no_color_pure_tags_produce_empty_string() {
        if color_enabled() {
            return;
        }
        let result = render(vec![
            Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
            Token::Tag(TagType::ResetAll),
        ]);
        assert_eq!(result, "");
    }
    #[test]
    fn test_render_no_color_reset_one_stripped() {
        if color_enabled() {
            return;
        }
        let result = render(vec![
            Token::Tag(TagType::ResetOne(ResetKind::Emphasis(EmphasisType::Bold))),
            Token::Text("plain".into()),
        ]);
        assert_eq!(result, "plain");
    }
    #[test]
    fn test_render_resumes_persisted_stack() {
        if !color_enabled() {
            return;
        }
        crate::clear_active_stack();

        let _ = render(vec![
            Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
            Token::Tag(TagType::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Foreground,
            }),
        ]);

        let result = render(vec![
            Token::Tag(TagType::ResetOne(ResetKind::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Foreground,
            })),
            Token::Text("ok".into()),
        ]);
        assert_eq!(result, "\x1b[0m\x1b[1mok");

        crate::clear_active_stack();
    }

    #[test]
    fn test_render_persists_active_stack() {
        if !color_enabled() {
            return;
        }
        crate::clear_active_stack();

        let _ = render(vec![Token::Tag(TagType::Emphasis(EmphasisType::Bold))]);
        assert_eq!(
            crate::active_stack(),
            vec![TagType::Emphasis(EmphasisType::Bold)]
        );

        crate::clear_active_stack();
    }

    #[test]
    fn test_render_reset_all_clears_persisted_stack() {
        if !color_enabled() {
            return;
        }
        crate::clear_active_stack();

        let _ = render(vec![
            Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
            Token::Tag(TagType::ResetAll),
        ]);
        assert!(crate::active_stack().is_empty());
    }
}
// Skipped (side effects): none: render() is a pure function.
