<div align="center">

# Farben

### Markup for the Terminal

[![Crate of the Week](https://img.shields.io/badge/Crate%20of%20The%20Week-648-orange?style=flat-square&logo=rust)](https://this-week-in-rust.org/blog/2026/04/22/this-week-in-rust-648/)
[![Crates.io Version](https://img.shields.io/crates/v/farben?style=flat-square)](https://crates.io/crates/farben)
[![docs.rs](https://img.shields.io/docsrs/farben?style=flat-square)](https://docs.rs/farben)
[![License Apache-2.0](https://img.shields.io/crates/l/farben?style=flat-square)](https://github.com/razkar-studio/farben/blob/main/LICENSE-APACHE)
[![Crates.io Downloads](https://img.shields.io/crates/d/farben?style=flat-square)](https://crates.io/crates/farben)
[![GitHub Stars](https://img.shields.io/github/stars/razkar-studio/farben?style=flat-square)](https://github.com/razkar-studio/farben/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/razkar-studio/farben?style=flat-square)](https://github.com/razkar-studio/farben/issues)
[![Rust Edition](https://img.shields.io/badge/rust%20edition-2024-orange?style=flat-square)](https://doc.rust-lang.org/edition-guide/rust-2024/)

</div>

## What Is Farben

Farben is a terminal styling library for Rust that uses markup syntax. Color your terminal without typing whatever the heck `\x1b[31m` is.

```rust
use farben::prelude::*;

cprintln!("[bold red]Error:[/] something went wrong.");
```

## Features

- **Markup-like Syntax**: Easy to read, easy to write.
- **Zero Required Runtime Dependencies**: Only path dependencies.
- **Opt-in Compile-time Processing**: Validate and process markup at compile time via the `compile` feature.
- **8 Color Formats**: Named, ANSI256, RGB, HSL, HSV/HSB, HWB, Lab, LCH, OKLCh, hex.
- **11 Emphasis Styles**: Bold, dim, italic, underline, double-underline, blink, rapid-blink, reverse, invisible, strikethrough, overline.
- **Inline Syntax**: `*bold*`, `/italic/`, `` `code` ``, `~strikethrough~`, `_underline_` via the `inline` feature.
- **Bleed Variants**: Skip trailing reset for chained output.
- **User-defined Styles**: `style!()` and `prefix!()` for custom tags.
- **anstyle Interop**: Convert to/from `anstyle::Style`.

## Documentation

- **User Guide**: [https://razkar-studio.github.io/farben](https://razkar-studio.github.io/farben)
- **API Reference**: [https://docs.rs/farben](https://docs.rs/farben)
- **Changelog**: [CHANGELOG.md](https://github.com/razkar-studio/farben/blob/main/CHANGELOG.md) or [on the guide site](https://razkar-studio.github.io/farben/changelog)

## License

Licensed under either of [MIT License](../LICENSE-MIT) or [Apache License, Version 2.0](../LICENSE-APACHE) at your option.

Cheers, RazkarStudio.
(c) 2026 RazkarStudio.
