import { createContentLoader } from "vitepress";

export interface Post {
  title: string;
  date: string;
  url: string;
}

declare const data: Post[];
export { data };

export default createContentLoader("news/*.md", {
  excerpt: false,
  transform(raw): Post[] {
    return raw
      .filter((page) => !page.url.endsWith("/news/"))
      .filter((page) => !page.url.includes("_template"))
      .filter(
        (page) => page.frontmatter.title && page.frontmatter.title !== "News",
      )
      .map((page) => ({
        title: page.frontmatter.title ?? "Untitled",
        date: page.frontmatter.date ?? "",
        url: page.url,
      }))
      .sort((a, b) => (a.date < b.date ? 1 : -1));
  },
});
