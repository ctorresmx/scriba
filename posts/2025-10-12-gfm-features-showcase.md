---
title: "GitHub Flavored Markdown Features Showcase"
date: 2025-10-12
author: "Scriba Team"
tags: ["markdown", "gfm", "features", "documentation"]
status: "published"
excerpt: "A comprehensive demonstration of all GitHub Flavored Markdown features supported by Scriba, including tables, strikethrough text, task lists, and automatic URL linking."
---

# GitHub Flavored Markdown Features Showcase

This post demonstrates all the GitHub Flavored Markdown (GFM) features that Scriba now supports!

## Tables

Tables are perfect for displaying structured data:

| Feature | Status | Priority |
|---------|--------|----------|
| Tables | ✅ Supported | High |
| Strikethrough | ✅ Supported | High |
| Task Lists | ✅ Supported | High |
| Autolinks | ✅ Supported | Medium |

### Table with Alignment

You can also control column alignment:

| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| Left text | Center text | Right text |
| More left | More center | More right |
| Data 1 | Data 2 | Data 3 |

### Complex Table Example

Here's a table with inline formatting:

| Function | Description | Example |
|----------|-------------|---------|
| `parse_markdown()` | Parses markdown to HTML | Returns `String` |
| **Bold Function** | Does something bold | Very important |
| ~~deprecated()~~ | Old function | Don't use this |

## Strikethrough Text

Use strikethrough to indicate corrections or outdated information:

- The meeting is on ~~Monday~~ Tuesday at 3 PM
- ~~Deno~~ Rust is our primary technology for this project
- Price: ~~$99.99~~ $79.99 (Limited time offer!)

You can combine strikethrough with other formatting:

- **Important:** ~~Old information~~ New information
- *Note:* ~~Outdated~~ Updated content

## Task Lists

Track your progress with interactive checkboxes:

### Project Status

- [x] Set up development environment
- [x] Implement basic markdown parsing
- [x] Add syntax highlighting
- [x] Add GFM table support
- [x] Add strikethrough support
- [x] Add task list support
- [ ] Deploy to production
- [ ] Write comprehensive documentation

### Nested Task Lists

You can also create nested task lists:

- [x] Backend Development
  - [x] Set up Rust project
  - [x] Implement markdown parser
  - [x] Add routing
  - [ ] Performance optimization
- [ ] Frontend Development
  - [x] Create basic templates
  - [ ] Add interactive features
  - [ ] Responsive design improvements

### Shopping List Example

- [ ] Groceries
  - [x] Milk
  - [x] Eggs
  - [ ] Bread
  - [ ] Butter
- [x] Office supplies
  - [x] Paper
  - [x] Pens
- [ ] Other items

## Automatic URL Linking

Links in angle brackets are automatically converted to clickable links:

- Project repository: <https://github.com/example/scriba>
- Documentation: <https://docs.example.com>
- Email: <contact@example.com>

## Combining Features

All these features work together seamlessly:

| Task | Status | Notes |
|------|--------|-------|
| Write tests | ✅ Done | ~~Pending~~ Completed |
| Review code | 🔄 In Progress | See <https://github.com/example/pr/123> |
| Deploy | ⏳ Waiting | - [x] Staging<br>- [ ] Production |

## Code Examples

Of course, all the existing features like syntax highlighting still work:

```rust
fn main() {
    println!("Hello, GFM!");

    // Tables, strikethrough, and task lists
    // are now fully supported!
}
```

```typescript
interface GFMFeatures {
    tables: boolean;
    strikethrough: boolean;
    taskLists: boolean;
    autolinks: boolean;
}

const features: GFMFeatures = {
    tables: true,
    strikethrough: true,
    taskLists: true,
    autolinks: true
};
```

## Summary

With these GFM features, Scriba now provides a complete and modern Markdown experience:

- ✅ **Tables** - Perfect for structured data
- ✅ **Strikethrough** - Show corrections and changes
- ✅ **Task Lists** - Track progress with checkboxes
- ✅ **Autolinks** - Automatic URL linking in angle brackets

Happy blogging with Scriba! 🚀
