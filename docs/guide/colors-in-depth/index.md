# Colors In Depth

So you've seen `[red]` and `[blue]`. Those are great, but Farben supports a lot more than eight colors. Let's go deeper.

## Foreground and Background

By default, every color tag applies to the **foreground**, which is the text itself. But you can explicitly target the background too using the `bg:` and `fg:` prefixes.
```rust
cprintln!("[bg:red]Red background!");
cprintln!("[fg:white bg:blue]White text on a blue background.");
cprintln!("[bold fg:yellow bg:red]Chaotic but valid.");
```

::: tip
`[red]` and `[fg:red]` are identical. The `fg:` prefix is just explicit, useful when combining foreground and background in the same bracket.
:::

## ANSI 256-Color Palette

Your terminal supports 256 colors. Access any of them with `ansi(n)` where `n` is a number from 0 to 255.
```rust
cprintln!("[ansi(214)]This is a nice orange.");
cprintln!("[ansi(93)]Deep purple. My favorite.");
cprintln!("[bg:ansi(236)]Don't use black text on me.");
```

::: info
Not sure which number maps to which color? Look up an ANSI 256 color chart, there are plenty online.
:::

## RGB Colors

For full 24-bit color, use `rgb(r,g,b)` with values from 0 to 255.
```rust
cprintln!("[rgb(255,128,0)]Orange, but uses unnecessary values. Do we really need RGB for this..?");
cprintln!("[rgb(0,200,100)]Look at that tree!");
cprintln!("[bold rgb(255,0,128)]Is it hot here?");
cprintln!("[fg:rgb(255,255,255) bg:rgb(30,30,30)]I'm light on the inside, dark on the outside.");
```

::: warning
Spaces inside `rgb()` are not supported. Use `rgb(255,128,0)`, not `rgb(255, 128, 0)`.
:::

## Combining It All

All color formats work inside multi-tag brackets alongside emphasis styles.
```rust
cprintln!("[bold bg:ansi(236) fg:rgb(255,200,0)]I'm a gold text on dark grey, I'm bold!");
```

The order of tags inside a bracket doesn't matter, Farben processes them all. 
But it's better to be sure if you're following the rules, hence why we go towards Conventions.
