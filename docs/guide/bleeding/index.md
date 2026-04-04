# Help, I'm Bleeding!

Sometimes, letting it bleed is just better. In cases where you'll need to bleed color across multiple statement,
whether it be styled descriptions or just for the sake of separating lines, Farben got you covered.

::: details
Bleeding in this context **does not** mean bleeding as in an injury. It means to bleed color to the terminal, 
which is printing a color and not doing a reset.
:::

Farben has 2 ways you can purposefully bleed, and both of them are printers. No, not those printers...

## `cprintb!()`

Prints without a newline and does not reset. It's like `cprint!()`, but Farben doesn't reset the color at the end.

```rust
cprintb!("[red]I'm red! ");
cprintb!("Hey, me too! High five! ");
```

::: info
The code above prints "I'm red! Hey, me too! High five! " all colored red.
:::

## `cprintbln!()`

Prints with newline and bleeds color. Like `cprintln!()`, but bleeds (doesn't append reset).

```rust
cprintbln!("[red]I'm bleeding boss..."); // red
cprintbln!("I don't care. I'm also bleeding!"); // red
cprintln!("Guys, I hope the next guy doesn't bleed."); // red, then reset because cprintln
cprintln!("I'm alive!"); // normal
```

## Writer Variants

If you need to write to something other than stdout or stderr, Farben provides writer variants that work with any `Write` implementor.

### `cwriteb!()` and `cwritebln!()`

These work just like `cprintb!()` and `cprintbln!()` but write to a custom writer.

```rust
use farben::prelude::*;
use std::io::Write;

let mut buffer = Vec::new();
cwriteb!(buffer, "[bold red]Error: ");
cwrite!(buffer, "connection refused[/]"); // inherits bold red
cwritebln!(buffer, "[blue]Additional info: ");
cwrite!(buffer, "server not responding[/]"); // inherits blue
```

::: tip
All writer variants are available: `cwrite!`, `cwriteln!`, `cwriteb!`, and `cwritebln!`. They behave the same as their stdout counterparts but target any `Write` implementor.
:::

::: tip
Idiomatic ways to write bleeding text can be seen in [Conventions](../../conventions/colors-and-printing/#idiomatic-ways-to-bleed)
:::
