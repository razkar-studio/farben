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

::: details
The specific reset wipes *all* styles of that type, so this
```rust
cprintln!("[bold][bold]I'm bold[/bold] I'm not bold!");
```
wouldn't wipe the first `bold`, but all `bold`s.
:::
