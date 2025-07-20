import { assertEquals } from "$std/assert/mod.ts";
import { getBlogConfig, getBlogHeaderTitle } from "./config.ts";

Deno.test("getBlogConfig - returns default values when env vars are not set", () => {
  const originalEnv = {
    BLOG_NAME: Deno.env.get("BLOG_NAME"),
    BLOG_TITLE: Deno.env.get("BLOG_TITLE"),
    BLOG_COPYRIGHT: Deno.env.get("BLOG_COPYRIGHT"),
  };

  try {
    Deno.env.delete("BLOG_NAME");
    Deno.env.delete("BLOG_TITLE");
    Deno.env.delete("BLOG_COPYRIGHT");

    const config = getBlogConfig();

    assertEquals(config.name, "Scriba");
    assertEquals(config.title, "Scriba");
    assertEquals(config.copyright, "Scriba");
  } finally {
    if (originalEnv.BLOG_NAME) Deno.env.set("BLOG_NAME", originalEnv.BLOG_NAME);
    if (originalEnv.BLOG_TITLE) {
      Deno.env.set("BLOG_TITLE", originalEnv.BLOG_TITLE);
    }
    if (originalEnv.BLOG_COPYRIGHT) {
      Deno.env.set("BLOG_COPYRIGHT", originalEnv.BLOG_COPYRIGHT);
    }
  }
});

Deno.test("getBlogConfig - returns env values when set", () => {
  const originalEnv = {
    BLOG_NAME: Deno.env.get("BLOG_NAME"),
    BLOG_TITLE: Deno.env.get("BLOG_TITLE"),
    BLOG_COPYRIGHT: Deno.env.get("BLOG_COPYRIGHT"),
  };

  try {
    Deno.env.set("BLOG_NAME", "Test Blog");
    Deno.env.set("BLOG_TITLE", "Test Title");
    Deno.env.set("BLOG_COPYRIGHT", "Test Copyright");

    const config = getBlogConfig();

    assertEquals(config.name, "Test Blog");
    assertEquals(config.title, "Test Title");
    assertEquals(config.copyright, "Test Copyright");
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
