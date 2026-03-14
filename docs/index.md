---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "farben"
  text: "The Official User Guide for Farben"
  tagline: "Farben, a zero-dependency terminal coloring library using markup-like syntax."
  actions:
    - theme: brand
      text: User Guide
      link: /guide/
    - theme: alt
      text: API Examples
      link: /examples/api-examples/
    - theme: alt
      text: Project Examples
      link: /examples/project-examples/

features:
  - title: Zero-dependencies
    details: Farben has absolutely zero runtime required dependencies, only path ones, and it'll be like that, forever.
  - title: Markup-like Syntax
    details: Color your terminal output using intuitive tags like [red], [bold], [rgb(255,0,0)], and chain them like [bold red] or [italic ansi(1)]
  - title: Opt-in Compile-time Processing
    details: Zero runtime overhead, completely opt-in using the library's "compile" feature. Changes nothing but color being a macro.
---
