# API Reference

## `color(input)`

Parses and renders a farben markup string, appending a final SGR reset.

```rust
pub fn color(input: impl Into<String>) -> String
```

Not available when the `compile` feature is enabled: `color` becomes a proc-macro instead. See [`color!(input)`](#color-input-compile-feature) below.

**Example**

```rust
use farben::color;

println!("{}", color("[red]I'm red!"));
println!("{}", color("[bold green]I'm bold and green!"));
println!("{}", color("[rgb(255,128,0)]I'm orange![/] Back to normal."));
```

::: warning
`color()` panics on invalid markup. Use [`try_color()`](#try-color-input) to handle errors explicitly.
:::



## `colorb(input)`

Like `color()`, but does not append a trailing reset. Styles bleed into subsequent output.

```rust
pub fn colorb(input: impl Into<String>) -> String
```

Not available when the `compile` feature is enabled.

::: tip
Use `colorb()` when chaining colored segments and you want the style to carry forward into the next print call.
:::



## `color!(input)`, `compile` feature

Compile-time version of `color()`. Parses and validates markup at compile time, emitting the final ANSI string as a string literal baked into the binary. Requires the `compile` feature.

```toml
# Cargo.toml
farben = { version = "...", features = ["compile"] }
```

```rust
use farben::color;

println!("{}", color!("[red]I'm red!"));
println!("{}", color!("[bold green]I'm bold and green!"));
```

::: warning
`color!` only accepts string literals. For runtime format args, use [`color_fmt!`](#color-fmt).
:::



## `try_color(input)`

Like `color()`, but returns a `Result` instead of panicking on invalid markup. Always available regardless of feature flags.

```rust
pub fn try_color(input: impl Into<String>) -> Result<String, LexError>
```

**Example**

```rust
use farben::try_color;

match try_color("[red]Hello!") {
    Ok(s) => println!("{s}"),
    Err(e) => eprintln!("Parse error: {e}"),
}
```

::: tip
Prefer `try_color()` in library code or anywhere you don't fully control the input string.
:::

## `color_runtime(input, bleed)`

The runtime fallback used internally by `color_fmt!`, `cprint!`, and `cprintln!`. Always a function regardless of active feature flags.

```rust
pub fn color_runtime(input: impl Into<String>, bleed: bool) -> String
```

When `bleed` is `false`, a trailing reset is appended. When `true`, it is not.

::: warning
This is an internal function. Prefer `color()`, `colorb()`, or the `cprint` macros in your own code.
:::

## `color_fmt!(...)`

Behaves like `format!` but processes farben markup on the result. Panics on invalid markup.

When the `compile` feature is enabled, the format string is validated at compile time via `validate_color!`.

```rust
use farben::*;

let name = "Razkar";
println!("{}", color_fmt!("[green]Hello, {}!", name));
```

## `cprint!(...)`

Prints farben-colored markup to stdout without a trailing newline. Behaves like `print!`.

When the `compile` feature is enabled, the format string is validated at compile time.

```rust
use farben::*;

let message = "I don't know";
cprint!("[red]Error: [/]{}", message);
```

## `cprintln!(...)`

Prints farben-colored markup to stdout with a trailing newline. Behaves like `println!`.

When the `compile` feature is enabled, the format string is validated at compile time.

```rust
use farben::*;

let result = "We did it!";
cprintln!("[green]Success: [/]{}", result);
```

## `cprintb!(...)`

Like `cprint!`, but does not append a trailing reset. Styles bleed into subsequent output.

```rust
use farben::*;

cprintb!("[red]Error: ");
cprintln!("something went wrong"); // inherits red
```

## `cprintbln!(...)`

Like `cprintln!`, but does not append a trailing reset. Styles bleed into subsequent output.

```rust
use farben::*;

cprintbln!("[bold red]Section header");
cprintln!("still bold and red here"); // inherits style
```

## `style!(name, markup)`: `format` feature

Registers a named style in the global style registry. Requires the `format` feature.

```toml
# Cargo.toml
farben = { version = "...", features = ["format"] }
```

```rust
macro_rules! style {
    ($name:expr, $markup:expr) => { ... }
}
```

`$name` is a string key used to reference the style in markup. `$markup` is a farben markup string defining the style's colors and emphasis. If a style with that name already exists, it is replaced.

**Example**

```rust
use farben::*;

style!("ok",  "[bold green]");
style!("err", "[bold red]");

cprintln!("[ok]Build passed.");
cprintln!("[err]Build failed.");
```

::: warning
Panics if `$markup` is not valid farben markup.
:::

## `prefix!(name, prefix)`: `format` feature

Attaches a literal string prefix to an already-registered named style. Requires the `format` feature.

```rust
macro_rules! prefix {
    ($name:expr, $prefix:expr) => { ... }
}
```

The prefix is prepended as plain text before the style's ANSI escape sequence at render time. It can be any string: an icon, a label, whitespace, or a log-level tag.

**Example**

```rust
use farben::*;

style!("ok",   "[bold green]");
style!("warn", "[bold yellow]");
style!("err",  "[bold red]");

prefix!("ok",   "✔ ");
prefix!("warn", "⚠ ");
prefix!("err",  "✖ ");

cprintln!("[ok]All checks passed.");
cprintln!("[warn]Disk usage is high.");
cprintln!("[err]Connection refused.");
```

::: warning
Panics if `$name` has not been registered with `style!` first.
:::

## Tag Syntax

Tags are written as `[tag]` and apply from that point forward in the string.

### Named Colors

```rust
color("[red]red [green]green [blue]blue")
```

| Tag | Color |
|-----|-------|
| `[black]` | Black |
| `[red]` | Red |
| `[green]` | Green |
| `[yellow]` | Yellow |
| `[blue]` | Blue |
| `[magenta]` | Magenta |
| `[cyan]` | Cyan |
| `[white]` | White |

### Background Colors

Prefix any color tag with `bg:` to apply it to the background instead.

```rust
color("[bg:red]red background")
color("[fg:white bg:blue]white on blue")
```

### RGB Colors

```rust
color("[rgb(255,128,0)]This is orange!")
```

::: warning
Spaces inside `rgb()` are not supported in format strings when the `compile` feature is enabled. Use `rgb(255,128,0)`, not `rgb(255, 128, 0)`.
:::

### 256-Color Palette

```rust
color("[ansi(214)]This uses palette index 214!")
```

### Emphasis

```rust
color("[bold]Bold! [italic]Italic! [underline]Underlined!")
```

| Tag | Effect |
|-----|--------|
| `[bold]` | Bold |
| `[dim]` | Dim |
| `[italic]` | Italic |
| `[underline]` | Underline |
| `[blink]` | Blink |
| `[strikethrough]` | Strikethrough |

### Multi-tag Brackets

Multiple tags can be combined inside a single bracket, separated by spaces.

```rust
color("[bold red]I'm bold and red!")
color("[italic rgb(0,200,100)]I'm italic and green!")
```

### Reset

`[/]` resets all active styles and colors. `color()` and most macros also append a reset automatically at the end of every string.

```rust
color("[red]I'm red[/] but I'm not.")
```

### Escape Sequence

Prefix a `[` with `\` to treat it as a literal bracket instead of a tag.

```rust
color("Use \\[red] to set a red color.")
// Output: Use [red] to set a red color.
```

## Error Types

`try_color()` returns a `LexError` on failure. The variants are:

| Variant | Cause |
|---------|-------|
| `LexError::UnclosedTag` | A `[` was opened but never closed with `]` |
| `LexError::InvalidTag(String)` | An unrecognized tag name was used |
| `LexError::InvalidValue(String)` | A value inside `ansi()` or `rgb()` could not be parsed |
| `LexError::InvalidArgumentCount { expected, got }` | Wrong number of arguments passed to `rgb()` |
| `LexError::UnclosedValue` | A color function like `rgb(` or `ansi(` was opened but never closed |

**Example**

```rust
use farben::{try_color, errors::LexError};

match try_color("[rgb(255,0)]oops") {
    Err(LexError::InvalidArgumentCount { expected, got }) => {
        eprintln!("Expected {expected} args, got {got}");
    }
    _ => {}
}
```

---

The API covered in this page only covers Farben's public-facing API. For internals, look for the [docs.rs](https://docs.rs/farben) link for [farben-core](https://docs.rs/farben-core) or [farben-macros](https://docs.rs/farben-macros).
