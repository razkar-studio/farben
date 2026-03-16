# Getting Started

Okay, now you have Farben set up, quick, clean, and hopefully safe. If not, submit an issue to the GitHub repository.

::: tip
Just as a reminder, if you used the `compile` optional feature, you'll need to change the `color()` function calls to the `color!()` macro calls. For calls like `color(format!(...))`, use `color_fmt!(...)`. It probably won't matter because we will mostly use `cprintln!()` and `cprint!()` which works with both cases.
:::

Let's go into your `src/main.rs` file that we edited before to get a starting template, and explore how you can use Farben. Don't worry, it's easy.

## Basic Colors

In Farben, basic colors use direct names for the tags, like for basic red using the `[red]` tag name.
This isn't a random decision, it actually comes from the basic ANSI colors itself.

::: info
Bright color variants is here too!
:::

```rust
cprintln!("[red]Basic red [blue]Basic blue [yellow]Basic yellow [magenta]You get the point.");
```

## Emphasis

Give your text emphasis by making it **bold**, *italic*, ~~strikethrough~~, ~~***or even combine them***~~! 
Doing this is also easy, using the tag name `bold`, `italic`, `strikethrough`, `dim`, you get the point.

```rust
cprintln!("[bold]I'm bold! [/][italic]I'm italian [/][strikethrough]I was struck!");
cprintln!("[blink]I don't have eyes but I can blink. What am I? The blink tag!");
```

::: info
The full table of available styles can be seen in the [API reference](/examples/api-examples/).
:::

## Combining Tags

Why settle for one style when you can have many? Farben lets you stack multiple tags inside
a single bracket, separated by spaces.
```rust
cprintln!("[bold red]I'm bold AND red!");
cprintln!("[italic blue]I'm italic AND blue!");
cprintln!("[bold italic strikethrough yellow]I'm a mess, but a stylish one.");
```

::: tip
Order doesn't matter inside a bracket. `[bold red]` and `[red bold]` are identical.
:::

## Reset

Styles are sticky — once you apply `[red]`, everything after it is red until you say otherwise.
That's what `[/]` is for. It clears everything: colors, emphasis, all of it.
```rust
cprintln!("[red]I'm red [/]but I'm not anymore.");
cprintln!("[bold]Bold [italic]bold and italic [/]back to nothing.");
```

::: info
`color()`, `color_fmt!()`, `cprint!()`, and `cprintln!()` all automatically append a reset
at the end of every string, so styles never bleed into your next `println!` call.
:::

## Escape Sequence

What if you actually want to print `[red]` as literal text? Prefix the `[` with a backslash.
```rust
cprintln!("To make red text, write \\[red] before your text.");
// Output: To make red text, write [red] before your text.
```

::: warning
In Rust string literals, `\\` is a single backslash. So `\\[` is what farben sees as `\[`.
:::

::: danger
Do not, at all costs, write `\[` directly as it is an **invalid escape sequence**. 
Use `\\[` instead.
:::

## Error Handling

By default, `cprintln!()` and `color()` panic on invalid markup. If you want to handle errors
yourself, use `try_color()`:
```rust
use farben::try_color;

match try_color("[notacolor]oops") {
    Ok(s) => println!("{s}"),
    Err(e) => eprintln!("Markup error: {e}"),
}
```

::: tip
In library code or anywhere you don't control the input string, prefer `try_color()` over `color()`.
:::
