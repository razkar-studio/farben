//! Conversions between farben types and [`anstyle`] types.
//!
//! This module provides bidirectional [`From`] implementations for converting
//! between farben's color and style types and the [`anstyle`] crate's
//! equivalent types. Enables interoperability with libraries that use
//! [`anstyle`] for terminal styling.
//!
//! Requires the `anstyle` feature flag.

use crate::ansi::{Color, NamedColor, Style};

/// Converts a farben [`Color`] into an [`anstyle::Color`].
impl From<Color> for anstyle::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Named(n) => anstyle::Color::Ansi(n.into()),
            Color::Ansi256(n) => anstyle::Color::Ansi256(anstyle::Ansi256Color(n)),
            Color::Rgb(r, g, b) => anstyle::Color::Rgb(anstyle::RgbColor(r, g, b)),
        }
    }
}

/// Converts a farben [`NamedColor`] into an [`anstyle::AnsiColor`].
impl From<NamedColor> for anstyle::AnsiColor {
    fn from(color: NamedColor) -> Self {
        match color {
            NamedColor::Black => anstyle::AnsiColor::Black,
            NamedColor::Red => anstyle::AnsiColor::Red,
            NamedColor::Green => anstyle::AnsiColor::Green,
            NamedColor::Yellow => anstyle::AnsiColor::Yellow,
            NamedColor::Blue => anstyle::AnsiColor::Blue,
            NamedColor::Magenta => anstyle::AnsiColor::Magenta,
            NamedColor::Cyan => anstyle::AnsiColor::Cyan,
            NamedColor::White => anstyle::AnsiColor::White,
            NamedColor::BrightBlack => anstyle::AnsiColor::BrightBlack,
            NamedColor::BrightRed => anstyle::AnsiColor::BrightRed,
            NamedColor::BrightGreen => anstyle::AnsiColor::BrightGreen,
            NamedColor::BrightYellow => anstyle::AnsiColor::BrightYellow,
            NamedColor::BrightBlue => anstyle::AnsiColor::BrightBlue,
            NamedColor::BrightMagenta => anstyle::AnsiColor::BrightMagenta,
            NamedColor::BrightCyan => anstyle::AnsiColor::BrightCyan,
            NamedColor::BrightWhite => anstyle::AnsiColor::BrightWhite,
        }
    }
}

/// Converts a farben [`Style`] into an [`anstyle::Style`].
impl From<Style> for anstyle::Style {
    fn from(style: Style) -> Self {
        let mut out = anstyle::Style::new();

        if let Some(fg) = style.fg {
            out = out.fg_color(Some(fg.into()));
        }
        if let Some(bg) = style.bg {
            out = out.bg_color(Some(bg.into()));
        }

        let mut effects = anstyle::Effects::new();
        if style.bold {
            effects = effects.insert(anstyle::Effects::BOLD);
        }
        if style.dim {
            effects = effects.insert(anstyle::Effects::DIMMED);
        }
        if style.italic {
            effects = effects.insert(anstyle::Effects::ITALIC);
        }
        if style.underline {
            effects = effects.insert(anstyle::Effects::UNDERLINE);
        }
        if style.double_underline {
            effects = effects.insert(anstyle::Effects::DOUBLE_UNDERLINE);
        }
        if style.strikethrough {
            effects = effects.insert(anstyle::Effects::STRIKETHROUGH);
        }
        if style.blink {
            effects = effects.insert(anstyle::Effects::BLINK);
        }
        if style.reverse {
            effects = effects.insert(anstyle::Effects::INVERT);
        }
        if style.invisible {
            effects = effects.insert(anstyle::Effects::HIDDEN);
        }

        let _ = (style.overline, style.rapid_blink);

        out.effects(effects)
    }
}

/// Converts an [`anstyle::Style`] into a farben [`Style`].
impl From<anstyle::Style> for Style {
    fn from(style: anstyle::Style) -> Self {
        let mut out = Style::default();

        out.fg = style.get_fg_color().map(|c| c.into());
        out.bg = style.get_bg_color().map(|c| c.into());

        let effects = style.get_effects();
        out.bold = effects.contains(anstyle::Effects::BOLD);
        out.dim = effects.contains(anstyle::Effects::DIMMED);
        out.italic = effects.contains(anstyle::Effects::ITALIC);
        out.underline = effects.contains(anstyle::Effects::UNDERLINE);
        out.double_underline = effects.contains(anstyle::Effects::DOUBLE_UNDERLINE);
        out.strikethrough = effects.contains(anstyle::Effects::STRIKETHROUGH);
        out.blink = effects.contains(anstyle::Effects::BLINK);
        out.reverse = effects.contains(anstyle::Effects::INVERT);
        out.invisible = effects.contains(anstyle::Effects::HIDDEN);

        let _ = (out.overline, out.rapid_blink);

        out
    }
}

/// Converts an [`anstyle::Color`] into a farben [`Color`].
impl From<anstyle::Color> for Color {
    fn from(color: anstyle::Color) -> Self {
        match color {
            anstyle::Color::Ansi(a) => Color::Named(a.into()),
            anstyle::Color::Ansi256(n) => Color::Ansi256(n.0),
            anstyle::Color::Rgb(rgb) => Color::Rgb(rgb.0, rgb.1, rgb.2),
        }
    }
}

/// Converts an [`anstyle::AnsiColor`] into a farben [`NamedColor`].
impl From<anstyle::AnsiColor> for NamedColor {
    fn from(color: anstyle::AnsiColor) -> Self {
        match color {
            anstyle::AnsiColor::Black => NamedColor::Black,
            anstyle::AnsiColor::Red => NamedColor::Red,
            anstyle::AnsiColor::Green => NamedColor::Green,
            anstyle::AnsiColor::Yellow => NamedColor::Yellow,
            anstyle::AnsiColor::Blue => NamedColor::Blue,
            anstyle::AnsiColor::Magenta => NamedColor::Magenta,
            anstyle::AnsiColor::Cyan => NamedColor::Cyan,
            anstyle::AnsiColor::White => NamedColor::White,
            anstyle::AnsiColor::BrightBlack => NamedColor::BrightBlack,
            anstyle::AnsiColor::BrightRed => NamedColor::BrightRed,
            anstyle::AnsiColor::BrightGreen => NamedColor::BrightGreen,
            anstyle::AnsiColor::BrightYellow => NamedColor::BrightYellow,
            anstyle::AnsiColor::BrightBlue => NamedColor::BrightBlue,
            anstyle::AnsiColor::BrightMagenta => NamedColor::BrightMagenta,
            anstyle::AnsiColor::BrightCyan => NamedColor::BrightCyan,
            anstyle::AnsiColor::BrightWhite => NamedColor::BrightWhite,
        }
    }
}
