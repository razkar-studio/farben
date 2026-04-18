//! Debug macros for expanding and inspecting farben markup.

/// Debug macro to expand farben markup and print intermediate representations.
///
/// Prints the input markup, the re-serialized token stream, and the final ANSI output.
/// Useful for debugging markup parsing issues.
///
/// # Example
///
/// ```ignore
/// expand!("[bold red]hello");
/// // input:    [bold red]hello
/// // expanded: [bold][red]hello
/// // ansi:     "\x1b[1;31mhello"
/// ```
#[macro_export]
macro_rules! expand {
    ($markup:expr) => {
        match $crate::try_color($markup) {
            Ok(ansi) => {
                let tokens = $crate::core::tokenize($markup).unwrap();
                eprintln!("input:    {}", $markup);
                eprintln!("expanded: {}", $crate::core::tokens_to_markup(&tokens));
                eprintln!("ansi:     {:?}", ansi);
            }
            Err(e) => eprintln!("expand!: {e}"),
        }
    };
}
