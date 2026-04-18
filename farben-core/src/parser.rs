//! Token stream renderer.
//!
//! Converts a sequence of [`Token`] values produced by the lexer into a final
//! ANSI-escaped string ready for terminal output. This module is the last stage
//! in the farben pipeline: tokenize with [`crate::lexer::tokenize`], then render
//! with [`render`].

use crate::{
    ansi::{color_to_ansi, emphasis_to_ansi},
    env::color_enabled,
    lexer::{TagType, Token},
    state::{active_stack, set_active_stack},
};

/// Renders a token stream into a raw ANSI-escaped string.
///
/// Text tokens are appended as-is. Tag tokens are converted to their corresponding
/// ANSI escape sequences. The active style stack persists across calls via thread-local state
/// callers using non-bleed semantics should call clear_active_stack() after their reset.
///
/// # Example
///
/// ```ignore
/// let tokens = tokenize("[red]hello[/]")?;
/// let output = render(tokens);
/// assert_eq!(output, "\x1b[31mhello\x1b[0m");
/// ```
pub fn render(tokens: Vec<Token>) -> String {
    if !color_enabled() {
        return tokens
            .into_iter()
            .filter_map(|t| match t {
                Token::Text(s) => Some(s.into_owned()),
                Token::Tag(TagType::Prefix(s)) => Some(s),
                _ => None,
            })
            .collect();
    }
    let mut result = String::with_capacity(tokens.len() * 16);
    let mut active: Vec<TagType> = active_stack();
    for t in tokens {
        match t {
            Token::Text(s) => result.push_str(&s),
            Token::Tag(TagType::Prefix(s)) => result.push_str(&s),
            Token::Tag(TagType::Color { color, ground }) => {
                #[cfg(feature = "lossy")]
                let color = crate::degrader::degrade(color);
                result.push_str(&color_to_ansi(&color, ground.clone()));
                active.push(TagType::Color { color, ground });
            }
            Token::Tag(TagType::Emphasis(e)) => {
                result.push_str(&emphasis_to_ansi(&e));
                active.push(TagType::Emphasis(e));
            }
            Token::Tag(TagType::ResetAll) => {
                result.push_str("\x1b[0m");
                active.clear();
            }
            Token::Tag(TagType::ResetOne(r)) => {
                result.push_str("\x1b[0m");
                active.retain(|x| x != r.as_ref());
                for a in &active {
                    match a {
                        TagType::Color { color, ground } => {
                            result.push_str(&color_to_ansi(color, ground.clone()))
                        }
                        TagType::Emphasis(e) => result.push_str(&emphasis_to_ansi(e)),
                        _ => {}
                    }
                }
            }
        }
    }
    set_active_stack(active);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::{Color, Ground, NamedColor};
    use crate::env::color_enabled;
    use crate::lexer::{EmphasisType, TagType, Token};

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
            Token::Tag(TagType::ResetOne(Box::new(TagType::Emphasis(
                EmphasisType::Bold,
            )))),
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
            Token::Tag(TagType::ResetOne(Box::new(TagType::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Foreground,
            }))),
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
