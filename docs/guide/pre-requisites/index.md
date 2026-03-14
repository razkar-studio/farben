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
use farben::*;

fn main() {
    println!("{}")
}
```
