# Errors and Compiling

## Panicking vs try_color()

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

## The compile Feature

Enable it when your markup is all fixed string literals and you want two things: zero runtime
parsing overhead, and compile-time errors instead of runtime panics.
```toml
farben = { version = "x.x.x", features = ["compile"] }
```

With `compile` enabled, `color!("[red]hello")` becomes a string literal baked into your
binary at compile time. Invalid markup becomes a compiler error, not a surprise at runtime.
```rust
color!("[notacolor]oops"); // compiler error, not a panic
```

::: info
The `compile` feature only eliminates overhead for `color!()` with fixed strings. `color_fmt!()`,
`cprintln!()`, and `cprint!()` still run at runtime, but their format strings are validated
at compile time.
:::

Skip the `compile` feature if most of your markup is dynamic or comes from runtime values.
The validation benefit is still there for format strings, but the binary size tradeoff may
not be worth it for smaller projects.