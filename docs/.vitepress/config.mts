import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  base: "/farben/",

  title: "farben",
  description: "Terminal Coloring Library",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/" },
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
