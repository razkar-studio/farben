# Coloring and Printing

Farben gives you a few ways to color text and print it. Here's when to use each one.

## Picking a Color Format

Start with named colors. They're readable, short, and cover the basics.
```rust
cprintln!("[red]Error! [green]Success! [yellow]Warning!");
```

Reach for ANSI256 when you want more variety but don't need exact control. A color chart
is your best friend here.
```rust
cprintln!("[ansi(208)]A warm orange. [ansi(27)]A vivid blue.");
cprintln!("Orange is an awesome way to use ANSI256 since named colors don't have them.")
```

Use RGB when you have a specific color in mind, like a brand color or a precise shade.
```rust
cprintln!("[rgb(255,90,0)]Fancy Orange (TM).");
cprintln!("That orange looks [italic]fancy...");
```

::: tip
When in doubt, named colors first. They're the most readable at a glance and work
everywhere.
:::

## `color()` vs `color_fmt!()` vs `cprintln!()`

A confusing amount of choices that all do different things.

Use `color()` when you need the colored string itself, not just to print it. For example,
passing it to a logger or building a larger string.
```rust
let msg = color("[red]Something went wrong!");
log::error!("{msg}"); // This is not a Farben built-in.
```

Use `color_fmt!()` when you need to interpolate runtime values into a colored string and store it.
```rust
let path = "/some/file.txt";
let msg = color_fmt!("[red]File not found: [/]{path}");
```

Use `cprintln!()` for everything else. It's the shortest path from markup to terminal output.
```rust
cprintln!("[green]Done in {}ms.", elapsed);
```

::: tip
`cprint!()` works the same as `cprintln!()` but without the trailing newline, useful when
you're building output incrementally.
:::

## Idiomatic Ways to Bleed

Idioms, idioms. There's a ton of things that have their own idioms, even bleeding!

The main use case for bleeding is to separate printed text into multiple lines that share the same style. 
If that's not your use case for bleeding, I don't know *what* is.

To *elegantly* bleed appropriately, we *declare the style first*, and then let the text below use that style.
Like the following:

```rust
cprintb!("[italic blue]"); // Declare the style.
cprintb!("I'm blue and I'm italian. This is a description text yadda yadda, "); // Leading space for printing in-line
cprintb!("Lorem ipsum dolor sit amet. Consectur adipiscing elit. What does that mean? ");
cprintln!("Final description text uses `cprintln!` to finally reset the style.");

cprintln!("\nGuys, what did I miss?");
```

::: info
This is the idiomatic way to use bleeds. It doesn't matter if you use newlines or not,
but declaring the style and then using an in-line print is the correct way.
:::

## Writer Variants

Sometimes you need to write to somewhere other than stdout or stderr. Maybe a file, a `String`, or a custom buffer. Farben has you covered with writer variants that work with any `Write` implementor.

```rust
use farben::prelude::*;
use std::io::Write;

// Write to a String
let mut output = String::new();
cwriteln!(output, "[green]Success![/] operation completed.");
cwrite!(output, "[yellow]Warning: [/]{} items remaining.", count);

// Write to a file
let mut file = std::fs::File::create("log.txt")?;
cwriteb!(file, "[red]ERROR: ");
cwriteln!(file, "{}[/]", error_message);
```

All four variants are available:
- `cwrite!` writes without a newline, appends reset
- `cwriteln!` writes with a trailing newline, appends reset
- `cwriteb!` writes without a newline, does not reset (bleeds)
- `cwritebln!` writes with a newline, does not reset (bleeds)

::: tip
The writer variants use the exact same markup processing as the stdout macros. They support all colors, emphasis styles, RGB, ANSI256, and everything else Farben offers.
:::
