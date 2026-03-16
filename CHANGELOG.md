# Changelog

All notable changes to Farben will be documented here.



## [0.3.0] - 2026-03-16 - farben-macros

### Changed

- `colorb!` — replaced one-line stub doc ("Same as `color!`, but bleeds.") with a full
  doc comment explaining what bleed means, when to use it, and how it differs from
  `color!`. Includes a working example.
- `validate_color!` — removed misleading user-facing example. The macro is internal
  infrastructure used by `color_fmt!`, `cprint!`, and `cprintln!`; the example implied
  direct use was intended. Doc comment now explicitly marks it as internal and directs
  users toward `color!` and `color_fmt!` instead.



## [0.7.0] - 2026-03-16 - farben

### Added

- `colorb` — added missing doc comment explaining bleed behavior and when to use it
  over `color`.
- `color_fmt!` (compile-time variant) — added missing doc comment. Previously the
  runtime variant was documented but the `compile`-feature counterpart had none.

### Changed

- Crate-level doc comment revised: fixed grammar ("uses a markup-language-like syntax
  to your string" → "applies a markup-like syntax to your strings") and capitalized
  "German".

### Fixed

- `cprint!` (compile-time variant) — example referenced unbound variable `message`.
  Added `let message = "I don't know";` to the example so it compiles as a doctest.
- `cprintln!` (compile-time variant) — example referenced unbound variable `result`.
  Added `let result = "We did it!";` to the example so it compiles as a doctest.
- `test_try_color_inline_reset` — strengthened assertion from
  `s.contains("\x1b[0m")` (always true due to trailing reset) to a full equality check
  against the expected output `"\x1b[31mbefore\x1b[0mafter\x1b[0m"`.



## [0.6.0] - 2026-03-16 - farben-core

### Added

- `LexError::InvalidResetTarget` — new error variant returned when a reset tag targets
  something that cannot be reset (e.g. `[//]` or `[/prefix]`). Previously caused a panic.
- `LexError::UnknownStyle` — new error variant returned by `registry::set_prefix` when
  the given style name has not been registered.
- `registry::set_prefix` now returns `Result<(), LexError>` instead of `()`, allowing
  callers to handle unknown style names without panicking.

### Changed

- `ansi::style_to_ansi` promoted from `pub(crate)` to `pub`. Users building on top of
  `farben-core` can now call it directly to convert a `Style` into a combined SGR sequence.
- `ansi::style_to_ansi` — removed `#[allow(unused)]` attribute now that the function is
  part of the public API.
- `registry::prefix!` macro updated to call `.expect()` on the `Result` returned by
  `set_prefix`, preserving the existing panic-on-misuse behavior at the macro callsite
  while keeping the underlying function non-panicking.
- `LexError::InvalidArgumentCount` display message improved from
  `"expected N, got M"` to `"expected N arguments, got M"` for clarity.

### Fixed

- `lexer::parse_part` — replaced `panic!` with `Err(LexError::InvalidResetTarget)` when
  a reset tag targets a `Reset` or `Prefix` node. User-supplied markup can no longer crash
  the process through the `try_color` path.
- `registry::set_prefix` — replaced `panic!` with `Err(LexError::UnknownStyle)` when the
  style name is not found in the registry.
- `errors.rs` — corrected typo in `UnclosedValue` display message:
  "parantheses" → "parentheses".
- `ansi::NamedColor` doc comment — corrected "eight standard ANSI named colors" to
  "sixteen ANSI named colors" (eight standard + eight bright variants).
- `ansi::style_to_ansi` — added a working doctest demonstrating bold + named color output.
- `parser::render` — removed unnecessary `.as_str()` calls on `String` return values from
  `color_to_ansi` and `emphasis_to_ansi`; `push_str` accepts `&str` via `Deref` directly.



## [0.6.0] - 2026-03-16

Global Farben Update

### Added
- Specified resets — `[/bold]`, `[/red]`, `[/italic]` etc. reset only the named style, leaving all other active styles intact
- `TagType::Reset` now takes `Option<Box<TagType>>` — `None` for full reset `[/]`, `Some(tag)` for partial reset
- `parser::render` now maintains an active tag stack, re-emitting surviving styles after a partial reset
- Panics if a partial reset targets `TagType::Reset` or `TagType::Prefix` — both are invalid reset targets

### Changed
- `TagType::Reset` changed from a unit variant to `Reset(Option<Box<TagType>>)`
- All existing `[/]` full reset behavior is preserved via `Reset(None)`

### Fixed
- Fixed a bug where `colorb!()` did not exist when using the `compile` feature



## [0.2.6] - 2026-03-16 - farben/farben-macros

### Added
- Minor dependency update to `farben-core`



## [0.5.1] - 2026-03-16 - farben

### Added
- Minor dependency update to `farben-core`



## [0.2.5] - 2026-03-15 - farben-macros

### Added
- Minor dependency update to `farben-core`



## [0.4.2] - 2026-03-15 - farben-core

### Added
- Bright ansi variants now exist, I guess.



## [0.5.0] - 2026-03-15

Global Farben update due to changes to `farben-core`.

### Added
- Added bugs
- `Style::prefix` field, optional text prepended before the style's ANSI codes when the style is applied
- `TagType::Prefix(String)` variant, carries prefix text through the token pipeline to the renderer
- `set_prefix()`, sets the prefix on an existing registry entry, panics if the style is not found
- `prefix!()` macro, user-facing API for binding a text prefix to a named style
- `style_to_tags()` now emits `TagType::Prefix` as the first tag when a prefix is present
- `parser::render()` now handles `TagType::Prefix` by appending the text directly to output
- `format` default feature, gates logic for `style!()` and `prefix!()`

### Changed
- `style_to_tags()` no longer returns early on `reset` when a prefix is present, prefix is always emitted first
- `style!()` is now gated to the `format` feature



## [0.4.1] - 2026-03-15 - farben-core

### Fixed
- Bug where `prefix!()` interferes with actual color styling



## [0.2.4] - 2026-03-15 - farben-macros

### Added
- Minor dependency update to `farben-core`


## [0.2.3] - 2026-03-15 - farben-macros

### Added
- Minor dependency update to `farben-core`



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



## [0.2.2] - 2026-03-15 - farben-macros

### Added
- Dependency update to `farben_core`
- `colorb!()` macro that bleeds

### Fixed
- `color!()` now auto-resets



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



## [0.3.3] - 2026-03-14b - farben

### Added
- `cprintb!` and `cprintbln!` for color bleeding printing.



## [0.2.1] - 2026-03-14 - farben-macros

### Added
- 100% documentation coverage



## [0.2.1] - 2026-03-14 - farben-core

### Added
- 100% documentation coverage



## [0.3.2] - 2026-03-14 — farben

### Added
- `cprint!()` — prints farben-colored markup to stdout without a newline, behaves like `print!`
- `cprintln!()` — prints farben-colored markup to stdout with a trailing newline, behaves like `println!`
- Both macros support format args and compile-time validation when the `compile` feature is enabled



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



## [0.2.0] - 2026-03-14 — farben-core

### Added
- `bg:` and `fg:` prefix support for color tags — `[bg:red]`, `[fg:white]`, etc.
- `Ground` field added to `TagType::Color` variant — now `TagType::Color { color, ground }`
- Background color support in `encode_color_sgr` and `color_to_ansi`
- New tests for background color parsing, tokenizing, and rendering

### Changed
- `TagType::Color(Color)` restructured to `TagType::Color { color: Color, ground: Ground }`



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
