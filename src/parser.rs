use crate::ansi::{Color, Ground, color_to_ansi, emphasis_to_ansi};
use crate::lexer::{TagType, Token};

pub(crate) fn render(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for tok in tokens {
        match tok {
            Token::Text(text) => result.push_str(text.as_str()),
            Token::Tag(tag) => match tag {
                TagType::Color(color) => {
                    result.push_str(color_to_ansi(&color, Ground::Foreground).as_str())
                }
                TagType::Emphasis(emphasis) => {
                    result.push_str(emphasis_to_ansi(&emphasis).as_str())
                }
                TagType::Reset => result.push_str("\x1b[0m"),
            },
        }
    }

    result
}
