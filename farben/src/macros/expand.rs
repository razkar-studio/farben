#[macro_export]
macro_rules! expand {
    ($markup:expr) => {
        match farben::try_color($markup) {
            Ok(ansi) => {
                let tokens = farben_core::lexer::tokenize($markup).unwrap();
                eprintln!("input:    {}", $markup);
                eprintln!(
                    "expanded: {}",
                    farben_core::debug::tokens_to_markup(&tokens)
                );
                eprintln!("ansi:     {:?}", ansi);
            }
            Err(e) => eprintln!("expand!: {e}"),
        }
    };
}
