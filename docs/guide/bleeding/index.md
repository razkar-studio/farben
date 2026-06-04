# Help, I'm Bleeding!

Sometimes, letting it bleed is just better. In cases where you'll need to bleed color across multiple statements,
whether it be styled descriptions or just for the sake of separating lines, Farben got you covered.

::: details
Bleeding in this context **does not** mean bleeding as in an injury. It means to bleed color to the terminal, 
which is printing a color and not doing a reset.
:::

Farben has several ways you can purposefully bleed, and all of them share the `b` suffix in their names.

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

## `cformatb!()`

Like `cformat!` but without the trailing reset. Useful when building a string whose style should carry forward.

```rust
use farben::prelude::*;

let bleed = cformatb!("[red]This style bleeds ");
let reset = cformat!("into this but not after this.");
let combined = format!("{bleed}{reset}");
```

## `colorb()`

The function version of `cprintb!`. Parses and renders markup without a trailing reset.

```rust
use farben::prelude::*;

let s = colorb("[bold yellow]Warning: ");
// s ends with the yellow ANSI code, no reset
```

## `cwriteb!()` and `cwritebln!()`

Bleed variants of the writer macros. Work with any `Write` implementor.

```rust
use farben::prelude::*;
use std::io::Write;

let mut buffer = Vec::new();
cwriteb!(buffer, "[bold red]Error: ");
cwrite!(buffer, "connection refused[/]"); // inherits bold red
cwritebln!(buffer, "[blue]Additional info: ");
cwrite!(buffer, "server not responding[/]"); // inherits blue
```

## Bleed Variants on Stderr

All stderr macros have bleed variants too: `ceprintb!`, `ceprintbln!`, and `cwriteb!`/`cwritebln!` on stderr writers.

```rust
use farben::prelude::*;

ceprintb!("[bold red]fatal: ");
ceprintln!("something went wrong."); // inherits bold red
```

::: tip
Idiomatic ways to write bleeding text can be seen in [Conventions](../../conventions/colors-and-printing/#idiomatic-ways-to-bleed)
:::
