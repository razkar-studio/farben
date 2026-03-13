<div align="center">

![banner logo](images/farben.png)

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

### A minimal terminal coloring library using markup-like syntax.

</div>

> [!WARNING]
> Farben, in this current state, is **extremely unstable**. It only has one or two public interface,
> unfinished features, and is experimetal.
> I do not recommend using it in production, at least not yet.

## What Is Farben

Look at the tagline up there ^

## Documentation

> [!NOTE]
> The user guide right now is literally unreadable.

- **User Guide**: [https://razkar-studio.github.io/farben](https://razkar-studio.github.io/farben)
- **API Reference**: [https://docs.rs/farben](https://docs.rs/farben)

## Usage
```rust
// Using no features
use farben::color;

println!("{}", color("[red]I'm red!")); // Runtime
let color = "red";
println!("{}", color_fmt!("[bold red]I'm bold and {color}!"));
println!("{}", color("[rgb(255,128,0)]I'm orange![/] Back to normal."));
```

```rust
// Using the "compile" feature
use farben::color;

println!("{}", color!("[red]I'm red!")); // Compile-time
let color = "red";
println!("{}", color_fmt!("[bold red]I'm bold and {color}!")); // Compile time validation
println!("{}", color!("[rgb(255,128,0)]I'm orange![/] Back to normal."));
```

## Features
* **Markup-like Syntax**: Easy to parse, understand, and powerful when used.
* **Only 1 required runtime dependencies**: Having only 1 dependency, and that being the logic itself, farben doesn't introduce a complicated dependency tree.
* **Opt-in Compile-time Processing**: Colorize at compile time with no runtime overhead, completely opt-in with one additional dependency: `farben-macros`.
* **Complete Toolkit**: Supports basic named ANSI, ANSI256, and even RGB.

## Syntax

Tags are written as `[tag]` and apply from that point forward. Multiple tags can be combined in a single bracket: `[bold red]`.

> [!WARNING]
> Spaces inside `ansi()` and `rgb()` are not supported at the moment, and it will error.

| Tag | Description |
|-----|-------------|
| `[red]`, `[blue]`, ... | Named colors (black, red, green, yellow, blue, magenta, cyan, white) |
| `[rgb(r,g,b)]` | 24-bit RGB color |
| `[ansi(n)]` | 256-color palette index |
| `[bold]`, `[italic]`, `[dim]`, `[underline]`, `[blink]`, `[strikethrough]` | Emphasis styles |
| `[/]` | Reset all styles |
| `\\[` | Escaped bracket, treated as literal `[` (notice the double escape `\\`) |

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
