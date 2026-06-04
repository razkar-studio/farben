<div align="center">

# farben-core

### The internal core library for [Farben](https://github.com/razkar-studio/farben)

[![Crates.io Version](https://img.shields.io/crates/v/farben)](https://crates.io/crates/farben)
[![docs.rs](https://img.shields.io/docsrs/farben)](https://docs.rs/farben)
[![License MIT](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE-MIT)
[![License Apache-2.0](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE-APACHE)
[![Crates.io Downloads](https://img.shields.io/crates/d/farben)](https://crates.io/crates/farben)

</div>

> [!WARNING]
> This crate is not intended for direct use. Its API is not stable and may change at any time without notice. Use the [`farben`](https://crates.io/crates/farben) crate instead.

## Contents

This crate contains the shared logic used by `farben` and `farben-macros`:

- **Lexer**: tokenizes farben markup strings into a `Vec<Token>`
- **Parser**: renders a token stream into an ANSI escape sequence string
- **ANSI**: color and emphasis encoding via SGR escape codes
- **Errors**: `LexError`, `RegistryError`, and `LexErrorDisplay`
- **Registry**: global named style store for `style!()` and `prefix!()`
- **Strip**: utilities for stripping ANSI, stripping markup tags, and escaping brackets
- **Degrader**: color degradation for terminals without truecolor support
- **Inline**: pre-processor for inline syntax (`*bold*`, `/italic/`, `` `code` ``)
- **Env**: runtime detection of `NO_COLOR`, `FORCE_COLOR`, and TTY status
- **State**: per-thread persistent style stack for bleed support
- **anstyle_conv**: bidirectional conversion with `anstyle` types (requires `anstyle` feature)

## License

Licensed under either of [MIT License](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.

Cheers, RazkarStudio.
(c) 2026 RazkarStudio. All rights reserved.
