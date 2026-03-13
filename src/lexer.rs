use crate::{
    ansi::{Color, NamedColor},
    errors::LexError,
};

#[derive(Debug, PartialEq)]
pub(crate) enum EmphasisType {
    Dim,
    Italic,
    Underline,
    Bold,
    Strikethrough,
    Blink,
}

impl EmphasisType {
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

#[derive(Debug, PartialEq)]
pub(crate) enum TagType {
    Reset,
    Emphasis(EmphasisType),
    Color(Color),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Tag(TagType),
    Text(String),
}

fn parse_part(part: &str) -> Result<TagType, LexError> {
    if part == "/" {
        Ok(TagType::Reset)
    } else if let Some(color) = NamedColor::from_str(part) {
        Ok(TagType::Color(Color::Named(color)))
    } else if let Some(emphasis) = EmphasisType::from_str(part) {
        Ok(TagType::Emphasis(emphasis))
    } else if let Some(ansi_val) = part.strip_prefix("ansi(").and_then(|s| s.strip_suffix(")")) {
        match ansi_val.trim().parse::<u8>() {
            Ok(code) => Ok(TagType::Color(Color::Ansi256(code))),
            Err(_) => Err(LexError::InvalidValue(ansi_val.to_string())),
        }
    } else if let Some(rgb_val) = part.strip_prefix("rgb(").and_then(|s| s.strip_suffix(")")) {
        let parts: Result<Vec<u8>, _> =
            rgb_val.split(',').map(|v| v.trim().parse::<u8>()).collect();
        match parts {
            Ok(v) if v.len() == 3 => Ok(TagType::Color(Color::Rgb(v[0], v[1], v[2]))),
            Ok(v) => Err(LexError::InvalidArgumentCount {
                expected: 3,
                got: v.len(),
            }),
            Err(_) => Err(LexError::InvalidValue(rgb_val.to_string())),
        }
    } else {
        Err(LexError::InvalidTag(part.to_string()))
    }
}

fn parse_tag(raw_tag: &str) -> Result<Vec<TagType>, LexError> {
    raw_tag.split_whitespace().map(parse_part).collect()
}

pub(crate) fn tokenize(input: impl Into<String>) -> Result<Vec<Token>, LexError> {
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
        // escape logic
        if abs_starting > 0 && input[abs_starting - 1..abs_starting] == "\\".to_string() {
            tokens.push(Token::Text(input[pos..abs_starting - 1].to_string()));
            tokens.push(Token::Text(String::from('[')));
            pos = abs_starting + 1;
            continue;
        }

        if pos != abs_starting {
            tokens.push(Token::Text(input[pos..abs_starting].to_string()));
        }

        let Some(closing) = input[abs_starting..].find(']') else {
            return Err(LexError::UnclosedTag);
        };
        let abs_closing = closing + abs_starting;
        let raw_tag = &input[abs_starting + 1..abs_closing];
        for tag in parse_tag(raw_tag)? {
            tokens.push(Token::Tag(tag));
        }
        pos = abs_closing + 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_color_tag() {
        let tokens = tokenize("[red]I'm red!");
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Color(Color::Named(NamedColor::Red))),
                Token::Text("I'm red!".to_string()),
            ]
        );
    }

    #[test]
    fn test_multiple_color_tag() {
        let tokens =
            tokenize("[bold][red]I'm bold and [blue]blue [italic]italian![/] ...or something");
        assert_eq!(
            tokens,
            vec![
                Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
                Token::Tag(TagType::Color(Color::Named(NamedColor::Red))),
                Token::Text("I'm bold and ".to_string()),
                Token::Tag(TagType::Color(Color::Named(NamedColor::Blue))),
                Token::Text("blue ".to_string()),
                Token::Tag(TagType::Emphasis(EmphasisType::Italic)),
                Token::Text("italian!".to_string()),
                Token::Tag(TagType::Reset),
                Token::Text(" ...or something".to_string()),
            ]
        );
    }
}
