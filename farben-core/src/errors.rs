//! Error types for farben markup tokenization and registry operations.
//!
//! [`LexError`] covers failures during tokenization of markup strings like `[bold red]text[/]`.
//! [`RegistryError`] covers failures in registry operations such as `set_prefix` and `insert_style`.
//! [`LexErrorDisplay`] wraps a [`LexError`] with the original input to produce compiler-style
//! diagnostic output with a caret pointing at the offending byte offset.

/// Errors produced by registry operations (`set_prefix`, `insert_style`).
///
/// These errors carry no source position because registry calls happen outside of markup
/// parsing, where no input string is available to point into.
#[derive(Debug)]
pub enum RegistryError {
    /// The `prefix!` macro was called with a style name that has not been registered.
    UnknownStyle(String),
    /// A style name was registered that contains `[` or `]`, which are reserved as tag delimiters.
    InvalidName(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownStyle(s) => write!(f, "unknown style: '{s}' has not been registered"),
            Self::InvalidName(s) => write!(
                f,
                "invalid style name '{s}': '[' and ']' are not allowed in style names"
            ),
        }
    }
}

/// Diagnostic formatter that renders a [`LexError`] alongside the original markup input.
///
/// Formats output in the style of the Rust compiler: the input string on one line, followed by
/// a caret (`^`) on the next line aligned to the byte offset stored in the error variant.
///
/// # Example
///
/// ```text
///    | [bold unknown]text[/]
///    |  ^^^^ invalid tag: 'bold unknown'
/// ```
pub struct LexErrorDisplay<'a> {
    /// The error to render.
    pub error: &'a LexError,
    /// The full markup string that was being tokenized when the error occurred.
    pub input: &'a str,
}

impl std::fmt::Display for LexErrorDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   | {}", self.input)?;
        match self.error {
            LexError::UnclosedTag(position) => {
                write!(f, "   | {}^ unclosed tag", " ".repeat(*position))
            }
            LexError::InvalidTag {
                tag_content,
                position,
            } => {
                write!(
                    f,
                    "   | {}{} invalid tag: '{tag_content}'",
                    " ".repeat(*position + 1),
                    "^".repeat(tag_content.len())
                )
            }
            LexError::UnclosedValue(position) => {
                write!(
                    f,
                    "   | {}^ unclosed parentheses for color value",
                    " ".repeat(*position + 1)
                )
            }
            LexError::InvalidArgumentCount {
                expected,
                got,
                position,
            } => {
                write!(
                    f,
                    "   | {}^ expected {expected} arguments, got {got}",
                    " ".repeat(*position + 1)
                )
            }
            LexError::InvalidValue { value, position } => {
                write!(
                    f,
                    "   | {}{} invalid value: '{value}'",
                    " ".repeat(*position + 1),
                    "^".repeat(value.len())
                )
            }
            LexError::InvalidResetTarget(position) => {
                write!(
                    f,
                    "   | {}^ reset target must be a color or emphasis tag",
                    " ".repeat(*position + 1)
                )
            }
        }
    }
}

/// Errors produced during tokenization of a farben markup string.
#[derive(Debug)]
pub enum LexError {
    /// A `[` was found with no matching `]`.
    UnclosedTag(usize),
    /// The tag name is not a recognized keyword or color form.
    InvalidTag {
        /// The raw text inside the brackets, e.g. `"floob"` for `[floob]`.
        tag_content: String,
        /// Byte offset of the opening `[` in the original input.
        position: usize,
    },
    /// A color value function (e.g. `rgb(` or `ansi(`) was opened but never closed.
    UnclosedValue(usize),
    /// A color function received the wrong number of arguments.
    InvalidArgumentCount {
        /// How many arguments the function requires (e.g. 3 for `rgb`).
        expected: usize,
        /// How many arguments were actually provided.
        got: usize,
        /// Byte offset of the function call in the original input.
        position: usize,
    },
    /// An argument could not be parsed into the expected numeric type.
    InvalidValue {
        /// The offending argument as written.
        value: String,
        /// Byte offset of the function call in the original input.
        position: usize,
    },
    /// A reset tag was given a target that cannot be reset (e.g. `[/reset]` or `[/prefix]`).
    InvalidResetTarget(usize),
}

impl LexError {
    /// Returns a helper that formats this error with a caret pointing at the
    /// offending byte in the original input.
    ///
    /// # Example
    ///
    /// ```
    /// use farben_core::errors::LexError;
    /// use farben_core::lexer::tokenize;
    ///
    /// let input = "[bold red unkn]oops";
    /// match tokenize(input) {
    ///     Ok(_) => {}
    ///     Err(e) => eprintln!("{}", e.display(input)),
    /// }
    /// ```
    pub fn display<'a>(&'a self, input: &'a str) -> LexErrorDisplay<'a> {
        LexErrorDisplay { error: self, input }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnclosedTag(pos) => write!(f, "unclosed tag at position {pos}"),
            LexError::InvalidTag {
                tag_content,
                position,
            } => write!(f, "invalid tag '{tag_content}' at position {position}"),
            LexError::UnclosedValue(pos) => {
                write!(f, "unclosed parentheses for color value at position {pos}")
            }
            LexError::InvalidArgumentCount {
                expected,
                got,
                position,
            } => {
                write!(
                    f,
                    "expected {expected} arguments, got {got} at position {position}"
                )
            }
            LexError::InvalidValue { value, position } => {
                write!(f, "invalid value '{value}' at position {position}")
            }
            LexError::InvalidResetTarget(pos) => write!(
                f,
                "reset target must be a color or emphasis tag at position {pos}"
            ),
        }
    }
}

impl std::error::Error for LexError {}
