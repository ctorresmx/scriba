import { walk } from "$std/fs/walk.ts";
import { ParsedPost, PostAttributes } from "../types/blog.ts";
import { extract } from "@std/front-matter/yaml";
import { getBlogConfig } from "./config.ts";
import { dirname, join, relative } from "$std/path/mod.ts";

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
    const processedContent = processImagePaths(body, filepath);

    return {
      attributes: attrs,
      content: processedContent,
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

function processImagePaths(content: string, filepath: string): string {
  const { postsDir } = getBlogConfig();
  const postDir = dirname(filepath);

  return content.replace(
    /!\[([^\]]*)\]\(([^)]+)\)/g,
    (_match, alt, imagePath) => {
      // Skip absolute URLs (http://, https://, //, etc.)
      if (imagePath.match(/^(https?:\/\/|\/\/|mailto:|tel:|#)/)) {
        return `![${alt}](${imagePath})`;
      }

      // Skip absolute paths (starting with /)
      if (imagePath.startsWith("/")) {
        return `![${alt}](${imagePath})`;
      }

      // Block any path traversal sequences
      if (imagePath.includes("../") || imagePath.includes("..\\")) {
        console.warn(
          `Blocked path traversal attempt in image path: ${imagePath}`,
        );
        return `![${alt}](#blocked-path-traversal)`;
      }

      // Block protocol-based paths and UNC paths
      if (imagePath.match(/^(file:|ftp:|\\\\)/)) {
        console.warn(
          `Blocked potentially unsafe protocol in image path: ${imagePath}`,
        );
        return `![${alt}](#blocked-unsafe-protocol)`;
      }

      // Normalize path - remove leading ./ if present
      const normalizedPath = imagePath.startsWith("./")
        ? imagePath.slice(2)
        : imagePath;

      const fullImagePath = join(postDir, normalizedPath);
      const relativeToPostsDir = relative(postsDir, fullImagePath);

      // Additional check: ensure the resolved path is still within the posts directory
      if (relativeToPostsDir.startsWith("../")) {
        console.warn(
          `Blocked image path outside posts directory: ${imagePath}`,
        );
        return `![${alt}](#blocked-outside-posts)`;
      }

      return `![${alt}](/assets/${relativeToPostsDir})`;
    },
  );
}
