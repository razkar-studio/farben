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
    ($s:expr) => {{
        $crate::strip_ansi(&$s.to_string())
    }};
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::strip_ansi(&format!($fmt $(, $arg)*))
    }};
}

/// Formats a string and strips all Farben markup tags from the result.
///
/// Accepts the same arguments as [`format!`]. After formatting, every sequence of the form of
/// Farben markup, `[tag]` is removed. Invalid tags are left as-is without panic.
///
/// The return type is `String`
///
/// # Example
/// ```
/// use farben::prelude::markup_strip;
///
/// let stripped = markup_strip!("[bold red]Just the text");
/// assert_eq!("Just the text", stripped);
///
/// let invalid = markup_strip!("[I'm unclosed");
/// assert_eq!("[I'm unclosed", invalid);
///
/// let text = "hey!";
/// let formatted = markup_strip!("[bold red]{text}");
/// assert_eq!("hey!", formatted);
/// ```
#[macro_export]
macro_rules! markup_strip {
    ($s:expr) => {{
        $crate::strip_markup(&$s.to_string())
    }};
    ($($arg:tt)*) => {{
        $crate::strip_markup(&format!($($arg)*))
    }};
}
