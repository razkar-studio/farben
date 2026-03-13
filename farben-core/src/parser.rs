use crate::ansi::{Ground, color_to_ansi, emphasis_to_ansi};
use crate::lexer::{TagType, Token};

/// Renders a token stream into a raw ANSI-escaped string.
///
/// Text tokens are appended as-is. Tag tokens are converted to their corresponding
/// ANSI escape sequences. Does not append a trailing reset; callers are responsible
/// for that if needed.
pub fn render(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for tok in tokens {
        match tok {
            Token::Text(text) => result.push_str(text.as_str()),
            Token::Tag(tag) => match tag {
                TagType::Color(color) => {
                    result.push_str(color_to_ansi(&color, Ground::Foreground).as_str())
                }
                TagType::Emphasis(emphasis) => {
                    result.push_str(emphasis_to_ansi(&emphasis).as_str())
                }
                TagType::Reset => result.push_str("\x1b[0m"),
            },
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::{Color, NamedColor};
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
        let result = render(vec![Token::Tag(TagType::Color(Color::Named(
            NamedColor::Red,
        )))]);
        assert_eq!(result, "\x1b[31m");
    }

    #[test]
    fn test_render_emphasis_tag_bold() {
        let result = render(vec![Token::Tag(TagType::Emphasis(EmphasisType::Bold))]);
        assert_eq!(result, "\x1b[1m");
    }

    #[test]
    fn test_render_reset_tag() {
        let result = render(vec![Token::Tag(TagType::Reset)]);
        assert_eq!(result, "\x1b[0m");
    }

    #[test]
    fn test_render_color_then_text() {
        let result = render(vec![
            Token::Tag(TagType::Color(Color::Named(NamedColor::Red))),
            Token::Text("hello".into()),
        ]);
        assert_eq!(result, "\x1b[31mhello");
    }

    #[test]
    fn test_render_color_text_reset() {
        let result = render(vec![
            Token::Tag(TagType::Color(Color::Named(NamedColor::Green))),
            Token::Text("go".into()),
            Token::Tag(TagType::Reset),
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
        let result = render(vec![Token::Tag(TagType::Color(Color::Ansi256(21)))]);
        assert_eq!(result, "\x1b[38;5;21m");
    }

    #[test]
    fn test_render_rgb_color_tag() {
        let result = render(vec![Token::Tag(TagType::Color(Color::Rgb(255, 0, 0)))]);
        assert_eq!(result, "\x1b[38;2;255;0;0m");
    }

    #[test]
    fn test_render_does_not_append_trailing_reset() {
        // render() itself never appends a reset; that is the caller's job (try_color)
        let result = render(vec![Token::Text("plain".into())]);
        assert!(!result.ends_with("\x1b[0m"));
    }
}

// Skipped (side effects): none: render() is a pure function.
