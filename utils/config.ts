import { BlogConfig } from "../types/blog.ts";

export function getBlogHeaderTitle(pageHeader?: string): string {
  const { title } = getBlogConfig();
  if (pageHeader) {
    return `${pageHeader} - ${title}`;
  }
  return `${title}`;
}

export function getBlogConfig(): BlogConfig {
  return {
    name: Deno.env.get("BLOG_NAME") || "Scriba",
    title: Deno.env.get("BLOG_TITLE") || "Scriba",
    copyright: Deno.env.get("BLOG_COPYRIGHT") || "Scriba",
    postsDir: Deno.env.get("BLOG_POSTS_DIR") || "./posts",
    faviconText: Deno.env.get("BLOG_FAVICON_TEXT") || "Scr",
  };
}
