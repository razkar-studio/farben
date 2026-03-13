use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

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

    let result = farben_core::parser::render(tokens);
    quote! { #result }.into()
}

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
