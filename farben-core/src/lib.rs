//! Core library for the farben terminal styling crate.
//!
//! Provides the full pipeline for parsing farben markup strings into ANSI escape sequences:
//! tokenization ([`lexer`]), ANSI encoding ([`ansi`]), rendering ([`parser`]),
//! named style registration ([`registry`]), and error types ([`errors`]).
//!
//! Typical usage flows through the [`lexer::tokenize`] and [`parser::render`] functions,
//! with optional style definitions via the [`registry`] module and its macros.
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]

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
