# Examples

## Simple Colored Output

Print a few lines with different colors.

```rust
use farben::color;

fn main() {
    println!("{}", color("[green]Success! [/]Operation completed."));
    println!("{}", color("[yellow]Warning: [/]Proceeding with defaults."));
    println!("{}", color("[red]Error: [/]Something went wrong."));
}
```

## Status Logger

A tiny logger that prefixes messages with a colored status tag.

```rust
use farben::color;

fn log(level: &str, message: &str) {
    let tag = match level {
        "info"  => color("[bold blue]\\[INFO][/] "),
        "warn"  => color("[bold yellow]\\[WARN][/] "),
        "error" => color("[bold red]\\[ERROR][/] "),
        _       => color("[dim]\\[UNKNOWN][/] "),
    };
    println!("{}{}", tag, message);
}

fn main() {
    log("info", "Server started on port 8080.");
    log("warn", "Config file not found, using defaults.");
    log("error", "Failed to connect to database.");
}
```

## RGB Gradient Labels

Use RGB colors to render visually distinct labels.

```rust
use farben::color;

fn main() {
    let labels = vec![
        ("[rgb(255,80,80)]CRITICAL[/]",  "Disk usage above 95%"),
        ("[rgb(255,180,0)]HIGH[/]",      "Memory usage above 80%"),
        ("[rgb(100,220,100)]NORMAL[/]",  "CPU usage nominal"),
    ];

    for (label, message) in labels {
        println!("{} — {}", color(label), message);
    }
}
```

## CLI Argument Error Display

Use farben to make CLI error messages more readable.

```rust
use farben::color;

fn require_arg(name: &str, value: Option<&str>) -> Result<String, String> {
    value
        .map(|v| v.to_string())
        .ok_or_else(|| color(&format!("[bold red]Missing required argument:[/] [yellow]--{}[/]", name)))
}

fn main() {
    match require_arg("output", None) {
        Ok(val) => println!("Output: {val}"),
        Err(e)  => eprintln!("{e}"),
    }

    match require_arg("input", Some("data.csv")) {
        Ok(val) => println!("{}", color(&format!("[green]Input:[/] {val}"))),
        Err(e)  => eprintln!("{e}"),
    }
}
```

## Progress Steps

Display a sequence of steps with visual status indicators.

```rust
use farben::color;

fn step(done: bool, label: &str) {
    if done {
        println!("{}", color(&format!("[bold green] ✓ [/]{label}")));
    } else {
        println!("{}", color(&format!("[dim] ○ [/]{label}")));
    }
}

fn main() {
    println!("{}", color("[bold]Build Steps[/]"));
    println!();
    step(true,  "Fetch dependencies");
    step(true,  "Compile sources");
    step(false, "Run tests");
    step(false, "Package binary");
}
```

::: tip
All examples use `color()` which panics on invalid markup. In production code, consider using `try_color()` for input you don't fully control.
:::
