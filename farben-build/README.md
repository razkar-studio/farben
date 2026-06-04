<div align="center">

# farben-build

### Build script support for [Farben](https://github.com/razkar-studio/farben)

**Parses `.frb.toml` style files and generates compile-time style registries.**

[![Crates.io Version](https://img.shields.io/crates/v/farben-build)](https://crates.io/crates/farben-build)
[![docs.rs](https://img.shields.io/docsrs/farben-build)](https://docs.rs/farben-build)
[![License MIT](https://img.shields.io/crates/l/farben-build)](https://github.com/razkar-studio/farben/blob/main/LICENSE-MIT)
[![License Apache-2.0](https://img.shields.io/crates/l/farben-build)](https://github.com/razkar-studio/farben/blob/main/LICENSE-APACHE)

</div>

## Usage

Add `farben-build` as a build dependency:

```sh
cargo add farben-build --build
```

Create a `farben.frb.toml` file in your project root:

```toml
[styles]
error = "bold red"
warn  = "bold yellow"
ok    = "bold green"

[prefixes]
error = "[ERROR]"
warn  = "[WARN]"
ok    = "[OK]"
```

Create a `build.rs` file:

```rust
fn main() {
    farben_build::run();
}
```

In your `main.rs`:

```rust
use farben::prelude::*;
farben::load_styles!();

fn main() {
    init_styles();
    cprintln!("[error]Something went wrong.");
    cprintln!("[ok]All good!");
}
```

## Multiple Files

Use `farben_build::run_with(&["theme.frb.toml", "brand.frb.toml"])` to load multiple config files.

## How It Works

`farben-build` generates two artifacts in `OUT_DIR`:

1. `farben_styles.rs` -- a Rust source file with an `init_styles()` function, included via `load_styles!()`
2. `farben_registry.lsv` -- a line-separated value file read by `farben-macros` proc macros at compile time

Both artifacts are produced from the same `.frb.toml` input, allowing custom styles to work with both runtime and compile-time macros.

## License

Licensed under either of [MIT License](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.

RazkarStudio
(c) 2026 RazkarStudio. All rights reserved.
