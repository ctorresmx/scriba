import { assertEquals, assertRejects } from "$std/assert/mod.ts";
import {
  formatDate,
  generateSlug,
  generateUrl,
  getAllPosts,
  parseMarkdownFile,
} from "./parsing.ts";

Deno.test("generateSlug - basic title conversion", () => {
  assertEquals(generateSlug("Hello World"), "hello-world");
  assertEquals(generateSlug("My First Blog Post"), "my-first-blog-post");
  assertEquals(generateSlug("Testing 123"), "testing-123");
});

Deno.test("generateSlug - special characters removal", () => {
  assertEquals(generateSlug("Hello, World!"), "hello-world");
  assertEquals(generateSlug("C++ Programming"), "c-programming");
  assertEquals(generateSlug("React & TypeScript"), "react-typescript");
  assertEquals(generateSlug("@username: A Story"), "username-a-story");
});

Deno.test("generateSlug - multiple spaces and hyphens", () => {
  assertEquals(generateSlug("Hello    World"), "hello-world");
  assertEquals(generateSlug("Hello -- World"), "hello-world");
  assertEquals(generateSlug("Hello   --   World"), "hello-world");
  assertEquals(generateSlug("---Hello World---"), "hello-world");
});

Deno.test("generateSlug - edge cases", () => {
  assertEquals(generateSlug(""), "");
  assertEquals(generateSlug("   "), "");
  assertEquals(generateSlug("123"), "123");
  assertEquals(generateSlug("---"), "");
});

Deno.test("generateUrl - basic URL generation", () => {
  assertEquals(
    generateUrl(new Date("2025-01-15"), "hello-world"),
    "/2025/01/15/hello-world",
  );
  assertEquals(
    generateUrl(new Date("2025-12-31"), "year-end-post"),
    "/2025/12/31/year-end-post",
  );
  assertEquals(
    generateUrl(new Date("2024-07-04"), "independence-day"),
    "/2024/07/04/independence-day",
  );
});

Deno.test("parseMarkdownFile - valid markdown file", async () => {
  const testContent = `---
title: "Test Post"
date: 2025-01-15
author: "Test Author"
tags: ["test", "markdown"]
status: "published"
---

# Test Content

This is a test post.`;

  const tempFile = await Deno.makeTempFile({ suffix: ".md" });
  await Deno.writeTextFile(tempFile, testContent);

  try {
    const result = await parseMarkdownFile(tempFile);

    assertEquals(result.attributes.title, "Test Post");
    assertEquals(result.attributes.date, new Date("2025-01-15"));
    assertEquals(result.attributes.author, "Test Author");
    assertEquals(result.attributes.tags, ["test", "markdown"]);
    assertEquals(result.attributes.status, "published");
    assertEquals(
      result.content.trim(),
      "# Test Content\n\nThis is a test post.",
    );
    assertEquals(result.slug, "test-post");
    assertEquals(result.url, "/2025/01/15/test-post");
  } finally {
    await Deno.remove(tempFile);
  }
});

Deno.test("parseMarkdownFile - invalid file", async () => {
  await assertRejects(
    () => parseMarkdownFile("/nonexistent/file.md"),
    Error,
    "Failed to parse",
  );
});

Deno.test("parseMarkdownFile - invalid frontmatter", async () => {
  const testContent = `---
invalid yaml content
---

Content here`;

  const tempFile = await Deno.makeTempFile({ suffix: ".md" });
  await Deno.writeTextFile(tempFile, testContent);

  try {
    await assertRejects(
      () => parseMarkdownFile(tempFile),
      Error,
      "Failed to parse",
    );
  } finally {
    await Deno.remove(tempFile);
  }
});

Deno.test("getAllPosts - with test directory", async () => {
  const tempDir = await Deno.makeTempDir();

  const post1Content = `---
title: "First Post"
date: 2025-01-15
author: "Author"
tags: ["test"]
status: "published"
---

First post content`;

  const post2Content = `---
title: "Second Post"
date: 2025-01-20
author: "Author"
tags: ["test"]
status: "published"
---

Second post content`;

  await Deno.writeTextFile(`${tempDir}/post1.md`, post1Content);
  await Deno.writeTextFile(`${tempDir}/post2.md`, post2Content);

  const originalDir = Deno.cwd();

  try {
    Deno.chdir(tempDir);
    await Deno.mkdir("posts");
    await Deno.writeTextFile("posts/post1.md", post1Content);
    await Deno.writeTextFile("posts/post2.md", post2Content);

    const posts = await getAllPosts();

    assertEquals(posts.length, 2);
    assertEquals(posts[0].attributes.title, "Second Post");
    assertEquals(posts[1].attributes.title, "First Post");
  } finally {
    Deno.chdir(originalDir);
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("getAllPosts - missing posts directory", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalDir = Deno.cwd();

  try {
    Deno.chdir(tempDir);

    await assertRejects(
      () => getAllPosts(),
      Error,
      "Failed to read posts directory",
    );
  } finally {
    Deno.chdir(originalDir);
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("getAllPosts - skips invalid files", async () => {
  const tempDir = await Deno.makeTempDir();

  const validContent = `---
title: "Valid Post"
date: 2025-01-15
author: "Author"
tags: ["test"]
status: "published"
---

Valid content`;

  const invalidContent = `---
invalid: yaml: content
---

Invalid content`;

  const originalDir = Deno.cwd();

  try {
    Deno.chdir(tempDir);
    await Deno.mkdir("posts");
    await Deno.writeTextFile("posts/valid.md", validContent);
    await Deno.writeTextFile("posts/invalid.md", invalidContent);

    const posts = await getAllPosts();

    assertEquals(posts.length, 1);
    assertEquals(posts[0].attributes.title, "Valid Post");
  } finally {
    Deno.chdir(originalDir);
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("formatDate - basic date formatting", () => {
  assertEquals(formatDate(new Date("2025-01-15")), "2025/01/15");
  assertEquals(formatDate(new Date("2025-12-31")), "2025/12/31");
  assertEquals(formatDate(new Date("2024-07-04")), "2024/07/04");
});

Deno.test("formatDate - single digit months and days", () => {
  assertEquals(formatDate(new Date("2025-01-01")), "2025/01/01");
  assertEquals(formatDate(new Date("2025-01-09")), "2025/01/09");
  assertEquals(formatDate(new Date("2025-09-01")), "2025/09/01");
  assertEquals(formatDate(new Date("2025-02-05")), "2025/02/05");
});

Deno.test("formatDate - leap year dates", () => {
  assertEquals(formatDate(new Date("2024-02-29")), "2024/02/29");
  assertEquals(formatDate(new Date("2020-02-29")), "2020/02/29");
});

Deno.test("formatDate - edge case dates", () => {
  // New Year's Day
  assertEquals(formatDate(new Date("2025-01-01")), "2025/01/01");
  // New Year's Eve
  assertEquals(formatDate(new Date("2025-12-31")), "2025/12/31");
  // Different centuries
  assertEquals(formatDate(new Date("1999-12-31")), "1999/12/31");
  assertEquals(formatDate(new Date("2000-01-01")), "2000/01/01");
});

Deno.test("formatDate - various years", () => {
  assertEquals(formatDate(new Date("1990-06-15")), "1990/06/15");
  assertEquals(formatDate(new Date("2010-03-20")), "2010/03/20");
  assertEquals(formatDate(new Date("2030-11-08")), "2030/11/08");
});

Deno.test("formatDate - all months", () => {
  assertEquals(formatDate(new Date("2025-01-15")), "2025/01/15");
  assertEquals(formatDate(new Date("2025-02-15")), "2025/02/15");
  assertEquals(formatDate(new Date("2025-03-15")), "2025/03/15");
  assertEquals(formatDate(new Date("2025-04-15")), "2025/04/15");
  assertEquals(formatDate(new Date("2025-05-15")), "2025/05/15");
  assertEquals(formatDate(new Date("2025-06-15")), "2025/06/15");
  assertEquals(formatDate(new Date("2025-07-15")), "2025/07/15");
  assertEquals(formatDate(new Date("2025-08-15")), "2025/08/15");
  assertEquals(formatDate(new Date("2025-09-15")), "2025/09/15");
  assertEquals(formatDate(new Date("2025-10-15")), "2025/10/15");
  assertEquals(formatDate(new Date("2025-11-15")), "2025/11/15");
  assertEquals(formatDate(new Date("2025-12-15")), "2025/12/15");
});

Deno.test("parseMarkdownFile - processes image paths correctly", async () => {
  const testContent = `---
title: "Test Post with Images"
date: 2025-01-15
author: "Test Author"
tags: ["test", "images"]
status: "published"
---

# Test Content

Here's an image: ![Alt text](./subfolder/image.png)

And another: ![Another image](./test.jpg)

Regular link should not change: [Link](./page.html)`;

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const tempFile = `${tempDir}/test-post.md`;
    await Deno.writeTextFile(tempFile, testContent);

    const result = await parseMarkdownFile(tempFile);

    // Check that image paths are transformed
    assertEquals(
      result.content.includes("![Alt text](/assets/subfolder/image.png)"),
      true,
    );
    assertEquals(
      result.content.includes("![Another image](/assets/test.jpg)"),
      true,
    );

    // Check that non-image links are not affected
    assertEquals(result.content.includes("[Link](./page.html)"), true);
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("parseMarkdownFile - handles nested directory image paths", async () => {
  const testContent = `---
title: "Nested Images Test"
date: 2025-01-15
author: "Test Author"
tags: ["test"]
status: "published"
---

![Image](./images/nested/deep.png)`;

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    await Deno.mkdir(`${tempDir}/subdir`, { recursive: true });
    const tempFile = `${tempDir}/subdir/test-post.md`;
    await Deno.writeTextFile(tempFile, testContent);

    const result = await parseMarkdownFile(tempFile);

    // Image path should be relative to posts directory
    assertEquals(
      result.content.includes(
        "![Image](/assets/subdir/images/nested/deep.png)",
      ),
      true,
    );
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("parseMarkdownFile - ignores absolute and external image URLs", async () => {
  const testContent = `---
title: "Mixed Images Test"
date: 2025-01-15
author: "Test Author"
tags: ["test"]
status: "published"
---

![Local](./local.png)
![Absolute](/static/absolute.png)
![External](https://example.com/external.png)
![Root](../root.png)`;

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const tempFile = `${tempDir}/test-post.md`;
    await Deno.writeTextFile(tempFile, testContent);

    const result = await parseMarkdownFile(tempFile);

    // Only relative paths starting with ./ should be transformed
    assertEquals(result.content.includes("![Local](/assets/local.png)"), true);
    assertEquals(
      result.content.includes("![Absolute](/static/absolute.png)"),
      true,
    );
    assertEquals(
      result.content.includes("![External](https://example.com/external.png)"),
      true,
    );
    assertEquals(result.content.includes("![Root](../root.png)"), true);
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("parseMarkdownFile - handles edge cases in image processing", async () => {
  const testContent = `---
title: "Edge Cases Test"
date: 2025-01-15
author: "Test Author"
tags: ["test"]
status: "published"
---

![](./empty-alt.png)
![Complex alt text with "quotes" and special chars!](./complex.png)
![Multiple images](./first.png) and ![inline](./second.png) in same paragraph`;

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const tempFile = `${tempDir}/test-post.md`;
    await Deno.writeTextFile(tempFile, testContent);

    const result = await parseMarkdownFile(tempFile);

    assertEquals(result.content.includes("![](/assets/empty-alt.png)"), true);
    assertEquals(
      result.content.includes(
        '![Complex alt text with "quotes" and special chars!](/assets/complex.png)',
      ),
      true,
    );
    assertEquals(
      result.content.includes("![Multiple images](/assets/first.png)"),
      true,
    );
    assertEquals(
      result.content.includes("![inline](/assets/second.png)"),
      true,
    );
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});
