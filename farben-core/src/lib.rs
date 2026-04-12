//! Core library for the farben terminal styling crate.
//!
//! Provides the full pipeline for parsing farben markup strings into ANSI escape sequences:
//! tokenization ([`lexer`]), ANSI encoding ([`ansi`]), rendering ([`parser`]),
//! named style registration ([`registry`]), and error types ([`errors`]).
//!
//! v0.7.0 adds two supporting modules:
//!
//! - [`env`]: runtime detection of whether ANSI output should be enabled. [`env::color_enabled`]
//!   respects `NO_COLOR` and `FORCE_COLOR`, falling back to TTY detection (`isatty(1)` on Unix,
//!   `GetConsoleMode` on Windows).
//! - [`strip`]: exposes [`strip::strip_ansi`], which removes CSI escape sequences from a string.
//!   Useful for measuring display width, plain-text logging, or piping output to tools that do
//!   not interpret ANSI codes.
//!
//! Typical usage flows through the [`lexer::tokenize`] and [`parser::render`] functions,
//! with optional style definitions via the [`registry`] module and its macros.

pub mod ansi;
pub mod degrader;
pub mod env;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod registry;
pub mod strip;

#[cfg(feature = "anstyle")]
pub mod anstyle_conv;
