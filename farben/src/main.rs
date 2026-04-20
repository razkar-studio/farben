//! Demo binary showcasing the full farben feature set.

use farben::prelude::*;
use farben::try_color;

fn main() {
    cprintln!("[rgb(128,0,255)]RGB purple");
    cprintln!("[ansi(200)]ANSI 200");
    cprintln!("[red]Named red");
    showcase();
}

fn showcase() {
    section("Named Colors");
    cprintln!(
        "[black]black [red]red [green]green [yellow]yellow [blue]blue [magenta]magenta [cyan]cyan [white]white"
    );

    section("Bright Variants");
    cprintln!(
        "[bright-black]bright-black [bright-red]bright-red [bright-green]bright-green [bright-yellow]bright-yellow"
    );
    cprintln!(
        "[bright-blue]bright-blue [bright-magenta]bright-magenta [bright-cyan]bright-cyan [bright-white]bright-white"
    );

    section("Foreground & Background");
    cprintln!("[fg:red]explicit foreground");
    cprintln!("[bg:blue]blue background");
    cprintln!("[fg:white bg:blue]white text on blue background");
    cprintln!("[fg:black bg:yellow]black text on yellow background");

    section("RGB Colors");
    cprintln!("[rgb(255,80,80)]rgb(255,80,80) -soft red");
    cprintln!("[rgb(80,200,120)]rgb(80,200,120) -soft green");
    cprintln!("[rgb(100,150,255)]rgb(100,150,255) -cornflower blue");
    cprintln!("[bg:rgb(30,30,30) fg:rgb(220,220,220)]dark background, light foreground");

    section("ANSI 256");
    cprintln!(
        "[ansi(1)]ansi(1) [ansi(2)]ansi(2) [ansi(3)]ansi(3) [ansi(4)]ansi(4) [ansi(5)]ansi(5) [ansi(6)]ansi(6)"
    );
    cprintln!(
        "[ansi(196)]ansi(196) [ansi(214)]ansi(214) [ansi(226)]ansi(226) [ansi(46)]ansi(46) [ansi(21)]ansi(21) [ansi(93)]ansi(93)"
    );
    cprintln!("[bg:ansi(236) fg:ansi(255)]dark ansi background");

    section("Emphasis");
    cprintln!(
        "[bold]bold[/bold]  [dim]dim[/dim]  [italic]italic[/italic]  [underline]underline[/underline]  [blink]blink[/blink]  [strikethrough]strikethrough[/strikethrough]"
    );

    section("Multi-tag Brackets");
    cprintln!("[bold red]bold and red");
    cprintln!("[italic blue]italic and blue");
    cprintln!("[bold italic underline rgb(255,180,0)]bold italic underlined orange");
    cprintln!("[bold bg:blue fg:white]bold white on blue");

    section("Reset -Full");
    cprintln!("[bold red]styled text [/] back to normal after [/]");

    section("Reset -Specific");
    cprintln!("[bold red]bold and red [/bold]just red now [/red]unstyled");
    cprintln!(
        "[underline italic green]underline italic green [/underline]italic green [/italic]green [/]unstyled"
    );

    section("Escape Sequence");
    cprintln!("To apply red, write \\[red] in your markup.");
    cprintln!("Tags like \\[bold] are just literal text when escaped.");

    section("Format Arguments");
    let name = "Farben";
    let version = env!("CARGO_PKG_VERSION");
    cprintln!(
        "[bold green]{} v{}[/] is the current release.",
        name,
        version
    );
    let pct = 91u8;
    println!("{}", cformat!("Disk usage: [bold red]{}%[/]", pct));

    section("color() and try_color()");
    println!(
        "{}",
        color("[bold cyan]color()[/] panics on invalid markup.")
    );
    println!("{}", colorb("[dim]colorb()[/] -no trailing reset, styles"));
    cprintln!("[/] -reset that bleed.");
    match try_color("[red]try_color[/] returns a Result") {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("error: {e}"),
    }
    match try_color("[doesnotexist]oops") {
        Ok(s) => println!("{s}"),
        Err(e) => cprintln!("[yellow]try_color error:[/] {e}"),
    }

    section("Style Bleed");
    cprintb!("[bold yellow]this line bleeds -");
    cprintln!("still bold yellow here");
    cprintln!("[/]reset.");
    cprintbln!("[red]line one bleeds into");
    cprintln!("line two (still red)[/]");

    section("Custom Tags -style! and prefix!");
    style!("ok", "[bold green]");
    style!("warn", "[bold yellow]");
    style!("err", "[bold red]");
    prefix!("ok", "✔ ");
    prefix!("warn", "⚠ ");
    prefix!("err", "✖ ");
    cprintln!("[ok]all checks passed.");
    cprintln!("[warn]deprecated API in use.");
    cprintln!("[err]connection refused.");

    section("Chained Custom Tags");
    style!("error", "[bold red]");
    style!("critical", "[error underline]");
    prefix!("error", "[ERROR] ");
    prefix!("critical", "[CRIT]  ");
    cprintln!("[error]something went wrong.");
    cprintln!("[critical]unrecoverable failure.");

    section("ANSI Stripping");
    let colored = color("[bold red]this is colored[/]");
    let plain = ansi_strip!(&colored);
    println!("colored  : {colored}");
    println!("stripped : {plain}");
    let stripped_fmt = ansi_strip!("{}", color("[rgb(255,128,0)]formatted and stripped[/]"));
    println!("ansi_strip!: {stripped_fmt}");

    section("Stderr (ceprint variants)");
    ceprintln!("[bold red]error:[/] this went to stderr");
    ceprintb!("[yellow]warning: ");
    ceprintln!("continued on stderr (inherited yellow)[/]");

    #[cfg(feature = "markdown")]
    {
        section("Inline Markdown");
        mdprintln!("**bold**, *italic*, __underline__, ~~strikethrough~~, `inline code`");
        mdprintln!("**chaining *italic inside bold* works too**");
        let who = "world";
        mdprintln!("**hello, {}!**", who);
        let s = md_fmt!("rendered to string: **{}**", "bold value");
        println!("{s}");
        mdeprintln!("**markdown** also works on *stderr*");
    }
}

fn section(title: &str) {
    println!();
    cprintln!("[bold underline]{title}");
}
