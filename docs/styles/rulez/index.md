# The Rulez

Since Farben creates its own "TOML-like parser," while not actually being a TOML parser at all, it has some rulesets different than TOML.
This is the reason why the `.frb` part of the extension exists, it's to signal that "I agree that this file will be following the Farben-TOML ruleset," and will error on breaking it.

## The Farben-TOML-2026-04 Ruleset

Otherwise can be referenced as "FRB-TOML-2026-04-04" "FRB-TOML-2026-04," "FRB-2026-04," "Farben-TOML-2026-04-04," or any similar names.

### Only String Values

Only string values are supported. Values must be wrapped in double quotes.

```toml
# Good
error = "bold red"

# Bad - no quotes, other types
error = bold red
error = true
error = { foo = "bar" }
error = ["foo", "bar"]
```

### Style Values Are Plain Strings

In the config file, style values are stored as plain strings without brackets.

```toml
[styles]
error = "bold red"
```

The config stores `"bold red"`, not `"[bold red]"`. The brackets `[...]` are added by the generated code when calling `farben::Style::parse()`.

#### Exception

When you directly use `farben_build::core::parse()` (the parse function), for example in cases where you want to use Farben's custom theming to power *your* app's custom theming, the config must store as `"[bold red]"`, unless you added the brackets. This is because `core::parse()` does not add the brackets `[...]`.

::: tip
Most of the time, it's better to explicitly use brackets. Get used to it, because you'll need it. Read the section above me for more information.
:::

### No Inline Comments

Comments must be at the start of a line. The parser uses `line.starts_with('#')`, so inline `# comments` become part of the value.

```toml
# Good
error = "bold red"

# Bad - errors
error = "bold red" # This is a comment
```

### Only Two Section Types

Only `[styles]` and `[prefixes]` (and their namespaced variants) are valid. Any other section name will error.

```toml
# Bad
[colors]
red = "bold red"

# Good
[styles]
error = "bold red"
```

### No Quoted Identifiers

Identifiers cannot be quoted. The parser trims the key but doesn't handle quotes.

```toml
# Good
error = "bold red"

# Bad - will error
"error" = "bold red"
```

Directly input the identifier, even with non-identifier characters:

```toml
# Good
my::thing = "yellow"

# Bad
"my::thing" = "yellow"
```

### Duplicate Keys

The parser allows duplicates, but `farben-build` panics at compile time if duplicate keys are found.

```toml
# Will cause compile error
[styles]
error = "bold red"
error = "red"
```

---

That being said, as I develop the parser to be more versatile, you should expect breaking changes to this ruleset.
