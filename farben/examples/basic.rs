use farben::prelude::*;

fn main() {
    // Runtime version
    // println!("{}", color("[red]I'm red!"));

    // Compile-time version
    // println!("{}", color!("[blue]I'm blue, compiled!"));

    // Format args version
    let name = "Razkar";
    cprintln!("[green]Hello, {name}!");
    cprintln!("[green]Hey, {}.", name);
    cprintln!("[green]What's cooking, {name}?", name = name);
    cprintln!("[red]Literal here.");
}
