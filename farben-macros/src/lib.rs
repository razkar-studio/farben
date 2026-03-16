use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

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
/// ```
#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
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
/// print!("{}", colorb!("[bold red]Warning: "));
/// println!("this text is still bold red");
/// ```
#[proc_macro]
pub fn colorb(input: TokenStream) -> TokenStream {
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
/// ```rust
/// use farben_macros::markdown;
/// println!("{}", markdown!("**bold** and *italic*"));
/// ```
#[cfg(feature = "markdown")]
#[proc_macro]
pub fn markdown(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let result = farben_md::renderer::render(&farben_md::lexer::tokenize(&input.value()));
    quote! { #result }.into()
}
