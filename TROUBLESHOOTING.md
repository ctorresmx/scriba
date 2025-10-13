# Troubleshooting Guide

Common issues and solutions when setting up and running your Scriba blog.

## Posts Not Appearing

### Post doesn't show up on the website

**Most common causes:**

1. **Invalid YAML frontmatter syntax**
   ```bash
   # Check server console for warnings
   cargo run
   # Look for error messages during post parsing
   ```

   **Fix:** Validate your frontmatter syntax
   ```yaml
   ---
   title: "Your Post Title"    # Use quotes for titles with special characters
   date: 2025-01-15           # Use YYYY-MM-DD format
   author: "Your Name"
   tags: ["tag1", "tag2"]     # Array format
   status: "published"        # Must be "published", "draft", or "scheduled"
   ---
   ```

2. **Missing required fields**

   Every post needs: `title`, `date`, `author`, `tags`, `status`

3. **Incorrect date format**
   ```yaml
   # Wrong
   date: 2025/01/15
   date: January 15, 2025
   date: 2025-1-5

   # Correct
   date: 2025-01-15
   ```

4. **Post status is "draft"**

   Draft posts won't appear on the site. Change `status: "draft"` to
   `status: "published"`

### Posts appear in wrong order

**Cause:** Incorrect date format or inconsistent dating

**Fix:** Ensure all posts use `YYYY-MM-DD` format consistently

## Development Server Issues

### "Address already in use" error

```
Error: Os { code: 48, kind: AddrInUse, message: "Address already in use" }
```

**Fix:**

```bash
# Find what's using port 8000
lsof -i :8000

# Kill the process or use a different port
PORT=8001 cargo run
```

### "Permission denied" reading posts

**Fix:**

```bash
# Check and fix directory permissions
ls -la ./posts
chmod -R 755 posts
```

### Posts directory not found

**Fix:**

```bash
# Create the posts directory
mkdir posts

# Or set custom directory
BLOG_POSTS_DIR="./my-posts" cargo run
```

### Compilation errors

**Fix:**

```bash
# Clean build artifacts and rebuild
cargo clean
cargo build

# Update dependencies
cargo update

# Check for specific errors
cargo check
```

## Docker Issues

### Empty blog (no posts visible)

**Cause:** Posts directory not mounted properly

**Fix:** Check your `docker-compose.yml`:

```yaml
services:
  scriba:
    volumes:
      - ./posts:/app/posts:ro # This line must be present
```

### Container shows as "unhealthy"

**Diagnosis:**

```bash
# Check container logs
docker-compose logs scriba

# Test health check manually
docker exec -it container_name curl -f http://localhost:8000/
```

**Fix:**

- Ensure posts directory exists and has content
- Verify PORT environment variable is set to 8000
- Check if application is running on 0.0.0.0:8000 (not localhost)

### Permission denied in Docker

**Cause:** Host/container user ID mismatch

**Fix:**

```bash
# Fix host directory permissions
sudo chown -R 1001:1001 ./posts
```

### Docker build fails

**Fix:**

```bash
# Clear cache and rebuild
docker-compose build --no-cache

# Check Dockerfile syntax
docker build -t scriba .
```

## Configuration Issues

### Blog shows default "Scriba" branding

**Cause:** Environment variables not set

**Fix:**

```bash
# Set environment variables
export BLOG_NAME="Your Blog Name"
export BLOG_TITLE="Your Blog Title"
export BLOG_COPYRIGHT="© 2025 Your Name"

# Or create .env file
echo 'BLOG_NAME="Your Blog Name"' > .env

# Run with environment variables
BLOG_NAME="Your Blog Name" cargo run
```

### Custom posts directory not working

**Fix:**

```bash
# Use absolute path in production
BLOG_POSTS_DIR="/app/posts"

# Use relative path in development
BLOG_POSTS_DIR="./my-posts"
```

## Build Issues

### TailwindCSS compilation fails

**Cause:** TailwindCSS CLI not found or not executable

**Fix:**

```bash
# Ensure tailwindcss binary is in the project root and executable
chmod +x tailwindcss
ls -la tailwindcss

# Or download it again
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
chmod +x tailwindcss-macos-arm64
mv tailwindcss-macos-arm64 tailwindcss
```

### Rust version too old

```
error: package `scriba` cannot be compiled with Rust 1.xx.x
```

**Fix:**

```bash
# Update Rust
rustup update

# Check version
rustc --version  # Should be 1.70 or later
```

## URL and Routing Issues

### 404 errors for valid posts

**Causes:**

1. **Post not published**

   Check post status is `"published"` not `"draft"`

2. **Special characters in title**

   Titles like `"C++ Tips"` become URLs like `/2025/01/15/c-tips` (special chars removed)

3. **Date mismatch**

   Ensure the URL matches the date in the post's frontmatter

## Common YAML Mistakes

```yaml
# ❌ Wrong
title: Hello, World! # Missing quotes
date: 2025/1/5 # Wrong format
author: John Doe # Missing quotes
tags: tech, programming # Wrong format
status: publish # Wrong value

# ✅ Correct
title: "Hello, World!" # Quoted
date: 2025-01-05 # ISO format
author: "John Doe" # Quoted
tags: ["tech", "programming"] # Array
status: "published" # Correct value
```

## Quick Diagnosis

When something goes wrong:

1. **Check server console** for error messages and warnings
2. **Test with minimal post** to isolate complex content issues
3. **Verify file permissions** on posts directory
4. **Validate YAML** using an online YAML validator
5. **Check Docker logs** if using containers
6. **Try cargo clean && cargo build** to rebuild from scratch
7. **Run cargo test** to ensure core functionality works

## Rust-Specific Issues

### Cargo dependency resolution fails

**Fix:**

```bash
# Update Cargo.lock
cargo update

# Or delete and regenerate
rm Cargo.lock
cargo build
```

### Out of memory during compilation

**Fix:**

```bash
# Use fewer parallel jobs
cargo build -j 2

# Or build in release mode (less memory for debug symbols)
cargo build --release
```

## Still Having Issues?

1. **Validate your post** against the working examples in the `posts/` directory
2. **Check the console output** when starting the server - it shows parsing
   errors
3. **Test with a fresh, minimal post** to rule out content-specific issues
4. **Verify environment variables** are set correctly for your deployment
5. **Run tests** with `cargo test` to verify core functionality

Most issues are caused by YAML syntax errors or Docker configuration problems.
The server will skip problematic posts and continue running, so check the
console output first.
