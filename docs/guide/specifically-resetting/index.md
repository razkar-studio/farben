# Death To Inclusivity (in resetting)

::: warning
That title is a joke.
:::

Have you ever gotten into a situation where you want one word to stand out in your colored mess, but then you'd have to reset
all styles and reapply them? Yeah, that's annoying. But don't worry! Farben 0.6 got you covered.

```rust
cprintln!("[italic blue]Detail text, because lorem ipsum [underline]loremizises [/][italic blue]this.")
// It's annoying that you have to reset all styles instead of just the underline!
```

Introducing specific resets, where you just put the style you want to reset in-front of the slash (`[/style-here]`).
This style is very familiar to those of you from HTML, where you close specific tags.

```rust
// instead of that previous one...
cprintln!("[italic blue]Detail text, because lorem ipsum [underline]loremizises[/underline] this."); // Cleaner.
```

You can reset color or emphasis specifically:

```rust
cprintln!("[bold red]Bold and red [/bold]just red now");
cprintln!("[bold red]Bold and red [/red]just bold now");
cprintln!("[bold italic underline]All three [/underline]no underline");
```

::: details
The specific reset wipes *all* styles of that type, so this
```rust
cprintln!("[bold][bold]I'm bold[/bold] I'm not bold!");
```
wouldn't wipe the first `bold`, but all `bold`s.
:::

## Reset Targets

You can reset any of the following:

- Emphasis styles: `[/bold]`, `[/italic]`, `[/underline]`, `[/dim]`, `[/strikethrough]`, `[/blink]`, `[/overline]`, `[/invisible]`, `[/reverse]`, `[/rapid-blink]`, `[/double-underline]`
- Named colors: `[/red]`, `[/blue]`, etc.
- RGB colors: `[/rgb(255,0,0)]`
- ANSI 256 colors: `[/ansi(196)]`
- All HSL/HSV/Lab/etc. formats: `[/hsl(0,100,50)]`, `[/hwb(0,0,0)]`, etc.
- Background colors: `[/bg:red]`, `[/bg:rgb(255,0,0)]`, etc.
