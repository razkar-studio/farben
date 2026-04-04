# Setting Up Farben

Every adventure has a starting point, so does every rabbit hole -- even shallow ones!
In this adventure (or rabbit hole, depends on you), we start at humble beginnings, since Farben is already published on [Crates.io](https://crates.io/crates/farben).

So, you want to color your terminal without touching this abomination: `\x1b[31m`? Farben got you covered.
With markup-like syntax `[bold red]`, you can learn Farben in **seconds**.

## Making A New Project

Let's start by creating your first Rust project using Cargo. Install Cargo somewhere, I don't care where, and run the following command:

```sh
cargo new my-cool-farben-project
cd my-cool-farben-project
```

::: info
Change `my-cool-farben-project` to whatever you want.
:::

You should see a classic Cargo project file structure -- `src/main.rs`, `Cargo.toml`, `Cargo.lock`, and `.gitignore`.
Add whatever you'll need there before moving on.

## Adding Farben

Let's add Farben to your project. Since you're using Cargo, this process should be straightforward and easy,
as easy as running this command:

```sh
cargo add farben
```

Run it, wait, and once it's done, you're set! Alternatively, you can add the version manually to your `Cargo.toml`:

```toml
[dependencies]
farben = "insert.version.here"
```

::: tip
To search the versions, visit the [Crates.io](https://crates.io/crates/farben) page for this project.
:::

To start, change the code inside `main.rs` to the following:

```rust
use farben::prelude::*;

fn main() {
    cprintln!("[bold blue]Hello, [italic cyan]world!");
}
```

## Opting-in to Compiling
Farben has this neat feature called `compile`, which parses your soon-to-be colored text *in compilation*, not runtime.
This makes for **zero runtime overhead**, and **compile-time errors** instead of unexpected runtime panics. 
I'm not saying you *should*, but yeah, you should enable this feature, by running this command instead of `cargo add farben`:

```sh
cargo add farben --features compile
```

### What's the Catch?

There HAS to be a catch, right? All these benefits and no catch? Well, there's not much.
The only change when you enable this feature is that `color()` (a function) becomes `color!()` (a macro). 
But that doesn't matter, since we'll be using `cprintln!()` anyway.

::: info
It also introduces a new dependency -- `farben-macros`, but you don't have to worry about it.
:::

::: details
The thing you should note though: "Zero runtime overhead" only applies to the `color!()` macro, which takes a fixed, non-formatted string. This doesn't mean there's no benefits though, as the other functions you'll be seeing will have *compile-time validation*, which is still a worth-it feature.
:::

::: info
In the newest version of Farben, version 0.4; `cprintln!()`, `cprint!()`, `cprintbln!()`, and `cprintb!()` supports compile-time parsing, that is if you pass in a static string. Huge win for the Rustaceans!
:::

---

Now that you got Farben all set up,
Let's. Get. Coloring!
