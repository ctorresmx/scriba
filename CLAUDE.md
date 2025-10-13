# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

Scriba is a minimalist blogging platform built with Rust and Axum. It transforms
Markdown files into a fast, SEO-friendly blog with server-side rendering. The
architecture follows a file-based approach where blog posts are Markdown files
with YAML frontmatter.

## Development Commands

```bash
# Start development server with hot reload
cargo run

# Build for production (release mode)
cargo build --release

# Run release build
./target/release/scriba

# Run tests
cargo test

# Check code quality
cargo check      # Type check
cargo fmt        # Format code
cargo clippy     # Lint code

# Combined check (run all quality checks)
cargo check && cargo fmt --check && cargo clippy -- -D warnings
```

## Architecture

### Core Components

- **Axum Framework**: Rust-based web framework with async support
- **Route Structure**: Dynamic routes for blog posts (`/{year}/{month}/{day}/{slug}`)
- **Content Management**: Markdown files in `/posts/` directory with YAML
  frontmatter
- **Templating**: Askama for type-safe HTML templates
- **Styling**: TailwindCSS for styling

### Key Files

- `src/main.rs`: Application entry point and route configuration
- `src/handlers.rs`: Request handlers for index, post pages, favicon, and 404
- `src/markdown.rs`: Core blog functionality - parses Markdown files, generates
  URLs and slugs
- `src/config.rs`: Blog configuration from environment variables
- `src/models.rs`: Rust structs for posts and configuration
- `src/templates.rs`: Askama template definitions
- `src/assets.rs`: Asset serving for images in posts
- `templates/`: HTML templates (base.html, index.html, post.html, 404.html)
- `static/`: Static assets (CSS files)
- `Cargo.toml`: Rust dependencies and project metadata
- `build.rs`: Build script for TailwindCSS compilation

### Dependencies

- **axum** (0.8.4): Web framework
- **tokio** (1.47.1): Async runtime
- **askama** (0.14.0): Template engine
- **pulldown-cmark** (0.13.0): Markdown parser with GitHub Flavored Markdown
- **gray_matter** (0.3.2): YAML frontmatter parser
- **syntect** (5.2.0): Syntax highlighting
- **tower-http** (0.6.6): HTTP middleware (static file serving)
- **serde** (1.0.219): Serialization/deserialization
- **chrono** (0.4.41): Date/time handling
- **regex** (1.11.2): Regular expressions

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

Posts are accessible at `/{YYYY}/{MM}/{DD}/{slug}` where slug is auto-generated
from the title.

## Configuration

Blog configuration is managed through environment variables:

- `BLOG_NAME`: Blog name (default: "Scriba")
- `BLOG_TITLE`: Blog title (default: "Scriba")
- `BLOG_COPYRIGHT`: Copyright text (default: "Scriba")
- `BLOG_POSTS_DIR`: Posts directory path (default: "./posts")
- `BLOG_FAVICON_TEXT`: Favicon text (default: "Scr")
- `PORT`: Server port (default: "8000")

## Testing

Tests are colocated with source files using `#[cfg(test)]` modules and `#[test]`
attributes. Run with `cargo test`. Tests include unit tests for parsing,
URL generation, markdown processing, and integration tests for handlers.

## Documentation

The project includes comprehensive documentation:

- `README.md`: Project overview, quick start, and basic setup
- `CONTENT.md`: Complete guide for creating and managing blog content
- `DEPLOYMENT.md`: Production deployment with Docker and docker-compose
- `TROUBLESHOOTING.md`: Solutions for common issues and problems
- `CONTRIBUTING.md`: Development setup and contribution guidelines
