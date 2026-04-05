use farben::prelude::*;
farben::load_styles!();

fn main() {
    eprintln!("color_enabled: {}", farben_core::env::color_enabled());
    init_styles();
    eprintln!("tokens: {:?}", farben_core::lexer::tokenize("[error]"));
    eprintln!(
        "rendered: {:?}",
        farben_core::parser::render(farben_core::lexer::tokenize("[error]hello").unwrap())
    );
    eprintln!("runtime color: {:?}", farben::try_color("[error]test"));
    println!(color!("[error] something went wrong"));
    println!("{}", color_fmt!("[error] something went wrong"));

    eprintln!("compile time result: {}", color!("[error]test"));
    eprintln!("compile time result raw: {:?}", color!("[error]test"));
    color!("[]");

    cprintln!("[error] something went wrong");
    cprintln!("[my:error] something really went wrong");
}
