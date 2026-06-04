# Errors and Compiling

## Panicking vs `try_color()`

`color()`, `cprintln!()`, and friends all panic on invalid markup. That's fine for most cases
where you wrote the markup yourself and you know it's valid.

```rust
cprintln!("[bold green]All good here.");
```

Switch to `try_color()` when the markup comes from somewhere you don't fully control, like
user input, a config file, or a database.

```rust
match try_color(&user_input) {
    Ok(s) => println!("{s}"),
    Err(e) => eprintln!("Invalid markup: {e}"),
}
```

::: warning
Never use `color()` on unvalidated external input. A bad tag will panic your whole program.
:::

## The `compile` Feature

Enable it when your markup is all fixed string literals and you want two things: zero runtime
parsing overhead, and compile-time errors instead of runtime panics.

```toml
farben = { version = "x.x.x", features = ["compile"] }
```

With `compile` enabled, `color!("[red]hello")` becomes a string literal baked into your
binary at compile time. Invalid markup becomes a compiler error -- not a surprise at runtime.

```rust
color!("[notacolor]oops"); // compiler error, not a panic
```

::: info
The `compile` feature also improves `cformat!`, `cprintln!`, `cprint!`, and all other macros
in the `c*` family by validating static markup at compile time. Only `{...}` format arguments
are evaluated at runtime.
:::

## Error Display

When a `LexError` occurs, you can use the `.display()` method to produce compiler-style
output with a caret pointing at the offending byte:

```rust
use farben_core::lexer::tokenize;

let input = "[bold unknown]oops";
match tokenize(input) {
    Ok(_) => {}
    Err(e) => eprintln!("{}", e.display(input)),
}
```

This prints:

```
   | [bold unknown]oops
   |  ^^^^^^^^^^^ invalid tag: 'bold unknown'
```

## All Error Variants

`try_color()` returns a `LexError` on failure. The variants are:

| Variant | Cause |
|---------|-------|
| `LexError::UnclosedTag(usize)` | A `[` was opened but never closed with `]` |
| `LexError::InvalidTag { tag_content, position }` | An unrecognized tag name was used |
| `LexError::InvalidValue { value, position }` | A value inside `ansi()`, `rgb()`, etc. could not be parsed |
| `LexError::InvalidArgumentCount { expected, got, position }` | Wrong number of arguments passed to a color function |
| `LexError::UnclosedValue(usize)` | A color function like `rgb(` or `ansi(` was opened but never closed with `)` |
| `LexError::InvalidResetTarget(usize)` | A reset tag targeted something that cannot be reset |

## The `lossy` Feature

When the `lossy` feature is enabled (it is on by default), unknown tags produce a warning
rather than an error. The tag is silently consumed and the text continues. This is useful
when processing markup from sources that might use tags from a newer version of Farben.

```toml
[dependencies]
farben = { version = "...", features = ["lossy"] }
```

To disable lenient parsing, opt out of the default features:

```toml
[dependencies]
farben = { version = "...", default-features = false, features = ["format"] }
```
