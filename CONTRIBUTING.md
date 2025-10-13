# Contributing to Scriba

Thank you for your interest in contributing to Scriba! This guide will help you
get started.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later
- [NPM](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) (for CSS processing)
- Git

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/scriba.git
   cd scriba
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Start the development server**
   ```bash
   cargo run
   ```

   The blog will be available at `http://localhost:8000`.

4. **Run tests**
   ```bash
   cargo test
   ```

5. **Check code quality**
   ```bash
   cargo check               # Type check
   cargo fmt                 # Format code
   cargo clippy              # Lint code
   ```

## Development Workflow

### Code Style

- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Run `cargo clippy` to catch issues
- **Type checking**: Run `cargo check` to verify compilation
- **All checks**: Use `cargo check && cargo fmt --check && cargo clippy -- -D warnings` to run everything

### Testing

- Tests are colocated with source files using `#[cfg(test)]` modules
- Run with `cargo test`
- Write tests for new functionality and bug fixes
- Maintain existing test coverage

### Project Structure

```
scriba/
├── src/
│   ├── main.rs          # Application entry point
│   ├── handlers.rs      # Request handlers
│   ├── markdown.rs      # Markdown parsing and post management
│   ├── config.rs        # Configuration management
│   ├── models.rs        # Data models
│   ├── templates.rs     # Template definitions
│   └── assets.rs        # Asset serving
├── templates/           # Askama HTML templates
├── static/              # Static assets (CSS)
├── posts/               # Example blog posts
├── Cargo.toml           # Rust dependencies
└── build.rs             # Build script
```

### Key Areas

- **`src/markdown.rs`**: Core blog functionality - parses Markdown files and
  generates URLs
- **`src/config.rs`**: Environment variable handling and blog configuration
- **`src/models.rs`**: Rust structs for posts and configuration
- **`src/handlers.rs`**: Axum request handlers for routes
- **`templates/`**: Askama templates for HTML generation

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
   cargo test
   cargo check
   cargo clippy
   cargo fmt
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

- [ ] Tests pass (`cargo test`)
- [ ] Code compiles (`cargo check`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] No clippy warnings (`cargo clippy`)
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

### Rust

- Use idiomatic Rust patterns
- Leverage the type system - avoid `unwrap()` in production code
- Use `Result` and `Option` properly
- Add documentation comments for public APIs

### Axum Handlers

- Keep handlers focused and composable
- Use extractors for request data
- Return proper HTTP status codes
- Handle errors gracefully

### File Naming

- Use snake_case for Rust files: `my_module.rs`
- Place tests in `#[cfg(test)]` modules within the same file
- Use descriptive names that reflect purpose

### Environment Variables

- Add new env vars to `src/config.rs`
- Provide sensible defaults
- Document in README.md configuration section

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function("input");
        assert_eq!(result, "expected");
    }
}
```

### Integration Tests

- Test the full parsing pipeline
- Verify URL generation
- Test environment variable handling

### Async Tests

```rust
#[tokio::test]
async fn test_async_handler() {
    let result = my_async_function().await;
    assert!(result.is_ok());
}
```

### What to Test

- Core parsing functionality
- URL slug generation
- Configuration handling
- Error handling and edge cases
- Handler responses and status codes

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
- Prefer well-maintained crates
- Document reasons for adding new dependencies
- Use specific versions in Cargo.toml

### Performance

- Server-side render everything possible
- Minimize client-side JavaScript
- Optimize for fast page loads
- Consider the impact on build times

## Release Process

(For maintainers)

1. Update version numbers in Cargo.toml
2. Update CHANGELOG.md
3. Create release notes
4. Tag the release
5. Build and push Docker image

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
Scriba better!
