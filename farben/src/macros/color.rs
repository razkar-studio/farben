//! Print and format macros for farben markup.
//!
//! Covers [`color_fmt!`], [`cprint!`], [`cprintln!`], [`cprintb!`], [`cprintbln!`],
//! [`cwrite!`], [`cwriteln!`], [`cwriteb!`], [`cwritebln!`]. Each macro has two implementations
//! selected by `#[cfg]`: a runtime variant and a compile-time variant activated by the `compile` feature.

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
$crate::color_runtime(format!($($arg)*), false)
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
        $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), false)
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
/// use farben::prelude::*;
/// let message = "I don't know";
/// cprint!("[red]Error: [/]{}", message);
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprint {
    () => {
        print!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), false))
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
/// use farben::prelude::*;
/// let message = "I don't know";
/// cprint!("[red]Error: [/]{}", message);
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprint {
    () => {
        print!()
    };
    ($fmt:literal) => {
        print!("{}", $crate::color!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), false))
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
/// use farben::prelude::*;
/// let result = "We did it!";
/// cprintln!("[green]Success: [/]{}", result);
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprintln {
    () => {
        println!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), false))
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
/// use farben::prelude::*;
/// let result = "We did it!";
/// cprintln!("[green]Success: [/]{}", result);
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprintln {
    () => {
        println!()
    };
    ($fmt:literal) => {
        println!("{}", $crate::color!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), false))
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
/// use farben::prelude::*;
///
/// cprintb!("[red]Error: ");
/// cprintln!("something went wrong"); // inherits red
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprintb {
    () => {
        print!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), true))
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
    () => {
        print!()
    };
    ($fmt:literal) => {
        print!("{}", $crate::colorb!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        print!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), true))
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
/// use farben::prelude::*;
///
/// cprintbln!("[bold red]Section header");
/// cprintln!("still bold and red here"); // inherits style
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cprintbln {
    () => {
        println!()
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", $crate::color_runtime(format!($fmt $(, $arg)*), true))
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
    () => {
        println!()
    };
    ($fmt:literal) => {
        println!("{}", $crate::colorb!($fmt))
    };
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), true))
    };
}

/// Writes farben-colored markup to a writer without a newline.
///
/// Behaves like [`write!`] but processes farben markup tags before output.
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
/// use std::io::Cursor;
///
/// let mut buf = Cursor::new(Vec::new());
/// cwrite!(buf, "[red]Error: [/]{}", "something went wrong");
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwrite {
    ($writer:expr $(, $arg:tt)*) => {
        write!($writer, "{}", $crate::color_runtime(format!($($arg)*), false))
    };
}

/// Writes farben-colored markup to a writer without a newline.
///
/// Behaves like [`write!`] but processes farben markup tags before output.
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
/// use std::io::Cursor;
///
/// let mut buf = Cursor::new(Vec::new());
/// cwrite!(buf, "[red]Error: [/]{}", "something went wrong");
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwrite {
    ($writer:expr, $fmt:literal) => {
        write!($writer, "{}", $crate::color!($fmt))
    };
    ($writer:expr, $fmt:literal $(, $arg:expr)*) => {
        write!($writer, "{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), false))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline.
///
/// Behaves like [`writeln!`] but processes farben markup tags before output.
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
/// use std::io::Cursor;
///
/// let mut buf = Cursor::new(Vec::new());
/// cwriteln!(buf, "[green]Success: [/]{}", "all done");
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwriteln {
    ($writer:expr $(, $arg:tt)*) => {
        writeln!($writer, "{}", $crate::color_runtime(format!($($arg)*), false))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline.
///
/// Behaves like [`writeln!`] but processes farben markup tags before output.
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
/// use std::io::Cursor;
///
/// let mut buf = Cursor::new(Vec::new());
/// cwriteln!(buf, "[green]Success: [/]{}", "all done");
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwriteln {
    ($writer:expr, $fmt:literal) => {
        writeln!($writer, "{}", $crate::color!($fmt))
    };
    ($writer:expr, $fmt:literal $(, $arg:expr)*) => {
        writeln!($writer, "{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), false))
    };
}

/// Writes farben-colored markup to a writer without a newline, without appending a reset.
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
/// use std::io::Cursor;
///
/// let mut buf = Cursor::new(Vec::new());
/// cwriteb!(buf, "[red]Error: ");
/// cwrite!(buf, "something went wrong"); // inherits red
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwriteb {
    ($writer:expr $(, $arg:tt)*) => {
        write!($writer, "{}", $crate::color_runtime(format!($($arg)*), true))
    };
}

/// Writes farben-colored markup to a writer without a newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwriteb {
    ($writer:expr, $fmt:literal) => {
        write!($writer, "{}", $crate::colorb!($fmt))
    };
    ($writer:expr, $fmt:literal $(, $arg:expr)*) => {
        write!($writer, "{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), true))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline, without appending a reset.
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
/// use std::io::Cursor;
///
/// let mut buf = Cursor::new(Vec::new());
/// cwritebln!(buf, "[bold red]Fatal error");
/// cwrite!(buf, "still bold and red here"); // inherits style
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwritebln {
    ($writer:expr $(, $arg:tt)*) => {
        writeln!($writer, "{}", $crate::color_runtime(format!($($arg)*), true))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwritebln {
    ($writer:expr, $fmt:literal) => {
        writeln!($writer, "{}", $crate::colorb!($fmt))
    };
    ($writer:expr, $fmt:literal $(, $arg:expr)*) => {
        writeln!($writer, "{}", $crate::color_runtime(format!($crate::validate_color!($fmt) $(, $arg)*), true))
    };
}
