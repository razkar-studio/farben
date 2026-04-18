//! Core library for the farben terminal styling crate.
//!
//! Provides the full pipeline for parsing farben markup strings into ANSI escape sequences:
//! tokenization ([`lexer`]), ANSI encoding ([`ansi`]), rendering ([`parser`]),
//! named style registration ([`registry`]), and error types ([`errors`]).
//!
//! Typical usage flows through the [`lexer::tokenize`] and [`parser::render`] functions,
//! with optional style definitions via the [`registry`] module and its macros.

pub mod ansi;
pub mod debug;
pub mod degrader;
pub mod env;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod registry;
pub mod strip;

#[cfg(feature = "anstyle")]
pub mod anstyle_conv;
