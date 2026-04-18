# Changelog

All notable changes to Farben will be documented here.

farben, farben-core, farben-macros, farben-build, farben-md.

## [farben-core 0.13.1 / farben-macros 0.5.2 / farben-build 0.1.1 / farben-md 0.2.2 / farben 0.17.1] - 2026-04-18 - DOCUMENTED!

### Added

- Comprehensive docstrings for all public items across the workspace!

### Fixed

- Fixed bugs

## [farben-core 0.13.0 / farben 0.17.0] - 2026-04-18 - Expand Macro & New SGR Tags
### Added

- `EmphasisType::Overline` (SGR 53), `EmphasisType::Invisible` (SGR 8),
  `EmphasisType::Reverse` (SGR 7), `EmphasisType::RapidBlink` (SGR 6),
  `EmphasisType::DoubleUnderline` (SGR 21) — new emphasis variants with
  corresponding `[overline]`, `[invisible]`, `[reverse]`, `[rapid-blink]`,
  `[double-underline]` tags.
- `debug` module in `farben-core` — `tokens_to_markup(tokens)` reconstructs
  a farben markup string from a parsed token slice, collapsing consecutive
  tags into a single bracket.
- `expand!(markup)` in `farben` — diagnostic macro that prints three lines to
  stderr: the raw input, the fully expanded markup (registry styles resolved
  to their constituent tags), and the rendered ANSI escape string.

### Changed

- `farben-core` dependency in `farben` bumped to `0.13.0`.

## [farben-core 0.12.1 / farben 0.16.3] - 2026-04-18 - Bug Fix
### Fixed

- `tokenize` no longer treats `\x1b[` (CSI ANSI escape sequences embedded in the
  input string) as tag openers. The byte preceding each `[` is now checked; if it is
  `ESC` (`0x1b`), the character is pushed as plain text and the search continues.
  Previously, strings like `"\x1b[31m"` passed directly to `color()` or

## [farben 0.16.2] - 2026-04-13 - Style to ANSI
### Added

- `farben::core` module - exposes `style_to_ansi()` function for converting
  `farben::Style` into ANSI escape sequences.

## [farben 0.16.1] - 2026-04-13 - Bug Fixes
### Fixed

- `anstyle!` macro no longer requires `farben_core` crate as a direct dependency
  for users, the macro now uses `farben::Style` instead.
- Tests no longer fail in non-TTY environments, the parser now respects the
  `FORCE_COLOR` environment variable in all tests.

## [farben-core 0.12.0 / farben 0.16.0] - 2026-04-12 - anstyle Interop
### Added

- `anstyle` feature flag - enables interoperability with the `anstyle` crate.
- `anstyle_conv` module in `farben-core` - provides bidirectional `From`
  implementations for converting between farben types (`Color`, `NamedColor`,
  `Style`) and `anstyle` types (`anstyle::Color`, `anstyle::AnsiColor`,
  `anstyle::Style`).
- `anstyle!` macro in `farben` - parses farben markup and converts the result
  into an `anstyle::Style`. Requires the `anstyle` feature flag.
- `anstyle` crate added as an optional dependency to both `farben-core` and
  `farben`.

## [farben-core 0.11.0 / farben 0.15.0] - 2026-04-12 - Global
### Performance Optimizations

v0.15 is **~1.4x faster on average** across the core pipeline (geometric mean of 7 benchmarks).
ANSI encoding is the biggest winner: named color encoding is **1.9x faster**, RGB encoding **1.7x faster**,
and emphasis sequences **1.6x faster**. The full tokenize-to-render pipeline is **1.35x faster**.
All figures measured against the v0.14 criterion baseline on x86-64 Linux.

> [!IMPORTANT]
> **Breaking changes in farben-core 0.11.0.** See the Breaking Changes section below.

### Breaking Changes (farben-core)

- `TagType::Reset(Option<Box<TagType>>)` has been split into two variants. `TagType::ResetAll`
  replaces `Reset(None)` and `TagType::ResetOne(Box<TagType>)` replaces `Reset(Some(...))`.
  Update all match arms that pattern-match on `TagType::Reset`.
- `Token::Text(String)` is now `Token::Text(Cow<'static, str>)`. Construction sites must use
  `Cow::Owned(string)` for runtime strings or `Cow::Borrowed("literal")` for static string
  literals. Pattern-matching and `push_str` usage are unaffected, since `Cow<'static, str>`
  deref-coerces to `&str`.

### Changed

- `tokenize` pre-allocates the token `Vec` with capacity `input.len() / 4`, reducing
  reallocations for typical markup strings.
- `parse_part` (lexer) consolidates the `ansi(...)` and `rgb(...)` prefix checks into a
  single `strip_prefix` call each, removing a redundant scan of the tag string per call.
- The style registry now stores `Arc<Style>` internally. Each lookup clones an `Arc`
  (one atomic increment) instead of cloning the full `Style` struct.
- `emphasis_to_ansi` bypasses the intermediate `Vec<u8>` and formats the escape sequence
  directly via `format!`, saving a heap allocation per emphasis tag.
- `color_to_ansi` bypasses the intermediate `Vec<u8>` for all color types, formatting the
  escape sequence directly. A new private `named_sgr` const fn maps `NamedColor` to its
  base SGR code without a runtime lookup table.
- `render` pre-allocates the output buffer with capacity `tokens.len() * 16`.
- Release profile: `lto = "thin"`, `codegen-units = 1`, `opt-level = 3` added to root
  `Cargo.toml`, enabling cross-crate inlining in optimized builds.
- `env.rs` unsafe blocks now carry `// SAFETY:` comments documenting the invariants for
  `isatty(1)` (Unix) and the `GetStdHandle` query path (Windows).

### Added

- Criterion benchmark suite in `farben-core/benches/farben_bench.rs` covering: tokenize
  (plain and complex), render, full tokenize + render pipeline, `emphasis_to_ansi`,
  `color_to_ansi` (named and RGB), and registry lookup via tokenize.
- Tests for `render` with color disabled: verify that tag tokens are stripped and only
  `Token::Text` and `Token::Tag(Prefix(...))` content is emitted.
- Edge case tests for `strip_ansi`: empty string, bare `ESC` byte, sequences-only input,
  mixed content, RGB and ANSI256 sequences.

### Performance

Measured against the v0.14 baseline using criterion 0.5.1 on x86-64 Linux (CachyOS, Rust 1.94.1):

|       Benchmark     |   v0.14   |   v0.15   |  Change |
|---------------------|-----------|-----------|---------|
| tokenize plain      | 61.1 ns   | 50.6 ns   | -22%    |
| tokenize complex    | 1.176 µs  | 866.6 ns  | -26%    |
| render              | 188.6 ns  | 182.9 ns  | -2% (within noise) |
| pipeline            | 1.485 µs  | 1.099 µs  | -26%    |
| emphasis_to_ansi    | 91.1 ns   | 56.0 ns   | -37%    |
| color_to_ansi named | 111.6 ns  | 59.5 ns   | -47%    |
| color_to_ansi rgb   | 367.3 ns  | 214.3 ns  | -43%    |
| registry lookup     | 361.3 ns  | 346.6 ns  | -5%     |

---

## [farben-build 0.1.0 / farben-macros 0.5.0 / farben-core 0.10.0 / farben-md 0.2.0 / farben 0.14.0] - 2026-04-05 - Global
### Compile-time Custom Style Support

### Added

- `farben-build` 0.1.0 - new build script helper crate. Call `farben_build::run()` from
  `build.rs` to read `farben.frb.toml` and generate two artifacts in `OUT_DIR`:
  - `farben_styles.rs` - a Rust source file containing an `init_styles()` function that
    registers all styles and prefixes at runtime. Include it via `load_styles!()` at startup.
  - `farben_registry.lsv` - a line-separated values file consumed by `farben-macros` proc
    macros at compile time, so `color!`, `colorb!`, and `validate_color!` can see user-defined
    style tags. Use `farben_build::run_with(&[paths])` to supply custom config file paths
    instead of the default `farben.frb.toml`.
  - Config format: INI-like `[styles]` and `[prefixes]` sections with `key = "value"` pairs.
    Namespaced sections like `[styles.myns]` produce keys of the form `myns:key`.
- **Absolutely zero external dependencies added.**

### Changed

- `farben-macros` bumped to `0.5.0`. Every proc macro invocation now calls `load_registry()`
  at startup, which reads `farben_registry.lsv` from `OUT_DIR` and pre-populates the
  compile-time registry. As a result, `color!("[myTag]text")` now compiles successfully when
  `myTag` was declared in a `.frb.toml` config file.
- `farben-core` bumped to `0.10.0`. No new public API.
- `farben-md` bumped to `0.2.0`, picking up the `farben-core 0.10.0` dependency. No new
  public API.

## [0.9.0] - 2026-04-04 - farben-core

### Added

- Lossy degrading. When the terminal does not support 24-bit true color, RGB values are 
  automatically degraded to the nearest ANSI256 color. When the terminal only supports 
  basic ANSI colors (8/16 colors), RGB and ANSI256 values are degraded to the nearest 
  named color. The degrader module uses the `COLORTERM` and `TERM` environment variables 
  to detect terminal color capabilities at runtime.

## [0.13.0] - 2026-04-04 - farben

### Changed

- `farben-core` dependency bumped to `0.9.0`, picking up lossy degrading support.

## [0.12.0] - 2026-04-04 - farben

### Added

- `cwrite!`, `cwriteln!`, `cwriteb!`, `cwritebln!` - writer variants of the colored print macros.
  Work with any `Write` implementor. Useful for writing to files, `String` buffers, or custom writers.
  All four support the same markup features as the stdout variants (named colors, RGB, ANSI256, emphasis,
  bleeding via the `b` variants).

## [0.8.0] - 2026-04-04 - farben-core

### Added

- `RegistryError` enum - a separate error type for registry operations (`set_prefix`, `insert_style`).
  Split out from `LexError` because registry errors have no source position (they occur outside markup
  parsing). Has one variant: `UnknownStyle(String)`.
- `LexErrorDisplay<'a>` struct - wraps a `&LexError` and the original `&str` input to produce
  compiler-style diagnostic output. Renders two lines: the full input string, then a caret (`^`)
  aligned to the byte offset of the error. Example:
  ```
     | [bold unknown]text
     |       ^^^^^^^ invalid tag: 'unknown'
  ```

### Changed

- All `LexError` variants now carry a `position: usize` field (byte offset into the markup string).
  Affected variants: `UnclosedTag`, `InvalidTag`, `UnclosedValue`, `InvalidArgumentCount`,
  `InvalidValue`, `InvalidResetTarget`. Previously no variants stored position info.
- `LexError::UnknownStyle` removed - registry errors now use `RegistryError::UnknownStyle` instead.
- `LexError`'s `Display` impl now includes position in every message
  (e.g. `"invalid tag 'foo' at position 5"`).

## [0.11.0] - 2026-04-04 - farben

### Added

- `farben::prelude` module - the recommended import path going forward. `use farben::prelude::*`
  brings every user-facing item into scope (functions, macros, types) gated by the same feature
  flags as their definitions. Prefer this over `use farben::*`, which also pulls in
  `color_runtime` and `validate_color` - items that are `pub` only to satisfy macro expansion,
  not intended for direct use.

### Changed

- `farben-core` dependency bumped to `0.8.0`, picking up position-aware `LexError` variants and
  the new `LexErrorDisplay` diagnostic formatter. `try_color` error messages now include the byte
  offset of the offending token.
- All documentation and examples updated to use `use farben::prelude::*`.

## [0.10.0] - 2026-04-04 - farben

### Added

- `ansi_strip!(...)` - macro that accepts `format!`-style arguments, builds the string,
  then strips all CSI ANSI escape sequences from the result. Non-CSI `ESC` bytes pass
  through unchanged. Returns `String`.
- `strip_ansi` re-exported at the `farben` crate root from `farben-core::strip::strip_ansi`,
  making it available via `use farben::*` and as the expansion target for `ansi_strip!`.

### Changed

- `farben-core` dependency bumped to `0.7.0`.

## [0.7.0] - 2026-04-04 - farben-core

### Added

- `env` module: runtime detection of whether ANSI color output should be enabled.
  Respects the `NO_COLOR` and `FORCE_COLOR` environment variable conventions (in that
  order), then falls back to TTY detection. Result is computed once per process and
  cached via `OnceLock`.
  + `color_enabled()`: returns the cached bool for the current process.
  + TTY detection on Unix via `isatty(1)`; on Windows via `GetStdHandle` +
    `GetConsoleMode`; `false` on all other targets.
- `strip` module: utilities for removing ANSI escape sequences from strings.
  + `strip_ansi(input)`: strips CSI sequences (`ESC [ ... <letter>`) from a string
    and returns plain text. Non-CSI `ESC` bytes are passed through unchanged.
    Useful for measuring display width, plain-text logging, or piping to tools that
    do not interpret ANSI codes.

## [0.9.0] - 2026-03-20 - farben

### Added

- `ceprint!`, `ceprintln!`, `ceprintb!`, `ceprintbln!` - stderr variants of the colored print macros.
- `mdeprint!`, `mdeprintln!` - stderr variants of the inline markdown print macros.
- Empty invocation support for all print macros. `cprintln!()` now prints a bare newline,
  `cprint!()` prints nothing - consistent with how `println!()` and `print!()` behave in std.
  Applies to `ceprint!`, `ceprintln!`, `mdprint!`, `mdprintln!`, `mdeprint!`, `mdeprintln!`,
  and all bleed variants.

### Changed

- `src/lib.rs` split into focused modules: `functions.rs`, `macros/color.rs`,
  `macros/format.rs`, `macros/markdown.rs`, `macros/eprint.rs`, and `tests.rs`.
  No public API changes.

## [0.8.3] - 2026-03-20 - farben

### Fixed

- `prefix!` macro no longer requires users to manually add `farben-core` as a dependency.
  The `set_prefix` function is now re-exported through `farben` and the macro expands via `farben::set_prefix`.
- `color_fmt!`, `cprint!`, `cprintln!`, `cprintb!`, and `cprintbln!` (compile feature) no longer require
  `farben-macros` as a direct dependency. `validate_color` is now re-exported through `farben`.

## [0.1.2] - 2026-03-17 - farben-md

### Fixed
- `tokenize_inner` - unclosed delimiters (`**`, `*`, `_`, `__`, `~~`) were
  incorrectly producing styled tokens instead of falling back to plain text.
  The return type of `tokenize_inner` was changed from `Vec<MdToken>` to
  `(Vec<MdToken>, bool)` so callers can distinguish between a found closing
  delimiter and end-of-input exhaustion.
- Added `tokens_to_text` helper to correctly reconstruct plain text from
  partially parsed token trees when a closing delimiter is never found.

### Added
- Full unit test suite for `lexer.rs` and `renderer.rs` covering plain text,
  all six token types, nesting, unclosed delimiters, empty input, mixed
  content, and consecutive spans.

## [0.8.1 / 0.6.3 / 0.4.1 / 0.1.1] - 2026-03-17 - Global

### Changed
- Changed LICENSE to MIT/Apache-2.0

## [0.1.0] - 2026-03-17 - farben-md

### Added
- `tokenize()` - parses inline markdown into a recursive `MdToken` tree.
  Supports `**bold**`, `*italic*`, `_italic_`, `__underline__`, `~~strikethrough~~`,
  and `` `inline code` ``. Unclosed delimiters are treated as plain text.
- `render()` - converts an `MdToken` tree into an ANSI-escaped string with a
  trailing reset. Nested spans are handled via an active style stack that
  re-emits surviving styles after each reset.
- `MdToken` - recursive token enum with `Text(String)` as the only leaf node
  and `Bold`, `Italic`, `Underline`, `Strikethrough` carrying `Vec<MdToken>`,
  and `Code(String)` for literal inline code.



## [0.4.0] - 2026-03-17 - farben-macros

### Added
- `markdown!()` - proc macro that parses and renders inline markdown at compile
  time, emitting the final ANSI-escaped string as a string literal baked into
  the binary. Enabled via the `markdown` feature.



## [0.8.0] - 2026-03-17 - farben

### Added
- `markdown()` - runtime function that parses and renders inline markdown into
  an ANSI-escaped string. Always succeeds. Enabled via the `markdown` feature.
- `md_fmt!()` - renders inline markdown with format arguments. Always runtime.
  Enabled via the `markdown` feature.
- `mdprint!()` - prints inline markdown to stdout without a newline. Runtime
  under `markdown`, compile-time under `markdown-compile`.
- `mdprintln!()` - prints inline markdown to stdout with a trailing newline.
  Runtime under `markdown`, compile-time under `markdown-compile`.

### Changed
- `style!()` and `prefix!()` macros moved from `farben-core` to `farben`,
  under the `format` feature flag.

### Added (features)
- `markdown` feature - enables runtime markdown rendering via `farben-md`.
- `markdown-compile` feature - enables both `markdown` and `compile`, with
  compile-time markdown rendering via `farben-macros`.


## [0.8.0 / 0.4.0 / 0.1.0] - 2026-03-17 - Global

Markdown update. Introduces `farben-md` as a new workspace crate and wires
inline markdown rendering into the full Farben pipeline.

### Added
- `farben-md` - new crate providing inline markdown tokenization and rendering.
  Depends on `farben-core` for ANSI encoding.
- `color_to_ansi()` and `emphasis_to_ansi()` made public in `farben-core`
  0.6.2, enabling `farben-md` to delegate ANSI encoding without reimplementing it.

## [0.6.2] - 2026-03-17 - farben-core

### Changed
- Made `style_to_ansi()` and `emphasis_to_ansi()` public functions

## [0.7.1] - 2026-03-16 - farben

### Added
- `style!()` macro - moved from `farben-core` to `farben`. Defines a named style in the
  global registry. Gated behind the `format` feature.
- `prefix!()` macro - moved from `farben-core` to `farben`. Sets a prefix string on a
  previously defined named style. Gated behind the `format` feature.

## [0.6.1] - 2026-03-16 - farben-core

### Removed
- `style!()` macro - moved to `farben`. Users importing from `farben-core` directly
  should update to use `farben::style!()` instead.
- `prefix!()` macro - moved to `farben`. Users importing from `farben-core` directly
  should update to use `farben::prefix!()` instead.

## [0.3.1] - 2026-03-16 - farben-maros

### Added
- Minor dependency update to `farben-core`

## [0.3.0] - 2026-03-16 - farben-macros

### Changed

- `colorb!` - replaced one-line stub doc ("Same as `color!`, but bleeds.") with a full
  doc comment explaining what bleed means, when to use it, and how it differs from
  `color!`. Includes a working example.
- `validate_color!` - removed misleading user-facing example. The macro is internal
  infrastructure used by `color_fmt!`, `cprint!`, and `cprintln!`; the example implied
  direct use was intended. Doc comment now explicitly marks it as internal and directs
  users toward `color!` and `color_fmt!` instead.



## [0.7.0] - 2026-03-16 - farben

### Added

- `colorb` - added missing doc comment explaining bleed behavior and when to use it
  over `color`.
- `color_fmt!` (compile-time variant) - added missing doc comment. Previously the
  runtime variant was documented but the `compile`-feature counterpart had none.

### Changed

- Crate-level doc comment revised: fixed grammar ("uses a markup-language-like syntax
  to your string" → "applies a markup-like syntax to your strings") and capitalized
  "German".

### Fixed

- `cprint!` (compile-time variant) - example referenced unbound variable `message`.
  Added `let message = "I don't know";` to the example so it compiles as a doctest.
- `cprintln!` (compile-time variant) - example referenced unbound variable `result`.
  Added `let result = "We did it!";` to the example so it compiles as a doctest.
- `test_try_color_inline_reset` - strengthened assertion from
  `s.contains("\x1b[0m")` (always true due to trailing reset) to a full equality check
  against the expected output `"\x1b[31mbefore\x1b[0mafter\x1b[0m"`.



## [0.6.0] - 2026-03-16 - farben-core

### Added

- `LexError::InvalidResetTarget` - new error variant returned when a reset tag targets
  something that cannot be reset (e.g. `[//]` or `[/prefix]`). Previously caused a panic.
- `LexError::UnknownStyle` - new error variant returned by `registry::set_prefix` when
  the given style name has not been registered.
- `registry::set_prefix` now returns `Result<(), LexError>` instead of `()`, allowing
  callers to handle unknown style names without panicking.

### Changed

- `ansi::style_to_ansi` promoted from `pub(crate)` to `pub`. Users building on top of
  `farben-core` can now call it directly to convert a `Style` into a combined SGR sequence.
- `ansi::style_to_ansi` - removed `#[allow(unused)]` attribute now that the function is
  part of the public API.
- `registry::prefix!` macro updated to call `.expect()` on the `Result` returned by
  `set_prefix`, preserving the existing panic-on-misuse behavior at the macro callsite
  while keeping the underlying function non-panicking.
- `LexError::InvalidArgumentCount` display message improved from
  `"expected N, got M"` to `"expected N arguments, got M"` for clarity.

### Fixed

- `lexer::parse_part` - replaced `panic!` with `Err(LexError::InvalidResetTarget)` when
  a reset tag targets a `Reset` or `Prefix` node. User-supplied markup can no longer crash
  the process through the `try_color` path.
- `registry::set_prefix` - replaced `panic!` with `Err(LexError::UnknownStyle)` when the
  style name is not found in the registry.
- `errors.rs` - corrected typo in `UnclosedValue` display message:
  "parantheses" → "parentheses".
- `ansi::NamedColor` doc comment - corrected "eight standard ANSI named colors" to
  "sixteen ANSI named colors" (eight standard + eight bright variants).
- `ansi::style_to_ansi` - added a working doctest demonstrating bold + named color output.
- `parser::render` - removed unnecessary `.as_str()` calls on `String` return values from
  `color_to_ansi` and `emphasis_to_ansi`; `push_str` accepts `&str` via `Deref` directly.



## [0.6.0] - 2026-03-16

Global Farben Update

### Added
- Specified resets - `[/bold]`, `[/red]`, `[/italic]` etc. reset only the named style, leaving all other active styles intact
- `TagType::Reset` now takes `Option<Box<TagType>>` - `None` for full reset `[/]`, `Some(tag)` for partial reset
- `parser::render` now maintains an active tag stack, re-emitting surviving styles after a partial reset
- Panics if a partial reset targets `TagType::Reset` or `TagType::Prefix` - both are invalid reset targets

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
- `Style::parse()` - builds a `Style` from farben markup string
- `Style::reset` field - when `true`, overrides all other style attributes with a full SGR reset
- `registry` module - global style registry backed by `OnceLock<Mutex<HashMap<String, Style>>>`
- `insert_style()` - inserts a named style into the global registry
- `search_registry()` - looks up a named style from the global registry
- `style!()` macro - user-facing API for defining custom named styles
- `style_to_tags()` - converts a `Style` into a `Vec<TagType>` for lexer expansion
- Custom tag resolution in `parse_part()` - unknown tags now check the registry before returning `InvalidTag`
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



## [0.3.0] - 2026-03-15 - farben-core

### Added
- `Style::parse()` - builds a `Style` from farben markup string
- `Style::reset` field - when `true`, overrides all other style attributes with a full SGR reset
- `registry` module - global style registry backed by `OnceLock<Mutex<HashMap<String, Style>>>`
- `insert_style()` - inserts a named style into the global registry
- `search_registry()` - looks up a named style from the global registry
- `style!()` macro - user-facing API for defining custom named styles
- `style_to_tags()` - converts a `Style` into a `Vec<TagType>` for lexer expansion
- Custom tag resolution in `parse_part()` - unknown tags now check the registry before returning `InvalidTag`
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



## [0.3.2] - 2026-03-14 - farben

### Added
- `cprint!()` - prints farben-colored markup to stdout without a newline, behaves like `print!`
- `cprintln!()` - prints farben-colored markup to stdout with a trailing newline, behaves like `println!`
- Both macros support format args and compile-time validation when the `compile` feature is enabled



## [0.3.0] - 2026-03-14 - farben

### Added
- `color!()` - compile-time markup processing via optional `compile` feature flag
- `color_fmt!()` - format args support with compile-time tag validation when `compile` feature is enabled
- `validate_color!()` - proc-macro that validates farben markup at compile time
- `color_runtime()` - internal runtime fallback used by `color_fmt!`
- `bg:` and `fg:` prefix support in color tags - `[bg:red]`, `[fg:white bg:blue]`
- `farben-core` and `farben-macros` as separate workspace crates
- `compile` feature flag for opt-in compile-time processing

### Changed
- Internal logic moved to `farben-core`
- `color()` replaced by `color!` proc-macro when `compile` feature is enabled



## [0.2.0] - 2026-03-14 - farben-core

### Added
- `bg:` and `fg:` prefix support for color tags - `[bg:red]`, `[fg:white]`, etc.
- `Ground` field added to `TagType::Color` variant - now `TagType::Color { color, ground }`
- Background color support in `encode_color_sgr` and `color_to_ansi`
- New tests for background color parsing, tokenizing, and rendering

### Changed
- `TagType::Color(Color)` restructured to `TagType::Color { color: Color, ground: Ground }`



## [0.2.0] - 2026-03-14

### Added
- `color!()` - compile-time markup processing via optional `compile` feature flag
- `color_fmt!()` - format args support with compile-time tag validation when `compile` feature is enabled
- `validate_color!()` - proc-macro that validates farben markup at compile time, emitting the original string literal unchanged on success
- `color_runtime()` - internal runtime fallback used by `color_fmt!`
- `farben-core` - extracted shared logic crate containing lexer, parser, ANSI encoding, and error types
- `farben-macros` - proc-macro crate powering compile-time processing
- Cargo workspace setup with `farben`, `farben-core`, and `farben-macros` as members
- `compile` feature flag - opt-in compile-time processing via `farben-macros`

### Changed
- `color()` is now replaced by the `color!` proc-macro when the `compile` feature is enabled
- Internal logic extracted from `farben` into `farben-core` for shared use across crates



## [0.1.0] - 2026-03-14

### Added
- `color()` - colorizes a string using markup-like syntax, panics on invalid markup
- `try_color()` - same as `color()` but returns `Result<String, LexError>`
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
