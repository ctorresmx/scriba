// deno-lint-ignore-file no-explicit-any
import { assertEquals } from "$std/assert/mod.ts";
import { handler } from "./[...path].ts";

Deno.test("image asset handler - serves supported image types", async () => {
  const testImages = [
    {
      ext: "jpg",
      content: new Uint8Array([0xFF, 0xD8, 0xFF]),
      mimeType: "image/jpeg",
    },
    {
      ext: "jpeg",
      content: new Uint8Array([0xFF, 0xD8, 0xFF]),
      mimeType: "image/jpeg",
    },
    {
      ext: "png",
      content: new Uint8Array([0x89, 0x50, 0x4E, 0x47]),
      mimeType: "image/png",
    },
    {
      ext: "gif",
      content: new Uint8Array([0x47, 0x49, 0x46, 0x38]),
      mimeType: "image/gif",
    },
    {
      ext: "webp",
      content: new Uint8Array([0x52, 0x49, 0x46, 0x46]),
      mimeType: "image/webp",
    },
    {
      ext: "svg",
      content: new TextEncoder().encode("<svg></svg>"),
      mimeType: "image/svg+xml",
    },
  ];

  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    for (const { ext, content, mimeType } of testImages) {
      const filename = `test.${ext}`;
      await Deno.writeFile(`${tempDir}/${filename}`, content);

      const request = new Request(`http://localhost:8000/assets/${filename}`);
      const ctx = {
        params: { path: filename },
      };

      const response = await handler.GET!(request, ctx as any);

      assertEquals(response.status, 200);
      assertEquals(response.headers.get("Content-Type"), mimeType);
      assertEquals(
        response.headers.get("Cache-Control"),
        "public, max-age=31536000, immutable",
      );

      const responseBody = new Uint8Array(await response.arrayBuffer());
      assertEquals(responseBody, content);
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

Deno.test("image asset handler - rejects unsupported file types", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const unsupportedFiles = ["test.txt", "test.pdf", "test.doc", "test.exe"];

    for (const filename of unsupportedFiles) {
      await Deno.writeTextFile(`${tempDir}/${filename}`, "test content");

      const request = new Request(`http://localhost:8000/assets/${filename}`);
      const ctx = {
        params: { path: filename },
      };

      const response = await handler.GET!(request, ctx as any);

      assertEquals(response.status, 400);
      assertEquals(await response.text(), "Unsupported file type");
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

Deno.test("image asset handler - returns 404 for non-existent files", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const request = new Request("http://localhost:8000/assets/nonexistent.png");
    const ctx = {
      params: { path: "nonexistent.png" },
    };

    const response = await handler.GET!(request, ctx as any);

    assertEquals(response.status, 404);
    assertEquals(await response.text(), "Image not found");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("image asset handler - serves nested path images", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    await Deno.mkdir(`${tempDir}/subfolder`, { recursive: true });
    const imageContent = new Uint8Array([0x89, 0x50, 0x4E, 0x47]);
    await Deno.writeFile(`${tempDir}/subfolder/nested.png`, imageContent);

    const request = new Request(
      "http://localhost:8000/assets/subfolder/nested.png",
    );
    const ctx = {
      params: { path: "subfolder/nested.png" },
    };

    const response = await handler.GET!(request, ctx as any);

    assertEquals(response.status, 200);
    assertEquals(response.headers.get("Content-Type"), "image/png");

    const responseBody = new Uint8Array(await response.arrayBuffer());
    assertEquals(responseBody, imageContent);
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("image asset handler - handles case insensitive extensions", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const imageContent = new Uint8Array([0x89, 0x50, 0x4E, 0x47]);
    await Deno.writeFile(`${tempDir}/test.PNG`, imageContent);

    const request = new Request("http://localhost:8000/assets/test.PNG");
    const ctx = {
      params: { path: "test.PNG" },
    };

    const response = await handler.GET!(request, ctx as any);

    assertEquals(response.status, 200);
    assertEquals(response.headers.get("Content-Type"), "image/png");
  } finally {
    if (originalEnv) {
      Deno.env.set("BLOG_POSTS_DIR", originalEnv);
    } else {
      Deno.env.delete("BLOG_POSTS_DIR");
    }
    await Deno.remove(tempDir, { recursive: true });
  }
});

Deno.test("image asset handler - blocks path traversal attacks", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const maliciousPaths = [
      "../../../etc/passwd",
      "..\\..\\..\\windows\\system32\\config\\sam",
      "../config.json",
      "subfolder/../../../secret.txt",
      "..\\..\\sensitive.png",
    ];

    for (const maliciousPath of maliciousPaths) {
      const request = new Request(
        `http://localhost:8000/assets/${maliciousPath}`,
      );
      const ctx = {
        params: { path: maliciousPath },
      };

      const response = await handler.GET!(request, ctx as any);

      assertEquals(response.status, 403);
      assertEquals(await response.text(), "Forbidden");
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

Deno.test("image asset handler - blocks unsafe protocols", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const unsafePaths = [
      "file:///etc/passwd",
      "ftp://malicious.com/image.png",
      "\\\\server\\share\\image.png",
    ];

    for (const unsafePath of unsafePaths) {
      const request = new Request(`http://localhost:8000/assets/${unsafePath}`);
      const ctx = {
        params: { path: unsafePath },
      };

      const response = await handler.GET!(request, ctx as any);

      assertEquals(response.status, 403);
      assertEquals(await response.text(), "Forbidden");
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

Deno.test("image asset handler - allows legitimate nested paths", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    await Deno.mkdir(`${tempDir}/images/gallery`, { recursive: true });
    const imageContent = new Uint8Array([0x89, 0x50, 0x4E, 0x47]);
    await Deno.writeFile(`${tempDir}/images/gallery/photo.png`, imageContent);

    const legitimatePaths = [
      "images/gallery/photo.png",
      "subfolder/image.jpg",
      "deep/nested/folder/image.png",
    ];

    for (const path of legitimatePaths) {
      // Only test the first path (actual file), others just test path validation
      if (path === "images/gallery/photo.png") {
        const request = new Request(`http://localhost:8000/assets/${path}`);
        const ctx = {
          params: { path },
        };

        const response = await handler.GET!(request, ctx as any);

        assertEquals(response.status, 200);
        assertEquals(response.headers.get("Content-Type"), "image/png");
      } else {
        // For non-existent files, we just want to ensure they don't get blocked
        // by security checks (they should get 404, not 403)
        const request = new Request(`http://localhost:8000/assets/${path}`);
        const ctx = {
          params: { path },
        };

        const response = await handler.GET!(request, ctx as any);

        // Should be 404 (file not found) not 403 (forbidden)
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

Deno.test("image asset handler - rejects files without extensions", async () => {
  const tempDir = await Deno.makeTempDir();
  const originalEnv = Deno.env.get("BLOG_POSTS_DIR");

  try {
    Deno.env.set("BLOG_POSTS_DIR", tempDir);

    const filesWithoutExtensions = ["README", "LICENSE", "dockerfile"];

    for (const filename of filesWithoutExtensions) {
      await Deno.writeTextFile(`${tempDir}/${filename}`, "test content");

      const request = new Request(`http://localhost:8000/assets/${filename}`);
      const ctx = {
        params: { path: filename },
      };

      const response = await handler.GET!(request, ctx as any);

      assertEquals(response.status, 400);
      assertEquals(await response.text(), "Unsupported file type");
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
