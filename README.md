<div align="center">

![banner logo](farben/images/farben.png)

### Markup for the Terminal

[![Crates.io Version](https://img.shields.io/crates/v/farben)](https://crates.io/crates/farben)
[![docs.rs](https://img.shields.io/docsrs/farben)](https://docs.rs/farben)
[![License MIT](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE-MIT)
[![License Apache-2.0](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE-APACHE)
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

Farben is a terminal styling library for Rust that uses markup syntax. Color your terminal without typing whatever the heck `\x1b[31m` is.

```rust
use farben::prelude::*;

cprintln!("[bold red]Error:[/] something went wrong.");
```

## Documentation

- **User Guide**: [https://razkar-studio.github.io/farben](https://razkar-studio.github.io/farben)
- **API Reference**: [https://docs.rs/farben](https://docs.rs/farben)
- **News**: [https://razkar-studio.github.io/farben/news](https://razkar-studio.github.io/farben/news)
- **Changelog**: [CHANGELOG.md](./CHANGELOG.md)

> [!NOTE]
> Pre-1.0 versions have iterated quickly. Most minor bumps reflect internal changes; the public macro API (`cprint!`, `cprintln!`, `style!`, etc.) has been stable since 0.10. Farben aims for 1.0 once the API fully stabilizes with complete features, or once I get bored developing it.

## Install

```bash
cargo add farben
```

For compile-time markup validation:

```bash
cargo add farben --features compile
```

## Usage

### Default Features (Runtime)

```rust
use farben::prelude::*;

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

### With the `compile` Feature

The same code, but markup is parsed and validated at compile time. Invalid markup becomes a compile error instead of a runtime panic.

```rust
use farben::prelude::*;

cprintln!("[bold underline red]error: [/]Something bad happened.");
cprintln!("[bg:blue fg:white]White on blue!");

let name = "Razkar";
cprintln!("[bold green]Hello, {}!", name);

cprintb!("[red]This bleeds ");
cprintln!("into this.");
```

## Features

- **Markup-like Syntax**: Easy to read, easy to write, powerful when used.
- **Zero Required Runtime Dependencies**: Only `farben-core` as a path dependency. Farben introduces no complicated dependency tree.
- **Opt-in Compile-time Processing**: Validate and process markup at compile time with no runtime overhead, via the `compile` feature flag.
- **Complete Toolkit**: Named colors, ANSI256, RGB, emphasis styles, style chaining, foreground and background support.
- **Drop-in Print Macros**: `cprint!`, `cprintln!`, `cprintb!`, `cprintbln!` work just like `print!` and `println!` but with markup support. Writer variants (`cwrite!`, `cwriteln!`, `cwriteb!`, `cwritebln!`) work with any `Write` implementor.
- **Stderr Variants**: All print macros have `e` variants (`ceprint!`, `ceprintln!`, etc.) that target stderr.
- **Bleed Variants**: `cprintb!`, `cprintbln!`, `colorb()`, and `colorb!()` skip the trailing reset, letting styles carry forward across multiple calls.
- **User-defined Styles**: Define your own tags with `style!()` that expand to any combination of supported tags.
- **Custom Style Files**: Drop a `name.frb.toml` file in your project, write one line in `build.rs` and two in `main.rs`, and all styles from that file are registered.
- **anstyle Interop**: Convert to and from `anstyle::Style` with the sugar of markup. See the docs for details.

## Workspace

Farben is a Cargo workspace. The main crate is what most users want, but the others are published independently for advanced use:

| Crate | Purpose |
|-------|---------|
| [`farben`](https://crates.io/crates/farben) | Main user-facing crate. Macros, prelude, runtime entry points. |
| [`farben-core`](https://crates.io/crates/farben-core) | Zero-dependency core. Lexer, parser, ANSI encoding, registry. |
| [`farben-macros`](https://crates.io/crates/farben-macros) | Proc-macros powering the `compile` feature. |
| [`farben-build`](https://crates.io/crates/farben-build) | Build script support for `.frb.toml` style files. |
| [`farben-md`](https://crates.io/crates/farben-md) | Inline markdown rendering to ANSI. |

## Syntax

Tags are written as `[tag]` and apply from that point forward. Multiple tags can be combined in a single bracket: `[bold red]`.

> [!WARNING]
> Spaces inside `ansi()` and `rgb()` are not supported at the moment.

| Tag | Description |
|-----|-------------|
| `[red]`, `[blue]`, ... | Named colors (black, red, green, yellow, blue, magenta, cyan, white) |
| `[fg:red]`, `[bg:red]` | Explicit foreground/background color, works with all color formats |
| `[rgb(r,g,b)]` | 24-bit RGB color |
| `[ansi(n)]` | 256-color palette index |
| `[bold]`, `[italic]`, `[dim]`, `[underline]`, `[blink]`, `[strikethrough]` | Emphasis styles |
| `[overline]`, `[reverse]`, `[invisible]`, `[rapid-blink]`, `[double-underline]` | Extended emphasis |
| `[/]` | Reset all styles |
| `[/red]`, `[/bold]` | Reset a specific style, leaving others active |
| `\[` | Escaped bracket, treated as literal `[` |
| `[yourname]` | User-defined style registered via `style!()` |

## Error Handling

`color()` panics on invalid markup. For graceful error handling, use `try_color()`:

```rust
use farben::try_color;

match try_color("[invalid]oops") {
    Ok(s) => println!("{s}"),
    Err(e) => eprintln!("Error: {e}"),
}
```

## Known Limitations

The `compile` feature processes markup at build time and bakes ANSI codes directly into the binary. As a result, `NO_COLOR`, `FORCE_COLOR`, and TTY detection are only respected at build time, not at end-user runtime. They are fundamentally different, and I cannot solve this. Use the runtime macros (default features) if you need full environment awareness.

## Contributing

Contributions are welcome. Feel free to open an issue or submit a Pull Request.

## License

Licensed under either of [MIT License](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.

Cheers, RazkarStudio.
© 2026 RazkarStudio.
