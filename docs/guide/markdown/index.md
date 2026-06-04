# Inline Markdown (Deprecated)

The `markdown` and `markdown-compile` features are **deprecated** since Farben 0.18 and will be removed after 0.19 stable.

**Use the [`inline`](../inline/) feature instead.** It integrates directly into the `c*` macro family (no separate `mdprint!` / `mdprintln!` / `md_fmt!` macros), supports compile-time processing with the `compile` feature, and works the same way across stdout, stderr, and writer variants.

## Migration

```toml
# Before (deprecated)
farben = { version = "...", features = ["markdown"] }
# or
farben = { version = "...", features = ["markdown-compile"] }

# After
farben = { version = "...", features = ["inline"] }
```

Replace `mdprintln!` and related macros with their `c*` equivalents:

```rust
use farben::prelude::*;

// Before (deprecated):
// mdprintln!("**bold** and *italic*");

// After:
cprintln!("*bold* and /italic/");
```

See the [Inline Syntax guide](../inline/) for the full syntax reference.

## Old Syntax Reference

The deprecated markdown feature supported:
- `**bold**`
- `*italic*`, `_italic_`
- `__underline__`
- `~~strikethrough~~`
- `` `inline code` ``

Block-level elements like headings and lists were never supported.

## Why the Change

The `markdown` feature required a separate family of macros (`mdprint!`, `mdprintln!`, `md_fmt!`, etc.) and a separate compile-time code path. The `inline` feature eliminates this duplication by applying inline syntax processing as a transparent pre-processing pass before the standard Farben pipeline. Every existing macro benefits automatically.
