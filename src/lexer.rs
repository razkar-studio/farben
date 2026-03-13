use crate::{
    ansi::{Color, NamedColor},
    errors::LexError,
};

/// A text emphasis modifier supported by farben markup.
#[derive(Debug, PartialEq)]
pub(crate) enum EmphasisType {
    Dim,
    Italic,
    Underline,
    Bold,
    Strikethrough,
    Blink,
}

impl EmphasisType {
    /// Parses an emphasis keyword into an `EmphasisType`.
    ///
    /// Returns `None` if the string is not a recognized emphasis name.
    /// Matching is case-sensitive.
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "dim" => Some(Self::Dim),
            "italic" => Some(Self::Italic),
            "underline" => Some(Self::Underline),
            "bold" => Some(Self::Bold),
            "strikethrough" => Some(Self::Strikethrough),
            "blink" => Some(Self::Blink),
            _ => None,
        }
    }
}

/// The kind of styling operation a tag represents.
#[derive(Debug, PartialEq)]
pub(crate) enum TagType {
    /// Resets all active styles (`[/]`).
    Reset,
    /// Applies a text emphasis attribute.
    Emphasis(EmphasisType),
    /// Sets the foreground color.
    Color(Color),
}

/// A single unit produced by the tokenizer: either a styling tag or a run of plain text.
#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Tag(TagType),
    Text(String),
}

/// Parses a single whitespace-delimited tag part into a `TagType`.
///
/// Recognizes:
/// - `/` as a reset
/// - Named colors (`red`, `blue`, etc.)
/// - Emphasis keywords (`bold`, `italic`, etc.)
/// - `ansi(N)` for ANSI 256-palette colors
/// - `rgb(R,G,B)` for true-color values
///
/// # Errors
///
/// Returns `LexError::InvalidTag` if the part matches none of the above forms.
/// Returns `LexError::InvalidValue` if a numeric argument cannot be parsed.
/// Returns `LexError::InvalidArgumentCount` if `rgb(...)` does not receive exactly three values.
fn parse_part(part: &str) -> Result<TagType, LexError> {
    if part == "/" {
        Ok(TagType::Reset)
    } else if let Some(color) = NamedColor::from_str(part) {
        Ok(TagType::Color(Color::Named(color)))
    } else if let Some(emphasis) = EmphasisType::from_str(part) {
        Ok(TagType::Emphasis(emphasis))
    } else if let Some(ansi_val) = part.strip_prefix("ansi(").and_then(|s| s.strip_suffix(")")) {
        match ansi_val.trim().parse::<u8>() {
            Ok(code) => Ok(TagType::Color(Color::Ansi256(code))),
            Err(_) => Err(LexError::InvalidValue(ansi_val.to_string())),
        }
    } else if let Some(rgb_val) = part.strip_prefix("rgb(").and_then(|s| s.strip_suffix(")")) {
        let parts: Result<Vec<u8>, _> =
            rgb_val.split(',').map(|v| v.trim().parse::<u8>()).collect();
        match parts {
            Ok(v) if v.len() == 3 => Ok(TagType::Color(Color::Rgb(v[0], v[1], v[2]))),
            Ok(v) => Err(LexError::InvalidArgumentCount {
                expected: 3,
                got: v.len(),
            }),
            Err(_) => Err(LexError::InvalidValue(rgb_val.to_string())),
        }
    } else {
        Err(LexError::InvalidTag(part.to_string()))
    }
}

/// Splits a raw tag string on whitespace and parses each part into a `TagType`.
///
/// A tag like `"bold red"` produces two `TagType` values.
///
/// # Errors
///
/// Propagates any error from `parse_part`.
fn parse_tag(raw_tag: &str) -> Result<Vec<TagType>, LexError> {
    raw_tag.split_whitespace().map(parse_part).collect()
}

/// Tokenizes a farben markup string into a sequence of `Token`s.
///
/// Tags are delimited by `[` and `]`. A `[` preceded by `\` is treated as a literal
/// bracket rather than the start of a tag.
///
/// # Errors
///
/// Returns `LexError::UnclosedTag` if a `[` has no matching `]`.
/// Returns any error produced by `parse_tag` for malformed tag contents.
///
/// # Example
///
/// ```ignore
/// let tokens = tokenize("[red]hello")?;
/// // => [Token::Tag(TagType::Color(Color::Named(NamedColor::Red))), Token::Text("hello".into())]
/// ```
pub(crate) fn tokenize(input: impl Into<String>) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let input = input.into();
    let mut pos = 0;
    loop {
        let Some(starting) = input[pos..].find('[') else {
            if pos < input.len() {
                tokens.push(Token::Text(input[pos..].to_string()));
            }
            break;
        };
        let abs_starting = starting + pos;
        // escape logic
        if abs_starting > 0 && input[abs_starting - 1..abs_starting] == "\\".to_string() {
            let before = &input[pos..abs_starting - 1];
            if !before.is_empty() {
                tokens.push(Token::Text(before.to_string()));
            }
            tokens.push(Token::Text(String::from('[')));
            pos = abs_starting + 1;
            continue;
        }

        if pos != abs_starting {
            tokens.push(Token::Text(input[pos..abs_starting].to_string()));
        }

        let Some(closing) = input[abs_starting..].find(']') else {
            return Err(LexError::UnclosedTag);
        };
        let abs_closing = closing + abs_starting;
        let raw_tag = &input[abs_starting + 1..abs_closing];
        for tag in parse_tag(raw_tag)? {
            tokens.push(Token::Tag(tag));
        }
        pos = abs_closing + 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::{Color, NamedColor};

    // --- EmphasisType::from_str ---

    #[test]
    fn test_emphasis_from_str_all_known() {
        assert_eq!(EmphasisType::from_str("dim"), Some(EmphasisType::Dim));
        assert_eq!(EmphasisType::from_str("italic"), Some(EmphasisType::Italic));
        assert_eq!(
            EmphasisType::from_str("underline"),
            Some(EmphasisType::Underline)
        );
        assert_eq!(EmphasisType::from_str("bold"), Some(EmphasisType::Bold));
        assert_eq!(
            EmphasisType::from_str("strikethrough"),
            Some(EmphasisType::Strikethrough)
        );
        assert_eq!(EmphasisType::from_str("blink"), Some(EmphasisType::Blink));
    }

    #[test]
    fn test_emphasis_from_str_unknown_returns_none() {
        assert_eq!(EmphasisType::from_str("flash"), None);
    }

    #[test]
    fn test_emphasis_from_str_case_sensitive() {
        assert_eq!(EmphasisType::from_str("Bold"), None);
    }

    // --- parse_part ---

    #[test]
    fn test_parse_part_reset() {
        let result = parse_part("/");
        assert_eq!(result.unwrap(), TagType::Reset);
    }

    #[test]
    fn test_parse_part_named_color() {
        let result = parse_part("red");
        assert_eq!(
            result.unwrap(),
            TagType::Color(Color::Named(NamedColor::Red))
        );
    }

    #[test]
    fn test_parse_part_emphasis_bold() {
        let result = parse_part("bold");
        assert_eq!(result.unwrap(), TagType::Emphasis(EmphasisType::Bold));
    }

    #[test]
    fn test_parse_part_ansi256_valid() {
        let result = parse_part("ansi(200)");
        assert_eq!(result.unwrap(), TagType::Color(Color::Ansi256(200)));
    }

    #[test]
    fn test_parse_part_ansi256_with_whitespace() {
        let result = parse_part("ansi( 42 )");
        assert_eq!(result.unwrap(), TagType::Color(Color::Ansi256(42)));
    }

    #[test]
    fn test_parse_part_ansi256_invalid_value() {
        let result = parse_part("ansi(abc)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_part_rgb_valid() {
        let result = parse_part("rgb(255,128,0)");
        assert_eq!(result.unwrap(), TagType::Color(Color::Rgb(255, 128, 0)));
    }

    #[test]
    fn test_parse_part_rgb_with_spaces() {
        let result = parse_part("rgb( 10 , 20 , 30 )");
        assert_eq!(result.unwrap(), TagType::Color(Color::Rgb(10, 20, 30)));
    }

    #[test]
    fn test_parse_part_rgb_wrong_arg_count() {
        let result = parse_part("rgb(1,2)");
        assert!(result.is_err());
        if let Err(crate::errors::LexError::InvalidArgumentCount { expected, got }) = result {
            assert_eq!(expected, 3);
            assert_eq!(got, 2);
        }
    }

    #[test]
    fn test_parse_part_rgb_invalid_value() {
        let result = parse_part("rgb(r,g,b)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_part_unknown_tag_returns_error() {
        let result = parse_part("fuchsia");
        assert!(result.is_err());
    }

    // --- tokenize ---

    #[test]
    fn test_tokenize_plain_text() {
        let result = tokenize("hello world");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens, vec![Token::Text("hello world".into())]);
    }

    #[test]
    fn test_tokenize_empty_string() {
        let result = tokenize("");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_tokenize_single_color_tag() {
        let result = tokenize("[red]text");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Color(Color::Named(NamedColor::Red))),
                Token::Text("text".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_reset_tag() {
        let result = tokenize("[/]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![Token::Tag(TagType::Reset)]);
    }

    #[test]
    fn test_tokenize_compound_tag() {
        let result = tokenize("[bold red]hi");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
                Token::Tag(TagType::Color(Color::Named(NamedColor::Red))),
                Token::Text("hi".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_escaped_bracket_at_start() {
        let result = tokenize("\\[not a tag]");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(
            tokens,
            vec![Token::Text("[".into()), Token::Text("not a tag]".into()),]
        );
    }

    #[test]
    fn test_tokenize_escaped_bracket_with_prefix() {
        let result = tokenize("before\\[not a tag]");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Text("before".into()),
                Token::Text("[".into()),
                Token::Text("not a tag]".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_unclosed_tag_returns_error() {
        let result = tokenize("[red");
        assert!(result.is_err());
    }

    #[test]
    fn test_tokenize_invalid_tag_name_returns_error() {
        let result = tokenize("[fuchsia]");
        assert!(result.is_err());
    }

    #[test]
    fn test_tokenize_text_before_and_after_tag() {
        let result = tokenize("before[red]after");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Text("before".into()),
                Token::Tag(TagType::Color(Color::Named(NamedColor::Red))),
                Token::Text("after".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_ansi256_tag() {
        let result = tokenize("[ansi(1)]text");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens[0], Token::Tag(TagType::Color(Color::Ansi256(1))));
    }

    #[test]
    fn test_tokenize_rgb_tag() {
        let result = tokenize("[rgb(255,0,128)]text");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color(Color::Rgb(255, 0, 128)))
        );
    }
}

// Skipped (side effects): none: all functions in lexer.rs are pure.
