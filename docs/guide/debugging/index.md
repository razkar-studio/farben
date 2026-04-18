# When In Doubt, De-Bug

There are moments where you had a little *too* much fun with the style registry, and end up with a style called `[verycool]`
which references a style that references a style that... you get the point. You want to know what `[verycool]` actually IS.

Luckily, Farben has the `expand!()` diagnostic macro!

```rust
expand!("[verycool]What is this?")
```

Which outputs something like this:
```txt
input:    [verycool]
expanded: [bold strikethrough double-underline overline red bg:blue]
ansi:     "\u{1b}[1m\u{1b}[9m\u{1b}[21m\u{1b}[53m\u{1b}[31m\u{1b}[44m\u{1b}[0m"
```

Ouch, that's a really long line. What's the original code anyway?

```rust
fn main() {
    style!("cool", "[bold red]");
    style!("cool2", "[cool bg:blue]");
    style!("thing", "[strikethrough cool2]");
    style!("idkthing", "[cool thing]");
    style!("verycool", "[idkthing overline double-underline]");
    expand!("[verycool]");
    cprintln!("[verycool]What");
}
```

Dear God.
