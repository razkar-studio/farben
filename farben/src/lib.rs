//! # Farben
//!
//! Farben *(as in "color" in German)* is a zero-dependency terminal coloring library.
//! It uses a markup-like syntax to apply ANSI styles to your strings — named colors,
//! RGB, ANSI 256, emphasis styles, foreground and background targeting, custom named
//! tags, inline resets, and inline markdown rendering.
//!
//! ```
//! use farben::prelude::*;
//!
//! cprintln!("[bold green]Done![/] All tests passed.");
//! ```
//!
//! For a full walkthrough of everything Farben can do, check out the
//! [user guide](https://razkar-studio.github.io/farben/guide).
//!
//! # Features
//!
//! Farben is split into opt-in feature flags so you only pull in what you need:
//!
//! | Feature | What it adds |
//! |---|---|
//! | *(default)* | Runtime coloring: [`color`], [`colorb`], [`try_color`], [`cprint!`], [`cprintln!`], [`cprintb!`], [`cprintbln!`], [`cwrite!`], [`cwriteln!`], [`cwriteb!`], [`cwritebln!`], [`color_fmt!`] |
//! | `compile` | Compile-time validation of markup strings via proc macros |
//! | `format` | Named style registry: [`style!`], [`prefix!`] |
//! | `markdown` | Runtime inline markdown rendering: [`markdown`], [`md_fmt!`], [`mdprint!`], [`mdprintln!`] |
//! | `markdown-compile` | Compile-time inline markdown rendering |
//!
//! # Emphasis Styles
//!
//! Farben supports these emphasis types:
//!
//! | Tag | Description |
//! |---|---|
//! | `bold` | Bold (SGR 1) |
//! | `dim` | Dimmed (SGR 2) |
//! | `italic` | Italic (SGR 3) |
//! | `underline` | Underline (SGR 4) |
//! | `double-underline` | Double underline (SGR 21) |
//! | `blink` | Slow blink (SGR 5) |
//! | `rapid-blink` | Rapid blink (SGR 6) |
//! | `reverse` | Reverse video (SGR 7) |
//! | `invisible` | Hidden (SGR 8) |
//! | `strikethrough` | Strikethrough (SGR 9) |
//! | `overline` | Overline (SGR 53) |
//!
//! # Quick Examples
//!
//! ### Named colors and emphasis
//!
//! ```
//! use farben::prelude::*;
//!
//! cprintln!("[red]Error![/] Something went wrong.");
//! cprintln!("[bold underline]Important.[/]");
//! cprintln!("[bg:blue fg:white]Inverted.");
//! ```
//!
//! ### RGB and ANSI 256
//!
//! ```
//! use farben::prelude::*;
//!
//! cprintln!("[rgb(255,128,0)]Orange.");
//! cprintln!("[ansi(93)]Deep purple.");
//! ```
//!
//! ### Custom named tags (`format` feature)
//!
//! ```
//! use farben::prelude::*;
//!
//! style!("warn", "[bold yellow]");
//! prefix!("warn", "! ");
//! cprintln!("[warn]Watch out.");
//! ```
//!
//! ### Inline markdown (`markdown` feature)
//!
//! ```
//! use farben::prelude::*;
//!
//! mdprintln!("**bold**, *italic*, `code`, ~~strikethrough~~");
//! ```
//!
#![warn(missing_docs)]

#[cfg(feature = "compile")]
pub use farben_macros::{color, colorb, validate_color};

#[cfg(feature = "markdown-compile")]
pub use farben_macros::markdown;

pub use farben_core::ansi::Style;

pub use farben_core::errors::{LexError, LexErrorDisplay};

#[cfg(feature = "format")]
pub use farben_core::registry::{insert_style, set_prefix};

mod functions;
pub use functions::*;

pub mod prelude;

pub use farben_core::strip::{strip_ansi, strip_markup};

mod macros;

pub mod core;

#[cfg(test)]
mod tests;

pub use farben_core::env::color_enabled;

/// A compile-time colored string with both styled and plain variants.
/// Resolved at runtime based on environment and TTY detection.
pub struct FarbenStr {
    /// The string, when it is styled
    pub styled: &'static str,
    /// The string, without styling
    pub plain: &'static str,
}

impl FarbenStr {
    /// Returns the styled string if color is enabled, otherwise the plain string.
    #[inline]
    pub fn resolve(&self) -> &'static str {
        if color_enabled() {
            self.styled
        } else {
            self.plain
        }
    }
}

impl std::fmt::Display for FarbenStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.resolve())
    }
}
