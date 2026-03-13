#[derive(Debug)]
pub enum LexError {
    UnclosedTag,
    InvalidTag(String),
    UnclosedValue,
    InvalidArgumentCount { expected: usize, got: usize },
    InvalidValue(String),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnclosedTag => write!(f, "unclosed tag"),
            LexError::InvalidTag(tag) => write!(f, "invalid tag: {tag}"),
            LexError::UnclosedValue => write!(f, "unclosed parantheses for color value"),
            LexError::InvalidArgumentCount { expected, got } => {
                write!(f, "expected {}, got {}", expected, got)
            }
            LexError::InvalidValue(s) => write!(f, "invalid value '{}'", s),
        }
    }
}

impl std::error::Error for LexError {}
