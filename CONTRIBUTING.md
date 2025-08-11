# Contributing to Scriba

Thank you for your interest in contributing to Scriba! This guide will help you
get started.

## Getting Started

### Prerequisites

- [Deno](https://deno.land/) 2.4.2 or later
- Git

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/scriba.git
   cd scriba
   ```

2. **Start the development server**
   ```bash
   deno task start
   ```

   The blog will be available at `http://localhost:8000` with hot reload
   enabled.

3. **Run tests**
   ```bash
   deno task test
   ```

4. **Check code quality**
   ```bash
   deno task check  # Runs format, lint, and type checking
   ```

## Development Workflow

### Code Style

- **Formatting**: Run `deno fmt` before committing
- **Linting**: Run `deno lint` to catch issues
- **Type checking**: Run `deno check **/*.ts **/*.tsx`
- **All checks**: Use `deno task check` to run everything

### Testing

- Tests are colocated with source files using `*_test.ts` naming
- Run with full permissions:
  `deno test --allow-read --allow-write --allow-net --allow-env`
- Write tests for new functionality and bug fixes
- Maintain existing test coverage

### Project Structure

```
scriba/
├── components/          # Preact components
├── routes/             # Fresh file-based routes
├── utils/              # Core utilities (parsing, config)
├── types/              # TypeScript type definitions
├── static/             # Static assets
├── posts/              # Example blog posts
└── islands/            # Client-side interactive components
```

### Key Areas

- **`utils/parsing.ts`**: Core blog functionality - parses Markdown files and
  generates URLs
- **`utils/config.ts`**: Environment variable handling and blog configuration
- **`types/blog.ts`**: TypeScript interfaces for posts and configuration
- **`routes/`**: Fresh framework routes, including the dynamic post route
- **`components/`**: Reusable UI components

## Making Changes

### Before You Start

1. **Check existing issues** to see if your idea is already being discussed
2. **Open an issue** for new features or significant changes
3. **Keep changes focused** - one feature or fix per PR

### Pull Request Process

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow existing code patterns
   - Add tests for new functionality
   - Update documentation if needed

3. **Test thoroughly**
   ```bash
   deno task test
   deno task check
   ```

4. **Commit with clear messages**
   ```bash
   git add .
   git commit -m "Add feature: brief description of what you added"
   ```

5. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

   Then create a pull request on GitHub.

### PR Requirements

- [ ] Tests pass (`deno task test`)
- [ ] Code quality checks pass (`deno task check`)
- [ ] Clear commit messages
- [ ] Documentation updated if needed
- [ ] PR description explains the changes

## Types of Contributions

### Bug Fixes

- Include steps to reproduce the issue
- Add a test case if possible
- Reference the issue number in your PR

### New Features

- Discuss in an issue first for significant features
- Maintain backward compatibility
- Add tests and documentation
- Keep the minimalist philosophy of Scriba

### Documentation

- Fix typos or improve clarity
- Add examples or use cases
- Update guides for new features

### Examples

- Add sample posts showing new features
- Create deployment examples
- Document best practices

## Code Guidelines

### TypeScript

- Use strict typing - avoid `any`
- Define interfaces for data structures
- Export types from `types/blog.ts`

### Fresh/Preact Components

- Use functional components with JSX
- Follow existing component patterns
- Keep components focused and reusable

### File Naming

- Use kebab-case for files: `my-component.tsx`
- Use `*_test.ts` for test files
- Use descriptive names that reflect purpose

### Environment Variables

- Add new env vars to `utils/config.ts`
- Provide sensible defaults
- Document in README.md configuration section

## Testing Guidelines

### Unit Tests

```typescript
import { assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { myFunction } from "./my-module.ts";

Deno.test("myFunction should do X", () => {
  const result = myFunction("input");
  assertEquals(result, "expected");
});
```

### Integration Tests

- Test the full parsing pipeline
- Verify URL generation
- Test environment variable handling

### What to Test

- Core parsing functionality
- URL slug generation
- Configuration handling
- Error handling and edge cases

## Architecture Decisions

### Philosophy

Scriba values **simplicity over complexity**:

- File-based content management
- Minimal configuration
- Server-side rendering
- Docker-first deployment
- Git-based workflow

### Dependencies

- Keep dependencies minimal
- Prefer Deno standard library
- Document reasons for adding new dependencies
- Use specific versions (no `latest`)

### Performance

- Server-side render everything possible
- Minimize client-side JavaScript
- Optimize for fast page loads
- Consider the impact on build times

## Release Process

(For maintainers)

1. Update version numbers
2. Update CHANGELOG.md
3. Create release notes
4. Tag the release
5. Update Docker image

## Getting Help

- **Issues**: Ask questions or report bugs
- **Discussions**: General discussion and feature ideas
- **Email**: For sensitive issues

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions. This is a
learning-friendly project where questions and beginner contributions are
welcome.

## Recognition

All contributors will be recognized in the project. Thank you for helping make
Scriba better! 🚀
