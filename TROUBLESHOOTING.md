# Troubleshooting Guide

Common issues and solutions when setting up and running your Scriba blog.

## Posts Not Appearing

### Post doesn't show up on the website

**Most common causes:**

1. **Invalid YAML frontmatter syntax**
   ```bash
   # Check server console for warnings
   deno task start
   # Look for "Skipping file [filename]" messages
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
error: Uncaught AddrInUse: Address already in use (os error 48)
```

**Fix:**

```bash
# Find what's using port 8000
lsof -i :8000

# Use a different port
PORT=8001 deno task start
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
BLOG_POSTS_DIR="./my-posts" deno task start
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

# Update dependencies
deno cache --reload deno.json
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
```

### Custom posts directory not working

**Fix:**

```bash
# Use absolute path in production
BLOG_POSTS_DIR="/app/posts"

# Use relative path in development  
BLOG_POSTS_DIR="./my-posts"
```

## URL and Routing Issues

### 404 errors for valid posts

**Causes:**

1. **Fresh manifest out of date**
   ```bash
   deno task manifest
   deno task build
   ```

2. **Special characters in title**

   Titles like `"C++ Tips"` become URLs like `/c-tips` (special chars removed)

3. **Case sensitivity** (production only)

   Use consistent lowercase for all filenames

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
6. **Try a simple restart** of the development server

## Still Having Issues?

1. **Validate your post** against the working examples in the `posts/` directory
2. **Check the console output** when starting the server - it shows parsing
   warnings
3. **Test with a fresh, minimal post** to rule out content-specific issues
4. **Verify environment variables** are set correctly for your deployment

Most issues are caused by YAML syntax errors or Docker configuration problems.
The server will skip problematic posts and continue running, so check the
console output first.
