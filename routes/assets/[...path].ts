import { Handlers } from "$fresh/server.ts";
import { getBlogConfig } from "../../utils/config.ts";
import { join } from "$std/path/mod.ts";

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

    // Construct the full file path
    const filePath = join(postsDir, path);

    // Get file extension
    const ext = "." + filePath.split(".").pop()?.toLowerCase();

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
