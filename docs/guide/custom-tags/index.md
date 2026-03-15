# Defining Your Own Tags

Don't play by the rules and keep writing `[bold underline red]` every time you want to signal an error.
Farben fully supports user-defined tags, so define it once and use it everywhere.

For example, say you're building a logger and want consistent styling for different log levels. Instead of repeating the same tag combinations or wrapping everything in a helper function, use `style!()`.
```rust
use farben::*;

fn main() {
    style!("error", "[bold underline red]");
    style!("warn", "[bold yellow]");
    style!("info", "[bold cyan]");

    cprintln!("[error]error: [/]Something bad happened...");
    cprintln!("[warn]warn: [/]This looks suspicious.");
    cprintln!("[info]info: [/]Server started on port 8080.");
}
```

::: info
Think of `style!` like an alias, where `[error]` becomes shorthand for `[bold underline red]`. Farben expands it automatically at parse time, so there's no runtime overhead beyond what the tags themselves would cost. Using `compile` also works, and parses it at compile-time!
:::

::: warning
User-defined tags can only be built from built-in farben tags. You cannot define a style in terms of another user-defined tag — `style!("critical", "[error bold]")` will panic because `error` is not a built-in tag.
:::

::: tip
Call `style!()` once at the start of your program, before any `color()` or `cprintln!()` calls that use your custom tags.
:::
