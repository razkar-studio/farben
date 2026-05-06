use farben::prelude::*;
farben::load_styles!();

fn main() {
    init_styles();
    style!("code", "[dim green]");
    cprintln!("]]]]]] Bold red");
    // cprintln!("\\[ wouldn't work no more");
    cprintln!("`thing` yay!");
    cprintln!("[red]thing[/red]");
    cprintln!("/italic/ _underline_ ~strikethrough~ `code` *bold*");
}
