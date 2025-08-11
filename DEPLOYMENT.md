# Production Deployment Guide

This guide covers deploying Scriba in production using Docker.

## Simple Docker Deployment

### Single Container

For a basic production deployment:

```bash
docker run -d \
  --name scriba-blog \
  -p 8000:8000 \
  -v $(pwd)/posts:/app/posts:ro \
  -e BLOG_NAME="Your Blog Name" \
  -e BLOG_TITLE="Your Blog Title" \
  -e BLOG_COPYRIGHT="© 2025 Your Name" \
  -e BLOG_FAVICON_TEXT="YB" \
  --restart unless-stopped \
  ghcr.io/ctorresmx/scriba:latest
```

Your blog will be available at `http://your-server:8000`.

## Docker Compose (Recommended)

For easier management and potential reverse proxy setup:

**docker-compose.yml:**

```yaml
version: "3.8"

services:
  scriba:
    image: ghcr.io/ctorresmx/scriba:latest
    ports:
      - "8000:8000"
    volumes:
      - ./posts:/app/posts:ro
    environment:
      BLOG_NAME: "Your Blog Name"
      BLOG_TITLE: "Your Blog Title"
      BLOG_COPYRIGHT: "© 2025 Your Name | Powered by Scriba"
      BLOG_POSTS_DIR: "/app/posts"
      BLOG_FAVICON_TEXT: "YB"
      PORT: 8000
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

Deploy with:

```bash
docker-compose up -d
```

## With Reverse Proxy

If you want to use nginx or another reverse proxy:

```yaml
version: "3.8"

services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - scriba
    restart: unless-stopped

  scriba:
    image: ghcr.io/ctorresmx/scriba:latest
    expose:
      - "8000"
    volumes:
      - ./posts:/app/posts:ro
    environment:
      BLOG_NAME: "Your Blog Name"
      BLOG_TITLE: "Your Blog Title"
      BLOG_COPYRIGHT: "© 2025 Your Name | Powered by Scriba"
      BLOG_POSTS_DIR: "/app/posts"
      BLOG_FAVICON_TEXT: "YB"
      PORT: 8000
    restart: unless-stopped
```

**nginx.conf:**

```nginx
events {
    worker_connections 1024;
}

http {
    upstream scriba_backend {
        server scriba:8000;
    }

    server {
        listen 80;
        server_name localhost;

        location / {
            proxy_pass http://scriba_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
```

## Content Updates

To update your blog content:

```bash
# Update your posts directory
git pull  # if using git for content

# Restart the container to pick up changes
docker-compose restart scriba
```

## Environment Variables

All available configuration options:

```bash
BLOG_NAME="Your Blog Name"           # Site name
BLOG_TITLE="Your Blog Title"         # HTML title  
BLOG_COPYRIGHT="© 2025 Your Name"    # Footer copyright
BLOG_POSTS_DIR="/app/posts"          # Posts directory path
BLOG_FAVICON_TEXT="YB"               # Favicon text (2-3 chars)
PORT=8000                            # Server port
```

## Troubleshooting

**Container won't start:**

```bash
docker-compose logs scriba
```

**Posts not showing:**

- Check that `./posts` directory exists and has the correct permissions
- Verify posts have proper frontmatter format
- Check container logs for parsing errors

**Port conflicts:**

```bash
# Change the port in docker-compose.yml
ports:
  - "3000:8000"  # External port 3000, internal 8000
```

That's it! Deploy however you normally deploy Docker containers - Scriba doesn't
need anything special.
