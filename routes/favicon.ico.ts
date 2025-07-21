import { Handlers } from "$fresh/server.ts";
import { getBlogConfig } from "../utils/config.ts";

export const handler: Handlers = {
  GET() {
    const { faviconText } = getBlogConfig();

    const svg = `<svg width="32" height="32" xmlns="http://www.w3.org/2000/svg">
      <rect width="32" height="32" fill="#2a323c"/>
      <text x="16" y="20" font-family="monospace" font-size="14" 
        fill="#cdd6f4" text-anchor="middle">${faviconText}</text>
      </svg>`;

    return new Response(svg, {
      headers: {
        "Content-Type": "image/svg+xml",
        "Cache-Control": "public, max-age=86400",
      },
    });
  },
};
