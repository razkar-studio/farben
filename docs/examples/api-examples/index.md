# API Reference

## `cstr!(...)`

The canonical way to produce a colored string. Works in both runtime and compile-time modes.
With the `compile` feature, bare literals return a [`FarbenStr`](#farbenstr) rendered at compile time.
Format arguments (explicit or implicit) fall through to runtime rendering with compile-time markup validation.

```rust
use farben::prelude::*;

// Bare literal -- compile-time rendered when `compile` is enabled
let s = cstr!("[green]Done");
println!("{s}");

// With format arguments
let name = "World";
let s = cstr!("[bold]Hello, {name}!");
println!("{s}");

// Positional and named arguments
let s = cstr!("[red]{} and {a}", "first", a = "second");
```

::: tip
`cstr!()` returns a [`FarbenStr`](#farbenstr) when rendered at compile time, or a `String` otherwise.
Both types implement `Display`, so `println!("{}", cstr!(...))` works regardless of features.
:::

## `try_color(input)`

Returns a `Result` instead of panicking on invalid markup. Always available regardless of feature flags.

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

## `color(input)` *(legacy)*

Parses and renders a farben markup string, appending a final SGR reset. Prefer [`cstr!`](#cstr) for new code.

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

## `colorb(input)` *(legacy)*

Like `color()`, but does not append a trailing reset. Styles bleed into subsequent output. Prefer [`cstr!`](#cstr) for new code.

```rust
pub fn colorb(input: impl Into<String>) -> String
```

Not available when the `compile` feature is enabled. Use [`colorb!(input)`](#colorb-input-compile-feature) instead.

::: tip
Use `colorb()` when chaining colored segments and you want the style to carry forward into the next print call.
:::

## `color!(input)` *(legacy, compile feature)*

Compile-time version of `color()`. Parses and validates markup at compile time, emitting the final ANSI string as a `FarbenStr` baked into the binary. Prefer [`cstr!`](#cstr) for new code -- it provides the same compile-time behavior for bare literals and also handles format arguments.

```toml
farben = { version = "...", features = ["compile"] }
```

```rust
use farben::color;

println!("{}", color!("[red]I'm red!"));
println!("{}", color!("[bold green]I'm bold and green!"));
```

::: warning
`color!` only accepts string literals. For runtime format args, use [`cstr!`](#cstr) or [`cformat!`](#cformat).
:::

## `colorb!(input)` *(legacy, compile feature)*

Compile-time version of `colorb()`. Like `color!` but without the trailing reset.

```rust
use farben::colorb;

let s = colorb!("[red]This bleeds");
```

## `color_runtime(input, bleed)`

The runtime fallback used internally by `cstr!`, `cformat!`, `cprint!`, and `cprintln!`. Always a function regardless of active feature flags.

```rust
pub fn color_runtime(input: impl Into<String>, bleed: bool) -> String
```

When `bleed` is `false`, a trailing reset is appended. When `true`, it is not.

::: tip
This is an internal function. Prefer [`cstr!`](#cstr) or the `c*` print macros in your own code.
:::

## `cformat!(...)`

Formats a string with Farben markup, appending a trailing reset. Behaves like `format!` but processes Farben markup tags in the result. Supports positional, named, and implicit capture arguments, plus format specifiers.

```rust
use farben::prelude::*;

let name = "Razkar";
let msg = cformat!("[green]Hello, {name}![/] All done.");
println!("{msg}");

// Positional arguments
let msg = cformat!("[red]{} and {}", "a", "b");

// Named arguments
let msg = cformat!("[green]{greeting}", greeting = "Hello");

// Format specifiers
let pi = 3.14159;
let msg = cformat!("[yellow]Pi is {pi:.2}");
```

When the `compile` feature is enabled, the format string is rendered at compile time.

::: tip
`cformat!` replaces the deprecated `color_fmt!` macro.
:::

## `cformatb!(...)`

Like `cformat!` but does not append a trailing reset. Styles bleed into subsequent output.

```rust
use farben::prelude::*;

let msg = cformatb!("[red]This bleeds");
```

## `cprint!(...)`

Prints farben-colored markup to stdout without a trailing newline. Behaves like `print!`.

When the `compile` feature is enabled, the format string is validated at compile time.

```rust
use farben::prelude::*;

let message = "I don't know";
cprint!("[red]Error: [/]{}", message);
```

## `cprintln!(...)`

Prints farben-colored markup to stdout with a trailing newline. Behaves like `println!`.

When the `compile` feature is enabled, the format string is validated at compile time.

```rust
use farben::prelude::*;

let result = "We did it!";
cprintln!("[green]Success: [/]{}", result);
```

## `cprintb!(...)`

Like `cprint!`, but does not append a trailing reset. Styles bleed into subsequent output.

```rust
use farben::prelude::*;

cprintb!("[red]Error: ");
cprintln!("something went wrong"); // inherits red
```

## `cprintbln!(...)`

Like `cprintln!`, but does not append a trailing reset. Styles bleed into subsequent output.

```rust
use farben::prelude::*;

cprintbln!("[bold red]Section header");
cprintln!("still bold and red here"); // inherits style
```

## `ceprint!(...)` / `ceprintln!(...)` / `ceprintb!(...)` / `ceprintbln!(...)`

Stderr variants of the print macros. Same behavior, but output goes to stderr.

```rust
use farben::prelude::*;

ceprintln!("[bold red]error:[/] file not found.");
ceprintb!("[yellow]warning: ");
ceprintln!("config missing.");
```

## `cwrite!(writer, ...)`

Writes farben-colored markup to a writer without a trailing newline. Works with any `Write` implementor.

```rust
use farben::prelude::*;
use std::io::Write;

let mut buffer = Vec::new();
cwrite!(buffer, "[red]Error: [/]{}", message);
```

## `cwriteln!(writer, ...)`

Writes farben-colored markup to a writer with a trailing newline.

```rust
use farben::prelude::*;
use std::io::Write;

let mut buffer = Vec::new();
cwriteln!(buffer, "[green]Success: [/]{}", result);
```

## `cwriteb!(writer, ...)` / `cwritebln!(writer, ...)`

Bleed variants of the writer macros. Like `cwrite!` / `cwriteln!` but without a trailing reset.

```rust
use farben::prelude::*;
use std::io::Write;

let mut buffer = Vec::new();
cwriteb!(buffer, "[red]Error: ");
cwrite!(buffer, "something went wrong"); // inherits red

cwritebln!(buffer, "[bold red]Section header");
cwrite!(buffer, "still bold and red here"); // inherits style
```

## `unansi!(...)`

Removes all CSI ANSI escape sequences from a formatted string. Accepts `format!`-style arguments.

```rust
use farben::prelude::*;

let colored = "\x1b[31mError\x1b[0m";
let plain = unansi!("{}", colored);
assert_eq!(plain, "Error");
```

## `unmarkup!(...)`

Removes all Farben markup tags from a formatted string. Invalid markup is left as-is.

```rust
use farben::prelude::*;

let stripped = unmarkup!("[bold red]Just the text");
assert_eq!("Just the text", stripped);
```

## `untag!(...)`

Escapes markup brackets in a formatted string. Every `[` becomes `[[`, every `]` becomes `]]`.

```rust
use farben::prelude::*;

let safe = untag!("[bold]hello[/]");
assert_eq!(safe, "[[bold]]hello[[/]]");
```

## `expand!(markup)`

Debug macro that prints the input, expanded tokens, and final ANSI output of a markup string.

```rust
use farben::prelude::*;

expand!("[bold red]hello");
// input:    [bold red]hello
// expanded: [bold][red]hello
// ansi:     "\x1b[1;31mhello\x1b[0m"
```

## `style!(name, markup)` (format feature)

Registers a named style in the global style registry. Requires the `format` feature.

```rust
use farben::prelude::*;

style!("ok",  "[bold green]");
style!("err", "[bold red]");

cprintln!("[ok]Build passed.");
cprintln!("[err]Build failed.");
```

::: warning
Panics if `$markup` is not valid farben markup.
:::

## `prefix!(name, prefix)` (format feature)

Attaches a literal string prefix to an already-registered named style. Requires the `format` feature.

```rust
use farben::prelude::*;

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

## `load_styles!()`

Includes the build-generated `farben_styles.rs` file from `OUT_DIR`. Used with `farben-build` to register styles defined in `.frb.toml` files.

```rust
use farben::prelude::*;
farben::load_styles!();

fn main() {
    init_styles();
    cprintln!("[error]Something went wrong.");
}
```

## `anstyle!(markup)` (anstyle feature)

Parses farben markup and converts the result into an `anstyle::Style`. Requires the `anstyle` feature.

```rust
use anstyle::Style;
let style: Style = farben::anstyle!("[bold red]Warning: ");
```

## `FarbenStr`

A compile-time colored string returned by `color!()` and `colorb!()`. Stores only the styled ANSI variant; plain text is derived at runtime when color is disabled.

```rust
use farben::FarbenStr;

let s = FarbenStr { styled: "\x1b[31mhello\x1b[0m" };
assert_eq!(s.resolve(), "\x1b[31mhello\x1b[0m");
println!("{s}"); // Display resolves automatically
```

## `color_enabled()`

Returns whether ANSI color output is enabled for this process. Respects `NO_COLOR`, `FORCE_COLOR`, and TTY detection, in that order.

```rust
use farben::color_enabled;

if color_enabled() {
    cprintln!("[green]Color is on!");
}
```

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
| `[bright-black]` | Bright Black |
| `[bright-red]` | Bright Red |
| `[bright-green]` | Bright Green |
| `[bright-yellow]` | Bright Yellow |
| `[bright-blue]` | Bright Blue |
| `[bright-magenta]` | Bright Magenta |
| `[bright-cyan]` | Bright Cyan |
| `[bright-white]` | Bright White |

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

### HSL Colors

```rust
color("[hsl(0,100,50)]Red via HSL")
color("[hsl(120,100,50)]Green via HSL")
```

### HSV / HSB Colors

```rust
color("[hsv(0,100,100)]Red via HSV")
color("[hsb(200,80,90)]Sky blue via HSB")
```

### HWB Colors

```rust
color("[hwb(0,0,0)]Red via HWB")
color("[hwb(0,50,0)]Pink")
```

### Lab / LCH / OKLCh Colors

```rust
color("[lab(53,80,67)]Vivid red")
color("[lch(50,30,270)]A blue hue")
color("[oklch(0.6,0.15,280)]Cool purple")
```

### Hex Colors

```rust
color("[#ff0000]Red via hex")
color("[#f00]Shorthand hex red")
```

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
| `[double-underline]` | Double Underline |
| `[blink]` | Blink |
| `[rapid-blink]` | Rapid Blink |
| `[reverse]` | Reverse Video |
| `[invisible]` | Hidden |
| `[strikethrough]` | Strikethrough |
| `[overline]` | Overline |

### Multi-tag Brackets

Multiple tags can be combined inside a single bracket, separated by spaces.

```rust
color("[bold red]I'm bold and red!")
color("[italic rgb(0,200,100)]I'm italic and green!")
```

### Reset

`[/]` resets all active styles and colors. All `c*` macros also append a reset automatically at the end.

```rust
color("[red]I'm red[/] but I'm not.")
```

### Specific Reset

`[/tagname]` resets a single style or color, leaving others active.

```rust
color("[bold red]Bold and red[/bold] just red now")
color("[bold red]Bold and red[/red] just bold now")
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
| `LexError::InvalidTag` | An unrecognized tag name was used |
| `LexError::InvalidValue` | A value inside `ansi()` or `rgb()` could not be parsed |
| `LexError::InvalidArgumentCount` | Wrong number of arguments passed to a color function |
| `LexError::UnclosedValue` | A color function like `rgb(` or `ansi(` was opened but never closed |
| `LexError::InvalidResetTarget` | A reset tag targeted something that cannot be reset |

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

## Error Display

```rust
use farben_core::errors::LexError;
use farben_core::lexer::tokenize;

let input = "[bold unknown]oops";
match tokenize(input) {
    Ok(_) => {}
    Err(e) => eprintln!("{}", e.display(input)),
}
```

---

The API covered in this page only covers Farben's public-facing API. For internals, look for the [docs.rs](https://docs.rs/farben) link for [farben-core](https://docs.rs/farben-core) or [farben-macros](https://docs.rs/farben-macros).
