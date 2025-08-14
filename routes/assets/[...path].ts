import { Handlers } from "$fresh/server.ts";
import { getBlogConfig } from "../../utils/config.ts";
import { join, relative } from "$std/path/mod.ts";

const SUPPORTED_IMAGE_TYPES = {
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".png": "image/png",
  ".gif": "image/gif",
  ".webp": "image/webp",
  ".svg": "image/svg+xml",
} as const;

export const handler: Handlers = {
  async GET(_req, ctx) {
    const { path } = ctx.params;
    const { postsDir } = getBlogConfig();

    // Validate path for security - block path traversal attempts
    if (path.includes("../") || path.includes("..\\")) {
      console.warn(`Blocked path traversal attempt in asset path: ${path}`);
      return new Response("Forbidden", { status: 403 });
    }

    // Block unsafe protocols and UNC paths
    if (path.match(/^(file:|ftp:|\\\\)/)) {
      console.warn(`Blocked unsafe protocol in asset path: ${path}`);
      return new Response("Forbidden", { status: 403 });
    }

    // Construct the full file path
    const filePath = join(postsDir, path);

    // Additional security check: ensure resolved path is within posts directory
    const relativePath = relative(postsDir, filePath);
    if (relativePath.startsWith("../")) {
      console.warn(
        `Blocked asset path outside posts directory: ${path}`,
      );
      return new Response("Forbidden", { status: 403 });
    }

    // Get file extension
    const lastDotIndex = filePath.lastIndexOf(".");
    if (lastDotIndex === -1) {
      return new Response("Unsupported file type", { status: 400 });
    }
    const ext = filePath.slice(lastDotIndex).toLowerCase();

    // Check if it's a supported image type
    if (!SUPPORTED_IMAGE_TYPES[ext as keyof typeof SUPPORTED_IMAGE_TYPES]) {
      return new Response("Unsupported file type", { status: 400 });
    }

    try {
      // Read the file
      const file = await Deno.readFile(filePath);

      // Get MIME type
      const mimeType =
        SUPPORTED_IMAGE_TYPES[ext as keyof typeof SUPPORTED_IMAGE_TYPES];

      // Return the image with appropriate headers
      return new Response(file, {
        headers: {
          "Content-Type": mimeType,
          "Cache-Control": "public, max-age=31536000, immutable", // Cache for 1 year
        },
      });
    } catch (error) {
      if (error instanceof Deno.errors.NotFound) {
        return new Response("Image not found", { status: 404 });
      }

      console.error("Error serving image:", error);
      return new Response("Internal server error", { status: 500 });
    }
  },
};
