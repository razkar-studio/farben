---
title: "Thuli, the official Farben REPL"
date: 2026-04-19
---

# Thuli, the official Farben REPL

Farben now has an interactive REPL! Introducing Thuli, the interactive REPL I've been building, is now ready for use. It provides a quiet, focused space to type Farben markup and see the rendered output in real time.

## What it does

Thuli is a small interactive shell, similar to `python` or `irb`, but for Farben markup. You can:

- Type markup and see the rendered terminal output instantly
- Define custom styles on the fly with `/style`
- Inspect what the parser produces with `/show`
- Load themes from `.frb.toml` files
- Use one-shot mode from the command line

## Install

```bash
cargo install thuli
```

Or fetch from [Codeberg](https://codeberg.org/razkar/thuli).

## Commands

- `/help` — show all commands
- `/quit` — exit the REPL
- `/tags` — list all available tags
- `/style <name> <markup>` — register a new style
- `/show <markup>` — render and show token/ansi output
- `/theme` — show theme config status
- `/load <path>` — load styles from a file

## Why Thuli?

The name comes from Zulu, meaning "quiet" or "peace". A calm space for experimenting with markup, without noise.

```
$ thuli
thuli 1.0.0  ·  the official farben repl, previewing frbVER

>>> [bold red]Error:[/] something broke
    Error: something broke
>>> /quit
make great things with farben <3

$
```

Read more in the [Thuli README](https://codeberg.org/razkar/thuli).

Thuli is truly one of the most fun projects to make yet, and it couldn't be possible without Farben itself. Thank you all for supporting Farben!

Make great things with Farben.

Cheers, RazkarStudio.
