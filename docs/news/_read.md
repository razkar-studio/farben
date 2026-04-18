# Writing a news post

1. Copy `_template.md` to `YYYY-MM-DD-some-slug.md` in this folder.
2. Edit the frontmatter `title` and `date`. Write your post body below.
3. Commit and push. The post appears at `/news/YYYY-MM-DD-some-slug` and
   on the `/news/` index automatically.

## Naming convention

`YYYY-MM-DD-slug.md` — the date prefix keeps the folder sortable and
makes URLs self-documenting. The slug should be hyphenated lowercase
words, e.g. `2026-04-18-bleed-persistence-fix.md`.

## What goes in here?

Anything that doesn't fit in the changelog. Release announcements with
context, design rationale ("why we did X"), tutorials, roadmap updates,
"farben at $thing" event posts, retrospectives. The changelog answers
"what changed"; news answers "why and how it felt to change it."

## Excluding a draft

Prefix the filename with `_` (e.g. `_draft-thing.md`) and the data loader
will skip it. Same trick the template file uses.
