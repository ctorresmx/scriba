// deno-lint-ignore-file no-explicit-any
import { assertEquals } from "$std/assert/mod.ts";
import { handler } from "./[slug].tsx";

Deno.test("article page - serves published post correctly", async () => {
  const testPostContent = `---
title: "Test Post"
date: 2025-08-14
author: "Test Author"
tags: ["test"]
status: "published"
---

# Test Content

![Test Image](./test.png)`;

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    await Deno.writeTextFile(
      `${tempDir}/2025-08-14-test-post.md`,
      testPostContent,
    );

    const request = new Request("http://localhost:8000/2025/08/14/test-post");
    const ctx = {
      params: {
        year: "2025",
        month: "08",
        day: "14",
        slug: "test-post",
      },
      render: (data: any) => {
        return new Response(JSON.stringify(data), {
          headers: { "Content-Type": "application/json" },
        });
      },
      renderNotFound: () => new Response("Not Found", { status: 404 }),
    };

    const response = await handler.GET!(request, ctx as any);
    assertEquals(response.status, 200);

    const responseData = await response.json();
    assertEquals(responseData.post.attributes.title, "Test Post");
    assertEquals(responseData.post.slug, "test-post");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("article page - returns 404 for non-existent post", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const request = new Request("http://localhost:8000/2025/08/14/nonexistent");
    const ctx = {
      params: {
        year: "2025",
        month: "08",
        day: "14",
        slug: "nonexistent",
      },
      render: (data: any) => new Response(JSON.stringify(data)),
      renderNotFound: () => new Response("Not Found", { status: 404 }),
    };

    const response = await handler.GET!(request, ctx as any);
    assertEquals(response.status, 404);
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("article page - filters out draft posts", async () => {
  const draftPostContent = `---
title: "Draft Post"
date: 2025-08-14
author: "Test Author"
tags: ["test"]
status: "draft"
---

# Draft Content`;

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    await Deno.writeTextFile(
      `${tempDir}/2025-08-14-draft-post.md`,
      draftPostContent,
    );

    const request = new Request("http://localhost:8000/2025/08/14/draft-post");
    const ctx = {
      params: {
        year: "2025",
        month: "08",
        day: "14",
        slug: "draft-post",
      },
      render: (data: any) => new Response(JSON.stringify(data)),
      renderNotFound: () => new Response("Not Found", { status: 404 }),
    };

    const response = await handler.GET!(request, ctx as any);
    assertEquals(response.status, 404);
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("article page - matches correct date format", async () => {
  const postContent = `---
title: "Date Test"
date: 2025-08-14
author: "Test Author"
tags: ["test"]
status: "published"
---

# Content`;

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    await Deno.writeTextFile(`${tempDir}/2025-08-14-date-test.md`, postContent);

    // Test with different date formats to ensure exact matching
    const testCases = [
      {
        year: "2025",
        month: "08",
        day: "14",
        slug: "date-test",
        shouldFind: true,
      },
      {
        year: "2025",
        month: "8",
        day: "14",
        slug: "date-test",
        shouldFind: false,
      }, // single digit month
      {
        year: "2025",
        month: "08",
        day: "4",
        slug: "date-test",
        shouldFind: false,
      }, // single digit day
      {
        year: "25",
        month: "08",
        day: "14",
        slug: "date-test",
        shouldFind: false,
      }, // two digit year
    ];

    for (const testCase of testCases) {
      const request = new Request(
        `http://localhost:8000/${testCase.year}/${testCase.month}/${testCase.day}/${testCase.slug}`,
      );
      const ctx = {
        params: testCase,
        render: (data: any) => new Response(JSON.stringify(data)),
        renderNotFound: () => new Response("Not Found", { status: 404 }),
      };

      const response = await handler.GET!(request, ctx as any);

      if (testCase.shouldFind) {
        assertEquals(response.status, 200);
      } else {
        assertEquals(response.status, 404);
      }
    }
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});
