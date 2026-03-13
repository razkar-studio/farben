# Changelog

Notable changes to Farben will be documented here.

---

## [0.2.0] - 2026-03-14

### Added
- `color!()` — compile-time markup processing via optional `compile` feature flag
- `color_fmt!()` — format args support with compile-time tag validation when `compile` feature is enabled
- `validate_color!()` — proc-macro that validates farben markup at compile time, emitting the original string literal unchanged on success
- `color_runtime()` — internal runtime fallback used by `color_fmt!`
- `farben-core` — extracted shared logic crate containing lexer, parser, ANSI encoding, and error types
- `farben-macros` — proc-macro crate powering compile-time processing
- Cargo workspace setup with `farben`, `farben-core`, and `farben-macros` as members
- `compile` feature flag — opt-in compile-time processing via `farben-macros`

### Changed
- `color()` is now replaced by the `color!` proc-macro when the `compile` feature is enabled
- Internal logic extracted from `farben` into `farben-core` for shared use across crates

---

## [0.1.0] - 2026-03-14

### Added
- `color()` — colorizes a string using markup-like syntax, panics on invalid markup
- `try_color()` — same as `color()` but returns `Result<String, LexError>`
- Named color tags: `[black]`, `[red]`, `[green]`, `[yellow]`, `[blue]`, `[magenta]`, `[cyan]`, `[white]`
- 256-color palette support via `[ansi(n)]`
- 24-bit RGB support via `[rgb(r,g,b)]`
- Emphasis tags: `[bold]`, `[dim]`, `[italic]`, `[underline]`, `[blink]`, `[strikethrough]`
- Multi-tag brackets: `[bold red]`, `[italic rgb(255,0,0)]`
- Reset tag `[/]` to clear all active styles
- Escape sequence `\[` to treat `[` as a literal character
- `LexError` with variants `UnclosedTag`, `InvalidTag`, `InvalidValue`, `InvalidArgumentCount`
- Foreground and background color support via `Ground` enum
- Automatic reset appended to all `color()` and `try_color()` output
