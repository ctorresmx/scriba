import { BlogConfig } from "../types/blog.ts";

export function getBlogHeaderTitle(pageHeader?: string): string {
    const { name } = getBlogConfig();
    if (pageHeader) {
        return `${pageHeader} - ${name}`;
    }
    return `${name}`;
}

export function getBlogConfig(): BlogConfig {
    return {
        name: Deno.env.get("BLOG_NAME") || "Scriba",
        title: Deno.env.get("BLOG_TITLE") || "Scriba",
        copyright: Deno.env.get("BLOG_COPYRIGHT") || "Scriba",
    };
}