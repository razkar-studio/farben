# Taking the Color Out

Sometimes you need the plain text. Log files, display-width calculations, piping to tools that see ANSI sequences as noise: stripping color is a real use case, and Farben covers it.

## `ansi_strip!` - the Idiomatic Way

If you're already using `farben::prelude::*`, `ansi_strip!` is right there. It takes the same arguments as `format!`, formats the string, then strips any ANSI sequences before returning.

```rust
use farben::prelude::*;

let colored = "\x1b[31mError\x1b[0m";
let plain = ansi_strip!("{}", colored);
assert_eq!(plain, "Error");

// format args work exactly as you'd expect
let code = 42;
let plain = ansi_strip!("\x1b[1mcode {code}\x1b[0m");
assert_eq!(plain, "code 42");
```

The return type is `String`. No extra imports, no intermediate variables.

::: tip
`ansi_strip!` is the right call when you want to format and strip in one step. If you already have a `&str` and just want to clean it, reach for `strip_ansi` from `farben_core` instead.
:::

## `strip_ansi` - Lower-Level Control

For cases where you're holding a pre-colored string and want to strip it without going through a format step, `farben_core::strip::strip_ansi` takes a `&str` and returns a `String`.

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
let plain = ansi_strip!("{}", message);
let mut log = OpenOptions::new().append(true).open("app.log")?;
writeln!(log, "{plain}")?;
```
:::note
Runtime Farben already has TTY detection!
:::
