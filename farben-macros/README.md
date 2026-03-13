<div align="center">

# farben-macros

### Procedural macros for [Farben](https://github.com/razkar-studio/farben). Completely opt-in.

</div>

# farben-macros

Procedural macros for [farben](https://github.com/razkar-studio/farben).

> [!WARNING]
> This crate is not intended for direct use. Enable the `compile` feature on `farben` instead.

## Contents

- `color!` — parses and validates farben markup at compile time, emitting the final ANSI string as a string literal baked into the binary
- `validate_color!` — validates farben markup at compile time, emitting the original string literal unchanged on success

## License

This project is protected under the RazkarStudio Permissive License. See the [LICENSE](./LICENSE.md) for more details.

Cheers, RazkarStudio.

© 2026 RazkarStudio. All rights reserved.
