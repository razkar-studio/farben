# Theme Files

Guess what? On the newest version of Farben (v0.14), **style files** is now available on *both* runtime and *compile* lovers, 
and they even use the same exact workflow! This finally connects the *compile* feature with the joy of custom styles. Here's 
how to make one and integrate them.

The official extension for Farben theme files are `farben.frb.toml`, where `farben` can be changed with anything. Here's a
template to get things going:
```toml
[styles]
error = "bold red"

[styles.my]
error = "bold ansi(217)"

[prefixes]
error = "error:"

[prefixes.my]
error = "[ERROR]"
```
:::note
Notice how we don't wrap the codes around square brackets here, Farben automatically wraps them!
:::

The `.my` on the `styles` section indicate **namespaces**, basically a prefix on the style name itself. So in this case:
```rust
cprintln!("[error]") // bold red
cprintln!("[my:error]") // bold ansi(217)
```
With the colon `:` being a separator. Prefixes also follow the same rule, with the section named `prefixes`.

## The Startup

Now let's actually make Farben register the styles. 

1. Add `farben-build` as a **build dependency**:
```sh
cargo add farben-build --build
```

2. Create a `build.rs` file:
```sh
touch build.rs
```

3. Use the `farben_build::run()` command in the `build.rs` file:
```rust
fn main() {
    farben_build::run();
}
```

4. Now open `src/main.rs`, and add a couple lines of code:
```rust
use farben::prelude::*;
farben::load_styles!();

fn main() {
    init_styles();
}
```

5. And now `[error]` and `[my:error]` works as valid tags!

:::note
`init_styles()` is a function that basically registers the styles, so for anywhere else not on `main()` where you want the 
styles set, call it!
:::

## Multiple Files with `run_with()`

`farben_build::run()` always looks for `farben.frb.toml` by convention. If you want a different filename, or you want to
split your styles across multiple files, `run_with(&[paths])` takes a slice of paths instead:

```rust
fn main() {
    farben_build::run_with(&["theme.frb.toml", "brand.frb.toml"]);
}
```

Both files get merged into the same registry, so `[error]` from `theme.frb.toml` and `[brand:primary]` from
`brand.frb.toml` all just work.

:::tip
Split files by *concern*, not by environment. If the same key appears in two files, Farben panics at build time. Keeping
`theme.frb.toml` for colors and `brand.frb.toml` for prefixes means that collision never happens.
:::

## Works with `compile` Too

This is the big one. The `style!()` macro from the [Define Your Own Tags](/guide/custom-tags/) guide registers styles at
runtime, which means it's not visible to the compile-time macros. Style files are different. Because `farben-build` writes
the registry to `farben_registry.lsv` in `OUT_DIR` at build time, the `compile` feature macros can read it during
compilation.

After the full setup above, both of these work with `[error]`:

```rust
// runtime - works as always
cprintln!("[error] something went wrong");

// compile feature - also works, resolved at build time
println!("{}", color!("[error] something went wrong"));
println!("{}", color_fmt!("[error] something went wrong"));
```

No extra steps. The `color!()` and `color_fmt!()` macros pick up `[error]` from the generated registry file automatically.

:::note
If you don't need the `compile` feature, `style!()` from the [Define Your Own Tags](/guide/custom-tags/) guide is the
simpler path. No build script, no `farben-build` dependency. Just call `style!()` at runtime and move on.
:::

:::warning
The style file only reaches the compile-time macros if `farben_build::run()` (or `run_with`) actually runs in `build.rs`.
If you skip the build script and try to use `color!("[error]")`, it will fail to compile with an "invalid tag" error. The
macro has no way to know `[error]` exists without the generated registry file.

Also, fully `compile` doesn't respect `NO_COLOR` and `FORCE_COLOR` or TTY detection, it only does so at build-time.
:::
