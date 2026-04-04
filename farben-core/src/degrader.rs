use std::sync::OnceLock;

use crate::ansi::{Color, NamedColor};

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

static COLOR_LEVEL: OnceLock<ColorLevel> = OnceLock::new();

pub enum ColorLevel {
    TrueColor,
    Ansi256,
    Basic,
}

pub fn color_level() -> &'static ColorLevel {
    COLOR_LEVEL.get_or_init(|| {
        if let Ok(val) = std::env::var("COLORTERM") {
            if val == "truecolor" || val == "24bit" {
                return ColorLevel::TrueColor;
            }
        }

        if let Ok(val) = std::env::var("TERM") {
            if val.contains("256color") {
                return ColorLevel::Ansi256;
            }
        }

        ColorLevel::Basic
    })
}

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

pub fn nearest_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let r6 = (r as u16 * 5 / 255) as u8;
    let g6 = (g as u16 * 5 / 255) as u8;
    let b6 = (b as u16 * 5 / 255) as u8;
    16 + 36 * r6 + 6 * g6 + b6
}

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
