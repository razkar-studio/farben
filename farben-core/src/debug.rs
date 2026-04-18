use crate::{
    ansi::{Color, Ground, NamedColor},
    lexer::{EmphasisType, TagType, Token},
};

/// Converts a `TagType` into its farben markup representation.
///
/// Example: `TagType::Emphasis(EmphasisType::Bold)` → `"bold"`.
pub fn tag_to_markup_part(tag: &TagType) -> String {
    match tag {
        TagType::Emphasis(e) => match e {
            EmphasisType::Bold => "bold".to_string(),
            EmphasisType::Blink => "blink".to_string(),
            EmphasisType::Dim => "dim".to_string(),
            EmphasisType::Italic => "italic".to_string(),
            EmphasisType::Strikethrough => "strikethrough".to_string(),
            EmphasisType::Underline => "underline".to_string(),
            EmphasisType::DoubleUnderline => "double-underline".to_string(),
            EmphasisType::Overline => "overline".to_string(),
            EmphasisType::Invisible => "invisible".to_string(),
            EmphasisType::Reverse => "reverse".to_string(),
            EmphasisType::RapidBlink => "rapid-blink".to_string(),
        },
        TagType::Color { color, ground } => {
            let prepend = match ground {
                Ground::Background => "bg:",
                _ => "",
            };
            match color {
                Color::Ansi256(a) => format!("{prepend}ansi({a})"),
                Color::Named(n) => {
                    let name = match n {
                        NamedColor::Black => "black",
                        NamedColor::Red => "red",
                        NamedColor::Green => "green",
                        NamedColor::Yellow => "yellow",
                        NamedColor::Blue => "blue",
                        NamedColor::Magenta => "magenta",
                        NamedColor::Cyan => "cyan",
                        NamedColor::White => "white",
                        NamedColor::BrightBlack => "bright-black",
                        NamedColor::BrightRed => "bright-red",
                        NamedColor::BrightGreen => "bright-green",
                        NamedColor::BrightYellow => "bright-yellow",
                        NamedColor::BrightBlue => "bright-blue",
                        NamedColor::BrightMagenta => "bright-magenta",
                        NamedColor::BrightCyan => "bright-cyan",
                        NamedColor::BrightWhite => "bright-white",
                    };
                    format!("{prepend}{name}")
                }
                Color::Rgb(r, g, b) => format!("{prepend}rgb({r},{g},{b})"),
            }
        }
        TagType::ResetAll => "/".to_string(),
        TagType::ResetOne(inner) => format!("/{}", tag_to_markup_part(inner)),
        TagType::Prefix(_) => String::new(),
    }
}

/// Converts a sequence of `Token`s back into a farben markup string.
///
/// The inverse of [`tokenize`](crate::lexer::tokenize).
pub fn tokens_to_markup(tokens: &[Token]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Text(s) => {
                result.push_str(s);
                i += 1;
            }
            Token::Tag(_) => {
                let mut parts = Vec::new();
                while i < tokens.len() {
                    if let Token::Tag(tag) = &tokens[i] {
                        parts.push(tag_to_markup_part(tag));
                        i += 1;
                    } else {
                        break;
                    }
                }
                result.push('[');
                result.push_str(&parts.join(" "));
                result.push(']');
            }
        }
    }

    result
}
