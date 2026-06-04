<div align="center">

# farben-macros

### Procedural macros for [Farben](https://github.com/razkar-studio/farben). Completely opt-in.

[![Crates.io Version](https://img.shields.io/crates/v/farben)](https://crates.io/crates/farben)
[![docs.rs](https://img.shields.io/docsrs/farben)](https://docs.rs/farben)
[![License MIT](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE-MIT)
[![License Apache-2.0](https://img.shields.io/crates/l/farben)](https://github.com/razkar-studio/farben/blob/main/LICENSE-APACHE)

</div>

> [!WARNING]
> This crate is not intended for direct use. Enable the `compile` feature on `farben` instead.

## Contents

- `color!` -- parses and validates farben markup at compile time, emitting the final ANSI string as a `FarbenStr` baked into the binary
- `colorb!` -- like `color!` but without a trailing reset
- `validate_color!` -- validates farben markup at compile time, emitting the original string literal unchanged on success
- `cformat!` -- compile-time format string splitting with runtime argument interpolation
- `cformatb!` -- like `cformat!` but without a trailing reset

## License

Licensed under either of [MIT License](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.

Cheers, RazkarStudio.
(c) 2026 RazkarStudio. All rights reserved.
