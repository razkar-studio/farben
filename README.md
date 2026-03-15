<div align="center">

![banner logo](farben/images/farben.png)

### Color your terminal without typing whatever the heck '\x1b[31m' is.

</div>

> [!WARNING]
> Farben, in this current state, is **unstable**. It only has a few public interfaces,
> unfinished features, and is experimental.
> I do not recommend using it in production, at least not yet.

## What Is Farben

Look at the tagline up there ^

## Documentation
- **User Guide**: [https://razkar-studio.github.io/farben](https://razkar-studio.github.io/farben)
- **API Reference**: [https://docs.rs/farben](https://docs.rs/farben)

## Usage

```rust
// Using no features
use farben::{color, color_fmt, cprintln};

println!("{}", color("[red]I'm red!"));
println!("{}", color("[bg:blue fg:white]White on blue!"));

let name = "Razkar";
println!("{}", color_fmt!("[bold green]Hello, {}!", name));

cprintln!("[yellow]Warning: [/]Something looks off.");
```

```rust
// Using the "compile" feature
use farben::{color, color_fmt, cprintln};

println!("{}", color!("[red]I'm red!"));          // compile-time
println!("{}", color!("[bg:blue fg:white]White on blue!"));

let name = "Razkar";
println!("{}", color_fmt!("[bold green]Hello, {}!", name)); // compile-time validation

cprintln!("[yellow]Warning: [/]Something looks off.");
```

## Features

- **Markup-like Syntax**: Easy to read, write, and powerful when used.
- **Zero required runtime dependencies**: Only `farben-core` as a path dependency, Farben introduces no complicated dependency tree.
- **Opt-in Compile-time Processing**: Validate and process markup at compile time with no runtime overhead, via the `compile` feature flag.
- **Complete Toolkit**: Named colors, ANSI256, RGB, emphasis styles, style chaining, foreground and background support.
- **Drop-in Print Macros**: `cprint!` and `cprintln!` work just like `print!` and `println!` but with markup support.
- **Custom, user-defined styles**: Define your own tags that unwrap to supported tags. Chainable, too!

## Syntax

Tags are written as `[tag]` and apply from that point forward. Multiple tags can be combined in a single bracket: `[bold red]`.

> [!WARNING]
> Spaces inside `ansi()` and `rgb()` are not supported at the moment.

| Tag | Description |
|-----|-------------|
| `[red]`, `[blue]`, ... | Named colors (black, red, green, yellow, blue, magenta, cyan, white) |
| `[fg:red]`, `[bg:red]` | Explicit foreground/background color — works with all color formats |
| `[rgb(r,g,b)]` | 24-bit RGB color |
| `[ansi(n)]` | 256-color palette index |
| `[bold]`, `[italic]`, `[dim]`, `[underline]`, `[blink]`, `[strikethrough]` | Emphasis styles |
| `[/]` | Reset all styles |
| `\\[` | Escaped bracket, treated as literal `[` |

## Error Handling

`color()` panics on invalid markup. For graceful error handling, use `try_color()`:

```rust
use farben::try_color;

match try_color("[invalid]oops") {
    Ok(s) => println!("{s}"),
    Err(e) => eprintln!("Error: {e}"),
}
```

## Contributing

Contributions are welcome! Feel free to submit a Pull Request.

## License

This project is protected under the RazkarStudio Permissive License (RSPL). See [LICENSE.md](LICENSE.md) for more details.

Cheers, RazkarStudio.  
© 2026 RazkarStudio. All rights reserved.
