---
title: "Public crate now uses MPL-2.0"
date: 2026-06-05
---

# Public crate now uses MPL-2.0

Good day, night, and everything in between to you!

The latest Farben release updates the licensing structure of the workspace.

The public facing crate now uses MPL-2.0, while internal crates remain dual licensed under MIT or Apache-2.0.

The goal here is simple:

- public integration layers should stay open when modified and redistributed
- internal reusable crates should remain easy to adopt anywhere

This does not make larger projects using the crate open source. MPL-2.0 is a file level copyleft license, not a project wide one.

No breaking API changes were introduced as part of this transition. This release also includes a new API interface, the `cstr!` interface! More on the changelog.

Make great things with Farben.

Cheers, RazkarStudio.
