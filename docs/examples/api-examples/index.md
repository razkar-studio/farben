# API Reference

## `color(input)`

Colorizes a string using farben's markup syntax. Panics if the markup is invalid.

```rust
pub fn color(input: impl Into<String>) -> String
```

::: warning
When the `compile` feature is enabled, `color` becomes a proc-macro instead of a function. See [`color!(input)`](#color-input-compile-feature) below.
:::

**Example**

```rust
use farben::color;

println!("{}", color("[red]I'm red!"));
println!("{}", color("[bold green]I'm bold and green!"));
println!("{}", color("[rgb(255,128,0)]I'm orange![/] Back to normal."));
```

::: warning
`color()` will panic on invalid markup. If you need to handle errors gracefully, use [`try_color()`](#try-color-input) instead.
:::

---

## `color!(input)` — `compile` feature

Compile-time version of `color()`. Parses and validates markup at compile time, emitting the final ANSI string as a string literal baked into the binary. Requires the `compile` feature.

```rust
// Cargo.toml
// farben = { version = "0.2", features = ["compile"] }

color!("[red]I'm red!")
```

::: tip
`color!` and `color()` share the same name — when `compile` is enabled, `color!` replaces `color()` entirely. Your call sites don't need to change.
:::

**Example**

```rust
use farben::color;

println!("{}", color!("[red]I'm red!"));
println!("{}", color!("[bold green]I'm bold and green!"));
```

::: warning
`color!` only accepts string literals. For runtime format args, use [`color_fmt!`](#color-fmt-compile-feature).
:::

---

## `try_color(input)`

Like `color()`, but returns a `Result` instead of panicking on invalid markup.

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
Prefer `try_color()` in library code or anywhere you don't control the input string.
:::

---

## `color_fmt!(...)`

Format args version of `color()`. Behaves like `format!` but processes farben markup on the result.

When the `compile` feature is enabled, the format string is validated at compile time via `validate_color!`.

```rust
let name = "Razkar";
println!("{}", color_fmt!("[green]Hello, {}!", name));
```

::: warning
Spaces inside `ansi()` and `rgb()` are not supported in the format string. This will cause a compile-time error when the `compile` feature is enabled, and a panic at runtime otherwise.
:::

---

## `color_runtime(input)`

Internal runtime fallback used by `color_fmt!`. Behaves identically to `color()` but is always a function regardless of feature flags.

```rust
pub fn color_runtime(input: impl Into<String>) -> String
```

::: warning
This is an internal function. Prefer `color()` or `color_fmt!` in your own code.
:::

---

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

### RGB Colors

```rust
color("[rgb(255,128,0)]This is orange!")
```

::: warning
Spaces inside `rgb()` are not supported. Use `rgb(255,128,0)`, not `rgb(255, 128, 0)`.
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

`[/]` resets all active styles and colors.

```rust
color("[red]I'm red[/] but I'm not.")
```

::: important
`color()` automatically appends a reset at the end of every string, so styles don't bleed into subsequent output.
:::

### Escape Sequence

Prefix a `[` with `\` to treat it as a literal bracket instead of a tag.

```rust
color("Use \\[red] to set a red color.")
// Output: Use [red] to set a red color.
```

---

## Error Types

`try_color()` returns a `LexError` on failure. The variants are:

| Variant | Cause |
|---------|-------|
| `LexError::UnclosedTag` | A `[` was opened but never closed with `]` |
| `LexError::InvalidTag(String)` | An unrecognized tag name was used |
| `LexError::InvalidValue(String)` | A value inside `ansi()` or `rgb()` could not be parsed |
| `LexError::InvalidArgumentCount { expected, got }` | Wrong number of arguments passed to `rgb()` |

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
