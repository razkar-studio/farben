//! Tokenizer for farben markup strings.
//!
//! Parses bracket-delimited tag syntax (`[bold red]text[/]`) into a flat sequence of
//! [`Token`] values. Each token is either a [`Token::Tag`] carrying styling information
//! or a [`Token::Text`] carrying a run of literal characters.
//!
//! The main entry point is [`tokenize`]. The lower-level `parse_tag` and `parse_part`
//! functions handle individual tag strings and are not part of the public API.

use std::borrow::Cow;

use crate::{
    ansi::{Color, Ground, NamedColor, Style},
    errors::LexError,
    registry::search_registry,
};

/// A text emphasis modifier supported by farben markup.
#[derive(Debug, PartialEq, Clone)]
pub enum EmphasisType {
    /// Reduced intensity (SGR 2). Lower intensity.
    Dim,
    /// Italic text (SGR 3). Slanted text.
    Italic,
    /// Underlined text (SGR 4). Single underline.
    Underline,
    /// Double-underlined text (SGR 21). Two lines.
    DoubleUnderline,
    /// Bold text (SGR 1). Increased intensity.
    Bold,
    /// Crossed-out text (SGR 9). Strikethrough.
    Strikethrough,
    /// Blinking text (SGR 5). Slow blink.
    Blink,
    /// Overlined text (SGR 53). Line above text.
    Overline,
    /// Invisible text (SGR 8). Hidden but selectable.
    Invisible,
    /// Reverse video (SGR 7). Swaps foreground and background.
    Reverse,
    /// Rapid blinking (SGR 6). Faster than Blink.
    RapidBlink,
}

/// The kind of styling operation a tag represents.
#[derive(Debug, PartialEq, Clone)]
pub enum TagType {
    /// Resets all active styles (`[/]`).
    ResetAll,
    /// Resets one specific active style, then re-applies the rest.
    /// Example: `[/bold]` resets bold but keeps other active styles.
    ResetOne(Box<TagType>),
    /// Applies a text emphasis attribute.
    Emphasis(EmphasisType),
    /// Sets a foreground or background color.
    Color {
        /// The color to apply.
        color: Color,
        /// Whether foreground or background.
        ground: Ground,
    },
    /// A literal prefix string injected before the style sequence by the registry.
    Prefix(String),
}

/// A single unit produced by the tokenizer: either a styling tag or a run of plain text.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// A parsed styling tag (color, emphasis, reset).
    Tag(TagType),
    /// A run of plain text with no markup.
    Text(Cow<'static, str>),
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
            "double-underline" => Some(Self::DoubleUnderline),
            "bold" => Some(Self::Bold),
            "strikethrough" => Some(Self::Strikethrough),
            "blink" => Some(Self::Blink),
            "overline" => Some(Self::Overline),
            "invisible" => Some(Self::Invisible),
            "reverse" => Some(Self::Reverse),
            "rapid-blink" => Some(Self::RapidBlink),
            _ => None,
        }
    }
}

/// Expands a [`Style`] from the registry into its equivalent sequence of [`TagType`] values.
///
/// A `Prefix` tag is always prepended first, if one is set. A `reset` style short-circuits
/// after the prefix: no emphasis or color tags are emitted.
fn style_to_tags(style: &Style) -> Vec<TagType> {
    let mut res: Vec<TagType> = Vec::new();
    let prefix = style.prefix.clone();

    if style.reset {
        if let Some(p) = prefix {
            res.push(TagType::Prefix(p));
        }
        res.push(TagType::ResetAll);
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
        (
            style.double_underline,
            TagType::Emphasis(EmphasisType::DoubleUnderline),
        ),
        (style.overline, TagType::Emphasis(EmphasisType::Overline)),
        (style.invisible, TagType::Emphasis(EmphasisType::Invisible)),
        (style.reverse, TagType::Emphasis(EmphasisType::Reverse)),
        (
            style.rapid_blink,
            TagType::Emphasis(EmphasisType::RapidBlink),
        ),
    ] {
        if enabled {
            res.push(tag);
        }
    }

    if let Some(fg) = style.fg.clone() {
        res.push(TagType::Color {
            color: fg,
            ground: Ground::Foreground,
        });
    }
    if let Some(bg) = style.bg.clone() {
        res.push(TagType::Color {
            color: bg,
            ground: Ground::Background,
        });
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
/// - `hsl(H,S,L)` for HSL colors (H=0-360, S=0-100, L=0-100)
/// - `#fff` / `#ffffff` for hex colors
/// - A named style from the registry as a fallback
///
/// Parts may be prefixed with `bg:` to target the background ground, or `fg:` to
/// explicitly target the foreground. Unprefixed color parts default to foreground.
///
/// # Errors
///
/// Returns `LexError::InvalidTag` if the part matches none of the above forms.
/// Returns `LexError::InvalidValue` if a numeric argument cannot be parsed.
/// Returns `LexError::InvalidArgumentCount` if `rgb(...)` or `hsl(...)` does not receive exactly three values.
#[allow(clippy::too_many_lines)]
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
            Ok(vec![TagType::ResetAll])
        } else {
            let inner = parse_part(remainder, position + 1)?;
            if let [tag] = inner.as_slice() {
                match tag {
                    TagType::ResetAll | TagType::ResetOne(_) | TagType::Prefix(_) => {
                        Err(LexError::InvalidResetTarget(position))
                    }
                    _ => Ok(vec![TagType::ResetOne(Box::new(tag.clone()))]),
                }
            } else {
                let resets: Vec<TagType> = inner
                    .iter()
                    .filter(|t| {
                        !matches!(
                            t,
                            TagType::Prefix(_) | TagType::ResetAll | TagType::ResetOne(_)
                        )
                    })
                    .map(|t| TagType::ResetOne(Box::new(t.clone())))
                    .collect();
                if resets.is_empty() {
                    Err(LexError::InvalidResetTarget(position))
                } else {
                    Ok(resets)
                }
            }
        }
    } else if let Some(color) = NamedColor::from_str(part) {
        Ok(vec![TagType::Color {
            color: Color::Named(color),
            ground,
        }])
    } else if let Some(emphasis) = EmphasisType::from_str(part) {
        Ok(vec![TagType::Emphasis(emphasis)])
    } else if let Some(rest) = part.strip_prefix("ansi(") {
        if !rest.ends_with(')') {
            return Err(LexError::UnclosedValue(position));
        }
        let ansi_val = &rest[..rest.len() - 1];
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
    } else if let Some(rest) = part.strip_prefix("rgb(") {
        if !rest.ends_with(')') {
            return Err(LexError::UnclosedValue(position));
        }
        let rgb_val = &rest[..rest.len() - 1];
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
    } else if let Some(rest) = part.strip_prefix("hsl(") {
        if !rest.ends_with(')') {
            return Err(LexError::UnclosedValue(position));
        }
        let inner = &rest[..rest.len() - 1];
        let raw: Vec<&str> = inner.split(',').collect();
        if raw.len() != 3 {
            return Err(LexError::InvalidArgumentCount {
                expected: 3,
                got: raw.len(),
                position,
            });
        }
        let vals: Vec<f64> = raw
            .iter()
            .map(|v| v.trim().parse::<f64>())
            .collect::<Result<_, _>>()
            .map_err(|_| LexError::InvalidValue {
                value: inner.to_string(),
                position,
            })?;
        if !(0.0..=360.0).contains(&vals[0]) {
            return Err(LexError::InvalidValue {
                value: format!("hue {} out of range (0-360)", vals[0]),
                position,
            });
        }
        if !(0.0..=100.0).contains(&vals[1]) {
            return Err(LexError::InvalidValue {
                value: format!("saturation {} out of range (0-100)", vals[1]),
                position,
            });
        }
        if !(0.0..=100.0).contains(&vals[2]) {
            return Err(LexError::InvalidValue {
                value: format!("lightness {} out of range (0-100)", vals[2]),
                position,
            });
        }
        let (r, g, b) = hsl_to_rgb(vals[0], vals[1], vals[2]);
        Ok(vec![TagType::Color {
            color: Color::Rgb(r, g, b),
            ground,
        }])
    } else if let Some(hex) = part.strip_prefix('#') {
        if hex.is_empty() {
            return Err(LexError::InvalidValue {
                value: String::new(),
                position,
            });
        }
        let (r, g, b) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| {
                    LexError::InvalidValue {
                        value: hex.to_string(),
                        position,
                    }
                })?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| {
                    LexError::InvalidValue {
                        value: hex.to_string(),
                        position,
                    }
                })?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| {
                    LexError::InvalidValue {
                        value: hex.to_string(),
                        position,
                    }
                })?;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| LexError::InvalidValue {
                    value: hex.to_string(),
                    position,
                })?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| LexError::InvalidValue {
                    value: hex.to_string(),
                    position,
                })?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| LexError::InvalidValue {
                    value: hex.to_string(),
                    position,
                })?;
                (r, g, b)
            }
            _ => {
                return Err(LexError::InvalidValue {
                    value: hex.to_string(),
                    position,
                });
            }
        };
        Ok(vec![TagType::Color {
            color: Color::Rgb(r, g, b),
            ground,
        }])
    } else {
        match search_registry(part) {
            Ok(style) => Ok(style_to_tags(&style)),
            Err(_) => Err(LexError::InvalidTag {
                tag_content: part.to_string(),
                position,
            }),
        }
    }
}

/// Splits a raw tag string on whitespace, but not within `(…)` groups.
///
/// This allows constructs like `rgb(1, 2, 3)` or `ansi( 93 )` to survive as a
/// single part instead of being split on the inner whitespace.
fn split_tag_parts(raw_tag: &str) -> Vec<(usize, &str)> {
    let mut parts = Vec::new();
    let mut part_start = 0;
    let mut depth = 0usize;

    for (i, c) in raw_tag.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            c if c.is_whitespace() && depth == 0 => {
                if i > part_start {
                    parts.push((part_start, &raw_tag[part_start..i]));
                }
                part_start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    if part_start < raw_tag.len() {
        parts.push((part_start, &raw_tag[part_start..]));
    }
    parts
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

    for (offset, part) in split_tag_parts(raw_tag) {
        let abs_position = tag_start + offset;
        result.extend(parse_part(part, abs_position)?);
    }

    Ok(result)
}

/// Tokenizes a farben markup string into a sequence of `Token`s.
///
/// Tags are delimited by `[` and `]`. Use `[[` to emit a literal `[` and `]]` to emit
/// a literal `]`. Text between tags is emitted as [`Token::Text`]; tags are parsed and
/// emitted as [`Token::Tag`].
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
    let input = input.into();
    let mut tokens: Vec<Token> = Vec::with_capacity(input.len() / 4);
    let mut pos = 0;
    loop {
        let next = [
            input[pos..].find('[').map(|i| (i, b'[')),
            input[pos..].find(']').map(|i| (i, b']')),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(i, _)| *i);

        let Some((starting, kind)) = next else {
            if pos < input.len() {
                tokens.push(Token::Text(Cow::Owned(input[pos..].to_string())));
            }
            break;
        };
        let abs_starting = starting + pos;

        if kind == b']' {
            if pos != abs_starting {
                tokens.push(Token::Text(Cow::Owned(
                    input[pos..abs_starting].to_string(),
                )));
            }
            if input.as_bytes().get(abs_starting + 1) == Some(&b']') {
                tokens.push(Token::Text(Cow::Borrowed("]")));
                pos = abs_starting + 2;
            } else {
                tokens.push(Token::Text(Cow::Borrowed("]")));
                pos = abs_starting + 1;
            }
            continue;
        }

        // kind == b'['
        if abs_starting > 0 && input.as_bytes().get(abs_starting.wrapping_sub(1)) == Some(&b'\x1b')
        {
            tokens.push(Token::Text(Cow::Owned(
                input[pos..=abs_starting].to_string(),
            )));
            pos = abs_starting + 1;
            continue;
        }

        if input.as_bytes().get(abs_starting + 1) == Some(&b'[') {
            let before = &input[pos..abs_starting];
            if !before.is_empty() {
                tokens.push(Token::Text(Cow::Owned(before.to_string())));
            }
            tokens.push(Token::Text(Cow::Borrowed("[")));
            pos = abs_starting + 2;
            continue;
        }

        if pos != abs_starting {
            tokens.push(Token::Text(Cow::Owned(
                input[pos..abs_starting].to_string(),
            )));
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

/// Converts HSL (hue, saturation, lightness) to an RGB triple (0–255 each).
///
/// `hue` in [0, 360), `saturation` in [0, 100], `lightness` in [0, 100].
///
/// The cast from `f64` to `u16` / `u8` is intentional: the input values are validated
/// to be in range before this function is called, so no truncation or sign loss occurs.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn hsl_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (u8, u8, u8) {
    let saturation = saturation / 100.0;
    let lightness = lightness / 100.0;

    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let match_lightness = lightness - chroma / 2.0;

    let (red, green, blue) = match hue as u16 % 360 {
        0..=59 => (chroma, x, 0.0),
        60..=119 => (x, chroma, 0.0),
        120..=179 => (0.0, chroma, x),
        180..=239 => (0.0, x, chroma),
        240..=299 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };

    (
        ((red + match_lightness) * 255.0).round() as u8,
        ((green + match_lightness) * 255.0).round() as u8,
        ((blue + match_lightness) * 255.0).round() as u8,
    )
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
        assert_eq!(parse_part("/", 0).unwrap(), vec![TagType::ResetAll]);
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

    // --- hex (#fff / #ffffff) ---

    #[test]
    fn test_parse_part_hex_6digit() {
        assert_eq!(
            parse_part("#ff0000", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_hex_3digit() {
        assert_eq!(
            parse_part("#f00", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_hex_bg() {
        assert_eq!(
            parse_part("bg:#ffffff", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(255, 255, 255),
                ground: Ground::Background,
            }]
        );
    }

    #[test]
    fn test_parse_part_hex_invalid_length() {
        assert!(parse_part("#ff", 0).is_err());
        assert!(parse_part("#ffff", 0).is_err());
        assert!(parse_part("#fffffff", 0).is_err());
    }

    #[test]
    fn test_parse_part_hex_invalid_chars() {
        assert!(parse_part("#xyz", 0).is_err());
        assert!(parse_part("#xyzzzz", 0).is_err());
    }

    #[test]
    fn test_parse_part_hex_empty() {
        assert!(parse_part("#", 0).is_err());
    }

    // --- hsl(H,S,L) ---

    #[test]
    fn test_parse_part_hsl_red() {
        assert_eq!(
            parse_part("hsl(0,100,50)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_hsl_green() {
        assert_eq!(
            parse_part("hsl(120,100,50)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(0, 255, 0),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_hsl_blue() {
        assert_eq!(
            parse_part("hsl(240,100,50)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(0, 0, 255),
                ground: Ground::Foreground,
            }]
        );
    }

    #[test]
    fn test_parse_part_hsl_bg() {
        assert_eq!(
            parse_part("bg:hsl(0,100,50)", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Background,
            }]
        );
    }

    #[test]
    fn test_parse_part_hsl_wrong_arg_count() {
        let result = parse_part("hsl(0,50)", 0);
        assert!(result.is_err());
        if let Err(crate::errors::LexError::InvalidArgumentCount { expected, got, .. }) = result {
            assert_eq!(expected, 3);
            assert_eq!(got, 2);
        }
    }

    #[test]
    fn test_parse_part_hsl_hue_out_of_range() {
        let result = parse_part("hsl(400,50,50)", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_part_hsl_sat_out_of_range() {
        let result = parse_part("hsl(0,150,50)", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_part_hsl_light_out_of_range() {
        let result = parse_part("hsl(0,50,110)", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_part_hsl_invalid_value() {
        assert!(parse_part("hsl(a,b,c)", 0).is_err());
    }

    #[test]
    fn test_parse_part_hsl_unclosed() {
        assert!(parse_part("hsl(0,50,50", 0).is_err());
    }

    #[test]
    fn test_parse_part_hsl_with_spaces() {
        assert_eq!(
            parse_part("hsl( 120 , 100 , 50 )", 0).unwrap(),
            vec![TagType::Color {
                color: Color::Rgb(0, 255, 0),
                ground: Ground::Foreground,
            }]
        );
    }

    // --- tokenize ---

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
            vec![Token::Tag(TagType::ResetAll)]
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
    fn test_tokenize_double_bracket_escape() {
        let tokens = tokenize("[[not a tag]").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Text("[".into()),
                Token::Text("not a tag".into()),
                Token::Text("]".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_double_bracket_escape_with_prefix() {
        let tokens = tokenize("before[[not a tag]").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Text("before".into()),
                Token::Text("[".into()),
                Token::Text("not a tag".into()),
                Token::Text("]".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_double_bracket_symmetric() {
        let tokens = tokenize("[[thing]]").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Text("[".into()),
                Token::Text("thing".into()),
                Token::Text("]".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_bare_close_bracket_is_text() {
        let tokens = tokenize("hello]world").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Text("hello".into()),
                Token::Text("]".into()),
                Token::Text("world".into()),
            ]
        );
    }

    #[test]
    fn test_tokenize_double_close_bracket_emits_one() {
        let tokens = tokenize("]]").unwrap();
        assert_eq!(tokens, vec![Token::Text("]".into())]);
    }

    #[test]
    fn test_tokenize_triple_close_bracket_emits_two() {
        let tokens = tokenize("]]]").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Text("]".into()), Token::Text("]".into())]
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
    fn test_split_tag_parts_simple() {
        assert_eq!(
            split_tag_parts("bold red"),
            vec![(0usize, "bold"), (5, "red")]
        );
    }

    #[test]
    fn test_split_tag_parts_respects_parens() {
        assert_eq!(
            split_tag_parts("rgb(1, 2, 3)"),
            vec![(0usize, "rgb(1, 2, 3)")]
        );
    }

    #[test]
    fn test_split_tag_parts_mixed() {
        assert_eq!(
            split_tag_parts("bold rgb(255, 128, 0)"),
            vec![(0usize, "bold"), (5, "rgb(255, 128, 0)")]
        );
    }

    #[test]
    fn test_split_tag_parts_ansi_with_spaces() {
        assert_eq!(
            split_tag_parts("fg:ansi( 93 )"),
            vec![(0usize, "fg:ansi( 93 )")]
        );
    }

    #[test]
    fn test_tokenize_rgb_with_spaces_inside_parens() {
        let tokens = tokenize("[rgb(1, 2, 3)]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(1, 2, 3),
                ground: Ground::Foreground,
            })
        );
    }

    #[test]
    fn test_tokenize_mixed_bold_rgb_with_spaces() {
        let tokens = tokenize("[bold rgb(255, 128, 0)]text").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
                Token::Tag(TagType::Color {
                    color: Color::Rgb(255, 128, 0),
                    ground: Ground::Foreground,
                }),
                Token::Text("text".into()),
            ]
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

    // --- tokenization: hex ---

    #[test]
    fn test_tokenize_hex_tag() {
        let tokens = tokenize("[#ff0000]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Foreground,
            })
        );
    }

    #[test]
    fn test_tokenize_hex_3digit_tag() {
        let tokens = tokenize("[#f00]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Foreground,
            })
        );
    }

    #[test]
    fn test_tokenize_bg_hex_tag() {
        let tokens = tokenize("[bg:#fff]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(255, 255, 255),
                ground: Ground::Background,
            })
        );
    }

    // --- tokenization: hsl ---

    #[test]
    fn test_tokenize_hsl_tag() {
        let tokens = tokenize("[hsl(0,100,50)]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Foreground,
            })
        );
    }

    #[test]
    fn test_tokenize_bg_hsl_tag() {
        let tokens = tokenize("[bg:hsl(0,100,50)]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(255, 0, 0),
                ground: Ground::Background,
            })
        );
    }

    #[test]
    fn test_tokenize_hsl_with_spaces() {
        let tokens = tokenize("[hsl( 120 , 100 , 50 )]text").unwrap();
        assert_eq!(
            tokens[0],
            Token::Tag(TagType::Color {
                color: Color::Rgb(0, 255, 0),
                ground: Ground::Foreground,
            })
        );
    }

    #[test]
    fn test_tokenize_mixed_bold_hsl() {
        let tokens = tokenize("[bold hsl(0,100,50)]text").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
                Token::Tag(TagType::Color {
                    color: Color::Rgb(255, 0, 0),
                    ground: Ground::Foreground,
                }),
                Token::Text("text".into()),
            ]
        );
    }

    #[test]
    fn test_parse_part_custom_style_from_registry() {
        crate::registry::insert_style("danger", crate::ansi::Style::parse("[bold red]").unwrap())
            .unwrap();
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
