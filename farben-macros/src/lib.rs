//! Procedural macros for the Farben terminal styling library.
//!
//! Provides compile-time processing of farben markup and inline markdown,
//! baking the final ANSI-escaped strings directly into the binary with zero
//! runtime overhead. All macros in this crate are re-exported through
//! `farben` and should not be used directly in most cases.
//!
//! ## Features
//!
//! - `markdown`: enables `markdown!` and [`colorb!`] for compile-time
//!   markdown rendering via `farben_md`.

mod template;

use litext::litext;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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
            )
            .unwrap_or_else(|e| panic!("farben: invalid style name in registry '{key}': {e}"));
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
/// ```ignore
/// use farben_macros::color;
/// println!("{}", color!("[bold red]Hello!"));
/// ```
#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
    load_registry();

    let input = litext!(input as litext::LitStr);
    let value = input.value();
    let tokens = match farben_core::lexer::tokenize(value) {
        Ok(t) => t,
        Err(e) => {
            let msg = e.to_string();
            return comperr::error(input.span(), msg).into();
        }
    };
    let styled = format!("{}\x1b[0m", farben_core::parser::render_forced(tokens));
    quote! {
        ::farben::FarbenStr { styled: #styled }
    }
    .into()
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
/// ```ignore
/// use farben_macros::colorb;
/// println!("{}", colorb!("[bold red]Bleeds..."));
/// ```
#[proc_macro]
pub fn colorb(input: TokenStream) -> TokenStream {
    load_registry();

    let input = litext!(input as litext::LitStr);
    let value = input.value();
    let tokens = match farben_core::lexer::tokenize(value) {
        Ok(t) => t,
        Err(e) => {
            let msg = e.to_string();
            return comperr::error(input.span(), msg).into();
        }
    };
    let styled = farben_core::parser::render_forced(tokens);
    quote! {
        ::farben::FarbenStr { styled: #styled }
    }
    .into()
}

/// Validates farben markup at compile time and returns the original string literal unchanged.
///
/// On success, emits the input string literal as-is. On failure, emits a compiler error
/// at the call site. Used internally by `color_fmt!`, `cprint!`, and `cprintln!` to validate
/// the format string before runtime processing with format arguments.
///
/// This macro is part of the internal API. Prefer [`color!`] for static strings or
/// `color_fmt!` for strings with format arguments.
#[proc_macro]
pub fn validate_color(input: TokenStream) -> TokenStream {
    load_registry();

    let original = input.clone();

    let input = litext!(input as litext::LitStr);
    let value = input.value();

    match farben_core::lexer::tokenize(value) {
        Ok(_) => original,
        Err(e) => {
            let msg = e.to_string();
            comperr::error(input.span(), msg).into()
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
#[deprecated]
#[proc_macro]
pub fn markdown(input: TokenStream) -> TokenStream {
    let input = litext!(input as litext::LitStr);
    let value = input.value();
    let result = farben_md::renderer::render(&farben_md::lexer::tokenize(value));
    quote! { #result }.into()
}

/// Compiles a farben markup format string into an inlined `String`-building block.
///
/// Splits the format string at `{...}` placeholders at compile time. Static
/// segments between placeholders are rendered to ANSI escape sequences once,
/// baked into the binary as string literals, and written with a single
/// `push_str`. Dynamic arguments are written via `format_args!` at the call
/// site, preserving full format-spec support (`{name}`, `{0}`, `{:.2}`, etc.)
/// and allowing variable captures from the surrounding scope.
///
/// A trailing `\x1b[0m` reset is appended. For the bleed variant see
/// [`cformatb!`]. This macro is the compile-time backend for `cformat!` from
/// the `farben` crate.
#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    build_cformat(input, false)
}

/// Like [`cformat!`] but omits the trailing SGR reset.
///
/// Styles applied by the emitted block bleed into subsequent terminal output.
/// This macro is the compile-time backend for `cformatb!` from the `farben` crate.
#[proc_macro]
pub fn cformatb(input: TokenStream) -> TokenStream {
    build_cformat(input, true)
}

/// Shared implementation for [`cformat`] and [`cformatb`].
fn build_cformat(input: TokenStream, bleed: bool) -> TokenStream {
    load_registry();

    let mut iter = proc_macro2::TokenStream::from(input).into_iter();

    let Some(fmt_tt) = iter.next() else {
        return quote! { ::std::string::String::new() }.into();
    };

    let s = fmt_tt.to_string();
    let fmt_str = if s.starts_with('"') && s.ends_with('"') {
        let inner = &s[1..s.len() - 1];
        inner
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\\\", "\\")
            .replace("\\\"", "\"")
    } else {
        let msg = "cformat: expected a string literal as first argument";
        return comperr::error(fmt_tt.span(), msg).into();
    };

    let rest: Vec<proc_macro2::TokenTree> = iter.collect();
    let args: Vec<TokenStream2> = split_args(rest);

    let pieces = match template::split(&fmt_str, bleed) {
        Ok(p) => p,
        Err(e) => {
            return comperr::error(fmt_tt.span(), e.to_string()).into();
        }
    };

    let capacity: usize = pieces
        .iter()
        .filter_map(|p| {
            if let template::Piece::Static { ansi, .. } = p {
                Some(ansi.len())
            } else {
                None
            }
        })
        .sum();

    let mut arg_iter = args.iter();
    let mut stmts: Vec<TokenStream2> = Vec::new();

    for piece in &pieces {
        match piece {
            template::Piece::Static { ansi, plain } => {
                if ansi.is_empty() && plain.is_empty() {
                    continue;
                }
                stmts.push(quote! {
                    __out.push_str(if __color { #ansi } else { #plain });
                });
            }
            template::Piece::Arg(spec) => {
                let fmt_spec = format!("{{{spec}}}");
                let fmt_lit_str = format!(r#""{fmt_spec}""#);
                let fmt_lit: proc_macro2::TokenStream = fmt_lit_str.parse().unwrap();

                match arg_iter.next() {
                    Some(arg) => {
                        stmts.push(quote! {
                            ::std::fmt::Write::write_fmt(
                                &mut __out,
                                ::std::format_args!(#fmt_lit, #arg),
                            ).unwrap();
                        });
                    }
                    None => {
                        stmts.push(quote! {
                            ::std::fmt::Write::write_fmt(
                                &mut __out,
                                ::std::format_args!(#fmt_lit),
                            ).unwrap();
                        });
                    }
                }
            }
        }
    }

    quote! {
        {
            use ::std::fmt::Write as _;
            let __color = ::farben::color_enabled();
            let mut __out = ::std::string::String::with_capacity(#capacity);
            #(#stmts)*
            __out
        }
    }
    .into()
}

/// Splits a flat list of token trees on top-level commas into individual arg
/// token streams. The leading comma (between the format literal and first arg)
/// is consumed before this is called via `iter` already being past the literal.
fn split_args(tts: Vec<proc_macro2::TokenTree>) -> Vec<TokenStream2> {
    let mut args: Vec<TokenStream2> = Vec::new();
    let mut current: Vec<proc_macro2::TokenTree> = Vec::new();
    let mut depth = 0usize;

    for tt in tts {
        match &tt {
            proc_macro2::TokenTree::Punct(p) if p.as_char() == ',' && depth == 0 => {
                if !current.is_empty() {
                    args.push(current.drain(..).collect());
                }
            }
            proc_macro2::TokenTree::Group(_) => {
                depth += 1;
                current.push(tt);
                depth -= 1;
            }
            _ => current.push(tt),
        }
    }
    if !current.is_empty() {
        args.push(current.into_iter().collect());
    }
    args
}
