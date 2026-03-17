<div align="center">

![banner logo](images/farben.png)

### Markup for the Terminal

[![Crates.io Version](https://img.shields.io/crates/v/farben)](https://crates.io/crates/farben)
[![docs.rs](https://img.shields.io/docsrs/farben)](https://docs.rs/farben)
[![License](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE)
[![Crates.io Downloads](https://img.shields.io/crates/d/farben)](https://crates.io/crates/farben)
[![GitHub Stars](https://img.shields.io/github/stars/razkar-studio/farben)](https://github.com/razkar-studio/farben/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/razkar-studio/farben)](https://github.com/razkar-studio/farben/issues)
[![GitHub Last Commit](https://img.shields.io/github/last-commit/razkar-studio/farben)](https://github.com/razkar-studio/farben/commits/main)
[![Rust Edition](https://img.shields.io/badge/rust%20edition-2024-orange)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Deps.rs](https://deps.rs/repo/github/razkar-studio/farben/status.svg)](https://deps.rs/repo/github/razkar-studio/farben)
[![Repo Size](https://img.shields.io/github/repo-size/razkar-studio/farben)](https://github.com/razkar-studio/farben)
[![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen)](https://github.com/razkar-studio/farben)

</div>

## What Is Farben

Farben is a color library that uses markup-like syntax. Color your terminal without typing whatever the heck '\x1b[31m' is.

## Documentation

- **User Guide**: [https://razkar-studio.github.io/farben](https://razkar-studio.github.io/farben)
- **API Reference**: [https://docs.rs/farben](https://docs.rs/farben)
- **Changelog**: [CHANGELOG.md](./CHANGELOG.md)

## Usage

```rust
// Using no features
use farben::*;

style!("error", "[bold underline red]");
style!("warn",  "[bold yellow]");

cprintln!("[error]error: [/]Something bad happened.");
cprintln!("[warn]warn: [/]This looks suspicious.");
cprintln!("[bg:blue fg:white]White on blue!");

let name = "Razkar";
cprintln!("[bold green]Hello, {}!", name);

cprintb!("[red]This bleeds ");
cprintln!("into this.");
```

```rust
// Using the "compile" feature
use farben::*;

style!("error", "[bold underline red]");

cprintln!("[error]error: [/]Something bad happened."); // compile-time validation
cprintln!("[bg:blue fg:white]White on blue!");

let name = "Razkar";
cprintln!("[bold green]Hello, {}!", name);

cprintb!("[red]This bleeds ");
cprintln!("into this.");
```

## Features

- **Markup-like Syntax**: Easy to read, write, and powerful when used.
- **Zero required runtime dependencies**: Only `farben-core` as a path dependency, Farben introduces no complicated dependency tree.
- **Opt-in Compile-time Processing**: Validate and process markup at compile time with no runtime overhead, via the `compile` feature flag.
- **Complete Toolkit**: Named colors, ANSI256, RGB, emphasis styles, style chaining, foreground and background support.
- **Drop-in Print Macros**: `cprint!`, `cprintln!`, `cprintb!`, and `cprintbln!` work just like `print!` and `println!` but with markup support.
- **Bleed Variants**: `cprintb!`, `cprintbln!`, `colorb()`, and `colorb!()` skip the trailing reset, letting styles carry forward across multiple calls.
- **User-defined styles**: Define your own tags with `style!()` that expand to any combination of supported tags.
  + **Features shouldn't be forced upon you**: Opt-out from user-defined styles at anytime, making Farben lighter and easier to work with.
  + **Format using `prefix!()`**: Your styles, your rules. Make a prefix to go along with it, calling your custom style will resolve to the prefix you define.
- **Markdown Support**: While minimal, Farben has limited opt-in markdown support, just the basics that you need.

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
| `[yourname]` | User-defined style via `style!()` |

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

Licensed under either of [MIT License](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.

Cheers, RazkarStudio.  
© 2026 RazkarStudio. All rights reserved.
