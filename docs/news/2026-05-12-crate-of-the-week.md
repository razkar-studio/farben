---
title: "Crate of The Week!"
date: 2026-05-12
---

# Farben is "Crate of the Week" in This Week in Rust #648!

This Week in Rust issue #648 is at *April 22, 2026*. Check out the new sick badge at the README!

---

Well, this one came out of nowhere.

Farben was just named **Crate of the Week** in [This Week in Rust #648](https://this-week-in-rust.org/blog/2026/04/22/this-week-in-rust-648/), one of the most widely-read newsletters in the Rust community. I found out the same way most of you probably did: by reading TWiR like any other week, and then doing a double take.

## The Nomination

I didn't nominate it myself. A community member named [Nik Revenco](https://users.rust-lang.org/u/nik-rev) posted on the Rust Users Forum and made the case for farben better than I probably would have:

> *"This is a highly underrated library that has a very nice syntax for printing to the standard output. Fantastic for scripts."*

He listed out the compile-time feature, the zero-dependency design, the prelude, the markdown macros, the bleed variants, all things I built described back to me by someone who found them useful. That felt really good.

Thank you so, so much, Nik.

## What this means

When I started Farben, it was because I was tired of writing `\x1b[31m` or `.bold().red()` and wanted something that looked like `[bold red]`. That was the whole idea. I didn't expect it to go anywhere in particular.

The star count doubled overnight because of the nomination. People are opening issues, asking questions, trying it out. It turns out the Rust community really does appreciate libraries that stay small, stay focused, and don't drag in half of crates.io as dependencies. I didn't even realize this until just last Sunday!

## What's next

Farben isn't done. The API has been stable this last couple of versions and the path to 1.0 is clearer than ever. There's still work to do: more testing, more polish, more feedback, more features, more speed, more everything.

And if you're here because of TWiR: welcome! Feel free to open an issue, try the `compile` feature, break things, and tell me what you think.

```rust
use farben::prelude::*;

cprintln!("[bold red]<3[/] [bold green]Thanks for the love![/] 🦀");
```

Make great things with Farben.

Cheers, RazkarStudio
