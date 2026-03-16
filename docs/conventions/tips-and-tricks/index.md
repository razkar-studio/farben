# Tips and Tricks

Small things that make farben nicer to work with.

## Trailing Spaces When Switching Styles

When you switch from one style to another, put the trailing space inside the colored segment,
not outside it. Otherwise the gap between words might inherit the wrong style.
```rust
// Good
cprintln!("[red]Error: [yellow]something went wrong.");

// Bad
cprintln!("[red]Error:[yellow] something went wrong.");
```

::: tip
There are cases for when you want the latter instead of the former, like backgrounds. In that case there is no conventions,
like underlines. For cases where leaving any space would make it look ugly, simply immediately put `[/]` right after.

```rust
cprintln!("[underline red]error:[/] [yellow]Without the immediate reset, that underline would partly bleed towards me! Scary...");
```
:::

::: info
As of the newest version, you can now specifically reset a style/color.
```rust
cprintln!("[underline red]error:[/underline] I'm still red.")
```
:::

## No Spaces After Opening Tags

Tags apply immediately to whatever follows them. No need for a space after `[red]`.
```rust
// Good
cprintln!("[red]Hello!");

// Unnecessary space, will be printed as part of the output
cprintln!("[red] Hello!");
```

## Reset Early, Not Late

If you're switching between styles mid-string, reset before applying the next style rather
than relying on the auto-reset at the end.
```rust
// Good -- explicit reset between styles
cprintln!("[bold red]Bold red text. [/][blue]Blue text.");

// Works, but the bold bleeds until the auto-reset at the very end
cprintln!("[bold red]Bold red text. [blue]Blue text. If I'm bold, blame the guy before me.");
```

## Escape Brackets in Instructional Text

If you're printing text that shows farben markup as an example, escape the opening bracket.
```rust
cprintln!("To make red text, use \\[red] before your text.");
// Output: To make red text, use [red] before your text.
```

## Prefer `cprintln!()` Over `println!(color(...))`

Both work, but `cprintln!()` is shorter and handles the format args pattern cleanly.
```rust
// Good
cprintln!("[green]Done in {}ms.", elapsed);

// Unnecessary verbosity
println!("{}", color_fmt!("[green]Done in {}ms.", elapsed));
```

## Emphasis First, Color Second

When using both emphasis and color, always put emphasis first. It doesn't matter in the output, but it's significantly more readable.
```rust
// Good
cprintln!("[bold green]I'm bold green!");

// Bad
cprintln!("[green bold]I'm green bold!"); // Doesn't make sense
```

::: tip
Always sound your tag names out in English. Whichever one sounds better, go for that one.
:::

## Background First, then Foreground

When modifying both background and foreground values, always put background first. Again, for the same reason as above.
```rust
// Good
cprintln!("[bg:green fg:white]I'm white in a green background!");

// Bad
cprintln!("[fg:white bg:green]I have a green background and white text!"); // Too long
```

::: info
Unlike emphasis and color, background comes before foreground because you're describing
the environment first, then the text inside it. Think of it like painting a wall before
placing furniture.
:::

---

Idiomatic ways to bleed are on [Colors And Printing](../colors-and-printing/#idiomatic-ways-to-bleed).

---

That's it! You now know everything you need to color your terminal with Farben. Check out the
[API Reference](../../examples/api-examples/) for the full list of tags and functions, or browse the
[Project Examples](../../examples/project-examples/) for real-world usage.

Thank you for reading this documentation!
