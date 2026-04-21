use farben::prelude::*;
farben::load_styles!();

fn main() {
    init_styles();
    cprintln!("]]]]]] Bold red");
    // cprintln!("\\[ wouldn't work no more");
    cprintln!("`thing` yay!");
    cprintln!("[red]thing[/red]")
}
