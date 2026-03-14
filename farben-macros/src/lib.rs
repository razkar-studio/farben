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
/// use farben::color;
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

/// Same as [`color!`], but bleeds.
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
/// at the call site. Used internally by `color_fmt!` and `cprintln!` to validate the
/// format string before runtime processing.
///
/// # Examples
///
/// ```rust
/// // Valid markup passes through unchanged
/// let s = farben_macros::validate_color!("[bold red]");
/// assert_eq!(s, "[bold red]");
/// ```
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
