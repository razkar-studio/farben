//! Error types produced during farben markup tokenization.

/// Errors produced during tokenization of a farben markup string.
#[derive(Debug)]
pub enum LexError {
    /// A `[` was found with no matching `]`.
    UnclosedTag,
    /// The tag name is not a recognized keyword or color form.
    InvalidTag(String),
    /// A color value function (e.g. `rgb(` or `ansi(`) was opened but never closed.
    UnclosedValue,
    /// A color function received the wrong number of arguments.
    InvalidArgumentCount { expected: usize, got: usize },
    /// An argument could not be parsed into the expected numeric type.
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
