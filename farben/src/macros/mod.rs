//! Macro modules for farben.
//!
//! Organized by feature area: `color` for the print and format macros,
//! `format` for the style registry macros, and `markdown` for inline markdown printing.

pub mod color;
pub mod eprint;
pub mod expand;
pub mod format;
pub mod load;
pub mod markdown;
pub mod strip;

#[cfg(feature = "anstyle")]
pub mod anstyle;
