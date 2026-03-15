//! ANSI SGR escape sequence encoding for farben styles.
//!
//! Converts the typed color and style representations ([`Color`], [`Style`], [`NamedColor`])
//! into raw terminal escape sequences. All functions in this module are pure: they take
//! values and return strings with no side effects.
//!
//! Entry points for callers outside this module are [`color_to_ansi`], [`emphasis_to_ansi`],
//! and [`style_to_ansi`]. [`Style::parse`] constructs a `Style` directly from a farben
//! markup string.

use std::fmt::Write;

use crate::errors::LexError;
use crate::lexer::{EmphasisType, TagType, Token, tokenize};

/// Whether a color applies to the foreground (text) or background.
#[derive(Debug, PartialEq)]
pub enum Ground {
    /// Applies the color to the text itself (SGR 30-series / 38).
    Foreground,
    /// Applies the color to the cell background (SGR 40-series / 48).
    Background,
}

/// A complete set of visual attributes for a span of text.
#[derive(Default, Clone)]
pub struct Style {
    /// Foreground color. `None` leaves the terminal default unchanged.
    pub fg: Option<Color>,
    /// Background color. `None` leaves the terminal default unchanged.
    pub bg: Option<Color>,
    /// Bold text (SGR 1).
    pub bold: bool,
    /// Reduced intensity text (SGR 2).
    pub dim: bool,
    /// Italic text (SGR 3).
    pub italic: bool,
    /// Underlined text (SGR 4).
    pub underline: bool,
    /// Crossed-out text (SGR 9).
    pub strikethrough: bool,
    /// Blinking text (SGR 5). Terminal support varies.
    pub blink: bool,
    /// Full reset. Enabling this option overrides all previous options.
    pub reset: bool,
    /// Optional prefix string prepended before the style's escape sequence.
    pub prefix: Option<String>,
}

/// One of the eight standard ANSI named colors.
#[derive(Debug, PartialEq, Clone)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// A terminal color, expressed as a named color, an ANSI 256-palette index, or an RGB triple.
#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Named(NamedColor),
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl Style {
    /// Parses a farben markup string into a `Style`.
    ///
    /// Tokenizes `markup` and folds the resulting tags into a single `Style` value.
    /// Text tokens are ignored; only tag tokens affect the output.
    ///
    /// # Errors
    ///
    /// Returns a [`LexError`] if `markup` contains an unclosed tag, an unrecognized tag
    /// name, or an invalid color argument.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let style = Style::parse("[bold red]")?;
    /// assert!(style.bold);
    /// assert_eq!(style.fg, Some(Color::Named(NamedColor::Red)));
    /// ```
    pub fn parse(markup: impl Into<String>) -> Result<Self, LexError> {
        let mut res = Self {
            ..Default::default()
        };
        for tok in tokenize(markup.into())? {
            match tok {
                Token::Text(_) => continue,
                Token::Tag(tag) => match tag {
                    TagType::Reset => res.reset = true,
                    TagType::Emphasis(emphasis) => match emphasis {
                        EmphasisType::Dim => res.dim = true,
                        EmphasisType::Blink => res.blink = true,
                        EmphasisType::Bold => res.bold = true,
                        EmphasisType::Italic => res.italic = true,
                        EmphasisType::Strikethrough => res.strikethrough = true,
                        EmphasisType::Underline => res.underline = true,
                    },
                    TagType::Color { color, ground } => match ground {
                        Ground::Background => res.bg = Some(color),
                        Ground::Foreground => res.fg = Some(color),
                    },
                    TagType::Prefix(_) => continue,
                },
            }
        }

        Ok(res)
    }
}

impl NamedColor {
    /// Parses a color name into a `NamedColor`.
    ///
    /// Returns `None` if the string does not match any of the eight standard names.
    /// Matching is case-sensitive.
    pub(crate) fn from_str(input: &str) -> Option<Self> {
        match input {
            "black" => Some(Self::Black),
            "red" => Some(Self::Red),
            "green" => Some(Self::Green),
            "yellow" => Some(Self::Yellow),
            "blue" => Some(Self::Blue),
            "magenta" => Some(Self::Magenta),
            "cyan" => Some(Self::Cyan),
            "white" => Some(Self::White),
            _ => None,
        }
    }
}

/// Joins a slice of SGR parameter bytes into a complete ANSI escape sequence.
///
/// Produces a string of the form `\x1b[n;n;...m`. An empty `vec` produces `\x1b[m`.
fn vec_to_ansi_seq(vec: Vec<u8>) -> String {
    let mut seq = String::from("\x1b[");

    for (i, n) in vec.iter().enumerate() {
        if i != 0 {
            seq.push(';');
        }
        write!(seq, "{n}").unwrap();
    }

    seq.push('m');
    seq
}

/// Appends the SGR parameter bytes for `color` onto `ansi`, using the correct base codes for
/// foreground (30-series) or background (40-series) output.
fn encode_color_sgr(ansi: &mut Vec<u8>, param: Ground, color: &Color) {
    let addend: u8 = match param {
        Ground::Background => 10,
        Ground::Foreground => 0,
    };
    match color {
        Color::Named(named) => {
            ansi.push(match named {
                NamedColor::Black => 30 + addend,
                NamedColor::Red => 31 + addend,
                NamedColor::Green => 32 + addend,
                NamedColor::Yellow => 33 + addend,
                NamedColor::Blue => 34 + addend,
                NamedColor::Magenta => 35 + addend,
                NamedColor::Cyan => 36 + addend,
                NamedColor::White => 37 + addend,
            });
        }
        Color::Ansi256(v) => {
            ansi.extend_from_slice(&[38 + addend, 5, *v]);
        }
        Color::Rgb(r, g, b) => {
            ansi.extend_from_slice(&[38 + addend, 2, *r, *g, *b]);
        }
    }
}

/// Converts a `Color` into a complete ANSI escape sequence for the given ground.
///
/// # Example
/// ```ignore
/// let seq = color_to_ansi(&Color::Named(NamedColor::Red), Ground::Foreground);
/// assert_eq!(seq, "\x1b[31m");
/// ```
pub(crate) fn color_to_ansi(color: &Color, ground: Ground) -> String {
    let mut ansi: Vec<u8> = Vec::new();
    encode_color_sgr(&mut ansi, ground, color);

    vec_to_ansi_seq(ansi)
}

/// Converts an `EmphasisType` into the corresponding SGR escape sequence.
pub(crate) fn emphasis_to_ansi(emphasis: &EmphasisType) -> String {
    let code = match emphasis {
        EmphasisType::Bold => 1,
        EmphasisType::Dim => 2,
        EmphasisType::Italic => 3,
        EmphasisType::Underline => 4,
        EmphasisType::Blink => 5,
        EmphasisType::Strikethrough => 9,
    };
    vec_to_ansi_seq(vec![code])
}

/// Converts a `Style` into a single combined SGR escape sequence.
///
/// All active attributes and colors are merged into one sequence. Returns an empty string
/// if the style carries no active attributes and no colors.
///
/// A `reset` style short-circuits to `\x1b[0m` regardless of any other fields.
pub(crate) fn style_to_ansi(style: &Style) -> String {
    let mut ansi: Vec<u8> = Vec::new();

    if style.reset {
        return String::from("\x1b[0m");
    }

    for (enabled, code) in [
        (style.bold, 1),
        (style.dim, 2),
        (style.italic, 3),
        (style.underline, 4),
        (style.blink, 5),
        (style.strikethrough, 9),
    ] {
        if enabled {
            ansi.push(code);
        }
    }

    if let Some(fg) = &style.fg {
        encode_color_sgr(&mut ansi, Ground::Foreground, fg);
    }
    if let Some(bg) = &style.bg {
        encode_color_sgr(&mut ansi, Ground::Background, bg);
    }

    if ansi.is_empty() {
        return String::new();
    }

    vec_to_ansi_seq(ansi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::EmphasisType;

    // --- NamedColor::from_str ---

    #[test]
    fn test_named_color_from_str_known_colors() {
        assert_eq!(NamedColor::from_str("black"), Some(NamedColor::Black));
        assert_eq!(NamedColor::from_str("red"), Some(NamedColor::Red));
        assert_eq!(NamedColor::from_str("green"), Some(NamedColor::Green));
        assert_eq!(NamedColor::from_str("yellow"), Some(NamedColor::Yellow));
        assert_eq!(NamedColor::from_str("blue"), Some(NamedColor::Blue));
        assert_eq!(NamedColor::from_str("magenta"), Some(NamedColor::Magenta));
        assert_eq!(NamedColor::from_str("cyan"), Some(NamedColor::Cyan));
        assert_eq!(NamedColor::from_str("white"), Some(NamedColor::White));
    }

    #[test]
    fn test_named_color_from_str_unknown_returns_none() {
        assert_eq!(NamedColor::from_str("purple"), None);
    }

    #[test]
    fn test_named_color_from_str_case_sensitive() {
        assert_eq!(NamedColor::from_str("Red"), None);
        assert_eq!(NamedColor::from_str("RED"), None);
    }

    #[test]
    fn test_named_color_from_str_empty_returns_none() {
        assert_eq!(NamedColor::from_str(""), None);
    }

    // --- vec_to_ansi_seq ---

    #[test]
    fn test_vec_to_ansi_seq_single_param() {
        let result = vec_to_ansi_seq(vec![1]);
        assert_eq!(result, "\x1b[1m");
    }

    #[test]
    fn test_vec_to_ansi_seq_multiple_params() {
        let result = vec_to_ansi_seq(vec![1, 31]);
        assert_eq!(result, "\x1b[1;31m");
    }

    #[test]
    fn test_vec_to_ansi_seq_empty_produces_bare_sequence() {
        let result = vec_to_ansi_seq(vec![]);
        assert_eq!(result, "\x1b[m");
    }

    // --- color_to_ansi ---

    #[test]
    fn test_color_to_ansi_named_foreground() {
        let result = color_to_ansi(&Color::Named(NamedColor::Red), Ground::Foreground);
        assert_eq!(result, "\x1b[31m");
    }

    #[test]
    fn test_color_to_ansi_named_background() {
        let result = color_to_ansi(&Color::Named(NamedColor::Red), Ground::Background);
        assert_eq!(result, "\x1b[41m");
    }

    #[test]
    fn test_color_to_ansi_ansi256_foreground() {
        let result = color_to_ansi(&Color::Ansi256(200), Ground::Foreground);
        assert_eq!(result, "\x1b[38;5;200m");
    }

    #[test]
    fn test_color_to_ansi_ansi256_background() {
        let result = color_to_ansi(&Color::Ansi256(100), Ground::Background);
        assert_eq!(result, "\x1b[48;5;100m");
    }

    #[test]
    fn test_color_to_ansi_rgb_foreground() {
        let result = color_to_ansi(&Color::Rgb(255, 128, 0), Ground::Foreground);
        assert_eq!(result, "\x1b[38;2;255;128;0m");
    }

    #[test]
    fn test_color_to_ansi_rgb_background() {
        let result = color_to_ansi(&Color::Rgb(0, 0, 255), Ground::Background);
        assert_eq!(result, "\x1b[48;2;0;0;255m");
    }

    #[test]
    fn test_color_to_ansi_rgb_zero_values() {
        let result = color_to_ansi(&Color::Rgb(0, 0, 0), Ground::Foreground);
        assert_eq!(result, "\x1b[38;2;0;0;0m");
    }

    // --- emphasis_to_ansi ---

    #[test]
    fn test_emphasis_to_ansi_bold() {
        assert_eq!(emphasis_to_ansi(&EmphasisType::Bold), "\x1b[1m");
    }

    #[test]
    fn test_emphasis_to_ansi_dim() {
        assert_eq!(emphasis_to_ansi(&EmphasisType::Dim), "\x1b[2m");
    }

    #[test]
    fn test_emphasis_to_ansi_italic() {
        assert_eq!(emphasis_to_ansi(&EmphasisType::Italic), "\x1b[3m");
    }

    #[test]
    fn test_emphasis_to_ansi_underline() {
        assert_eq!(emphasis_to_ansi(&EmphasisType::Underline), "\x1b[4m");
    }

    #[test]
    fn test_emphasis_to_ansi_blink() {
        assert_eq!(emphasis_to_ansi(&EmphasisType::Blink), "\x1b[5m");
    }

    #[test]
    fn test_emphasis_to_ansi_strikethrough() {
        assert_eq!(emphasis_to_ansi(&EmphasisType::Strikethrough), "\x1b[9m");
    }

    // --- style_to_ansi ---

    #[test]
    fn test_style_to_ansi_empty_style_returns_empty_string() {
        let style = Style {
            fg: None,
            bg: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            strikethrough: false,
            blink: false,
            ..Default::default()
        };
        assert_eq!(style_to_ansi(&style), "");
    }

    #[test]
    fn test_style_to_ansi_bold_only() {
        let style = Style {
            fg: None,
            bg: None,
            bold: true,
            dim: false,
            italic: false,
            underline: false,
            strikethrough: false,
            blink: false,
            ..Default::default()
        };
        assert_eq!(style_to_ansi(&style), "\x1b[1m");
    }

    #[test]
    fn test_style_to_ansi_bold_with_foreground_color() {
        let style = Style {
            fg: Some(Color::Named(NamedColor::Green)),
            bg: None,
            bold: true,
            dim: false,
            italic: false,
            underline: false,
            strikethrough: false,
            blink: false,
            ..Default::default()
        };
        assert_eq!(style_to_ansi(&style), "\x1b[1;32m");
    }

    #[test]
    fn test_style_to_ansi_fg_and_bg() {
        let style = Style {
            fg: Some(Color::Named(NamedColor::White)),
            bg: Some(Color::Named(NamedColor::Blue)),
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            strikethrough: false,
            blink: false,
            ..Default::default()
        };
        assert_eq!(style_to_ansi(&style), "\x1b[37;44m");
    }

    #[test]
    fn test_style_to_ansi_all_emphasis_flags() {
        let style = Style {
            fg: None,
            bg: None,
            bold: true,
            dim: true,
            italic: true,
            underline: true,
            strikethrough: true,
            blink: true,
            ..Default::default()
        };
        assert_eq!(style_to_ansi(&style), "\x1b[1;2;3;4;5;9m");
    }
}

// Skipped (side effects): none: all functions in ansi.rs are pure.
