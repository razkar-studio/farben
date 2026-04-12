//! Color degradation for terminals that lack truecolor support.
//!
//! Detects the terminal's color level once at startup via `COLORTERM` and `TERM`
//! (`TrueColor`, `Ansi256`, or `Basic`), then provides conversion functions to
//! map arbitrary RGB colors to the nearest representable color in that level.

use std::sync::OnceLock;

use crate::ansi::{Color, NamedColor};

/// Approximate sRGB values for each of the 16 named ANSI colors.
///
/// Each entry is `(color, r, g, b)`. These values are used by [`nearest_named`]
/// to find the closest named color when degrading to `Basic` level.
const NAMED_COLOR_RGB: &[(NamedColor, u8, u8, u8)] = &[
    (NamedColor::Black, 0, 0, 0),
    (NamedColor::Red, 128, 0, 0),
    (NamedColor::Green, 0, 128, 0),
    (NamedColor::Yellow, 128, 128, 0),
    (NamedColor::Blue, 0, 0, 128),
    (NamedColor::Magenta, 128, 0, 128),
    (NamedColor::Cyan, 0, 128, 128),
    (NamedColor::White, 192, 192, 192),
    (NamedColor::BrightBlack, 64, 64, 64),
    (NamedColor::BrightRed, 255, 0, 0),
    (NamedColor::BrightGreen, 0, 255, 0),
    (NamedColor::BrightYellow, 255, 255, 0),
    (NamedColor::BrightBlue, 0, 0, 255),
    (NamedColor::BrightMagenta, 255, 0, 255),
    (NamedColor::BrightCyan, 0, 255, 255),
    (NamedColor::BrightWhite, 255, 255, 255),
];

/// Process-global cache for the detected terminal color level.
///
/// Initialized once by [`color_level`] on first call. Subsequent calls return
/// the cached value without re-reading environment variables.
static COLOR_LEVEL: OnceLock<ColorLevel> = OnceLock::new();

/// The three terminal color capability tiers.
///
/// Determined once at runtime by inspecting `COLORTERM` and `TERM`. Used by
/// [`degrade`] to decide how far to downconvert a color.
pub enum ColorLevel {
    /// Full 24-bit RGB. No degradation needed.
    TrueColor,
    /// 8-bit palette (256 colors). RGB values are mapped to the 6x6x6 cube.
    Ansi256,
    /// 16 named ANSI colors only. All colors are mapped to the nearest named color.
    Basic,
}

/// Detects and caches the terminal's color capability.
///
/// Reads `COLORTERM` first. `truecolor` or `24bit` maps to [`ColorLevel::TrueColor`].
/// If unset or unrecognized, reads `TERM`. A value containing `256color` maps to
/// [`ColorLevel::Ansi256`]. Anything else returns [`ColorLevel::Basic`].
/// The result is cached in [`COLOR_LEVEL`] and never recomputed.
pub fn color_level() -> &'static ColorLevel {
    COLOR_LEVEL.get_or_init(|| {
        if let Ok(val) = std::env::var("COLORTERM")
            && (val == "truecolor" || val == "24bit") {
                return ColorLevel::TrueColor;
            }

        if let Ok(val) = std::env::var("TERM")
            && val.contains("256color") {
                return ColorLevel::Ansi256;
            }

        ColorLevel::Basic
    })
}

/// Finds the named ANSI color closest to the given RGB triple.
///
/// Computes squared Euclidean distance in RGB space against each entry in
/// [`NAMED_COLOR_RGB`] and returns the color with the smallest distance.
pub fn nearest_named(r: u8, g: u8, b: u8) -> NamedColor {
    let mut best = &NAMED_COLOR_RGB[0];
    let mut best_dist = u32::MAX;

    for entry in NAMED_COLOR_RGB {
        let (_, er, eg, eb) = entry;
        let dist = ((r as i32 - *er as i32).pow(2)
            + (g as i32 - *eg as i32).pow(2)
            + (b as i32 - *eb as i32).pow(2)) as u32;
        if dist < best_dist {
            best_dist = dist;
            best = entry;
        }
    }

    best.0.clone()
}

/// Maps an RGB triple to the nearest index in the ANSI 256-color 6x6x6 cube.
///
/// Each channel is scaled from 0..=255 to 0..=5 and combined into a cube index.
/// Returns a value in the range `16..=231`.
pub fn nearest_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let r6 = (r as u16 * 5 / 255) as u8;
    let g6 = (g as u16 * 5 / 255) as u8;
    let b6 = (b as u16 * 5 / 255) as u8;
    16 + 36 * r6 + 6 * g6 + b6
}

/// Reverse-maps an ANSI 256-palette index to approximate RGB.
///
/// Handles three ranges: `0..=15` looks up [`NAMED_COLOR_RGB`], `16..=231` decodes
/// the 6x6x6 color cube, and `232..=255` decodes the 24-step grayscale ramp.
pub fn ansi256_to_rgb(n: u8) -> (u8, u8, u8) {
    match n {
        0..=15 => {
            let (_, r, g, b) = NAMED_COLOR_RGB[n as usize];
            (r, g, b)
        }
        16..=231 => {
            let n = n - 16;
            let r = (n / 36) * 51;
            let g = ((n % 36) / 6) * 51;
            let b = (n % 6) * 51;
            (r, g, b)
        }
        232..=255 => {
            let gray = 8 + (n - 232) * 10;
            (gray, gray, gray)
        }
    }
}

/// Downgrades a color to the highest fidelity the terminal can render.
///
/// Passes the color through unchanged when the terminal supports it natively.
/// For `Ansi256` terminals, `Color::Rgb` is mapped to the nearest cube index.
/// For `Basic` terminals, both `Color::Rgb` and `Color::Ansi256` are mapped to
/// the nearest named color via [`nearest_named`].
pub fn degrade(color: Color) -> Color {
    match color_level() {
        ColorLevel::TrueColor => color,
        ColorLevel::Ansi256 => match color {
            Color::Named(_) | Color::Ansi256(_) => color,
            Color::Rgb(r, g, b) => Color::Ansi256(nearest_ansi256(r, g, b)),
        },
        ColorLevel::Basic => match color {
            Color::Named(_) => color,
            Color::Ansi256(n) => {
                let (r, g, b) = ansi256_to_rgb(n);
                Color::Named(nearest_named(r, g, b))
            }
            Color::Rgb(r, g, b) => Color::Named(nearest_named(r, g, b)),
        },
    }
}
