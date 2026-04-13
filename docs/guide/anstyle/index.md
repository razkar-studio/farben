# anstyle Interoperability

Farben plays nice with other terminal libraries. The `anstyle` feature unlocks interoperability with the `anstyle` crate, used by libraries like `clap` and `ratatui` for terminal styling.

Enable it in your `Cargo.toml`:

```toml
[dependencies]
farben = { version = "x", features = ["anstyle"] }
```

Or,
```sh
cargo add farben --features anstyle
```

## Converting Styles

The `anstyle_conv` module provides `From` implementations for bidirectional conversion between farben and anstyle types.

```rust
use farben::Style;
use anstyle::{Color as AnsiColor, Style as AnsiStyle, AnsiColor as AnsiRed};

// farben -> anstyle
let farben_style = Style::parse("[bold red]").unwrap();
let anstyle_style: AnsiStyle = farben_style.into();

// anstyle -> farben
let anstyle_color = AnsiStyle::new().fg_color(Some(AnsiColor::Ansi(AnsiRed::Red)));
let farben_style: Style = anstyle_color.into();
```

### Supported Conversions

| farben type | anstyle type | note |
|------------|---------------|------|
| `Style` | `anstyle::Style` | re-exported from `farben` |
| `Color` | `anstyle::Color` | requires `farben_core` |
| `NamedColor` | `anstyle::AnsiColor` | requires `farben_core` |

::: tip
For most use cases, you only need `farben::Style`. The `Style` type contains foreground and background colors, so direct `Color` or `NamedColor` conversions are rarely needed.
:::

## The anstyle! Macro

The `anstyle!` macro parses farben markup and returns an `anstyle::Style` directly. This is handy when an API expects an anstyle style rather than a string.

```rust
use anstyle::Style;

fn main() {
    let style: Style = farben::anstyle!("[bold red]Warning: ");
    // use with clap, ratatui, or any anstyle consumer
}
```

::: info
The `anstyle!` macro panics on invalid markup, just like `color()` does. Wrap it in `std::panic::catch_unwind` if you need error handling.
:::

## When to Use This

The interop is useful when mixing farben with crates that speak anstyle. For example, passing a styled prefix to a CLI argument parser, or styling text in a TUI framework.

If you just need colored output to stdout, stick with the standard `cprintln!()` and `color()` APIs. No extra features required.
