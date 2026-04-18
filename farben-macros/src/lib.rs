//! Procedural macros for the Farben terminal styling library.
//!
//! Provides compile-time processing of farben markup and inline markdown,
//! baking the final ANSI-escaped strings directly into the binary with zero
//! runtime overhead. All macros in this crate are re-exported through
//! [`farben`] and should not be used directly in most cases.
//!
//! ## Features
//!
//! - `markdown`: enables [`markdown!`] and [`colorb!`] for compile-time
//!   markdown rendering via [`farben_md`].

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Reads `farben_registry.lsv` from `OUT_DIR` and pre-populates the compile-time registry.
///
/// Calls `insert_style` for each style entry and `set_prefix` for each prefix entry
/// written by the build script from `.frb` config files. Called at the start of each
/// proc macro invocation. If the file does not exist, the function returns silently.
fn load_registry() {
    let out_dir = std::env::var("OUT_DIR").unwrap_or_default();
    let path = std::path::Path::new(&out_dir).join("farben_registry.lsv");
    if let Ok(content) = std::fs::read_to_string(&path) {
        let mut sections = content.splitn(2, "---\n");
        let styles_section = sections.next().unwrap_or("");
        let prefixes_section = sections.next().unwrap_or("");

        for line in styles_section.lines() {
            if line.is_empty() {
                continue;
            }
            let (key, value) = line.split_once('=').unwrap();

            farben_core::registry::insert_style(
                key,
                farben_core::ansi::Style::parse(format!("[{value}]"))
                    .unwrap_or_else(|e| panic!("farben: invalid style in registry '{key}': {e}")),
            );
        }

        for line in prefixes_section.lines() {
            if line.is_empty() {
                continue;
            }
            let (key, value) = line.split_once('=').unwrap();

            farben_core::registry::set_prefix(key, value)
                .unwrap_or_else(|e| panic!("farben: failure while setting prefix '{key}': {e}"));
        }
    }
}

/// Parses and colorizes a farben markup string at compile time.
///
/// Tokenizes and renders the input at compile time, emitting the final ANSI-escaped
/// string as a string literal baked into the binary. Invalid markup produces a
/// compiler error at the call site.
///
/// # Examples
///
/// ```rust
/// use farben_macros::color;
/// println!("{}", color!("[bold red]Hello!"));
/// Compiles farben markup strings to ANSI escape codes at compile time.
///
/// Takes a string literal like `"[bold red]hello"` and produces an expression
/// that evaluates to the ANSI-escaped string at runtime.
#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
    load_registry();
    if std::env::var("NO_COLOR").is_err() {
        unsafe {
            std::env::set_var("FORCE_COLOR", "1");
        }
    }

    let input = parse_macro_input!(input as LitStr);
    let value = input.value();
    let tokens = match farben_core::lexer::tokenize(&value) {
        Ok(t) => t,
        Err(e) => {
            let msg = e.to_string();
            return syn::Error::new_spanned(&input, msg)
                .to_compile_error()
                .into();
        }
    };
    let result = format!("{}\x1b[0m", farben_core::parser::render(tokens));
    quote! { #result }.into()
}

/// Parses and colorizes a farben markup string at compile time, without appending a
/// trailing SGR reset.
///
/// Behaves identically to [`color!`] except the final `\x1b[0m` reset is omitted.
/// Styles applied by this macro bleed into subsequent terminal output, making it
/// useful for chaining multiple colored segments where the style should carry forward.
///
/// Invalid markup produces a compiler error at the call site.
///
/// # Examples
///
/// ```rust
/// use farben_macros::colorb;
/// // Style bleeds — subsequent output inherits bold red until a reset is issued.
/// Compiles farben markup to ANSI with a formatting writer.
///
/// Like [`color`] but returns a [`std::fmt::Write`] implementation for use
/// with format macros.
#[proc_macro]
pub fn colorb(input: TokenStream) -> TokenStream {
    load_registry();
    if std::env::var("NO_COLOR").is_err() {
        unsafe {
            std::env::set_var("FORCE_COLOR", "1");
        }
    }

    let input = parse_macro_input!(input as LitStr);
    let value = input.value();
    let tokens = match farben_core::lexer::tokenize(&value) {
        Ok(t) => t,
        Err(e) => {
            let msg = e.to_string();
            return syn::Error::new_spanned(&input, msg)
                .to_compile_error()
                .into();
        }
    };
    let result = farben_core::parser::render(tokens);
    quote! { #result }.into()
}

/// Validates farben markup at compile time and returns the original string literal unchanged.
///
/// On success, emits the input string literal as-is. On failure, emits a compiler error
/// at the call site. Used internally by `color_fmt!`, `cprint!`, and `cprintln!` to validate
/// the format string before runtime processing with format arguments.
///
/// This macro is part of the internal API. Prefer [`color!`] for static strings or
/// [`color_fmt!`] for strings with format arguments.
#[proc_macro]
pub fn validate_color(input: TokenStream) -> TokenStream {
    load_registry();

    let input = parse_macro_input!(input as LitStr);
    let value = input.value();

    match farben_core::lexer::tokenize(&value) {
        Ok(_) => quote! { #input }.into(),
        Err(e) => {
            let msg = e.to_string();
            syn::Error::new_spanned(&input, msg)
                .to_compile_error()
                .into()
        }
    }
}

/// Parses and renders an inline markdown string at compile time.
///
/// Tokenizes and renders the input at compile time, emitting the final
/// ANSI-escaped string as a string literal baked into the binary.
/// Supports `**bold**`, `*italic*`, `_italic_`, `__underline__`,
/// `~~strikethrough~~`, and `` `inline code` ``.
///
/// # Examples
///
/// Compiles inline markdown to ANSI-escaped terminal output at compile time.
///
/// Parses markdown syntax (`**bold**`, `*italic*`, `` `code` ``, `~~strike~~`, `__underline__`)
/// and produces terminal-compatible ANSI output.
#[cfg(feature = "markdown")]
#[proc_macro]
pub fn markdown(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let result = farben_md::renderer::render(&farben_md::lexer::tokenize(&input.value()));
    quote! { #result }.into()
}
