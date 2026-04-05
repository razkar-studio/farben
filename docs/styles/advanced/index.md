# Under The Hood

So you've got style files working and you're curious what's actually happening when you call `farben_build::run()`. Let's pull 
back the curtain.

## The Full Pipeline

When `build.rs` runs, `farben-build` reads your `.frb.toml` file and generates **two separate output files** in Cargo's `OUT_DIR
`. They have different consumers and different jobs, which is why there are two of them.

```
.frb.toml  →  farben-build  →  farben_styles.rs       (runtime)
                            →  farben_registry.lsv    (compile-time)
```

Let's go through each one.

## `farben_styles.rs`: The Runtime File

This is a Rust source file. Your `load_styles!()` call expands to a single `include!()`:

```rust
// what load_styles!() actually is
include!(concat!(env!("OUT_DIR"), "/farben_styles.rs"))
```

That `include!()` pastes the generated file directly at the call site. It's not an import. The file it pastes in looks roughly 
like this:

```rust
pub fn init_styles() {
    farben::insert_style("error", farben::Style::parse("[bold red]").unwrap_or_else(|e| panic!("{e}")));
    farben::set_prefix("error", "error:").unwrap_or_else(|e| panic!("{e}"));
}
```

This is where `init_styles()` comes from. When you call it at the top of `main()`, these `insert_style` and `set_prefix` calls 
run and register everything into the global registry.

::: note
Notice the brackets around `bold red` in the generated code. Your config has `error = "bold red"` (no brackets), but 
`farben-build` wraps the value automatically before writing it out. You don't have to think about it.
:::

## `farben_registry.lsv`: The Compile-Time File

LSV stands for "line separated values." It's a custom format invented specifically to avoid any parser headache or 
dependencies. The whole point is that `farben-build` has **zero runtime dependencies**, so it rolls its own minimal format:

```
error=bold red
my:error=bold underline red
---
error=error:
my:error=[ERROR]
```

Styles come first, then a `---` separator, and then prefixes. Keys use `=` with no spaces around it. That's the entire spec,
made specifically so I don't write any complicated stuff. Oh, and if it isn't obvious enough, don't edit the files unless you 
know what you're doing.

This file is read by the `farben-macros` proc macros: `color!()`, `colorb!()`, and `validate_color!()`. Every time one of those 
macros runs, it calls `load_registry()` first, which:

1. Reads `farben_registry.lsv` from `OUT_DIR`
2. Splits on `---\n` to separate styles from prefixes
3. Calls `farben_core::registry::insert_style` for each style line
4. Calls `farben_core::registry::set_prefix` for each prefix line
5. Returns silently if the file doesn't exist

It returns silently because no build script means no `.lsv` file, and the macros just continue without custom styles.

This is how `color!("[error]text")` resolves `[error]` at compile time. The proc macro runs inside the Cargo build process, 
which already executed `build.rs` and wrote the registry to `OUT_DIR` before any macro invocations happen.

::: info
The `load_registry()` call happens inside the proc macro crate, not in your code. You never call it directly. It's the 
invisible setup step before every `color!()` expansion.
:::

## Why Two Files?

Fair question. Why not just use `farben_registry.lsv` for both purposes?

The runtime side needs actual Rust function calls at program startup. You can't execute a data file. The `include!()` approach 
generates those calls as real Rust code, which is why `farben_styles.rs` exists at all.

The compile-time side has the opposite problem. Proc macros run inside the Cargo build process and have access to `OUT_DIR`. 
But they can't call `init_styles()` because that function lives in your code, not in the macro crate. They can read a file from 
disk though, which is exactly what the `.lsv` format enables.

So: one file for each phase of the build.

::: details Why not generate a third artifact the macros can `include!()` too?
Proc macros run in a sandboxed environment and can't use `include!()` from `OUT_DIR` the same way user code can. Reading a file directly with `std::fs::read_to_string` is simpler and more reliable inside a proc macro context.
:::

## The `.frb.toml` Parser

Despite the `.toml` extension, this file is **not parsed by the `toml` crate**. `farben-build` has zero dependencies and implements its own parser from scratch. It handles:

- `[styles]` and `[prefixes]` section headers
- `[styles.namespace]` and `[prefixes.namespace]` for namespaced keys like `[my:error]`
- `key = "value"` pairs where values must be double-quoted
- `#` line comments
- Blank lines (ignored)

Only two top-level sections are valid: `styles` and `prefixes`. Any other section name is a build error. Anything that doesn't 
match these rules produces a build error too.

::: tip
If your build fails with a parse error from `farben-build`, it's almost always a missing quote around a value or a typo in a section name. Check those first.
:::

The setup steps for creating your `.frb.toml` and wiring up `build.rs` are all covered on the [Theme Files](/styles/styles-toml/
) page if you need them.
