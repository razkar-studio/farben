# Inline Syntax

Write `*bold*`, `_underline_`, `` `code` ``, `~strikethrough~`, and `/italic/` directly inside your Farben markup strings. No extra functions, no separate macros. Just enable the `inline` feature and the `c*` family handles it transparently.

```toml
[dependencies]
farben = { version = "...", features = ["inline"] }
```

Or:

```sh
cargo add farben --features inline
```

## How It Works

The `inline` feature runs a pre-processing pass before tokenization. Shorthand delimiters are converted into their corresponding Farben tags, then parsed and rendered normally. This means every macro in the `c*` family supports inline syntax automatically: `cprintln!`, `cprint!`, `cformat!`, `cwrite!`, and all their stderr and bleed variants.

### Supported Syntax

| Delimiter | Produces | Example |
|-----------|----------|---------|
| `*text*` | Bold | `*important*` |
| `_text_` | Underline | `_underlined_` |
| `` `text` `` | Code (bright white on dark grey) | `` `fn main()` `` |
| `~text~` | Strikethrough | `~deprecated~` |
| `/text/` | Italic | `/emphasis/` |

### Examples

```rust
use farben::prelude::*;

cprintln!("This is *bold* and this is /italic/.");
cprintln!("You can _underline_ or ~strikethrough~ text.");
cprintln!("Use `inline code` for monospace snippets.");
```

All of this works with format arguments too:

```rust
use farben::prelude::*;

let name = "Farben";
cprintln!("Welcome to *{name}*!");
```

## Nesting

Inline syntax can be nested inside each other:

```rust
use farben::prelude::*;

cprintln!("*Bold text with /italic inside/*");
cprintln!("`code with *bold* inside`");
```

## Double Characters

To write a literal double character like `**` or `__` without triggering formatting, just type them as-is. The preprocessor treats doubled delimiters as escaped literals:

```rust
use farben::prelude::*;

cprintln!("This is not **bold**, just two asterisks.");
cprintln!("This is not __underlined__, just two underscores.");
```

## Inside Tags

Inline delimiters inside `[...]` brackets are passed through untouched. This means you can use `*`, `_`, `` ` ``, `~`, and `/` inside tag content without escaping:

```rust
use farben::prelude::*;

// The * inside [bg:red] is treated as a literal, not bold
cprintln!("[bg:red fg:white]*not bold*");
```

## Unclosed Delimiters

If a delimiter is opened but never closed, it is rendered as literal text rather than causing an error:

```rust
use farben::prelude::*;

cprintln!("This *is not closed"); // prints literally
```

## Replaces the `markdown` Feature

The `inline` feature is the replacement for the deprecated `markdown` and `markdown-compile` features. Unlike `markdown`, which required dedicated `mdprint!`/`mdprintln!`/`md_fmt!` macros, `inline` integrates directly into the existing `c*` macro family. No extra learning, no extra imports, no separate feature for compile-time processing.

```rust
// Old way (deprecated):
// mdprintln!("**bold** and *italic*");

// New way:
cprintln!("*bold* and /italic/");
```

If you were using `markdown-compile`, the `inline` feature works with the `compile` feature too:

```toml
[dependencies]
farben = { version = "...", features = ["inline", "compile"] }
```

```rust
use farben::prelude::*;

// Compile-time validated, inline syntax supported
cprintln!("*bold* and /italic/ and `code`");
```
