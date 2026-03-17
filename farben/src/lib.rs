//! # Introduction
//! Farben (as in "color" in German) is a zero-dependency terminal coloring library.
//! Farben applies a markup-like syntax to your strings and outputs them colored. For example:
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
pub use farben_macros::{color, colorb};

#[cfg(feature = "markdown-compile")]
pub use farben_macros::markdown;

#[cfg(feature = "format")]
pub use farben_core::{ansi::Style, registry::insert_style};

/// Defines a named style in the global registry.
///
/// Parses `$markup` as a farben markup string and stores the resulting style
/// under `$name`. The style can then be used in markup as `[$name]`.
/// Panics if the markup is invalid.
///
/// # Examples
///
/// ```
/// use farben::*;
///
/// style!("danger", "[bold red]");
/// // [danger] in markup now expands to bold red text
/// ```
#[cfg(feature = "format")]
#[macro_export]
macro_rules! style {
    ($name:expr, $markup:expr) => {
        farben::insert_style($name, farben::Style::parse($markup).unwrap());
    };
}

/// Sets a prefix string on a previously defined named style.
///
/// The prefix is injected as a literal string before the style's ANSI escape sequence
/// when rendered. The style must already exist in the registry; call [`style!`] first.
///
/// # Panics
///
/// Panics if `$name` has not been registered. Use [`farben_core::registry::set_prefix`]
/// directly to handle this case without panicking.
///
/// # Examples
///
/// ```
/// use farben::*;
///
/// style!("warn", "[yellow]");
/// prefix!("warn", "⚠ ");
/// // [warn] now renders "⚠ " followed by the yellow escape sequence
/// ```
#[cfg(feature = "format")]
#[macro_export]
macro_rules! prefix {
    ($name:expr, $prefix:expr) => {
        farben_core::registry::set_prefix($name, $prefix)
            .expect("prefix!() called with unregistered style name");
    };
}

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
    color_runtime(input, false)
}

/// Parses and renders a farben markup string without appending a trailing reset sequence.
///
/// Styles applied by this call bleed into subsequent terminal output. Use when chaining
/// multiple colored segments where you want the style to carry forward. For the
/// reset-appending variant, see [`color`].
///
/// # Panics
///
/// Panics if the input is not valid farben markup. Use [`try_color`] for error handling.
#[cfg(not(feature = "compile"))]
pub fn colorb(input: impl Into<String>) -> String {
    color_runtime(input, true)
}

/// Parses and renders a farben markup string, appending a final SGR reset.
///
/// The runtime fallback used internally by [`color_fmt!`], [`cprint!`], and [`cprintln!`].
/// Always a function regardless of active feature flags.
///
/// # Panics
///
/// Panics if the input contains invalid farben markup. Use [`try_color`] for error handling.
pub fn color_runtime(input: impl Into<String>, bleed: bool) -> String {
    let input = input.into();
    let mut res = parser::render(lexer::tokenize(input).expect("Failed to colorize"));
    if !bleed {
        res.push_str("\x1b[0m");
    }
    res
}

/// Parses and renders a farben markup string with format arguments, appending a final SGR reset.
///
/// Behaves like [`format!`] but processes farben markup tags in the resulting string.
///
/// # Panics
///
/// Panics if the input is not valid farben markup.
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! color_fmt {
    ($($arg:tt)*) => {
        farben::color_runtime(format!($($arg)*), false)
    };
}

/// Prints farben-colored markup to stdout without a newline.
///
/// Behaves like [`print!`] but processes farben markup tags before output.
/// The format string is validated at compile time when the `compile` feature is enabled.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
/// ```
/// use farben::*;
/// let message = "I don't know";
/// cprint!("[red]Error: [/]{}", message);
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprint {
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", farben::color_runtime(format!($fmt $(, $arg)*), false))
    };
}

/// Prints farben-colored markup to stdout without a newline.
///
/// Behaves like [`print!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::*;
/// let message = "I don't know";
/// cprint!("[red]Error: [/]{}", message);
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprint {
    ($fmt:literal) => {
        print!("{}", farben::color!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", farben::color_runtime(format!(farben_macros::validate_color!($fmt) $(, $arg)*), false))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline.
///
/// Behaves like [`println!`] but processes farben markup tags before output.
/// The format string is validated at compile time when the `compile` feature is enabled.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
/// ```
/// use farben::*;
/// let result = "We did it!";
/// cprintln!("[green]Success: [/]{}", result);
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprintln {
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", farben::color_runtime(format!($fmt $(, $arg)*), false))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline.
///
/// Behaves like [`println!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
/// ```
/// use farben::*;
/// let result = "We did it!";
/// cprintln!("[green]Success: [/]{}", result);
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprintln {
    ($fmt:literal) => {
        println!("{}", farben::color!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", farben::color_runtime(format!(farben_macros::validate_color!($fmt) $(, $arg)*), false))
    };
}

/// Parses and renders a farben markup string with format arguments, appending a final SGR reset.
///
/// Behaves like [`format!`] but processes farben markup tags in the resulting string.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the input is not valid farben markup.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! color_fmt {
    ($fmt:literal $(, $arg:expr)*) => {
        farben::color_runtime(format!(farben_macros::validate_color!($fmt) $(, $arg)*), false)
    };
}

/// Prints farben-colored markup to stdout without a newline, without appending a reset.
///
/// Styles bleed into subsequent output. Use when chaining multiple colored segments
/// where you want the style to carry forward.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::*;
///
/// cprintb!("[red]Error: ");
/// cprintln!("something went wrong"); // inherits red
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprintb {
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", farben::color_runtime(format!($fmt $(, $arg)*), true))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline, without appending a reset.
///
/// Styles bleed into subsequent output. Use when chaining multiple colored segments
/// where you want the style to carry forward.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::*;
///
/// cprintbln!("[bold red]Section header");
/// cprintln!("still bold and red here"); // inherits style
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprintbln {
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", farben::color_runtime(format!($fmt $(, $arg)*), true))
    };
}

/// Prints farben-colored markup to stdout without a newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprintb {
    ($fmt:literal) => {
        print!("{}", farben::colorb!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", farben::color_runtime(format!(farben_macros::validate_color!($fmt) $(, $arg)*), true))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprintbln {
    ($fmt:literal) => {
        println!("{}", farben::colorb!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", farben::color_runtime(format!(farben_macros::validate_color!($fmt) $(, $arg)*), true))
    };
}

/// Parses and renders inline markdown into an ANSI-escaped string.
///
/// Processes inline markdown syntax at runtime, converting bold, italic,
/// underline, strikethrough, and inline code spans into ANSI escape sequences.
/// Always succeeds — unclosed delimiters are treated as plain text.
///
/// # Examples
///
/// ```
/// use farben::*;
/// let s = markdown("**bold** and *italic*");
/// ```
#[cfg(feature = "markdown")]
pub fn markdown(input: impl Into<String>) -> String {
    farben_md::renderer::render(&farben_md::lexer::tokenize(&input.into()))
}

/// Parses and renders inline markdown with format arguments.
///
/// Behaves like [`format!`] but processes inline markdown syntax in the
/// resulting string. Always runtime — format arguments are substituted
/// before markdown rendering.
///
/// # Examples
///
/// ```
/// use farben::*;
/// let name = "world";
/// let s = md_fmt!("**hello {}**", name);
/// ```
#[cfg(feature = "markdown")]
#[macro_export]
macro_rules! md_fmt {
    ($($arg:tt)*) => {
        farben::markdown(format!($($arg)*))
    };
}

/// Prints inline markdown to stdout without a newline.
///
/// Parses and renders markdown at runtime, converting inline styles to ANSI
/// escape sequences before printing. Behaves like [`print!`] but processes
/// markdown syntax first.
///
/// # Examples
///
/// ```
/// use farben::*;
/// mdprint!("**bold** and *italic*");
/// ```
#[cfg(all(feature = "markdown", not(feature = "markdown-compile")))]
#[macro_export]
macro_rules! mdprint {
    ($($arg:tt)*) => {
        print!("{}", farben::markdown(format!($($arg)*)))
    };
}

/// Prints inline markdown to stdout with a trailing newline.
///
/// Parses and renders markdown at runtime, converting inline styles to ANSI
/// escape sequences before printing. Behaves like [`println!`] but processes
/// markdown syntax first.
///
/// # Examples
///
/// ```
/// use farben::*;
/// mdprintln!("**bold** and *italic*");
/// ```
#[cfg(all(feature = "markdown", not(feature = "markdown-compile")))]
#[macro_export]
macro_rules! mdprintln {
    ($($arg:tt)*) => {
        println!("{}", farben::markdown(format!($($arg)*)))
    };
}

/// Prints inline markdown to stdout without a newline.
///
/// Format string is rendered at compile time via [`farben_macros::markdown!`].
/// Behaves like [`print!`] but processes markdown syntax at compile time.
///
/// # Examples
///
/// ```
/// use farben::*;
/// mdprint!("**bold** and *italic*");
/// ```
#[cfg(feature = "markdown-compile")]
#[macro_export]
macro_rules! mdprint {
    ($fmt:literal) => {
        print!("{}", farben::markdown!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", farben::markdown(format!($fmt $(, $arg)*)))
    };
}

/// Prints inline markdown to stdout with a trailing newline.
///
/// Format string is rendered at compile time via [`farben_macros::markdown!`].
/// Behaves like [`println!`] but processes markdown syntax at compile time.
///
/// # Examples
///
/// ```
/// use farben::*;
/// mdprintln!("**bold** and *italic*");
/// ```
#[cfg(feature = "markdown-compile")]
#[macro_export]
macro_rules! mdprintln {
    ($fmt:literal) => {
        println!("{}", farben::markdown!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", farben::markdown(format!($fmt $(, $arg)*)))
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
        assert_eq!(result.unwrap(), "\x1b[31mbefore\x1b[0mafter\x1b[0m");
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

// Skipped (side effects): none
