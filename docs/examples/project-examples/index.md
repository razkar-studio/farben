# Examples

## Simple Colored Output

Print a few lines with different colors.

```rust
use farben::prelude::*;

fn main() {
    cprintln!("[green]Success! [/]Operation completed.");
    cprintln!("[yellow]Warning: [/]Proceeding with defaults.");
    cprintln!("[red]Error: [/]Something went wrong.");
}
```

## Colored Output with Inline Syntax

If the `inline` feature is enabled, use shorthand syntax directly.

```rust
use farben::prelude::*;

fn main() {
    cprintln!("This is *bold* and /italic/.");
    cprintln!("Use `inline code` for monospace.");
    cprintln!("_Underline_ and ~strikethrough~ work too.");
}
```

## Status Logger

A minimal logger that prefixes messages with a colored level tag.

```rust
use farben::prelude::*;

fn log(level: &str, message: &str) {
    match level {
        "info"  => cprintln!("[bold blue]\\[INFO][/]  {}", message),
        "warn"  => cprintln!("[bold yellow]\\[WARN][/]  {}", message),
        "error" => cprintln!("[bold red]\\[ERROR][/] {}", message),
        _       => cprintln!("[dim]\\[?][/]     {}", message),
    }
}

fn main() {
    log("info",  "Server started on port 8080.");
    log("warn",  "Config file not found, using defaults.");
    log("error", "Failed to connect to database.");
}
```

## Named Styles with `style!` and `prefix!`

Define reusable styles once and reference them anywhere in markup. Requires the `format` feature.

```toml
farben = { version = "...", features = ["format"] }
```

```rust
use farben::prelude::*;

fn main() {
    style!("ok",   "[bold green]");
    style!("warn", "[bold yellow]");
    style!("err",  "[bold red]");
    style!("info", "[bold blue]");

    prefix!("ok",   "✔ ");
    prefix!("warn", "⚠ ");
    prefix!("err",  "✖ ");
    prefix!("info", "• ");

    cprintln!("[ok]All tests passed.");
    cprintln!("[warn]Deprecated API in use.");
    cprintln!("[err]Segmentation fault.");
    cprintln!("[info]Listening on 0.0.0.0:3000.");
}
```

## RGB Gradient Labels

Use RGB colors to render visually distinct severity labels.

```rust
use farben::prelude::*;

fn main() {
    let labels = vec![
        ("[rgb(255,80,80)]CRITICAL[/]",  "Disk usage above 95%"),
        ("[rgb(255,180,0)]HIGH[/]",      "Memory usage above 80%"),
        ("[rgb(100,220,100)]NORMAL[/]",  "CPU usage nominal"),
    ];

    for (label, message) in labels {
        println!("{}: {}", color(label), message);
    }
}
```

## HSL Colors

Use HSL for more intuitive color selection.

```rust
use farben::prelude::*;

fn main() {
    cprintln!("[hsl(0,100,50)]Pure red");
    cprintln!("[hsl(120,100,50)]Pure green");
    cprintln!("[hsl(240,100,50)]Pure blue");
    cprintln!("[hsl(45,80,60)]Warm gold");
}
```

## Format Arguments with `cformat!`

Build colored strings with full format argument support.

```rust
use farben::prelude::*;

fn main() {
    let user = "Alice";
    let action = "logged in";
    let msg = cformat!("[bold green]{user}[/] [dim]{action}[/] at {}:{}", "host", 8080);
    println!("{msg}");
}
```

## CLI Argument Error Display

Use farben to make CLI error messages more readable.

```rust
use farben::prelude::*;

fn require_arg(name: &str, value: Option<&str>) -> Result<String, String> {
    value
        .map(|v| v.to_string())
        .ok_or_else(|| cformat!("[bold red]Missing required argument:[/] [yellow]--{name}[/]"))
}

fn main() {
    match require_arg("output", None) {
        Ok(val) => println!("Output: {val}"),
        Err(e)  => eprintln!("{e}"),
    }

    match require_arg("input", Some("data.csv")) {
        Ok(val) => cprintln!("[green]Input:[/] {val}"),
        Err(e)  => eprintln!("{e}"),
    }
}
```

## Progress Steps

Display a build pipeline's steps with visual status indicators.

```rust
use farben::prelude::*;

fn step(done: bool, label: &str) {
    if done {
        cprintln!("[bold green] ✓ [/]{label}", );
    } else {
        cprintln!("[dim] ○ [/]{label}", );
    }
}

fn main() {
    cprintln!("[bold]Build Steps[/]");
    println!();
    step(true,  "Fetch dependencies");
    step(true,  "Compile sources");
    step(false, "Run tests");
    step(false, "Package binary");
}
```

## Style Bleed

Use the bleed variants when a style should persist across multiple print calls without
repeating the markup.

```rust
use farben::prelude::*;

fn main() {
    cprintb!("[bold yellow]");
    println!("This line is bold yellow.");
    println!("So is this one.");
    cprintln!("[/]"); // reset
}
```

## Error Handling with `try_color`

Handle markup errors without panicking, useful in library code or when building
tools that accept user-supplied format strings.

```rust
use farben::{try_color, errors::LexError};

fn render_safe(input: &str) -> String {
    match try_color(input) {
        Ok(s) => s,
        Err(LexError::InvalidTag { tag_content, .. }) => {
            eprintln!("Unknown tag: [{tag_content}]");
            input.to_string()
        }
        Err(e) => {
            eprintln!("Markup error: {e}");
            input.to_string()
        }
    }
}

fn main() {
    println!("{}", render_safe("[green]All good!"));
    println!("{}", render_safe("[fuchsia]Oops.")); // unknown tag, falls back to plain
}
```

## Stripping ANSI with `unansi!`

Strip ANSI sequences for log files or plain-text display.

```rust
use farben::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let msg = cformat!("[red]error:[/] file not found.");
    println!("{msg}");

    let plain = unansi!("{}", msg);
    let mut log = OpenOptions::new().append(true).open("app.log");
    if let Ok(mut f) = log {
        let _ = writeln!(f, "{plain}");
    }
}
```

## Formatted Table Output

Color table headers and alternate row styles in terminal output.

```rust
use farben::prelude::*;

fn main() {
    cprintln!("[bold underline]{:<20}{:<10}{:<10}[/]", "Name", "Status", "Latency");

    let rows = vec![
        ("api-gateway",  "UP",   "12ms"),
        ("auth-service", "UP",   "8ms"),
        ("db-primary",   "DOWN", "\u{2014}"),
        ("cache",        "UP",   "2ms"),
    ];

    for (i, (name, status, latency)) in rows.iter().enumerate() {
        let status_fmt = match *status {
            "UP"   => cformat!("[green]{status}[/]"),
            "DOWN" => cformat!("[bold red]{status}[/]"),
            _      => status.to_string(),
        };
        if i % 2 == 0 {
            println!("{name:<20}{status_fmt:<10}{latency:<10}");
        } else {
            cprintln!("[dim]{name:<20}[/]{status_fmt:<10}{latency:<10}");
        }
    }
}
```

## Interactive Prompt

Color the prompt label and input hint in a terminal read loop.

```rust
use farben::prelude::*;
use std::io::{self, Write};

fn prompt(label: &str) -> String {
    cprint!("[bold cyan]{label}[/] [dim]\u{203a}[/] ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let name = prompt("Your name");
    let age  = prompt("Your age");
    cprintln!("[green]Hello, {name}! You are {age} years old.[/]");
}
```

## Diff-Style Output

Render line-by-line diffs with added/removed/unchanged indicators.

```rust
use farben::prelude::*;

enum DiffLine<'a> {
    Added(&'a str),
    Removed(&'a str),
    Context(&'a str),
}

fn print_diff(lines: &[DiffLine]) {
    for line in lines {
        match line {
            DiffLine::Added(s)   => cprintln!("[green]+ {s}[/]"),
            DiffLine::Removed(s) => cprintln!("[red]- {s}[/]"),
            DiffLine::Context(s) => cprintln!("[dim]  {s}[/]"),
        }
    }
}

fn main() {
    print_diff(&[
        DiffLine::Context("fn greet(name: &str) {"),
        DiffLine::Removed(r#"    println!("Hello, {}!", name);"#),
        DiffLine::Added(r#"    cprintln!("[green]Hello, {}![/]", name);"#),
        DiffLine::Context("}"),
    ]);
}
```

## Spinner / Loading Indicator

Animate a simple spinner using carriage return and bleed output.

```rust
use farben::prelude::*;
use std::{io::{self, Write}, thread, time::Duration};

fn main() {
    let frames = ["\u{280b}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283c}", "\u{2834}", "\u{2826}", "\u{2827}", "\u{2807}", "\u{280f}"];

    for i in 0..30 {
        let frame = frames[i % frames.len()];
        cprint!("\r[bold cyan]{frame}[/] [dim]Building...[/]  ");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(80));
    }

    cprintln!("\r[bold green]\u{2714}[/] Build complete.      ");
}
```

## Boxed Section Headers

Draw attention to major output sections with a simple colored box.

```rust
use farben::prelude::*;

fn section(title: &str) {
    let border = "\u{2500}".repeat(title.len() + 4);
    cprintln!("[bold blue]\u{250c}{border}\u{2510}[/]");
    cprintln!("[bold blue]\u{2502}[/]  {title}  [bold blue]\u{2502}[/]");
    cprintln!("[bold blue]\u{2514}{border}\u{2518}[/]");
}

fn main() {
    section("Dependency Check");
    cprintln!("[green]\u{2714}[/] serde 1.0.197");
    cprintln!("[green]\u{2714}[/] tokio 1.36.0");
    cprintln!("[yellow]\u{26a0}[/] openssl 0.10.62 (outdated)");
    println!();
    section("Build Summary");
    cprintln!("[green]\u{2714}[/] 0 errors, 2 warnings");
}
```

## Conditional Coloring Based on Value

Apply colors dynamically based on runtime values, such as metrics or thresholds.

```rust
use farben::prelude::*;

fn colored_percent(value: u8) -> String {
    match value {
        0..=60   => cformat!("[green]{value}%[/]"),
        61..=85  => cformat!("[yellow]{value}%[/]"),
        86..=100 => cformat!("[bold red]{value}%[/]"),
        _        => value.to_string(),
    }
}

fn main() {
    let metrics = vec![
        ("CPU",    42u8),
        ("Memory", 78),
        ("Disk",   91),
    ];

    for (label, value) in metrics {
        println!("{label:<10} {}", colored_percent(value));
    }
}
```

## Test Result Summary

Print a pass/fail summary at the end of a test run.

```rust
use farben::prelude::*;

struct TestResult {
    name: String,
    passed: bool,
}

fn print_summary(results: &[TestResult]) {
    for r in results {
        if r.passed {
            cprintln!("[green]  PASS[/] {}", r.name);
        } else {
            cprintln!("[bold red]  FAIL[/] {}", r.name);
        }
    }

    let total  = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = total - passed;

    println!();
    cprintln!(
        "[bold green]{passed} passed[/], [bold red]{failed} failed[/], {total} total",
    );
}

fn main() {
    let results = vec![
        TestResult { name: "test_parse_empty".into(),  passed: true  },
        TestResult { name: "test_parse_rgb".into(),    passed: true  },
        TestResult { name: "test_unclosed_tag".into(), passed: false },
        TestResult { name: "test_named_color".into(),  passed: true  },
    ];

    print_summary(&results);
}
```

## File Tree Display

Print a directory tree with colored file types.

```rust
use farben::prelude::*;

enum Entry<'a> {
    Dir(&'a str),
    File(&'a str),
    Last(&'a str),
}

fn print_tree(root: &str, entries: &[Entry]) {
    cprintln!("[bold]{root}[/]");
    for entry in entries {
        match entry {
            Entry::Dir(name)  => cprintln!("\u{251c}\u{2500}\u{2500} [bold blue]{name}/[/]"),
            Entry::File(name) => cprintln!("\u{251c}\u{2500}\u{2500} {name}"),
            Entry::Last(name) => cprintln!("\u{2514}\u{2500}\u{2500} [dim]{name}[/]"),
        }
    }
}

fn main() {
    print_tree("farben/", &[
        Entry::Dir("src"),
        Entry::File("Cargo.toml"),
        Entry::File("README.md"),
        Entry::Last(".gitignore"),
    ]);
}
```

## Paged Output Header

Print a consistent header for paged terminal output, like a `--help` screen.

```rust
use farben::prelude::*;

fn print_header(name: &str, version: &str, description: &str) {
    cprintln!("[bold]{name} [dim]v{version}[/]");
    cprintln!("[dim]{description}[/]");
    println!();
}

fn print_usage(usage: &[(&str, &str)]) {
    cprintln!("[bold underline]Options[/]");
    for (flag, desc) in usage {
        cprintln!("  [bold cyan]{flag:<20}[/] {desc}");
    }
}

fn main() {
    print_header("mytool", "1.0.0", "A CLI tool that does things.");
    print_usage(&[
        ("--input <file>",  "Path to the input file"),
        ("--output <file>", "Path to the output file"),
        ("--verbose",       "Enable verbose output"),
        ("--help",          "Show this help message"),
    ]);
}
```

---

::: tip
All printing macros (`cprint!`, `cprintln!`, etc.) and `color()` panic on invalid markup.
Use `try_color()` for input you don't fully control.
:::

---

If you've read until this far until the end, thank you. Genuinely. Either you've read from the start or you've skipped here to see what cool things you can make with Farben, I genuinely thank you for reading this documentation.

Now, go and make cool things with Farben.

Cheers, RazkarStudio.
