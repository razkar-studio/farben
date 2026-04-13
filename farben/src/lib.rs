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
//! # Known Limitations
//!
//! **Custom named tags are not validated at compile time.** The `compile` feature validates
//! markup strings via proc macros, but it has no visibility into styles registered at runtime
//! via [`style!`]. Using a custom tag like `[warn]` with `compile` active will fail to compile
//! even if the style is registered before use. Stick to runtime macros (`cprintln!` without
//! `compile`) when working with custom tags.
//!
//! This feature is a work in progress and I am currently working towards building it.
//! Make awesome things with Farben.

#[cfg(feature = "compile")]
pub use farben_macros::{color, colorb, validate_color};

#[cfg(feature = "markdown-compile")]
pub use farben_macros::markdown;

pub use farben_core::ansi::Style;

#[cfg(feature = "format")]
pub use farben_core::registry::{insert_style, set_prefix};

mod functions;
pub use functions::*;

pub mod prelude;

pub use farben_core::strip::strip_ansi;

mod macros;

pub mod core;

#[cfg(test)]
mod tests;
