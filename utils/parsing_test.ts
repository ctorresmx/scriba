import { assertEquals, assertRejects } from "$std/assert/mod.ts";
import { getAllPosts, parseMarkdownFile, generateSlug, generateUrl } from "./parsing.ts";

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
  assertEquals(generateUrl(new Date("2025-01-15"), "hello-world"), "/2025/01/15/hello-world");
  assertEquals(generateUrl(new Date("2025-12-31"), "year-end-post"), "/2025/12/31/year-end-post");
  assertEquals(generateUrl(new Date("2024-07-04"), "independence-day"), "/2024/07/04/independence-day");
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
    assertEquals(result.content.trim(), "# Test Content\n\nThis is a test post.");
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
    "Failed to parse"
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
      "Failed to parse"
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
      "Failed to read posts directory"
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
