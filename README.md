# Scriba

A minimalist blogging platform built for developers who want to write, not wrestle with complex content management systems.

## What is Scriba?

Scriba transforms simple Markdown files into a minimal, fast blog with a clean aesthetic. Write your posts in your favorite editor, commit them to Git, and deploy with Docker. No databases, no admin panels, no complexity.

## Key Features

- **Markdown-first**: Write posts in plain Markdown with frontmatter metadata
- **Git-based workflow**: Version control your content alongside your code
- **Docker deployment**: Deploy anywhere containers run
- **Fast & lightweight**: Server-side rendering with minimal JavaScript
- **SEO-friendly**: Semantic URLs and proper metadata handling

## Who is this for?

- Developers who want a personal blog without the overhead
- Technical writers who prefer Markdown to WYSIWYG editors
- Anyone who values simplicity and performance over complex features
- Teams who want to run multiple blogs from the same codebase

## Philosophy

Scriba believes that blogging software should get out of your way. Your content lives in simple text files. Your blog deploys like any other application. Your writing workflow integrates with your existing development tools.

No vendor lock-in. No proprietary formats. Just Markdown, Git, and Docker.

## Quick Start

```bash
# Clone this repository
git clone https://github.com/ctorresmx/scriba.git my-blog
cd my-blog

# Create your content
# ... add your posts and configuration

# Deploy
deno task start
```

Your blog is now running at `http://localhost:8000`. The development server will watch for file changes and automatically restart.

## License
AGPL v3 License - use it, modify it, share it. If you run a modified version as a service, you must share your changes.
