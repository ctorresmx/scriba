import { walk } from "$std/fs/walk.ts";
import { ParsedPost, PostAttributes } from "../types/blog.ts";
import { extract } from "@std/front-matter/yaml";
import { getBlogConfig } from "./config.ts";

export async function getAllPosts(): Promise<ParsedPost[]> {
  const posts: ParsedPost[] = [];
  const { postsDir } = getBlogConfig();

  try {
    for await (
      const entry of walk(postsDir, {
        exts: [".md"],
        includeDirs: false,
      })
    ) {
      try {
        const post = await parseMarkdownFile(entry.path);
        posts.push(post);
      } catch (error) {
        console.warn(`Skipping file ${entry.path}: ${error}`);
      }
    }

    return posts.sort((a, b) => {
      return b.attributes.date.getTime() - a.attributes.date.getTime();
    });
  } catch (error) {
    throw Error(`Failed to read posts directory: ${error}`);
  }
}

export async function parseMarkdownFile(filepath: string): Promise<ParsedPost> {
  try {
    const content = await Deno.readTextFile(filepath);
    const { attrs, body } = extract<PostAttributes>(content);
    const slug = generateSlug(attrs.title);
    const url = generateUrl(attrs.date, slug);
    const formattedDate = formatDate(attrs.date);

    return {
      attributes: attrs,
      content: body,
      slug: slug,
      url: url,
      formattedDate: formattedDate,
    };
  } catch (error) {
    throw new Error(`Failed to parse ${filepath}: ${error}`);
  }
}

export function generateSlug(title: string): string {
  return title
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, "") // Remove special characters
    .replace(/\s+/g, "-") // Replace spaces with hyphens
    .replace(/-+/g, "-") // Replace multiple hyphens with single
    .replace(/^-+|-+$/g, "") // Remove leading/trailing hyphens
    .trim();
}

export function generateUrl(date: Date, slug: string): string {
  return `/${formatDate(date)}/${slug}`;
}

export function formatDate(date: Date): string {
  const year = date.getUTCFullYear();
  const month = String(date.getUTCMonth() + 1).padStart(2, "0");
  const day = String(date.getUTCDate()).padStart(2, "0");

  return `${year}/${month}/${day}`;
}
