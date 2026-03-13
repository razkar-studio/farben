# API Reference

## `color(input)`

Colorizes a string using farben's markup syntax. Panics if the markup is invalid.

```rust
pub fn color(input: impl Into<String>) -> String
```

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

> [!NOTE]
> Spaces inside `rgb()` are not supported. Use `rgb(255,128,0)`, not `rgb(255, 128, 0)`.

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

> [!IMPORTANT]
> `color()` automatically appends a reset at the end of every string, so styles don't bleed into subsequent output.

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
