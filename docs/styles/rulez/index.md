# The Rulez

Since Farben creates its own "TOML-like parser," while not actually being a TOML parser at all, it has some rulesets different than TOML.
This is the reason why the `.frb` part of the extension exists, it's to signal that "I agree that this file will be following the Farben-TOML ruleset," and will error on breaking it.

The following is the serious spec:

# FarbenTOML/2026-04

**Full name:** FarbenTOML/2026-04

**Short aliases:** FRB-TOML-2026-04, FRB-2026-04, Farben-TOML-2026-04

**File extension:** `.frb.toml`

**Status:** Active, subject to breaking changes as the parser evolves

## Overview

FarbenTOML is a strict subset of TOML syntax with additional constraints imposed by Farben's config parser. It is not a TOML parser and does not aim for TOML compliance. The `.frb` segment of the file extension signals that the file follows this ruleset rather than standard TOML. A file that passes a TOML parser may still be invalid FarbenTOML, and a conforming FarbenTOML file may not be valid in all TOML parsers.

This document defines the FarbenTOML/2026-04 ruleset. All `.frb.toml` files are expected to conform to it.

## Rules

### R1: String-only values

Only string values are permitted. Every value must be wrapped in double quotes. No other TOML value types are accepted, including booleans, integers, floats, arrays, and inline tables.

```toml
# Valid
error = "bold red"

# Invalid
error = bold red
error = true
error = 42
error = { fg = "red" }
error = ["bold", "red"]
```

**Rationale:** The parser reads values as raw strings and passes them directly to the Farben rendering pipeline. Non-string types have no defined meaning in this context.

### R2: Style values are stored without brackets

Under `[styles]`, values are stored as plain markup strings without the surrounding `[...]` brackets. The brackets are added by the consumer.

```toml
[styles]
error = "bold red"
```

This is stored as `"bold red"`, not `"[bold red]"`. When `farben-build` processes this entry, it wraps the value in brackets before passing it to `farben::Style::parse()`.

#### Exception: direct use of `farben_build::core::parse()`

When calling `farben_build::core::parse()` directly (for example, to power your own application's theming system), the brackets are not added automatically. In this case, values must include the brackets explicitly:

```toml
[styles]
error = "[bold red]"
```

::: info
It is generally safer to always write values with explicit brackets and adjust the consumer accordingly. This avoids ambiguity when switching between `farben-build` codegen and direct `core::parse()` usage.
:::

### R3: No inline comments

Comments must occupy their own line and must start with `#` as the first character. The parser uses a `line.starts_with('#')` check, so any `#` that appears after a value is treated as part of the value string, not as a comment delimiter.

```toml
# Valid
# This is a comment
error = "bold red"

# Invalid: the comment text becomes part of the value
error = "bold red" # applies to errors
```

Inline comments produce a parse error. The # appearing after a value causes the parser to reject the line entirely. Comments must be on their own line with # as the first character.

### R4: Only `[styles]` and `[prefixes]` sections are valid

The only permitted section headers are `[styles]` and `[prefixes]`, and their namespaced variants (e.g. `[styles.myapp]` if supported by a given consumer). Any other section name causes a parse error.

```toml
# Valid
[styles]
error = "bold red"

[prefixes]
error = "!! "

# Invalid
[colors]
red = "bold red"

[meta]
author = "razkar"
```

### R5: Keys must not be quoted

Key identifiers must be written bare. The parser trims whitespace around keys but does not strip surrounding quotes. A quoted key is treated as a literal string including the quote characters, which will not match any valid identifier.

```toml
# Valid
error = "bold red"
my::namespace.key = "yellow"

# Invalid
"error" = "bold red"
"my::namespace.key" = "yellow"
```

Keys may contain characters that are not valid TOML bare keys (such as `:`). This is intentional and specific to FarbenTOML.

### R6: Duplicate keys are a compile-time error

The parser itself does not reject duplicate keys at parse time, but `farben-build` panics at compile time if the same key appears more than once within a section. The last value does not silently win as it would in some TOML implementations.

```toml
# Causes a compile-time panic via farben-build
[styles]
error = "bold red"
error = "red bold"
```

If you are using `core::parse()` directly without `farben-build`, duplicate key handling is the caller's responsibility. The returned data structure reflects the last-seen value; no error is raised.

## Conformance

A file conforms to FarbenTOML/2026-04 if and only if it satisfies all six rules above. Conformance is context-dependent for R2: the bracket convention depends on whether the file is consumed by `farben-build` codegen or by a direct `core::parse()` caller.

## Stability

FarbenTOML/2026-04 is the active revision as of April 2026. The ruleset is expected to evolve as the parser becomes more capable. Future revisions may relax or replace individual rules, and will be issued under a new revision identifier. Files conforming to an older revision are not guaranteed to conform to future ones.
