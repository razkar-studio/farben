# Quick, Mark that Down!

The first thing that comes to your mind after hearing *markup* is either HTML, or Markdown. As of 0.8, Farben now supports partial opt-in Markdown!

To turn the feature on, literally turn the feature on. Insert this in your dependency:
```toml
farben = { version = "0.8", features = ["markdown"] }
```

Or while adding as a dependency:
```sh
cargo add farben --features markdown
```

::: info
Markdown support is limited to inline spans only. Supported syntax: `**bold**`, `*italic*`,
`_this type of italic_`, `__underline__` (not officially standard, but why not?),
`~~strikethrough~~`, and `` `inline code` ``. Block-level elements like headings and
lists are not supported.

Even with this, Farben's markdown fully supports nesting.
So `**I'm bold *and italic*, heck yeah!**` works nicely.
:::

## Using Markdown

Farben has a handful of functions and macros for markdown, mirroring the existing `color` API:
```rust
use farben::prelude::*;

let bold = markdown("**bold**");
let formatted = md_fmt!("**{}** is the answer", 42); // Bold "42 is the answer"
mdprintln!("Directly print like you're *italic* or __underlined__.");
mdprint!("`mdprint!()` itself, inline coded.");
```

## Heil Compilers!

If you want zero runtime overhead, swap the `markdown` feature for `markdown-compile`:
```toml
farben = { version = "0.8", features = ["markdown-compile"] }
```

This enables both `markdown` and `compile` at once. Static strings are processed entirely
at compile time and baked into your binary as plain ANSI strings, no parsing happens at runtime.

::: info
The changes to `markdown-compile` is that the `markdown()` function becomes a macro: `markdown!()`. 
The rest stays the same.
:::

```rust
use farben::prelude::*;

// Rendered at compile time — zero runtime cost
mdprintln!("**bold** and *italic*");

// Format args fall back to runtime automatically
let name = "world";
mdprintln!("**hello {}**", name);
```

::: tip
`markdown-compile` is a strict superset of `markdown` -- you don't need to enable both.
Enabling `markdown-compile` is enough. Glory to Compilers!
:::
