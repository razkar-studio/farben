/*
 * Copyright (c) 2026 RazkarStudio
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Macros that strip ANSI, strip markup, and escape markup brackets.

/// Removes all CSI ANSI escape sequences from a formatted string.
///
/// Accepts the same arguments as [`format!`]. Every sequence of the form
/// `ESC [ ... <letter>` is removed. Non-CSI `ESC` bytes are left intact.
///
/// The return type is `String`.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
///
/// let colored = "\x1b[31mError\x1b[0m";
/// let plain = unansi!("{}", colored);
/// assert_eq!(plain, "Error");
/// ```
///
/// Format arguments work exactly as they do with `format!`:
///
/// ```
/// use farben::prelude::*;
///
/// let code = 42;
/// let plain = unansi!("\x1b[1mcode {code}\x1b[0m");
/// assert_eq!(plain, "code 42");
/// ```
#[macro_export]
macro_rules! unansi {
    ($fmt:literal $($rest:tt)*) => {{
        $crate::strip_ansi(&format!($fmt $($rest)*))
    }};
    ($s:expr) => {{
        $crate::strip_ansi(&$s.to_string())
    }};
}

/// Removes all Farben markup tags from a formatted string.
///
/// Accepts the same arguments as [`format!`]. Every tag like `[bold]` or `[/]`
/// is removed. Invalid markup is left as-is without panicking.
///
/// The return type is `String`.
///
/// # Example
/// ```
/// use farben::prelude::*;
///
/// let stripped = unmarkup!("[bold red]Just the text");
/// assert_eq!("Just the text", stripped);
///
/// let invalid = unmarkup!("[I'm unclosed");
/// assert_eq!("[I'm unclosed", invalid);
///
/// let text = "hey!";
/// let formatted = unmarkup!("[bold red]{text}");
/// assert_eq!("hey!", formatted);
/// ```
#[macro_export]
macro_rules! unmarkup {
    ($($arg:tt)*) => {{
        $crate::strip_markup(&format!($($arg)*))
    }};
}

/// Escapes markup brackets in a formatted string so they render as literal text.
///
/// Every `[` becomes `[[` and every `]` becomes `]]`. The lexer treats `[[` as
/// a literal `[` and `]]` as a literal `]`, so the result contains no parseable tags.
///
/// Accepts the same arguments as [`format!`].
///
/// The return type is `String`.
///
/// # Example
/// ```
/// use farben::prelude::*;
///
/// let safe = untag!("[bold]hello[/]");
/// assert_eq!(safe, "[[bold]]hello[[/]]");
///
/// let name = "world";
/// let safe = untag!("[bold]{name}[/]");
/// assert_eq!(safe, "[[bold]]world[[/]]");
/// ```
#[macro_export]
macro_rules! untag {
    ($($arg:tt)*) => {{
        $crate::escape_tags(&format!($($arg)*))
    }};
}

// --- Deprecated aliases ---------------------------------------------------

#[deprecated(since = "0.19.0", note = "renamed to `unansi!`")]
#[macro_export]
/// Deprecated in favor of [`unansi!`].
macro_rules! ansi_strip {
    ($($arg:tt)*) => {{
        $crate::unansi!($($arg)*)
    }};
}

#[deprecated(since = "0.19.0", note = "renamed to `unmarkup!`")]
#[macro_export]
/// Deprecated in favor of [`unmarkup!`]
macro_rules! markup_strip {
    ($($arg:tt)*) => {{
        $crate::unmarkup!($($arg)*)
    }};
}
