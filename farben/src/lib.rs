//! # Introduction
//! Farben (as in "color" in german) is a zero-dependency terminal coloring library.
//! Farben uses a markup-language-like syntax to your string and outputs them colored. For example:
//!
//! # Example
//! ```
//! use farben::*;
//!
//! let colored = color("[red]I'm red!");
//! assert_eq!(colored, "\x1b[31mI'm red!\x1b[0m");
//! ```
use farben_core::*;
#[cfg(feature = "compile")]
pub use farben_macros::color;

/// Parses and renders a farben markup string, appending a final SGR reset.
///
/// # Errors
///
/// Returns a `LexError` if the input contains an unclosed tag, an unknown tag name,
/// or a malformed color value.
pub fn try_color(input: impl Into<String>) -> Result<String, errors::LexError> {
    let input = input.into();
    let mut res = parser::render(lexer::tokenize(input)?);
    res.push_str("\x1b[0m");
    Ok(res)
}

/// Parses and renders a farben markup string, appending a final SGR reset.
///
/// # Panics
///
/// Panics if the input is not valid farben markup. Use `try_color` to handle errors explicitly.
#[cfg(not(feature = "compile"))]
pub fn color(input: impl Into<String>) -> String {
    color_runtime(input)
}

pub fn color_runtime(input: impl Into<String>) -> String {
    let input = input.into();
    try_color(input).expect("Failed to colorize")
}

/// Parses and renders a markup string and appends a SGR reset, with arguments supported.
///
/// # Panics
///
/// Panics if the input is not valid farben markup.
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! color_fmt {
    ($($arg:tt)*) => {
        farben::color_runtime(format!($($arg)*))
    };
}

#[cfg(feature = "compile")]
#[macro_export]
macro_rules! color_fmt {
    ($fmt:literal $(, $arg:expr)*) => {
        farben::color_runtime(format!(farben_macros::validate_color!($fmt) $(, $arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- try_color ---

    #[test]
    fn test_try_color_named_color() {
        let result = try_color("[red]I'm red!");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "\x1b[31mI'm red!\x1b[0m");
    }

    #[test]
    fn test_try_color_appends_trailing_reset() {
        let result = try_color("[blue]text");
        assert!(result.is_ok());
        assert!(result.unwrap().ends_with("\x1b[0m"));
    }

    #[test]
    fn test_try_color_plain_text_no_tags() {
        let result = try_color("no markup here");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "no markup here\x1b[0m");
    }

    #[test]
    fn test_try_color_empty_string() {
        let result = try_color("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "\x1b[0m");
    }

    #[test]
    fn test_try_color_invalid_tag_returns_error() {
        let result = try_color("[error]text");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_color_unclosed_tag_returns_error() {
        let result = try_color("[red");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_color_rgb_color() {
        let result = try_color("[rgb(255,0,0)]red");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
    }

    #[test]
    fn test_try_color_bold_and_named_color() {
        let result = try_color("[bold red]hi");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "\x1b[1m\x1b[31mhi\x1b[0m");
    }

    #[test]
    fn test_try_color_escaped_bracket() {
        let result = try_color("\\[not a tag]");
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with('['));
    }

    #[test]
    fn test_try_color_inline_reset() {
        let result = try_color("[red]before[/]after");
        assert!(result.is_ok());
        let s = result.unwrap();
        assert!(s.contains("\x1b[0m"));
    }

    // --- color ---

    #[test]
    fn test_color_valid_input_returns_string() {
        let result = color("[green]ok");
        assert_eq!(result, "\x1b[32mok\x1b[0m");
    }

    #[test]
    #[should_panic]
    fn test_color_invalid_input_panics() {
        color("[notacolor]text");
    }
}

// Skipped (side effects): none: both public functions are pure string transformations.
