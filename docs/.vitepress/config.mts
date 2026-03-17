import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  base: "/farben/",

  title: "Farben",
  description: "Markup for the Terminal",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/" },
      { text: "Changelog", link: "/changelog/" },
    ],

    sidebar: [
      // {
      //   text: "Examples",
      //   items: [
      //     { text: "Markdown Examples", link: "/markdown-examples" },
      //     { text: "Runtime API Examples", link: "/api-examples" },
      //   ],
      // },
      {
        text: "User Guide",
        link: "/guide/",
        items: [
          { text: "Setting Up Farben", link: "/guide/pre-requisites/" },
          { text: "Getting Started", link: "/guide/getting-started/" },
          { text: "Colors In Depth", link: "/guide/colors-in-depth/" },
          { text: "Purposefully Bleeding", link: "/guide/bleeding/" },
          { text: "Define Your Own Tags", link: "/guide/custom-tags/" },
          {
            text: "Exclusive Resetting",
            link: "/guide/specifically-resetting/",
          },
          { text: "Markdown", link: "/guide/markdown/" },
        ],
      },
      {
        text: "Conventions",
        link: "/conventions/",
        items: [
          {
            text: "Colors and Printers",
            link: "/conventions/colors-and-printing/",
          },
          {
            text: "Errors and Compilers",
            link: "/conventions/errors-and-compiling/",
          },
          { text: "Tips and Tricks", link: "/conventions/tips-and-tricks/" },
        ],
      },
      {
        text: "Examples",
        items: [
          { text: "API Reference", link: "/examples/api-examples/" },
          { text: "Project Examples", link: "/examples/project-examples/" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/razkar-studio/farben" },
    ],
  },
});
