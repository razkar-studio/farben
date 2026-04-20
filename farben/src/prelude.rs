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
//! cprintln!("[bold green]Done![/] All tests passed.");
//! ```
//!
//! # What is included
//!
//! | Item | Condition |
//! |------|-----------|
//! | [`color`], [`colorb`] | always |
//! | [`LexError`] | always |
//! | [`cformat!`], [`cprint!`], [`cprintln!`], [`cprintb!`], [`cprintbln!`] | always |
//! | [`cwrite!`], [`cwriteln!`], [`cwriteb!`], [`cwritebln!`] | always |
//! | [`ceprint!`], [`ceprintln!`], [`ceprintb!`], [`ceprintbln!`] | always |
//! | [`ansi_strip!`], [`markup_strip!`] | always |
//! | [`expand!`] | always |
//! | [`style!`], [`prefix!`] | `format` feature |

#[cfg(not(feature = "compile"))]
pub use crate::{color, colorb};

#[cfg(feature = "compile")]
pub use crate::{color, colorb};

pub use farben_core::errors::LexError;

#[allow(deprecated)]
pub use crate::color_fmt;

pub use crate::{
    cformat, cformatb, cprint, cprintb, cprintbln, cprintln, cwrite, cwriteb, cwritebln, cwriteln,
};

pub use crate::{ceprint, ceprintb, ceprintbln, ceprintln};

pub use crate::expand;

pub use crate::{ansi_strip, markup_strip};

#[cfg(feature = "format")]
pub use crate::{prefix, style};

#[cfg(any(feature = "markdown", feature = "markdown-compile"))]
pub use crate::{md_fmt, mdeprint, mdeprintln, mdprint, mdprintln};
