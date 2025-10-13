# Scriba

A minimalist blogging platform built for people who just want to write, not
wrestle with complex content management systems.

## What is Scriba?

Scriba transforms simple Markdown files into a minimal, fast blog with a clean
aesthetic. Write your posts in your favorite editor, commit them to Git, and
deploy with Docker. No databases, no admin panels, no complexity.

## Key Features

- **Markdown-first**: Write posts in plain Markdown with frontmatter metadata
- **Dynamic images**: Images stored alongside posts, perfect for Docker
  workflows
- **Git-based workflow**: Version control your content alongside your code
- **Docker deployment**: Deploy anywhere containers run
- **Fast & lightweight**: Server-side rendering with minimal JavaScript
- **SEO-friendly**: Semantic URLs and proper metadata handling

## Who is this for?

- People who want a personal blog without the overhead
- Technical writers who prefer Markdown to WYSIWYG editors
- Anyone who values simplicity and performance over complex features
- Teams who want to run multiple blogs from the same codebase

## Philosophy

Scriba believes that blogging software should get out of your way. Your content
lives in simple text files. Your blog deploys like any other application. Your
writing workflow integrates with your existing development tools.

No vendor lock-in. No proprietary formats. Just Markdown, Git, and Docker.

## Technology Stack

- **Rust**: Fast, safe, and reliable systems programming language
- **Axum**: Ergonomic web framework built on Tokio
- **Askama**: Type-safe templating with compile-time checking
- **TailwindCSS**: Utility-first CSS framework
- **Markdown + YAML**: Content format with frontmatter metadata
- **Docker**: Containerized deployment

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later
- [TailwindCSS CLI](https://github.com/tailwindlabs/tailwindcss/releases) (for CSS compilation)

### Install TailwindCSS CLI

You need the standalone TailwindCSS CLI to compile the CSS. Find your OS-specific binary on the [GitHub releases page](https://github.com/tailwindlabs/tailwindcss/releases).

```bash
# Download the TailwindCSS CLI executable (macOS ARM64 example)
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
chmod +x tailwindcss-macos-arm64
mv tailwindcss-macos-arm64 tailwindcss
```

### Get Started

```bash
# Clone this repository
git clone https://github.com/ctorresmx/scriba.git my-blog
cd my-blog

# Build the project (this will compile CSS via build.rs)
cargo build

# Start development server
cargo run
```

Your blog is now running at `http://localhost:8000`. The server will restart
when you make code changes (use `cargo watch -x run` for auto-reload).

## Creating Content

Create Markdown files in the `posts/` directory with YAML frontmatter. Posts are
automatically available at `/{YYYY}/{MM}/{DD}/{slug}` where the slug is generated from
the title.

**→ See [CONTENT.md](./CONTENT.md) for the complete content creation guide**

## Configuration

Customize your blog with environment variables:

| Variable            | Default     | Description                                   |
| ------------------- | ----------- | --------------------------------------------- |
| `BLOG_NAME`         | `"Scriba"`  | Site name displayed in header and footer      |
| `BLOG_TITLE`        | `"Scriba"`  | HTML title tag (appears in browser tab)       |
| `BLOG_COPYRIGHT`    | `"Scriba"`  | Footer copyright text                         |
| `BLOG_POSTS_DIR`    | `"./posts"` | Directory containing your Markdown posts      |
| `BLOG_FAVICON_TEXT` | `"Scr"`     | 2-3 character text for auto-generated favicon |
| `PORT`              | `8000`      | Server port                                   |

### Examples

```bash
# Minimal setup
BLOG_NAME="My Blog" \
BLOG_TITLE="My Personal Blog" \
BLOG_COPYRIGHT="© 2025 My Name" \
cargo run

# Full customization
BLOG_NAME="Tech Insights" \
BLOG_TITLE="Tech Insights - Deep Dives & Tutorials" \
BLOG_COPYRIGHT="© 2025 Jane Doe | Powered by Scriba" \
BLOG_POSTS_DIR="/content/posts" \
BLOG_FAVICON_TEXT="TI" \
PORT=3000 \
cargo run
```

## Development

```bash
# Development server
cargo run

# Build for production
cargo build --release

# Run production build
./target/release/scriba

# Run tests
cargo test

# Check code quality
cargo check                  # Type check
cargo fmt                    # Format code
cargo fmt --check            # Check formatting
cargo clippy                 # Lint code

# Auto-reload on changes (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

## Deployment

```bash
# Quick Docker deployment
docker run -d -p 8000:8000 -v $(pwd)/posts:/app/posts:ro ghcr.io/ctorresmx/scriba:latest

# Or build and run locally
cargo build --release
./target/release/scriba
```

**→ See [DEPLOYMENT.md](./DEPLOYMENT.md) for production Docker setups**

## Troubleshooting

Having issues? Check the [troubleshooting guide](./TROUBLESHOOTING.md) for
solutions to common problems.

## Contributing

Interested in contributing? Check out the
[contributing guide](./CONTRIBUTING.md) to get started.

## License

AGPL v3 License - use it, modify it, share it. If you run a modified version as
a service, you must share your changes.
