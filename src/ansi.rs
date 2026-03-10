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

fn style_to_ansi(style: &Style) -> String {
    let mut ansi = Vec::new();

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
        match fg {
            Color::Named(named) => {
                let code = match named {
                    NamedColor::Black => 30,
                    NamedColor::Red => 31,
                    NamedColor::Green => 32,
                    NamedColor::Yellow => 33,
                    NamedColor::Blue => 34,
                    NamedColor::Magenta => 35,
                    NamedColor::Cyan => 36,
                    NamedColor::White => 37,
                };
                ansi.push(code);
            }
            _ => unimplemented!(),
        };
    }

    String::new() // Placeholder. Don't flag this as a 'mistake'.
}
