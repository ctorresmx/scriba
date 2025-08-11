# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

Scriba is a minimalist blogging platform built with Deno Fresh. It transforms
Markdown files into a fast, SEO-friendly blog with server-side rendering. The
architecture follows a file-based approach where blog posts are Markdown files
with YAML frontmatter.

## Development Commands

```bash
# Start development server with hot reload
deno task start

# Build for production
deno task build

# Run production server
deno task preview

# Run tests
deno test --allow-read --allow-write --allow-net --allow-env

# Check code quality (format, lint, type check)
deno task check

# Fresh framework tasks
deno task manifest  # Generate Fresh routes manifest
deno task update    # Update Fresh framework

# Individual checks
deno fmt --check     # Format check
deno lint           # Lint check
deno check **/*.ts  # Type check TypeScript
deno check **/*.tsx # Type check TSX
```

## Architecture

### Core Components

- **Fresh Framework**: Deno-based full-stack web framework with server-side
  rendering
- **Route Structure**: File-based routing with dynamic routes for blog posts
  (`/[year]/[month]/[day]/[slug]`)
- **Content Management**: Markdown files in `/posts/` directory with YAML
  frontmatter
- **Styling**: TailwindCSS with DaisyUI components

### Key Files

- `dev.ts`: Development server entry point
- `main.ts`: Production server entry point
- `fresh.config.ts`: Fresh framework configuration
- `fresh.gen.ts`: Auto-generated Fresh routes manifest
- `utils/parsing.ts`: Core blog functionality - parses Markdown files, generates
  URLs and slugs
- `utils/config.ts`: Blog configuration from environment variables
- `types/blog.ts`: TypeScript interfaces for posts and configuration
- `routes/index.tsx`: Homepage showing published posts
- `routes/[year]/[month]/[day]/[slug].tsx`: Individual post pages

### Components

- `components/Header.tsx`: Site header component
- `components/Footer.tsx`: Site footer component
- `components/ArticleSummary.tsx`: Post summary card for homepage
- `components/AuthorDateDetails.tsx`: Author and date display component

### Post Structure

Posts are Markdown files with YAML frontmatter:

```yaml
---
title: "Post Title"
date: 2025-05-12
author: "Author Name"
tags: ["tag1", "tag2"]
status: "published" | "draft" | "scheduled"
excerpt: "Optional excerpt"
---
```

### URL Generation

Posts are accessible at `/YYYY/MM/DD/slug` where slug is auto-generated from the
title.

## Configuration

Blog configuration is managed through environment variables:

- `BLOG_NAME`: Blog name (default: "Scriba")
- `BLOG_TITLE`: Blog title (default: "Scriba")
- `BLOG_COPYRIGHT`: Copyright text (default: "Scriba")
- `BLOG_POSTS_DIR`: Posts directory path (default: "./posts")
- `BLOG_FAVICON_TEXT`: Favicon text (default: "Scr")
- `PORT`: Server port (default: 8000)

## Testing

Tests are colocated with source files using the `*_test.ts` naming convention.
Run with full permissions for file system and network access.

## Documentation

The project includes comprehensive documentation:

- `README.md`: Project overview, quick start, and basic setup
- `CONTENT.md`: Complete guide for creating and managing blog content
- `DEPLOYMENT.md`: Production deployment with Docker and docker-compose
- `TROUBLESHOOTING.md`: Solutions for common issues and problems
- `CONTRIBUTING.md`: Development setup and contribution guidelines
