//! Token tree renderer.
//!
//! Converts a [`MdToken`] tree produced by [`crate::lexer::tokenize`] into a
//! final ANSI-escaped string. Nested spans are handled via an active style stack
//! that re-emits surviving styles after each reset.

use crate::lexer::MdToken;
use farben_core::ansi::{Color, Ground, NamedColor, color_to_ansi, emphasis_to_ansi};
use farben_core::lexer::EmphasisType;

/// Renders a slice of [`MdToken`]s into an ANSI-escaped string.
///
/// Nested spans are handled correctly — styles are re-emitted after each
/// reset so that active styles are preserved across span boundaries.
/// Appends a final `\x1b[0m` reset to clean up after all output.
///
/// # Example
///
/// ```
/// use farben_md::{lexer::tokenize, renderer::render};
///
/// let tokens = tokenize("**bold** and *italic*");
/// let output = render(&tokens);
/// assert!(output.contains("\x1b[1m"));
/// ```
pub fn render(tokens: &[MdToken]) -> String {
    let mut out = String::new();
    let mut active: Vec<EmphasisType> = Vec::new();
    render_inner(tokens, &mut active, &mut out);
    out.push_str("\x1b[0m");
    out
}

fn render_inner(tokens: &[MdToken], active: &mut Vec<EmphasisType>, out: &mut String) {
    for token in tokens {
        match token {
            MdToken::Text(text) => out.push_str(text),
            MdToken::Code(text) => {
                out.push_str("\x1b[0m");
                out.push_str(&emphasis_to_ansi(&EmphasisType::Bold));
                out.push_str(&color_to_ansi(
                    &Color::Named(NamedColor::BrightWhite),
                    Ground::Foreground,
                ));
                out.push_str(&color_to_ansi(
                    &Color::Named(NamedColor::BrightBlack),
                    Ground::Background,
                ));
                out.push_str(text);
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Bold(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Bold));
                active.push(EmphasisType::Bold);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Italic(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Italic));
                active.push(EmphasisType::Italic);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Strikethrough(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Strikethrough));
                active.push(EmphasisType::Strikethrough);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
            MdToken::Underline(children) => {
                out.push_str(&emphasis_to_ansi(&EmphasisType::Underline));
                active.push(EmphasisType::Underline);
                render_inner(children, active, out);
                active.pop();
                out.push_str("\x1b[0m");
                for style in active.iter() {
                    out.push_str(&emphasis_to_ansi(style));
                }
            }
        }
    }
}
