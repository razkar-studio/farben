use farben::prelude::*;
farben::load_styles!();

fn main() {
    cprintb!("[bold red]");
    cprintln!("Bold red, [/red]Just bold.");
    cprintln!("Shouldn't be bold red.");
}
