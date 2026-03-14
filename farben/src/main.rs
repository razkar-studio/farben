use farben::{color, color_fmt, try_color};

fn main() {
    println!("{}", color("[red]I'm red!"));
    println!("Gee, I'm not red. Don't have anything to do with that guy.");
    println!();
    println!("{}", color("[rgb(255,0,0)]I'm red, but RGB!"));
    println!(
        "{}",
        color("[ansi(1)]Gee, I'm red, but ansi256. Don't have anything to do with that guy.")
    );
    println!();
    println!("{}", color("[bold red]I'm bold AND red!"));
    println!(
        "{}",
        color(
            "[italic red]Gee, I'm red first, [/]reset, [bold ansi(21)]then I'm bold blue in ansi256. Don't have anything to do with that guy."
        )
    );
    println!();
    println!("{:?}", try_color("[error] This would fail"));
    println!("{}", color("\\[error] But this wouldn't!"));

    let msg = "I'm a variable AND I'm highlighted!";
    println!(
        "{}",
        color_fmt!("[bold]I'm formatted! Here's the text: [bg:white fg:black]{msg}")
    )
}
