//! Style registry macros.
//!
//! [`style!`] registers a named style in the global registry. [`prefix!`] attaches
//! a literal prefix string to an already-registered style. Both require the `format` feature.

/// Defines a named style in the global registry.
///
/// Parses `$markup` as a farben markup string and stores the resulting style
/// under `$name`. The style can then be used in markup as `[$name]`.
/// Panics if the markup is invalid.
///
/// # Examples
///
/// ```
/// use $crate::prelude::*;
///
/// style!("danger", "[bold red]");
/// // [danger] in markup now expands to bold red text
/// ```
#[cfg(feature = "format")]
#[macro_export]
macro_rules! style {
    ($name:expr, $markup:expr) => {
        $crate::insert_style($name, $crate::Style::parse($markup).unwrap());
    };
}

/// Sets a prefix string on a previously defined named style.
///
/// The prefix is injected as a literal string before the style's ANSI escape sequence
/// when rendered. The style must already exist in the registry; call [`style!`] first.
///
/// # Panics
///
/// Panics if `$name` has not been registered. Use [`farben_core::registry::set_prefix`]
/// directly to handle this case without panicking.
///
/// # Examples
///
/// ```
/// use $crate::prelude::*;
///
/// style!("warn", "[yellow]");
/// prefix!("warn", "⚠ ");
/// // [warn] now renders "⚠ " followed by the yellow escape sequence
/// ```
#[cfg(feature = "format")]
#[macro_export]
macro_rules! prefix {
    ($name:expr, $prefix:expr) => {
        $crate::set_prefix($name, $prefix).unwrap_or_else(|e| panic!("{}", e));
    };
}
