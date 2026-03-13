<div align="center">

# farben-core

### The internal core library for [Farben](https://github.com/razkar-studio/farben)

[![Crates.io Version](https://img.shields.io/crates/v/farben)](https://crates.io/crates/farben)
[![docs.rs](https://img.shields.io/docsrs/farben)](https://docs.rs/farben)
[![License](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE)
[![Crates.io Downloads](https://img.shields.io/crates/d/farben)](https://crates.io/crates/farben)
[![GitHub Stars](https://img.shields.io/github/stars/razkar-studio/farben)](https://github.com/razkar-studio/farben/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/razkar-studio/farben)](https://github.com/razkar-studio/farben/issues)
[![GitHub Last Commit](https://img.shields.io/github/last-commit/razkar-studio/farben)](https://github.com/razkar-studio/farben/commits/main)
[![Rust Edition](https://img.shields.io/badge/rust%20edition-2024-orange)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Deps.rs](https://deps.rs/repo/github/razkar-studio/farben/status.svg)](https://deps.rs/repo/github/razkar-studio/farben)
[![Repo Size](https://img.shields.io/github/repo-size/razkar-studio/farben)](https://github.com/razkar-studio/farben)
[![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen)](https://github.com/razkar-studio/farben)

</div>

> [!WARNING]
> This crate is not intended for direct use. Its API is not stable and may change at any time without notice. Use the [`farben`](https://crates.io/crates/farben) crate instead.

## Contents

This crate contains the shared logic used by `farben` and `farben-macros`:

- **Lexer**: tokenizes farben markup strings into a `Vec<Token>`
- **Parser**: renders a token stream into an ANSI escape sequence string
- **ANSI**: color and emphasis encoding via SGR escape codes
- **Errors**: `LexError` and its variants

## License

This project is protected under the RazkarStudio Permissive License. See the  [LICENSE](./LICENSE.md) fosr more details.

Cheers, RazkarStudio.

© 2026 RazkarStudio. All rights reserved.
