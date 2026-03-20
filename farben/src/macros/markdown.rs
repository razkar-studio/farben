//! Inline markdown print macros.
//!
//! [`md_fmt!`], [`mdprint!`], and [`mdprintln!`] render inline markdown to ANSI escape sequences.
//! Runtime variants are gated on the `markdown` feature; compile-time variants on `markdown-compile`.

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
