//! Print and format macros for farben markup.
//!
//! Covers [`color_fmt!`], [`cprint!`], [`cprintln!`], [`cprintb!`], and [`cprintbln!`].
//! Each macro has two implementations selected by `#[cfg]`: a runtime variant and a
//! compile-time variant activated by the `compile` feature.

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
        farben::color_runtime(format!(farben::validate_color!($fmt) $(, $arg)*), false)
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
///
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
        print!("{}", farben::color_runtime(format!(farben::validate_color!($fmt) $(, $arg)*), false))
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
///
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
///
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
        println!("{}", farben::color_runtime(format!(farben::validate_color!($fmt) $(, $arg)*), false))
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
        print!("{}", farben::color_runtime(format!(farben::validate_color!($fmt) $(, $arg)*), true))
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
        println!("{}", farben::color_runtime(format!(farben::validate_color!($fmt) $(, $arg)*), true))
    };
}
