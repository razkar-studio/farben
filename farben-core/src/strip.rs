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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_empty_string() {
        assert_eq!(strip_ansi(""), "");
    }

    #[test]
    fn test_strip_ansi_plain_text_unchanged() {
        assert_eq!(strip_ansi("hello world"), "hello world");
    }

    #[test]
    fn test_strip_ansi_single_color_sequence() {
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
    }

    #[test]
    fn test_strip_ansi_bare_esc_not_followed_by_bracket_is_preserved() {
        assert_eq!(strip_ansi("\x1bhello"), "\x1bhello");
    }

    #[test]
    fn test_strip_ansi_bare_esc_at_end_is_preserved() {
        assert_eq!(strip_ansi("text\x1b"), "text\x1b");
    }

    #[test]
    fn test_strip_ansi_sequences_only_produces_empty() {
        assert_eq!(strip_ansi("\x1b[1m\x1b[31m\x1b[0m"), "");
    }

    #[test]
    fn test_strip_ansi_mixed_content_preserves_text() {
        assert_eq!(
            strip_ansi("\x1b[1mhello\x1b[0m world\x1b[32m!"),
            "hello world!"
        );
    }

    #[test]
    fn test_strip_ansi_rgb_sequence_stripped() {
        assert_eq!(strip_ansi("\x1b[38;2;255;0;0mred\x1b[0m"), "red");
    }

    #[test]
    fn test_strip_ansi_ansi256_sequence_stripped() {
        assert_eq!(strip_ansi("\x1b[38;5;200mcolor\x1b[0m"), "color");
    }
}

pub fn strip_ansi(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.next() {
                Some('[') => {
                    for c in chars.by_ref() {
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
