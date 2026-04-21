//! Deprecated. Will be removed at 0.19 stable in favor of the `inline` feature which works inline with the `c*` family.
//!
//! Inline markdown print macros.
//!
//! [`md_fmt!`], [`mdprint!`], [`mdprintln!`], [`mdeprint!`], and [`mdeprintln!`] render
//! inline markdown to ANSI escape sequences. Runtime variants are gated on the `markdown`
//! feature; compile-time variants on `markdown-compile`.

/// Parses and renders inline markdown with format arguments.
///
/// Behaves like [`format!`] but processes inline markdown syntax in the
/// resulting string. Always runtime — format arguments are substituted
/// before markdown rendering.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// let name = "world";
/// let s = md_fmt!("**hello {}**", name);
/// ```
#[cfg(feature = "markdown")]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! md_fmt {
    ($($arg:tt)*) => {
        $crate::markdown(format!($($arg)*))
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
/// use farben::prelude::*;
/// mdprint!("**bold** and *italic*");
/// ```
#[cfg(all(feature = "markdown", not(feature = "markdown-compile")))]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdprint {
    () => {
        print!()
    };
    ($($arg:tt)*) => {
        print!("{}", $crate::markdown(format!($($arg)*)))
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
/// use farben::prelude::*;
/// mdprintln!("**bold** and *italic*");
/// ```
#[cfg(all(feature = "markdown", not(feature = "markdown-compile")))]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdprintln {
    () => {
        println!()
    };
    ($($arg:tt)*) => {
        println!("{}", $crate::markdown(format!($($arg)*)))
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
/// use farben::prelude::*;
/// mdprint!("**bold** and *italic*");
/// ```
#[cfg(feature = "markdown-compile")]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdprint {
    () => {
        print!()
    };
    ($fmt:literal) => {
        print!("{}", $crate::markdown!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", $crate::markdown(format!($fmt $(, $arg)*)))
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
/// use farben::prelude::*;
/// mdprintln!("**bold** and *italic*");
/// ```
#[cfg(feature = "markdown-compile")]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdprintln {
    () => {
        println!()
    };
    ($fmt:literal) => {
        println!("{}", $crate::markdown!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", $crate::markdown(format!($fmt $(, $arg)*)))
    };
}

/// Prints inline markdown to stderr without a newline.
///
/// Parses and renders markdown at runtime, converting inline styles to ANSI
/// escape sequences before printing. Behaves like [`eprint!`] but processes
/// markdown syntax first.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// mdeprint!("**error:** something went wrong");
/// ```
#[cfg(all(feature = "markdown", not(feature = "markdown-compile")))]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdeprint {
    () => {
        eprint!()
    };
    ($($arg:tt)*) => {
        eprint!("{}", $crate::markdown(format!($($arg)*)))
    };
}

/// Prints inline markdown to stderr with a trailing newline.
///
/// Parses and renders markdown at runtime, converting inline styles to ANSI
/// escape sequences before printing. Behaves like [`eprintln!`] but processes
/// markdown syntax first.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// mdeprintln!("**error:** something went wrong");
/// ```
#[cfg(all(feature = "markdown", not(feature = "markdown-compile")))]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdeprintln {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {
        eprintln!("{}", $crate::markdown(format!($($arg)*)))
    };
}

/// Prints inline markdown to stderr without a newline.
///
/// Format string is rendered at compile time via [`farben_macros::markdown!`].
/// Behaves like [`eprint!`] but processes markdown syntax at compile time.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// mdeprint!("**error:** something went wrong");
/// ```
#[cfg(feature = "markdown-compile")]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdeprint {
    () => {
        eprint!()
    };
    ($fmt:literal) => {
        eprint!("{}", $crate::markdown!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprint!("{}", $crate::markdown(format!($fmt $(, $arg)*)))
    };
}

/// Prints inline markdown to stderr with a trailing newline.
///
/// Format string is rendered at compile time via [`farben_macros::markdown!`].
/// Behaves like [`eprintln!`] but processes markdown syntax at compile time.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// mdeprintln!("**error:** something went wrong");
/// ```
#[cfg(feature = "markdown-compile")]
#[deprecated(
    since = "0.18.0",
    note = "use the `inline` feature, it works with the `c*` family. will be deleted at 0.19 stable"
)]
#[macro_export]
macro_rules! mdeprintln {
    () => {
        eprintln!()
    };
    ($fmt:literal) => {
        eprintln!("{}", $crate::markdown!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprintln!("{}", $crate::markdown(format!($fmt $(, $arg)*)))
    };
}
