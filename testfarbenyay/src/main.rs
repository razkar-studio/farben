use farben::prelude::*;
farben::load_styles!();

fn main() {
    style!("cool", "[bold red]");
    style!("cool2", "[cool bg:blue]");
    style!("thing", "[strikethrough cool2]");
    style!("idkthing", "[cool thing]");
    style!("verycool", "[idkthing overline double-underline]");
    expand!("[verycool]");
    cprintln!("[verycool]What");
}
