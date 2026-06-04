//! # Deprecation
//!
//! Use the `inline` feature of Farben instead.
//!
//! ---
//!
//! Markdown rendering for the terminal.
//!
//! Parses a subset of inline markdown syntax and renders it as ANSI-escaped
//! terminal output. Supports bold, italic, underline, strikethrough, and inline
//! code. Delegates ANSI encoding to [`farben_core::ansi`].

#![deprecated(
    since = "0.2.7",
    note = "crate deprecated; use `inline` of Farben instead"
)]

pub mod lexer;
pub mod renderer;
