use farben::color_fmt; // the proc-macro version

fn main() {
    // Runtime version
    // println!("{}", color("[red]I'm red!"));

    // Compile-time version
    // println!("{}", color!("[blue]I'm blue, compiled!"));

    // Format args version
    let name = "Razkar";
    println!("{}", color_fmt!("[green]Hello, {}!", name));
}
