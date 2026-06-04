# Custom Styles

Farben's built-in tags cover a lot of ground, but sometimes you need your own. This section covers the style file system
powered by `farben-build`, which lets you define custom tags in a `.frb.toml` file and have them work with both runtime
and compile-time macros.

- [Styles-toml](/styles/styles-toml/) - creating and loading style files
- [.frb.toml's Ruleset](/styles/rulez/) - the FarbenTOML specification
- [Under The Hood](/styles/advanced/) - how the build pipeline works

## Quick Comparison

| Approach | When to use | Feature flag |
|----------|-------------|--------------|
| `style!()` / `prefix!()` | Simple runtime aliases, no build script needed | `format` (default) |
| `.frb.toml` + `farben-build` | Shared styles across projects, compile-time resolution | `format` or `compile` |
