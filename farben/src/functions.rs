//! Core coloring functions.
//!
//! Runtime entry points for parsing and rendering farben markup into ANSI escape sequences.
//! Also exposes the markdown rendering function when the `markdown` feature is enabled.

use std::sync::OnceLock;

use farben_core::*;

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
/// Panics if the input is not valid farben markup. Use [`try_color`] to handle errors explicitly.
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

static CODE_STYLE_INIT: OnceLock<()> = OnceLock::new();
/// Parses and renders a farben markup string, appending a final SGR reset.
///
/// The runtime fallback used internally by [`color_fmt!`], [`cprint!`], and [`cprintln!`].
/// Always a function regardless of active feature flags.
///
/// # Panics
///
/// Panics if the input contains invalid farben markup. Use [`try_color`] for error handling.
pub fn color_runtime(input: impl Into<String>, bleed: bool) -> String {
    CODE_STYLE_INIT.get_or_init(|| {
        if registry::search_registry("code").is_err() {
            let _ = registry::insert_style(
                "code",
                ansi::Style::parse("[bg:ansi(238) bright-white]").unwrap(),
            );
        }
    });
    let input = input.into();
    #[cfg(feature = "inline")]
    let input = inline::preprocess(input);
    let tokens = lexer::tokenize(&input).unwrap_or_else(|e| {
        panic!(
            "{}",
            errors::LexErrorDisplay {
                error: &e,
                input: &input
            }
        );
    });
    let mut res = parser::render(tokens);
    if !bleed {
        res.push_str("\x1b[0m");
        farben_core::clear_active_stack();
    }
    res
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
/// use farben::prelude::*;
/// let s = markdown("**bold** and *italic*");
/// ```
#[cfg(feature = "markdown")]
pub fn markdown(input: impl Into<String>) -> String {
    farben_md::renderer::render(&farben_md::lexer::tokenize(&input.into()))
}
