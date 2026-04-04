//! ANSI escape sequence stripping macro.
//!
//! [`ansi_strip!`] accepts format-string arguments, builds the resulting string,
//! then removes all CSI ANSI escape sequences from it. Non-CSI `ESC` bytes
//! pass through unchanged.

/// Formats a string and strips all CSI ANSI escape sequences from the result.
///
/// Accepts the same arguments as [`format!`]. After formatting, every sequence
/// of the form `ESC [ ... <letter>` is removed. Non-CSI `ESC` bytes are left intact.
///
/// The return type is `String`.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
///
/// let colored = "\x1b[31mError\x1b[0m";
/// let plain = ansi_strip!("{}", colored);
/// assert_eq!(plain, "Error");
/// ```
///
/// Format arguments work exactly as they do with `format!`:
///
/// ```
/// use farben::prelude::*;
///
/// let code = 42;
/// let plain = ansi_strip!("\x1b[1mcode {code}\x1b[0m");
/// assert_eq!(plain, "code 42");
/// ```
#[macro_export]
macro_rules! ansi_strip {
    ($($arg:tt)*) => {{
        farben::strip_ansi(&format!($($arg)*))
    }};
}
