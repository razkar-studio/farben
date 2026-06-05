//! Print and format macros for farben markup.
//!
//! Covers [`cformat!`], [`cformatb!`], [`cprint!`], [`cprintln!`], [`cprintb!`], [`cprintbln!`],
//! [`cwrite!`], [`cwriteln!`], [`cwriteb!`], [`cwritebln!`]. Each macro has two implementations
//! selected by `#[cfg]`: a runtime variant and a compile-time variant activated by the `compile` feature.
//!
//! [`color_fmt!`] is deprecated in favor of [`cformat!`].

/// Deprecated in favor of `cformat!`.
///
/// Parses and renders a farben markup string with format arguments, appending a final SGR reset.
///
/// Behaves like [`format!`] but processes farben markup tags in the resulting string.
///
/// # Panics
///
/// Panics if the input is not valid farben markup.
#[cfg(not(feature = "compile"))]
#[deprecated = "in favor of cformat"]
#[macro_export]
macro_rules! color_fmt {
    ($($arg:tt)*) => {
        $crate::cformat!($($arg)*)
    };
}

/// Parses and renders a farben markup string with format arguments, appending a final SGR reset.
///
/// Behaves like [`format!`] but processes farben markup tags in the resulting string.
///
/// # Panics
///
/// Panics if the input is not valid farben markup.
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cformat {
    ($($arg:tt)*) => {
        $crate::color_runtime(format!($($arg)*), false)
    };
}

/// Deprecated in favor of `cformat!`
///
/// Parses and renders a farben markup string with format arguments, appending a final SGR reset.
///
/// Behaves like [`format!`] but processes farben markup tags in the resulting string.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the input is not valid farben markup.
#[cfg(feature = "compile")]
#[deprecated = "in favor of cformat"]
#[macro_export]
macro_rules! color_fmt {
    ($fmt:literal $($rest:tt)*) => {{
        $crate::cformat!($fmt $($rest)*)
    }};
}

/// Parses and renders a farben markup string with format arguments, without appending a reset.
///
/// Like [`cformat!`] but styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the input is not valid farben markup.
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cformatb {
    ($($arg:tt)*) => {
        $crate::color_runtime(format!($($arg)*), true)
    };
}

/// Parses and renders a farben markup string at compile time when possible.
///
/// With the `compile` feature, a bare literal like `cstr!("[red]Error")` is
/// rendered at compile time and returns a [`FarbenStr`] — visible as such in
/// `cargo expand`. Format arguments (explicit or implicit) fall through to
/// runtime via `color_runtime`, with the markup validated at compile time first.
///
/// Without the `compile` feature, the markup is processed at runtime.
///
/// Returns a [`FarbenStr`] when rendered at compile time, or a [`String`]
/// otherwise. Both types implement [`Display`], so `println!("{}", cstr!(...))`
/// works regardless of features.
///
/// This is the canonical replacement for the `color()` / `color!()` duality.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`](crate::try_color) for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// cprintln!("{}", cstr!("[green]Success!"));
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cstr {
    ($($arg:tt)*) => {
        $crate::color_runtime(format!($($arg)*), false)
    };
}

/// Parses and renders a farben markup string at compile time when possible.
///
/// With the `compile` feature, a bare literal like `cstr!("[red]Error")` is
/// rendered at compile time and returns a [`FarbenStr`] — visible as such in
/// `cargo expand`. Format arguments (explicit or implicit) fall through to
/// runtime via `color_runtime`, with the markup validated at compile time first.
///
/// Without the `compile` feature, the markup is processed at runtime.
///
/// Returns a [`FarbenStr`] when rendered at compile time, or a [`String`]
/// otherwise. Both types implement [`Display`], so `println!("{}", cstr!(...))`
/// works regardless of features.
///
/// This is the canonical replacement for the `color()` / `color!()` duality.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`try_color`](crate::try_color) for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// cprintln!("{}", cstr!("[green]Success!"));
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cstr {
    ($fmt:literal) => {
        $crate::compile_cprint!($fmt)
    };
    ($fmt:literal $($rest:tt)*) => {
        $crate::color_runtime(::std::format!($fmt $($rest)*), false)
    };
}

/// Prints farben-colored markup to stdout without a newline.
///
/// Behaves like [`print!`] but processes farben markup tags before output.
/// The format string is validated at compile time when the `compile` feature is enabled.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
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
    ($fmt:literal $($rest:tt)*) => {
        print!("{}", $crate::color_runtime(format!($fmt $($rest)*), false))
    };
}

/// Prints farben-colored markup to stdout without a newline.
///
/// Behaves like [`print!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
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
        print!("{}", $crate::compile_cprint!($fmt))
    };
    ($fmt:literal $($rest:tt)*) => {
        print!("{}", $crate::cformat!($fmt $($rest)*))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline.
///
/// Behaves like [`println!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
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
    ($fmt:literal $($rest:tt)*) => {
        println!("{}", $crate::color_runtime(format!($fmt $($rest)*), false))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline.
///
/// Behaves like [`println!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
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
        println!("{}", $crate::compile_cprint!($fmt))
    };
    ($fmt:literal $($rest:tt)*) => {
        println!("{}", $crate::cformat!($fmt $($rest)*))
    };
}

/// Prints farben-colored markup to stdout without a newline, without appending a reset.
///
/// Styles bleed into subsequent output. Use when chaining multiple colored segments
/// where you want the style to carry forward.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
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
    ($fmt:literal $($rest:tt)*) => {
        print!("{}", $crate::color_runtime(format!($fmt $($rest)*), true))
    };
}

/// Prints farben-colored markup to stdout without a newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprintb {
    () => {
        print!()
    };
    ($fmt:literal) => {
        print!("{}", $crate::compile_cprintb!($fmt))
    };
    ($fmt:literal $($rest:tt)*) => {
        print!("{}", $crate::cformatb!($fmt $($rest)*))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline, without appending a reset.
///
/// Styles bleed into subsequent output. Use when chaining multiple colored segments
/// where you want the style to carry forward.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
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
    ($fmt:literal $($rest:tt)*) => {
        println!("{}", $crate::color_runtime(format!($fmt $($rest)*), true))
    };
}

/// Prints farben-colored markup to stdout with a trailing newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cprintbln {
    () => {
        println!()
    };
    ($fmt:literal) => {
        println!("{}", $crate::compile_cprintb!($fmt))
    };
    ($fmt:literal $($rest:tt)*) => {
        println!("{}", $crate::cformatb!($fmt $($rest)*))
    };
}

/// Writes farben-colored markup to a writer without a newline.
///
/// Behaves like [`write!`] but processes farben markup tags before output.
/// The format string is validated at compile time when the `compile` feature is enabled.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
///
/// # Examples
///
/// ```ignore
/// use farben::prelude::*;
/// use std::io::{Cursor, Write};
///
/// let mut buf = Cursor::new(Vec::new());
/// cwrite!(buf, "[red]Error: [/]{}", "something went wrong");
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwrite {
    ($writer:expr $(, $arg:tt)*) => {
        write!($writer, "{}", $crate::color_runtime(format!($($arg),*), false))
    };
}

/// Writes farben-colored markup to a writer without a newline.
///
/// Behaves like [`write!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
///
/// # Examples
///
/// ```ignore
/// use farben::prelude::*;
/// use std::io::{Write, Cursor};
///
/// let mut buf = Cursor::new(Vec::new());
/// cwrite!(buf, "[red]Error: [/]{}", "something went wrong");
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwrite {
    ($writer:expr, $fmt:literal) => {
        write!($writer, "{}", $crate::compile_cprint!($fmt))
    };
    ($writer:expr, $fmt:literal $($rest:tt)*) => {
        write!($writer, "{}", $crate::cformat!($fmt $($rest)*))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline.
///
/// Behaves like [`writeln!`] but processes farben markup tags before output.
/// The format string is validated at compile time when the `compile` feature is enabled.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
///
/// # Examples
///
/// ```ignore
/// use farben::prelude::*;
/// use std::io::{Write, Cursor};
///
/// let mut buf = Cursor::new(Vec::new());
/// cwriteln!(buf, "[green]Success: [/]{}", "all done");
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwriteln {
    ($writer:expr $(, $arg:tt)*) => {
        writeln!($writer, "{}", $crate::color_runtime(format!($($arg),*), false))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline.
///
/// Behaves like [`writeln!`] but processes farben markup tags before output.
/// The format string is validated at compile time.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
///
/// # Examples
///
/// ```ignore
/// use farben::prelude::*;
/// use std::io::{Write, Cursor};
///
/// let mut buf = Cursor::new(Vec::new());
/// cwriteln!(buf, "[green]Success: [/]{}", "all done");
/// ```
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwriteln {
    ($writer:expr, $fmt:literal) => {
        writeln!($writer, "{}", $crate::compile_cprint!($fmt))
    };
    ($writer:expr, $fmt:literal $($rest:tt)*) => {
        writeln!($writer, "{}", $crate::cformat!($fmt $($rest)*))
    };
}

/// Writes farben-colored markup to a writer without a newline, without appending a reset.
///
/// Styles bleed into subsequent output. Use when chaining multiple colored segments
/// where you want the style to carry forward.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// use std::io::{Write, Cursor};
///
/// let mut buf = Cursor::new(Vec::new());
/// cwriteb!(buf, "[red]Error: ");
/// cwrite!(buf, "something went wrong"); // inherits red
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwriteb {
    ($writer:expr $(, $arg:tt)*) => {
        write!($writer, "{}", $crate::color_runtime(format!($($arg),*), true))
    };
}

/// Writes farben-colored markup to a writer without a newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwriteb {
    ($writer:expr, $fmt:literal) => {
        write!($writer, "{}", $crate::compile_cprintb!($fmt))
    };
    ($writer:expr, $fmt:literal $($rest:tt)*) => {
        write!($writer, "{}", $crate::cformatb!($fmt $($rest)*))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline, without appending a reset.
///
/// Styles bleed into subsequent output. Use when chaining multiple colored segments
/// where you want the style to carry forward.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
///
/// # Examples
///
/// ```
/// use farben::prelude::*;
/// use std::io::{Write, Cursor};
///
/// let mut buf = Cursor::new(Vec::new());
/// cwritebln!(buf, "[bold red]Fatal error");
/// cwrite!(buf, "still bold and red here"); // inherits style
/// ```
#[cfg(not(feature = "compile"))]
#[macro_export]
macro_rules! cwritebln {
    ($writer:expr $(, $arg:tt)*) => {
        writeln!($writer, "{}", $crate::color_runtime(format!($($arg),*), true))
    };
}

/// Writes farben-colored markup to a writer with a trailing newline, without appending a reset.
///
/// Format string is validated at compile time. Styles bleed into subsequent output.
///
/// # Panics
///
/// Panics if the markup is invalid. Use [`crate::try_color`] directly for error handling.
#[cfg(feature = "compile")]
#[macro_export]
macro_rules! cwritebln {
    ($writer:expr, $fmt:literal) => {
        writeln!($writer, "{}", $crate::compile_cprintb!($fmt))
    };
    ($writer:expr, $fmt:literal $($rest:tt)*) => {
        writeln!($writer, "{}", $crate::cformatb!($fmt $($rest)*))
    };
}
