# Changelog

All notable changes to Farben will be documented here.

---

## [0.4.0] - 2026-03-15

Public Farben Update. Changes to `farben-core`, `farben`, and `farben-macros` is displayed here.

### Added
- `Style::parse()` — builds a `Style` from farben markup string
- `Style::reset` field — when `true`, overrides all other style attributes with a full SGR reset
- `registry` module — global style registry backed by `OnceLock<Mutex<HashMap<String, Style>>>`
- `insert_style()` — inserts a named style into the global registry
- `search_registry()` — looks up a named style from the global registry
- `style!()` macro — user-facing API for defining custom named styles
- `style_to_tags()` — converts a `Style` into a `Vec<TagType>` for lexer expansion
- Custom tag resolution in `parse_part()` — unknown tags now check the registry before returning `InvalidTag`
- `parse_part()` return type changed from `Result<TagType, LexError>` to `Result<Vec<TagType>, LexError>` to support style expansion
- `colorb!()` bleeds at compile-time with the `colorb()` runtime counterpart

### Changed
- `parse_tag()` updated to flatten nested `Vec<TagType>` results from `parse_part()`
- All functions, when using `compile` can now benefit from compile-time processing instead of just validation

### Fixed
- `color!()` now auto-resets.

---

## [0.2.2] - 2026-03-15 - farben-macros

### Added
- Dependency update to `farben_core`
- `colorb!()` macro that bleeds

### Fixed
- `color!()` now auto-resets

---

## [0.3.0] - 2026-03-15 — farben-core

### Added
- `Style::parse()` — builds a `Style` from farben markup string
- `Style::reset` field — when `true`, overrides all other style attributes with a full SGR reset
- `registry` module — global style registry backed by `OnceLock<Mutex<HashMap<String, Style>>>`
- `insert_style()` — inserts a named style into the global registry
- `search_registry()` — looks up a named style from the global registry
- `style!()` macro — user-facing API for defining custom named styles
- `style_to_tags()` — converts a `Style` into a `Vec<TagType>` for lexer expansion
- Custom tag resolution in `parse_part()` — unknown tags now check the registry before returning `InvalidTag`
- `parse_part()` return type changed from `Result<TagType, LexError>` to `Result<Vec<TagType>, LexError>` to support style expansion

### Changed
- `parse_tag()` updated to flatten nested `Vec<TagType>` results from `parse_part()`

---

## [0.3.3] - 2026-03-14b - farben

### Added
- `cprintb!` and `cprintbln!` for color bleeding printing.

---

## [0.2.1] - 2026-03-14 - farben-macros

### Added
- 100% documentation coverage

---

## [0.2.1] - 2026-03-14 - farben-core

### Added
- 100% documentation coverage

---

## [0.3.2] - 2026-03-14 — farben

### Added
- `cprint!()` — prints farben-colored markup to stdout without a newline, behaves like `print!`
- `cprintln!()` — prints farben-colored markup to stdout with a trailing newline, behaves like `println!`
- Both macros support format args and compile-time validation when the `compile` feature is enabled

---

## [0.3.0] - 2026-03-14 — farben

### Added
- `color!()` — compile-time markup processing via optional `compile` feature flag
- `color_fmt!()` — format args support with compile-time tag validation when `compile` feature is enabled
- `validate_color!()` — proc-macro that validates farben markup at compile time
- `color_runtime()` — internal runtime fallback used by `color_fmt!`
- `bg:` and `fg:` prefix support in color tags — `[bg:red]`, `[fg:white bg:blue]`
- `farben-core` and `farben-macros` as separate workspace crates
- `compile` feature flag for opt-in compile-time processing

### Changed
- Internal logic moved to `farben-core`
- `color()` replaced by `color!` proc-macro when `compile` feature is enabled

---

## [0.2.0] - 2026-03-14 — farben-core

### Added
- `bg:` and `fg:` prefix support for color tags — `[bg:red]`, `[fg:white]`, etc.
- `Ground` field added to `TagType::Color` variant — now `TagType::Color { color, ground }`
- Background color support in `encode_color_sgr` and `color_to_ansi`
- New tests for background color parsing, tokenizing, and rendering

### Changed
- `TagType::Color(Color)` restructured to `TagType::Color { color: Color, ground: Ground }`

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
