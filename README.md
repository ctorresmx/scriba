# Scriba

A minimalist blogging platform built for people who just want to write, not
wrestle with complex content management systems.

## What is Scriba?

Scriba transforms simple Markdown files into a minimal, fast blog with a clean
aesthetic. Write your posts in your favorite editor, commit them to Git, and
deploy with Docker. No databases, no admin panels, no complexity.

## Key Features

- **Markdown-first**: Write posts in plain Markdown with frontmatter metadata
- **Dynamic images**: Images stored alongside posts, perfect for Docker workflows
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

- **Deno Fresh**: Server-side rendering framework for Deno
- **TailwindCSS + DaisyUI**: Utility-first CSS with component library
- **Markdown + YAML**: Content format with frontmatter metadata
- **Docker**: Containerized deployment

## Quick Start

```bash
# Clone this repository
git clone https://github.com/ctorresmx/scriba.git my-blog
cd my-blog

# Start development server
deno task start
```

Your blog is now running at `http://localhost:8000`. The development server will
watch for file changes and automatically restart.

## Creating Content

Create Markdown files in the `posts/` directory with YAML frontmatter. Posts are
automatically available at `/YYYY/MM/DD/slug` where the slug is generated from
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
| `PORT`              | `8000`      | Server port (Docker and production)           |

### Examples

```bash
# Minimal setup
BLOG_NAME="My Blog"
BLOG_TITLE="My Personal Blog"
BLOG_COPYRIGHT="© 2025 My Name"

# Full customization
BLOG_NAME="Tech Insights"
BLOG_TITLE="Tech Insights - Deep Dives & Tutorials"
BLOG_COPYRIGHT="© 2025 Jane Doe | Powered by Scriba"
BLOG_POSTS_DIR="/content/posts"
BLOG_FAVICON_TEXT="TI"
PORT=3000
```

## Development

```bash
# Development server with hot reload
deno task start

# Build for production
deno task build

# Run production server
deno task preview

# Run tests
deno task test

# Check code quality (format, lint, type check)
deno task check
```

## Deployment

```bash
# Quick Docker deployment
docker run -d -p 8000:8000 -v $(pwd)/posts:/app/posts:ro ghcr.io/ctorresmx/scriba:latest

# Or for local development
deno task build && deno task preview
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
