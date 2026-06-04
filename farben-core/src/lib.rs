//! Core library for the farben terminal styling crate.
//!
//! Provides the full pipeline for parsing farben markup strings into ANSI escape sequences:
//! tokenization ([`lexer`]), ANSI encoding ([`ansi`]), rendering ([`parser`]),
//! named style registration ([`registry`]), error types ([`errors`]), color degradation
//! ([`degrader`]), inline syntax pre-processing ([`inline`]), stripping utilities
//! ([`strip`]), environment detection ([`mod@env`]), and persistent style state
//! (via [`active_stack`], [`set_active_stack`], [`clear_active_stack`]).
//!
//! Typical usage flows through the [`lexer::tokenize`] and [`parser::render`] functions,
//! with optional style definitions via the [`registry`] module.
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
// Single-letter names (`r`, `g`, `b`, `h`, `s`, `v`, `l`, `a`, `c`, etc.) are standard
// mathematical notation in color-space conversion code.
#![allow(clippy::many_single_char_names)]

pub mod ansi;
pub mod debug;
pub mod degrader;
pub mod env;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod registry;
mod state;
pub use state::{active_stack, clear_active_stack, set_active_stack};
pub mod strip;

#[cfg(feature = "anstyle")]
pub mod anstyle_conv;

#[cfg(feature = "inline")]
pub mod inline;
