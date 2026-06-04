//! # Farben
//!
//! Farben (as in "color" in German) is a zero-dependency terminal coloring library.
//! It uses a markup-like syntax to apply ANSI styles to your strings: named colors,
//! RGB, HSL, HSV/HSB, HWB, Lab, LCH, `OKLCh`, hex, ANSI 256, emphasis styles,
//! foreground and background targeting, custom named tags, inline resets, and
//! inline shorthand syntax.
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
//! | *(default)* | Runtime coloring: [`color`], [`colorb`], [`cformat!`], [`cprint!`], [`cprintln!`], [`cprintb!`], [`cprintbln!`], [`cwrite!`], [`cwriteln!`], [`cwriteb!`], [`cwritebln!`] |
//! | `compile` | Compile-time validation of markup strings via proc macros |
//! | `format` | Named style registry: [`style!`], [`prefix!`] |
//! | `inline` | Inline shorthand syntax (`*bold*`, `/italic/`, `` `code` ``) inside all `c*` macros |
//! | `lossy` | Lenient parsing for unknown tags (default) |
//! | `anstyle` | Interoperability with `anstyle::Style` |
//! | `markdown` | **Deprecated.** Runtime inline markdown rendering. Use `inline` instead. |
//! | `markdown-compile` | **Deprecated.** Compile-time inline markdown. Use `inline` + `compile` instead. |
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
//! ### RGB, HSL, hex, and ANSI 256
//!
//! ```
//! use farben::prelude::*;
//!
//! cprintln!("[rgb(255,128,0)]Orange.");
//! cprintln!("[hsl(120,100,50)]Green via HSL.");
//! cprintln!("[#ff8800]Orange via hex.");
//! cprintln!("[ansi(93)]Deep purple.");
//! ```
//!
//! ### Custom named tags (`format` feature)
//!
//! ```
//! use farben::prelude::*;
//! use farben::try_color;
//!
//! style!("warn", "[bold yellow]");
//! prefix!("warn", "! ");
//! println!("{}", try_color("[warn]Watch out.").unwrap());
//! ```
//!
//! ### Inline syntax (`inline` feature)
//!
//! ```
//! use farben::prelude::*;
//!
//! cprintln!("This is *bold* and /italic/ with `inline code`.");
//! ```
//!
#![warn(missing_docs)]

extern crate self as farben;

#[cfg(feature = "compile")]
pub use farben_macros::{cformat, cformatb, color, colorb, validate_color};

#[cfg(feature = "markdown-compile")]
pub use farben_macros::markdown;

pub use farben_core::ansi::Style;

pub use farben_core::errors::{LexError, LexErrorDisplay, RegistryError};

#[cfg(feature = "format")]
pub use farben_core::registry::{insert_style, set_prefix};

mod functions;
pub use functions::*;

pub mod prelude;

pub use farben_core::strip::{escape_tags, strip_ansi, strip_markup};

mod macros;

pub mod core;

#[cfg(test)]
mod tests;

pub use farben_core::env::color_enabled;

/// A compile-time colored string. Stores only the ANSI-styled variant;
/// plain text is derived at runtime via [`strip_ansi`] when color is disabled.
pub struct FarbenStr {
    /// The pre-rendered, ANSI-escaped string baked in at compile time.
    pub styled: &'static str,
}

impl FarbenStr {
    /// Returns the styled string if color is enabled, otherwise strips ANSI
    /// escapes and returns the plain text. Borrowing avoids allocation on the
    /// hot path; stripping only allocates when color is actually disabled.
    #[inline]
    #[must_use]
    pub fn resolve(&self) -> std::borrow::Cow<'static, str> {
        if color_enabled() {
            std::borrow::Cow::Borrowed(self.styled)
        } else {
            std::borrow::Cow::Owned(strip_ansi(self.styled))
        }
    }
}

impl std::fmt::Display for FarbenStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.resolve())
    }
}
