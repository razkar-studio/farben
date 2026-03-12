use std::fmt::Write;

enum Ground {
    Foreground,
    Background,
}

enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub(crate) enum Color {
    Named(NamedColor),
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    blink: bool,
}

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

fn style_to_ansi(style: &Style) -> String {
    let mut ansi: Vec<u8> = Vec::new();

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

    let mut seq = String::from("\x1b[");

    for (i, n) in ansi.iter().enumerate() {
        if i != 0 {
            seq.push(';');
        }
        write!(seq, "{n}").unwrap();
    }

    seq.push('m');
    seq
}
