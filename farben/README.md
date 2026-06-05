<div align="center">

# Farben

### Markup for the Terminal

[![Crate of the Week](https://img.shields.io/badge/Crate%20of%20The%20Week-648-orange?style=flat-square&logo=rust)](https://this-week-in-rust.org/blog/2026/04/22/this-week-in-rust-648/)
[![Crates.io Version](https://img.shields.io/crates/v/farben?style=flat-square)](https://crates.io/crates/farben)
[![docs.rs](https://img.shields.io/docsrs/farben?style=flat-square)](https://docs.rs/farben)
[![License Apache-2.0](https://img.shields.io/crates/l/farben-core?style=flat-square)](https://github.com/razkar-studio/farben/blob/main/LICENSE-APACHE)
[![License MPL-2.0](https://img.shields.io/crates/l/farben?style=flat-square)](https://github.com/razkar-studio/farben/blob/main/LICENSE-MPL)
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

This workspace contains crates under different licenses.

### Public API crate (`farben`)

The main Farben crate is licensed under the [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

In short:

* You may use this crate commercially and privately.
* You may modify and redistribute it.
* If you modify MPL-covered source files and distribute them, those modified files must remain available under MPL-2.0.
* Your larger project does not need to be open source.

### Internal crates

All `farben-*` internal crates are licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](https://opensource.org/licenses/MIT)

at your option.

In short:

* You may use, modify, redistribute, and sublicense them freely.
* They may be used in open-source or proprietary projects.
* Apache-2.0 additionally provides explicit patent protections.

See the corresponding `LICENSE-*` files for details.


Cheers, RazkarStudio.

Copyright (c) 2026 RazkarStudio. All rights reserved.
