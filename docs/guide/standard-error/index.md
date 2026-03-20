# Even Errors Have Standards...

Printing to `stderr` is something you, when making an app, most likely always do *at least* once. It's the place for errors, when things go unexpected (or expected, but failed) in your app.
::: info
`stderr` stands for Standard Error, and is an output stream dedicated for error messages and diagnostics, separate than `stdout` (which you most likely know).
Why a separate output stream? `stderr` is typically unbuffered, which means error messages are displayed immediately even if the program crashes.
:::

## `stdout` or `stderr`?

Here's a good rule of thumb: if it's something the user asked for, it goes to `stdout`. If it's about what the program is doing (errors, warnings, diagnostics), it goes to `stderr`.
```rust
cprintln!("[green]Done![/] Output written to file."); // stdout: this is the result
ceprintln!("[red]error:[/] file not found.");         // stderr: this is a problem
```

This matters more than it might seem. Tools that pipe your program's output (like `grep` or `>`) only capture `stdout` by default, so keeping errors on `stderr` means they don't pollute the actual output.

## Basic Usage

Farben's stderr macros mirror the stdout ones exactly, just with an `e` after the `c`:
```rust
use farben::*;

ceprint!("[red]error:[/] ");
ceprintln!("file not found.");

ceprintln!("[bold yellow]warning:[/] config missing, using defaults.");
ceprintln!(); // blank line for breathing room
```

::: tip
`ceprintln!()` with no arguments prints a bare newline to stderr, same as `eprintln!()` in std. Useful for spacing out error output.
This also applies to the other printing macros.
:::

## Bleed Variants

`ceprintb!` and `ceprintbln!` work the same as their stdout counterparts, styles carry forward without a trailing reset.
```rust
use farben::*;

ceprintb!("[bold red]fatal: ");
ceprintln!("something went very wrong."); // inherits bold red
```

See [Purposefully Bleeding](../bleeding/) for a full breakdown of how bleed works.

## Markdown on `stderr`

If you have the `markdown` feature enabled, `mdeprint!` and `mdeprintln!` bring inline markdown rendering to stderr too.
```rust
use farben::*;

mdeprintln!("**error:** the config file is *missing* or malformed.");
```

No reason your error messages can't look good.
