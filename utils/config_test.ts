import { assertEquals } from "$std/assert/mod.ts";
import { getBlogConfig, getBlogHeaderTitle } from "./config.ts";

Deno.test("getBlogConfig - returns default values when env vars are not set", () => {
  const originalEnv = {
    BLOG_NAME: Deno.env.get("BLOG_NAME"),
    BLOG_TITLE: Deno.env.get("BLOG_TITLE"),
    BLOG_COPYRIGHT: Deno.env.get("BLOG_COPYRIGHT"),
    BLOG_POSTS_DIR: Deno.env.get("BLOG_POSTS_DIR"),
    BLOG_FAVICON_TEXT: Deno.env.get("BLOG_FAVICON_TEXT"),
  };

  try {
    Deno.env.delete("BLOG_NAME");
    Deno.env.delete("BLOG_TITLE");
    Deno.env.delete("BLOG_COPYRIGHT");
    Deno.env.delete("BLOG_POSTS_DIR");
    Deno.env.delete("BLOG_FAVICON_TEXT");

    const config = getBlogConfig();

    assertEquals(config.name, "Scriba");
    assertEquals(config.title, "Scriba");
    assertEquals(config.copyright, "Scriba");
    assertEquals(config.postsDir, "./posts");
    assertEquals(config.faviconText, "Scr");
  } finally {
    if (originalEnv.BLOG_NAME) Deno.env.set("BLOG_NAME", originalEnv.BLOG_NAME);
    if (originalEnv.BLOG_TITLE) {
      Deno.env.set("BLOG_TITLE", originalEnv.BLOG_TITLE);
    }
    if (originalEnv.BLOG_COPYRIGHT) {
      Deno.env.set("BLOG_COPYRIGHT", originalEnv.BLOG_COPYRIGHT);
    }
    if (originalEnv.BLOG_POSTS_DIR) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv.BLOG_POSTS_DIR);
    }
    if (originalEnv.BLOG_FAVICON_TEXT) {
      Deno.env.set("BLOG_FAVICON_TEXT", originalEnv.BLOG_FAVICON_TEXT);
    }
  }
});

Deno.test("getBlogConfig - returns env values when set", () => {
  const originalEnv = {
    BLOG_NAME: Deno.env.get("BLOG_NAME"),
    BLOG_TITLE: Deno.env.get("BLOG_TITLE"),
    BLOG_COPYRIGHT: Deno.env.get("BLOG_COPYRIGHT"),
    BLOG_POSTS_DIR: Deno.env.get("BLOG_POSTS_DIR"),
    BLOG_FAVICON_TEXT: Deno.env.get("BLOG_FAVICON_TEXT"),
  };

  try {
    Deno.env.set("BLOG_NAME", "Test Blog");
    Deno.env.set("BLOG_TITLE", "Test Title");
    Deno.env.set("BLOG_COPYRIGHT", "Test Copyright");
    Deno.env.set("BLOG_POSTS_DIR", "/custom/articles");
    Deno.env.set("BLOG_FAVICON_TEXT", "TB");

    const config = getBlogConfig();

    assertEquals(config.name, "Test Blog");
    assertEquals(config.title, "Test Title");
    assertEquals(config.copyright, "Test Copyright");
    assertEquals(config.postsDir, "/custom/articles");
    assertEquals(config.faviconText, "TB");
  } finally {
    if (originalEnv.BLOG_NAME) {
      Deno.env.set("BLOG_NAME", originalEnv.BLOG_NAME);
    } else {
      Deno.env.delete("BLOG_NAME");
    }
    if (originalEnv.BLOG_TITLE) {
      Deno.env.set("BLOG_TITLE", originalEnv.BLOG_TITLE);
    } else {
      Deno.env.delete("BLOG_TITLE");
    }
    if (originalEnv.BLOG_COPYRIGHT) {
      Deno.env.set("BLOG_COPYRIGHT", originalEnv.BLOG_COPYRIGHT);
    } else {
      Deno.env.delete("BLOG_COPYRIGHT");
    }
    if (originalEnv.BLOG_POSTS_DIR) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv.BLOG_POSTS_DIR);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    if (originalEnv.BLOG_FAVICON_TEXT) {
      Deno.env.set("BLOG_FAVICON_TEXT", originalEnv.BLOG_FAVICON_TEXT);
    } else {
      Deno.env.delete("BLOG_FAVICON_TEXT");
    }
  }
});

Deno.test("getBlogHeaderTitle - returns blog name when no page header provided", () => {
  const originalEnv = Deno.env.get("BLOG_TITLE");

  try {
    Deno.env.set("BLOG_TITLE", "My Blog");

    const title = getBlogHeaderTitle();

    assertEquals(title, "My Blog");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_TITLE", originalEnv);
    } else {
      Deno.env.delete("BLOG_TITLE");
    }
  }
});

Deno.test("getBlogHeaderTitle - returns page header with blog name when page header provided", () => {
  const originalEnv = Deno.env.get("BLOG_TITLE");

  try {
    Deno.env.set("BLOG_TITLE", "My Blog");

    const title = getBlogHeaderTitle("About");

    assertEquals(title, "About - My Blog");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_TITLE", originalEnv);
    } else {
      Deno.env.delete("BLOG_TITLE");
    }
  }
});

Deno.test("getBlogHeaderTitle - uses default blog name when env not set", () => {
  const originalEnv = Deno.env.get("BLOG_NAME");

  try {
    Deno.env.delete("BLOG_NAME");

    const title = getBlogHeaderTitle("Contact");

    assertEquals(title, "Contact - Scriba");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_NAME", originalEnv);
    }
  }
});

Deno.test("getBlogConfig - uses default posts directory when BLOG_POSTS_DIR not set", () => {
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.delete("BLOG_POSTS_DIR");

    const config = getBlogConfig();

    assertEquals(config.postsDir, "./posts");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    }
  }
});

Deno.test("getBlogConfig - uses custom posts directory when BLOG_POSTS_DIR is set", () => {
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", "/var/blog/content");

    const config = getBlogConfig();

    assertEquals(config.postsDir, "/var/blog/content");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
  }
});

Deno.test("getBlogConfig - uses default favicon text when BLOG_FAVICON_TEXT not set", () => {
  const originalEnv = Deno.env.get("BLOG_FAVICON_TEXT");

  try {
    Deno.env.delete("BLOG_FAVICON_TEXT");

    const config = getBlogConfig();

    assertEquals(config.faviconText, "Scr");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_FAVICON_TEXT", originalEnv);
    }
  }
});

Deno.test("getBlogConfig - uses custom favicon text when BLOG_FAVICON_TEXT is set", () => {
  const originalEnv = Deno.env.get("BLOG_FAVICON_TEXT");

  try {
    Deno.env.set("BLOG_FAVICON_TEXT", "MB");

    const config = getBlogConfig();

    assertEquals(config.faviconText, "MB");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_FAVICON_TEXT", originalEnv);
    } else {
      Deno.env.delete("BLOG_FAVICON_TEXT");
    }
  }
});
