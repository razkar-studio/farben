//! Utilities for stripping ANSI escape sequences from strings.
//!
//! This module covers CSI sequences only: the `ESC [ ... <letter>` form
//! used by SGR color codes (e.g. `\x1b[31m`, `\x1b[0m`). OSC, DCS, and
//! other escape sequence types are passed through unchanged.

/// Remove all CSI ANSI escape sequences from `input` and return the plain text.
///
/// Scans `input` character by character. Any sequence matching `ESC [ <params> <letter>`
/// is consumed and dropped. All other characters, including bare `ESC` bytes that are
/// not followed by `[`, are passed through as-is.
///
/// Typical uses: measuring display width of colored strings, writing plain-text
/// log lines from pre-colored output, or feeding output to tools that do not
/// interpret ANSI codes.
///
/// # Arguments
///
/// * `input` - A string slice that may contain CSI ANSI escape sequences.
///
/// # Returns
///
/// A new [`String`] with all CSI sequences removed and all other content preserved.
///
/// # Examples
///
/// ```
/// use farben_core::strip::strip_ansi;
///
/// let colored = "\x1b[31mred text\x1b[0m";
/// assert_eq!(strip_ansi(colored), "red text");
///
/// // Bare ESC bytes not followed by '[' are preserved.
/// let bare_esc = "\x1bhello";
/// assert_eq!(strip_ansi(bare_esc), "\x1bhello");
/// ```
pub fn strip_ansi(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.next() {
                Some('[') => {
                    while let Some(c) = chars.next() {
                        if c.is_alphabetic() {
                            break;
                        }
                    }
                }
                Some(other) => {
                    output.push('\x1b');
                    output.push(other);
                }
                None => {
                    output.push('\x1b');
                }
            }
        } else {
            output.push(c);
        }
    }
    output
}
