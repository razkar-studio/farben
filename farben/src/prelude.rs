//! The recommended way to bring Farben's public API into scope.
//!
//! Prefer `use farben::prelude::*` over `use farben::*`. The raw glob import
//! pulls in internal items (`color_runtime`, `validate_color`) that are `pub`
//! only to satisfy macro expansion requirements. This prelude exports only the
//! intentional public surface, gated by the same feature flags as their
//! definitions.
//!
//! # Usage
//!
//! ```rust
//! use farben::prelude::*;
//!
//! // try_color returns Result<String, LexError>; LexError is re-exported
//! // here so callers can pattern-match without a separate import.
//! match try_color("[red]Hello[/]") {
//!     Ok(s) => print!("{s}"),
//!     Err(e) => eprintln!("parse error: {e}"),
//! }
//! ```
//!
//! # What is included
//!
//! | Item | Condition |
//! |------|-----------|
//! | [`try_color`], [`strip_ansi`] | always |
//! | [`color`], [`colorb`] | always (function without `compile`; proc-macro with `compile`) |
//! | [`markdown`] | `markdown` or `markdown-compile` feature |
//! | [`LexError`] | always (needed to match on [`try_color`] results) |
//! | [`color_fmt!`], [`cprint!`], [`cprintln!`], [`cprintb!`], [`cprintbln!`], [`cwrite!`], [`cwriteln!`], [`cwriteb!`], [`cwritebln!`] | always |
//! | [`ceprint!`], [`ceprintln!`], [`ceprintb!`], [`ceprintbln!`] | always |
//! | [`ansi_strip!`] | always |
//! | [`md_fmt!`], [`mdprint!`], [`mdprintln!`], [`mdeprint!`], [`mdeprintln!`] | `markdown` or `markdown-compile` feature |
//! | [`style!`], [`prefix!`] | `format` feature |
//! | [`Style`], [`insert_style`], [`set_prefix`] | `format` feature |
//!
//! `color_runtime` and `validate_color` are excluded. Both are `pub` only so
//! that Farben's procedural macros can call them; they carry no stability
//! guarantees and are not part of the user-facing API.

pub use crate::strip_ansi;
pub use crate::try_color;

#[cfg(not(feature = "compile"))]
pub use crate::{color, colorb};

#[cfg(feature = "compile")]
pub use crate::{color, colorb};

#[cfg(any(feature = "markdown", feature = "markdown-compile"))]
pub use crate::markdown;

pub use farben_core::errors::LexError;

pub use crate::{
    color_fmt, cprint, cprintb, cprintbln, cprintln, cwrite, cwriteb, cwritebln, cwriteln,
};

pub use crate::{ceprint, ceprintb, ceprintbln, ceprintln};

pub use crate::ansi_strip;

#[cfg(any(feature = "markdown", feature = "markdown-compile"))]
pub use crate::{md_fmt, mdeprint, mdeprintln, mdprint, mdprintln};

#[cfg(feature = "format")]
pub use crate::{insert_style, set_prefix};

#[cfg(feature = "format")]
pub use crate::{prefix, style};
