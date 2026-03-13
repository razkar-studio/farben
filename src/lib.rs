//! # Introduction
//! Farben (as in "color" in german) is a zero-dependency terminal coloring library.
//! Farben uses a markup-language-like syntax to your string and outputs them colored. For example:
//!
//! # Example
//! ```
//! use farben::*;
//!
//! let colored = color("[red]I'm red!");
//! assert_eq!(colored, "\x1b[31mI'm red!\x1b[0m");
//! ```
pub(crate) mod ansi;
pub(crate) mod lexer;
pub(crate) mod parser;

pub fn color(input: impl Into<String>) -> String {
    let input = input.into();
    let mut res = parser::render(lexer::tokenize(input));
    res.push_str("\x1b[0m");
    res
}
