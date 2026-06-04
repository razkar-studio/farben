# Taking the Color Out

Sometimes you need the plain text. Log files, display-width calculations, piping to tools that see ANSI sequences as noise: stripping color is a real use case, and Farben covers it.

Farben provides three macros for three different stripping needs. All of them accept `format!`-style arguments.

## `unansi!` - Strip ANSI Escape Sequences

Removes all CSI ANSI escape sequences from a formatted string. Every sequence of the form `ESC [ ... <letter>` is removed. Non-CSI `ESC` bytes are left intact.

```rust
use farben::prelude::*;

let colored = "\x1b[31mError\x1b[0m";
let plain = unansi!("{}", colored);
assert_eq!(plain, "Error");

// format args work exactly as you'd expect
let code = 42;
let plain = unansi!("\x1b[1mcode {code}\x1b[0m");
assert_eq!(plain, "code 42");
```

The return type is `String`. No extra imports, no intermediate variables.

## `unmarkup!` - Strip Farben Markup Tags

Removes all Farben markup tags (like `[bold]` or `[/]`) from a formatted string. Invalid markup is left as-is without panicking.

```rust
use farben::prelude::*;

let stripped = unmarkup!("[bold red]Just the text");
assert_eq!("Just the text", stripped);

let invalid = unmarkup!("[I'm unclosed");
assert_eq!("[I'm unclosed", invalid);

// format args work too
let msg = "hey!";
let formatted = unmarkup!("[bold red]{msg}");
assert_eq!("hey!", formatted);
```

## `untag!` - Escape Markup Brackets

Doubles every `[` and `]` character so the result contains no parseable tags. Useful when you want to display markup as literal text.

```rust
use farben::prelude::*;

let safe = untag!("[bold]hello[/]");
assert_eq!(safe, "[[bold]]hello[[/]]");

let name = "world";
let safe = untag!("[bold]{name}[/]");
assert_eq!(safe, "[[bold]]world[[/]]");
```

## Low-Level Functions

All three macros delegate to functions in `farben_core::strip`:

- `strip_ansi(input: &str) -> String` — strips CSI sequences
- `strip_markup(input: &str) -> String` — strips Farben markup tags
- `escape_tags(input: &str) -> String` — escapes brackets

```rust
use farben_core::strip::strip_ansi;

let colored = "\x1b[31mred text\x1b[0m";
assert_eq!(strip_ansi(colored), "red text");
```

### What Gets Stripped

`strip_ansi` targets CSI sequences specifically: `ESC [ <params> <letter>` (the sequences responsible for colors, bold, underline, and other SGR attributes). Things like `\x1b[31m`, `\x1b[0m`, `\x1b[1;4m`.

Bare `ESC` bytes not followed by `[` are passed through unchanged:

```rust
use farben_core::strip::strip_ansi;

let bare_esc = "\x1bhello";
assert_eq!(strip_ansi(bare_esc), "\x1bhello");
```

::: info
This is intentional. Non-CSI escape sequences serve other purposes (terminal mode switching, cursor movement outside of CSI, etc.). Stripping them blindly would corrupt output that uses those sequences legitimately.
:::

## When to Strip

A few situations where this comes up:

- **Log files.** Your terminal output looks great colored, but writing those bytes to a file produces unreadable noise for anything reading it back as plain text.
- **Display width.** ANSI sequences contribute zero visible characters but real byte count. Strip first, then measure with something like `unicode-width`.
- **Interop.** Feeding colored output to a tool (`grep`, `awk`, a Slack webhook, a CI log parser) that doesn't know about ANSI.

```rust
use farben::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

let message = cformat!("[red]error:[/] file not found.");

// print in color to the terminal
cprintln!("[red]error:[/] file not found.");

// write plain to the log
let plain = unansi!("{}", message);
let mut log = OpenOptions::new().append(true).open("app.log")?;
writeln!(log, "{plain}")?;
```

## Deprecated Aliases

The macros `ansi_strip!` and `markup_strip!` are deprecated in favor of `unansi!` and `unmarkup!` respectively. They still work but will be removed in a future release.

| Old Name | New Name |
|----------|----------|
| `ansi_strip!` | `unansi!` |
| `markup_strip!` | `unmarkup!` |
