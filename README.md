<div align="center">

![banner logo](images/farben.png)

### A zero-dependency terminal coloring library using markup-like syntax.

</div>

> [!WARNING]
> Farben, in this current state, is **extremely unstable**. It only has one or two public interface,
> unfinished features, and is experimetal.
> I do not recommend using it in production, at least not yet.

## What Is Farben

Look at the tagline up there ^

## Documentation

> [!NOTE]
> The user guide right now is literally unreadable.

- **User Guide**: [https://razkar-studio.github.io/farben](https://razkar-studio.github.io/farben)
- **API Reference**: [https://docs.rs/farben](https://docs.rs/farben)

## Usage
```rust
use farben::color;

println!("{}", color("[red]I'm red!"));
println!("{}", color("[bold red]I'm bold and red!"));
println!("{}", color("[rgb(255,128,0)]I'm orange![/] Back to normal."));
```

## Syntax

Tags are written as `[tag]` and apply from that point forward. Multiple tags can be combined in a single bracket: `[bold red]`.

> [!WARNING]
> Spaces inside `ansi()` and `rgb()` are not supported at the moment, and it will error.

| Tag | Description |
|-----|-------------|
| `[red]`, `[blue]`, ... | Named colors (black, red, green, yellow, blue, magenta, cyan, white) |
| `[rgb(r,g,b)]` | 24-bit RGB color |
| `[ansi(n)]` | 256-color palette index |
| `[bold]`, `[italic]`, `[dim]`, `[underline]`, `[blink]`, `[strikethrough]` | Emphasis styles |
| `[/]` | Reset all styles |
| `\\[` | Escaped bracket, treated as literal `[` (notice the double escape `\\`) |

## Error Handling

`color()` panics on invalid markup. For graceful error handling, use `try_color()`:
```rust
use farben::try_color;

match try_color("[invalid]oops") {
    Ok(s) => println!("{s}"),
    Err(e) => eprintln!("Error: {e}"),
}
```

## Contributing

Contributions are welcome! Feel free to submit a Pull Request.

## License

This project is protected under the RazkarStudio Permissive License (RSPL). See [LICENSE.md](LICENSE.md) for more details.

Cheers, RazkarStudio.

© 2026 RazkarStudio. All rights reserved.
