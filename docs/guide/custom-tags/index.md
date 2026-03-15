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

## Pre-fix

Sometimes I wonder why it's called a prefix. What do you fix beforehand? I may never know...

Talking about prefixes, Farben has this neat feature where your user-defined styles can also be bound an optional prefix!
Not sure what that means? Just look:

```rust
style!("error", "[underline red]");
prefix!("error", "[ERROR]");
cprintln!("[error] Something bad happened..");
// Output -> [ERROR] Something bad happened (in underline red)
```

That's right! Goodbye to those helpers, because setting prefixes using `prefix!("style-name", "prefix-string")` would
output the prefix-string *alongside* the colored style when it's called! This is one of the my personal favorite features.

::: details
Bit of a party trick, but you can bind `style!()` to a `[reset]` and set a prefix for that.
That makes the style reset, and then output the prefix.

```rust
style!("my-reset", "[/]");
prefix!("my-reset", "reset:");
cprintln!("[my-reset] Reset all progress");
// Output -> reset: Reset all progress
```
:::

::: tip
User-defined styles, other than being used as an alias, could also be used like design tokens or something!
It's completely up to you on how you use this feature.

```rust
style!("header", "[bold green]");
prefix!("header", "#");
cprintln!("[header] Cool Header");
```
:::
