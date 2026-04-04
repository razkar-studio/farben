//! Tokenizer for farben markup strings.
//!
//! Parses bracket-delimited tag syntax (`[bold red]text[/]`) into a flat sequence of
//! [`Token`] values. Each token is either a [`Token::Tag`] carrying styling information
//! or a [`Token::Text`] carrying a run of literal characters.
//!
//! The main entry point is [`tokenize`]. The lower-level [`parse_tag`] and [`parse_part`]
//! functions handle individual tag strings and are not part of the public API.

use crate::{
    ansi::{Color, Ground, NamedColor, Style},
    errors::LexError,
    registry::search_registry,
};

/// A text emphasis modifier supported by farben markup.
#[derive(Debug, PartialEq, Clone)]
pub enum EmphasisType {
    /// Reduced intensity (SGR 2).
    Dim,
    /// Italic text (SGR 3).
    Italic,
    /// Underlined text (SGR 4).
    Underline,
    /// Bold text (SGR 1).
    Bold,
    /// Crossed-out text (SGR 9).
    Strikethrough,
    /// Blinking text (SGR 5). Terminal support varies.
    Blink,
}

/// The kind of styling operation a tag represents.
#[derive(Debug, PartialEq, Clone)]
pub enum TagType {
    /// Resets all active styles (`[/]`).
    Reset(Option<Box<TagType>>),
    /// Applies a text emphasis attribute.
    Emphasis(EmphasisType),
    /// Sets a foreground or background color.
    Color { color: Color, ground: Ground },
    /// A literal prefix string injected before the style sequence by the registry.
    Prefix(String),
}

/// A single unit produced by the tokenizer: either a styling tag or a run of plain text.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// A parsed styling tag.
    Tag(TagType),
    /// A run of plain text with no markup.
    Text(String),
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

/// Expands a [`Style`] from the registry into its equivalent sequence of [`TagType`] values.
///
/// A `Prefix` tag is always prepended first, if one is set. A `reset` style short-circuits
/// after the prefix: no emphasis or color tags are emitted.
fn style_to_tags(style: Style) -> Vec<TagType> {
    let mut res: Vec<TagType> = Vec::new();
    let prefix = style.prefix;

    if style.reset {
        if let Some(p) = prefix {
            res.push(TagType::Prefix(p));
        }
        res.push(TagType::Reset(None));
        return res;
    }

    for (enabled, tag) in [
        (style.bold, TagType::Emphasis(EmphasisType::Bold)),
        (style.blink, TagType::Emphasis(EmphasisType::Blink)),
        (style.dim, TagType::Emphasis(EmphasisType::Dim)),
        (style.italic, TagType::Emphasis(EmphasisType::Italic)),
        (
            style.strikethrough,
            TagType::Emphasis(EmphasisType::Strikethrough),
        ),
        (style.underline, TagType::Emphasis(EmphasisType::Underline)),
    ] {
        if enabled {
            res.push(tag);
        }
    }

    if let Some(fg) = style.fg {
        res.push(TagType::Color {
            color: fg,
            ground: Ground::Foreground,
        })
    }
    if let Some(bg) = style.bg {
        res.push(TagType::Color {
            color: bg,
            ground: Ground::Background,
        })
    }

    if let Some(p) = prefix {
        res.push(TagType::Prefix(p));
    }

    res
}

/// Parses a single whitespace-delimited tag part into a `TagType`.
///
/// Recognizes:
/// - `/` as a reset
/// - Named colors (`red`, `blue`, etc.)
/// - Emphasis keywords (`bold`, `italic`, etc.)
/// - `ansi(N)` for ANSI 256-palette colors
/// - `rgb(R,G,B)` for true-color values
/// - A named style from the registry as a fallback
///
/// Parts may be prefixed with `bg:` to target the background ground, or `fg:` to
/// explicitly target the foreground. Unprefixed color parts default to foreground.
///
/// # Errors
///
/// Returns `LexError::InvalidTag` if the part matches none of the above forms.
/// Returns `LexError::InvalidValue` if a numeric argument cannot be parsed.
/// Returns `LexError::InvalidArgumentCount` if `rgb(...)` does not receive exactly three values.
fn parse_part(part: &str, position: usize) -> Result<Vec<TagType>, LexError> {
    let (ground, part) = if let Some(rest) = part.strip_prefix("bg:") {
        (Ground::Background, rest)
    } else if let Some(rest) = part.strip_prefix("fg:") {
        (Ground::Foreground, rest)
    } else {
        (Ground::Foreground, part)
    };
    if let Some(remainder) = part.strip_prefix('/') {
        if remainder.is_empty() {
            Ok(vec![TagType::Reset(None)])
        } else {
            let inner = parse_part(remainder, position + 1)?;
            match inner.as_slice() {
                [tag] => match tag {
                    TagType::Reset(_) | TagType::Prefix(_) => {
                        Err(LexError::InvalidResetTarget(position))
                    }
                    _ => Ok(vec![TagType::Reset(Some(Box::new(tag.clone())))]),
                },
                _ => Err(LexError::InvalidTag {
                    tag_content: part.to_string(),
                    position,
                }),
            }
        }
    } else if let Some(color) = NamedColor::from_str(part) {
        Ok(vec![TagType::Color {
            color: Color::Named(color),
            ground,
        }])
    } else if let Some(emphasis) = EmphasisType::from_str(part) {
        Ok(vec![TagType::Emphasis(emphasis)])
    } else if part.starts_with("ansi(") && !part.ends_with(')') {
        Err(LexError::UnclosedValue(position))
    } else if let Some(ansi_val) = part.strip_prefix("ansi(").and_then(|s| s.strip_suffix(")")) {
        match ansi_val.trim().parse::<u8>() {
            Ok(code) => Ok(vec![TagType::Color {
                color: Color::Ansi256(code),
                ground,
            }]),
            Err(_) => Err(LexError::InvalidValue {
                value: ansi_val.to_string(),
                position,
            }),
        }
    } else if part.starts_with("rgb(") && !part.ends_with(')') {
        Err(LexError::UnclosedValue(position))
    } else if let Some(rgb_val) = part.strip_prefix("rgb(").and_then(|s| s.strip_suffix(")")) {
        let parts: Result<Vec<u8>, _> =
            rgb_val.split(',').map(|v| v.trim().parse::<u8>()).collect();
        match parts {
            Ok(v) if v.len() == 3 => Ok(vec![TagType::Color {
                color: Color::Rgb(v[0], v[1], v[2]),
                ground,
            }]),
            Ok(v) => Err(LexError::InvalidArgumentCount {
                expected: 3,
                got: v.len(),
                position,
            }),
            Err(_) => Err(LexError::InvalidValue {
                value: rgb_val.to_string(),
                position,
            }),
        }
    } else {
        match search_registry(part) {
            Ok(style) => Ok(style_to_tags(style)),
            Err(_) => Err(LexError::InvalidTag {
                tag_content: part.to_string(),
                position,
            }),
        }
    }
}

/// Splits a raw tag string on whitespace and parses each part into a `TagType`.
///
/// A tag like `"bold red"` produces two `TagType` values. Whitespace between parts
/// is consumed and does not appear in the output.
///
/// # Errors
///
/// Propagates any error from `parse_part`.
fn parse_tag(raw_tag: &str, tag_start: usize) -> Result<Vec<TagType>, LexError> {
    let mut result = Vec::new();
    let mut search_from = 0;

    for part in raw_tag.split_whitespace() {
        let part_offset = raw_tag[search_from..].find(part).unwrap() + search_from;
        let abs_position = tag_start + part_offset;
        result.extend(parse_part(part, abs_position)?);
        search_from = part_offset + part.len();
    }

    Ok(result)
}

/// Tokenizes a farben markup string into a sequence of `Token`s.
///
/// Tags are delimited by `[` and `]`. A `[` preceded by `\` is treated as a literal
/// bracket rather than the start of a tag. Text between tags is emitted as
/// [`Token::Text`]; tags are parsed and emitted as [`Token::Tag`].
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
/// // => [Token::Tag(TagType::Color { color: Color::Named(NamedColor::Red), ground: Ground::Foreground }),
/// //     Token::Text("hello".into())]
/// ```
pub fn tokenize(input: impl Into<String>) -> Result<Vec<Token>, LexError> {
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
        // wtf does this mean
        if abs_starting > 0 && input.as_bytes().get(abs_starting.wrapping_sub(1)) == Some(&b'\\') {
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
            return Err(LexError::UnclosedTag(abs_starting));
        };
        let abs_closing = closing + abs_starting;
        let raw_tag = &input[abs_starting + 1..abs_closing];
        for tag in parse_tag(raw_tag, abs_starting)? {
            tokens.push(Token::Tag(tag));
        }
        pos = abs_closing + 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::{Color, Ground, NamedColor};

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
        assert_eq!(parse_part("/", 0).unwrap(), vec![TagType::Reset(None)]);
    }

    #[test]
    fn test_parse_part_named_color_foreground_default() {
        assert_eq!(
            parse_part("red", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_named_color_explicit_fg() {
        assert_eq!(
            parse_part("fg:red", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_named_color_bg() {
        assert_eq!(
            parse_part("bg:red", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Named(NamedColor::Red),
                ground: Ground::Background,
            }]
        );
    }

    #[test]
    fn test_parse_part_emphasis_bold() {
        assert_eq!(
            parse_part("bold", 0).unwrap(),
            vec![TagType::Emphasis(EmphasisType::Bold)]
        );
    }

    #[test]
    fn test_parse_part_ansi256_valid() {
        assert_eq!(
            parse_part("ansi(200)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Ansi256(200),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_ansi256_bg() {
        assert_eq!(
            parse_part("bg:ansi(200)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Ansi256(200),
                ground: Ground::Background,
            }]
        );
    }

    #[test]
    fn test_parse_part_ansi256_with_whitespace() {
        assert_eq!(
            parse_part("ansi( 42 )", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Ansi256(42),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_ansi256_invalid_value() {
        assert!(parse_part("ansi(abc)", 0).is_err());
    }

    #[test]
    fn test_parse_part_rgb_valid() {
        assert_eq!(
            parse_part("rgb(255,128,0)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(255, 128, 0),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_rgb_bg() {
        assert_eq!(
            parse_part("bg:rgb(255,128,0)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(255, 128, 0),
                ground: Ground::Background,
            }]
        );
    }

    #[test]
    fn test_parse_part_rgb_with_spaces() {
        assert_eq!(
            parse_part("rgb( 10 , 20 , 30 )", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(10, 20, 30),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_rgb_wrong_arg_count() {
        let result = parse_part("rgb(1,2)", 0);
        assert!(result.is_err());
        if let Err(crate::errors::LexError::InvalidArgumentCount { expected, got, .. }) = result {
            assert_eq!(expected, 3);
            assert_eq!(got, 2);
        }
    }

    #[test]
    fn test_parse_part_rgb_invalid_value() {
        assert!(parse_part("rgb(r,g,b)", 0).is_err());
    }

    #[test]
    fn test_parse_part_unknown_tag_returns_error() {
        assert!(parse_part("fuchsia", 0).is_err());
    }

    // --- tokenize ---

    #[test]
    fn test_tokenize_plain_text() {
        let tokens = tokenize("hello world").unwrap();
        assert_eq!(tokens, vec![Token::Text("hello world".into())]);
    }

    #[test]
    fn test_tokenize_empty_string() {
        assert!(tokenize("").unwrap().is_empty());
    }

    #[test]
    fn test_tokenize_single_color_tag() {
        let tokens = tokenize("[red]text").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Color {
                    color: Color::Named(NamedColor::Red),
                    ground: Ground::Foreground
                }),
                Token::Text("text".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_bg_color_tag() {
        let tokens = tokenize("[bg:red]text").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Color {
                    color: Color::Named(NamedColor::Red),
                    ground: Ground::Background
                }),
                Token::Text("text".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_fg_and_bg_in_same_bracket() {
        let tokens = tokenize("[fg:white bg:blue]text").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Color {
                    color: Color::Named(NamedColor::White),
                    ground: Ground::Foreground
                }),
                Token::Tag(TagType::Color {
                    color: Color::Named(NamedColor::Blue),
                    ground: Ground::Background
                }),
                Token::Text("text".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_reset_tag() {
        assert_eq!(
            tokenize("[/]").unwrap(),
            vec![Token::Tag(TagType::Reset(None))]
        );
    }

    #[test]
    fn test_tokenize_compound_tag() {
        let tokens = tokenize("[bold red]hi").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
                Token::Tag(TagType::Color {
                    color: Color::Named(NamedColor::Red),
                    ground: Ground::Foreground
                }),
                Token::Text("hi".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_escaped_bracket_at_start() {
        let tokens = tokenize("\\[not a tag]").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Text("[".into()), Token::Text("not a tag]".into()),]
        );
    }

    #[test]
    fn test_tokenize_escaped_bracket_with_prefix() {
        let tokens = tokenize("before\\[not a tag]").unwrap();
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
        assert!(tokenize("[red").is_err());
    }

    #[test]
    fn test_tokenize_invalid_tag_name_returns_error() {
        assert!(tokenize("[fuchsia]").is_err());
    }

    #[test]
    fn test_tokenize_text_before_and_after_tag() {
        let tokens = tokenize("before[red]after").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Text("before".into()),
                Token::Tag(TagType::Color {
                    color: Color::Named(NamedColor::Red),
                    ground: Ground::Foreground
                }),
                Token::Text("after".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_ansi256_tag() {
        let tokens = tokenize("[ansi(1)]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Ansi256(1),
                ground: Ground::Foreground,
            })
        );
    }

    #[test]
    fn test_tokenize_rgb_tag() {
        let tokens = tokenize("[rgb(255,0,128)]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(255, 0, 128),
                ground: Ground::Foreground,
            })
        );
    }

    #[test]
    fn test_tokenize_bg_rgb_tag() {
        let tokens = tokenize("[bg:rgb(0,255,0)]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(0, 255, 0),
                ground: Ground::Background,
            })
        );
    }

    #[test]
    fn test_parse_part_custom_style_from_registry() {
        crate::registry::insert_style("danger", crate::ansi::Style::parse("[bold red]").unwrap());
        let result = parse_part("danger", 0).unwrap();
        assert_eq!(
            result,
            vec![
                TagType::Emphasis(EmphasisType::Bold),
                TagType::Color {
                    color: Color::Named(NamedColor::Red),
                    ground: Ground::Foreground
                },
            ]
        );
    }
}
