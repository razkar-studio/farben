import { defineConfig } from "vitepress";

function sharedSidebar() {
  return [
    {
      text: "User Guide",
      link: "/guide/",
      items: [
        { text: "Setting Up Farben", link: "/guide/pre-requisites/" },
        { text: "Getting Started", link: "/guide/getting-started/" },
        { text: "Colors In Depth", link: "/guide/colors-in-depth/" },
        { text: "Purposefully Bleeding", link: "/guide/bleeding/" },
        { text: "Define Your Own Tags", link: "/guide/custom-tags/" },
        { text: "Exclusive Resetting", link: "/guide/specifically-resetting/" },
        { text: "Markdown", link: "/guide/markdown/" },
        { text: "Standard Error", link: "/guide/standard-error/" },
        { text: "Stripping ANSI", link: "/guide/stripping-ansi/" },
        { text: "anstyle Interop", link: "/guide/anstyle/" },
        { text: "Debugging", link: "/guide/debugging/" },
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
      text: "Custom Styles",
      link: "/styles/",
      items: [
        { text: "Styles-toml", link: "/styles/styles-toml/" },
        { text: "Under The Hood", link: "/styles/advanced/" },
      ],
    },
    {
      text: "Examples",
      items: [
        { text: "API Reference", link: "/examples/api-examples/" },
        { text: "Project Examples", link: "/examples/project-examples/" },
      ],
    },
  ];
}

// https://vitepress.dev/reference/site-config
export default defineConfig({
  base: "/farben/",

  title: "Farben",
  description: "Markup for the Terminal",
  themeConfig: {
    logo: "/logo-bracket-f.svg",

    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/" },
      { text: "Changelog", link: "/changelog/" },
      { text: "News", link: "/news/" },
    ],

    sidebar: {
      "/guide/": sharedSidebar(),
      "/conventions/": sharedSidebar(),
      "/styles/": sharedSidebar(),
      "/examples/": sharedSidebar(),
      "/news/": [],
      "/changelog/": [],
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/razkar-studio/farben" },
    ],
  },
});
