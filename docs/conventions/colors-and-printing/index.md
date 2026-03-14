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

## color() vs color_fmt!() vs cprintln!()

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