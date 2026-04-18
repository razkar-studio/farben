//! Print macros for farben markup targeting stderr.
//!
//! Stderr variants of [`cprint!`], [`cprintln!`], [`cprintb!`], and [`cprintbln!`].
//! Behavior is identical to the stdout variants but output is directed to stderr.
//! Each macro has two implementations selected by `#[cfg]`: a runtime variant and a
//! compile-time variant activated by the `compile` feature.

/// Prints farben-colored markup to stderr without a newline.
///
/// Behaves like [`eprint!`] but processes farben markup tags before output.
/// The format string is validated at compile time when the `compile` feature is enabled.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// ceprint!("[red]error:[/] something went wrong");
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! ceprint {
    () => {
        eprint!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprint!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), false))
    };
}

/// Prints farben-colored markup to stderr without a newline.
///
/// Behaves like [`eprint!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// ceprint!("[red]error:[/] something went wrong");
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! ceprint {
    () => {
        eprint!()
    };
    ($fmt:literal) => {
        eprint!("{}", $crate::color!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprint!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), false))
    };
}

/// Prints farben-colored markup to stderr with a trailing newline.
///
/// Behaves like [`eprintln!`] but processes farben markup tags before output.
/// The format string is validated at compile time when the `compile` feature is enabled.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// ceprintln!("[red]error:[/] something went wrong");
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! ceprintln {
    () => {
        eprintln!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprintln!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), false))
    };
}

/// Prints farben-colored markup to stderr with a trailing newline.
///
/// Behaves like [`eprintln!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// ceprintln!("[red]error:[/] something went wrong");
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! ceprintln {
    () => {
        eprintln!()
    };
    ($fmt:literal) => {
        eprintln!("{}", $crate::color!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprintln!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), false))
    };
}

/// Prints farben-colored markup to stderr without a newline, without appending a reset.
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
/// use farben::prelude::*;
///
/// ceprintb!("[red]error: ");
/// ceprintln!("something went wrong"); // inherits red
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! ceprintb {
    () => {
        eprint!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprint!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), true))
    };
}

/// Prints farben-colored markup to stderr without a newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! ceprintb {
    () => {
        eprint!()
    };
    ($fmt:literal) => {
        eprint!("{}", $crate::colorb!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprint!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), true))
    };
}

/// Prints farben-colored markup to stderr with a trailing newline, without appending a reset.
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
/// use farben::prelude::*;
///
/// ceprintbln!("[bold red]Fatal error");
/// ceprintln!("still bold and red here"); // inherits style
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! ceprintbln {
    () => {
        eprintln!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprintln!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), true))
    };
}

/// Prints farben-colored markup to stderr with a trailing newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! ceprintbln {
    () => {
        eprintln!()
    };
    ($fmt:literal) => {
        eprintln!("{}", $crate::colorb!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        eprintln!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), true))
    };
}
