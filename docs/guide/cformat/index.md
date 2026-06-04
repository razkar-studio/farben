# Format Macros

The `cformat!` and `cformatb!` macros format text with Farben markup and return the result as a `String`. Think of them as `format!` with full markup support built in.

## `cformat!`

Formats a string with Farben markup and appends a trailing reset.

```rust
use farben::prelude::*;

let name = "Farben";
let msg = cformat!("[bold green]Hello, {name}![/] Welcome.");
println!("{msg}");
```

### Format Argument Styles

`cformat!` supports all the same argument styles as `format!`:

```rust
use farben::prelude::*;

// Positional arguments
let msg = cformat!("[red]{} and {}", "a", "b");

// Named arguments
let msg = cformat!("[green]{greeting}", greeting = "Hello");

// Implicit capture (Rust 1.58+)
let user = "Alice";
let msg = cformat!("[dim]{user} joined the server.");

// Format specifiers
let pi = 3.14159;
let msg = cformat!("[yellow]Pi is approximately {pi:.2}");
```

Bleed variant:

```rust
use farben::prelude::*;

let msg = cformatb!("[red]This style bleeds");
// No trailing reset appended. msg ends with the ANSI red code
```

## When the `compile` Feature Is Enabled

With `compile` enabled, static markup segments in your format string are rendered to ANSI escape codes at compile time. Only the `{...}` argument placeholders are evaluated at runtime. This means the markup itself has zero runtime overhead.

```toml
[dependencies]
farben = { version = "...", features = ["compile"] }
```

```rust
use farben::prelude::*;

let user = "Alice";
// "[bold green]" is baked into the binary as ANSI codes at compile time
cprintln!("[bold green]{user} logged in.");
```

The `{user}` placeholder is the only part evaluated at runtime. The ANSI sequences for `[bold green]` are already in the binary as a static string literal.

## `cformat!` vs `cformatb!`

| Macro | Trailing Reset | Use Case |
|-------|---------------|----------|
| `cformat!` | Yes | Formatting a self-contained colored string |
| `cformatb!` | No | Formatting a string whose style bleeds into subsequent output |

## Replaces `color_fmt!`

The `color_fmt!` macro is deprecated in favor of `cformat!`. The new name is shorter and consistent with the `cprint!` / `cprintln!` naming convention.

```rust
// Old way (deprecated):
// let msg = color_fmt!("[red]Error: {err}");

// New way:
let msg = cformat!("[red]Error: {err}");
```

## All Macros Using `cformat!`

Every macro in the `c*` family uses `cformat!` internally when the `compile` feature is enabled:

- `cprint!` / `cprintln!`
- `cprintb!` / `cprintbln!`
- `ceprint!` / `ceprintln!`
- `ceprintb!` / `ceprintbln!`
- `cwrite!` / `cwriteln!`
- `cwriteb!` / `cwritebln!`

This means enabling `compile` improves the performance of all of them, not just `cformat!`.
