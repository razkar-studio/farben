//! Convenience re-export of Farben's full public API.
//!
//! Importing this module with `use farben::prelude::*` brings every user-facing
//! item into scope: functions, macros, and types; gated by the same feature
//! flags as their definitions.
//!
//! # What is included
//!
//! | Item | Condition |
//! |------|-----------|
//! | [`try_color`], [`strip_ansi`] | always |
//! | [`color`], [`colorb`] | always (function without `compile`; proc-macro with `compile`) |
//! | [`markdown`] | `markdown` or `markdown-compile` feature |
//! | [`LexError`] | always (needed to match on [`try_color`] results) |
//! | [`color_fmt!`], [`cprint!`], [`cprintln!`], [`cprintb!`], [`cprintbln!`] | always |
//! | [`ceprint!`], [`ceprintln!`], [`ceprintb!`], [`ceprintbln!`] | always |
//! | [`ansi_strip!`] | always |
//! | [`md_fmt!`], [`mdprint!`], [`mdprintln!`], [`mdeprint!`], [`mdeprintln!`] | `markdown` or `markdown-compile` feature |
//! | [`style!`], [`prefix!`] | `format` feature |
//! | [`Style`], [`insert_style`], [`set_prefix`] | `format` feature |
//!
//! `color_runtime` and `validate_color` are intentionally excluded — they are
//! public for macro expansion purposes but are not part of the user-facing API.

pub use crate::strip_ansi;
pub use crate::try_color;

#[cfg(not(feature = "compile"))]
pub use crate::{color, colorb};

#[cfg(feature = "compile")]
pub use crate::{color, colorb};

#[cfg(any(feature = "markdown", feature = "markdown-compile"))]
pub use crate::markdown;

pub use farben_core::errors::LexError;

pub use crate::{color_fmt, cprint, cprintb, cprintbln, cprintln};

pub use crate::{ceprint, ceprintb, ceprintbln, ceprintln};

pub use crate::ansi_strip;

#[cfg(any(feature = "markdown", feature = "markdown-compile"))]
pub use crate::{md_fmt, mdeprint, mdeprintln, mdprint, mdprintln};

#[cfg(feature = "format")]
pub use crate::{insert_style, set_prefix};

#[cfg(feature = "format")]
pub use farben_core::ansi::Style;

#[cfg(feature = "format")]
pub use crate::{prefix, style};
