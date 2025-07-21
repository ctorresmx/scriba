# Build stage - use Debian-based image for better compatibility with native modules
FROM denoland/deno:2.4.2 AS builder

# Set working directory
WORKDIR /app

# Copy configuration files first for better caching
COPY deno.json fresh.config.ts ./

# Cache dependencies by copying import map first
RUN deno cache --reload deno.json

# Copy source code (excluding posts directory via .dockerignore)
COPY . .

# Install dependencies and build the application
RUN deno cache main.ts dev.ts
RUN deno task manifest
RUN deno task build

# Runtime stage - use Alpine for smaller image size
FROM denoland/deno:alpine-2.4.2

# Install curl for health check (Alpine Linux) - must be done as root
RUN apk add --no-cache curl

# Create non-root user for security (Alpine Linux)
RUN addgroup -g 1001 scriba && \
    adduser -u 1001 -G scriba -s /bin/sh -D scriba

# Set working directory
WORKDIR /app

# Copy built application from builder stage, excluding heavy/unnecessary files
COPY --from=builder --chown=scriba:scriba /app .

# Remove unnecessary files from runtime image to keep it lean
RUN rm -rf node_modules/ \
    deno.lock \
    .env* \
    **/*_test.ts \
    **/*_test.tsx \
    **/*.test.ts \
    **/*.test.tsx 2>/dev/null || true

# Create posts directory and deno cache directory for volume mounting and ensure proper permissions
RUN mkdir -p /app/posts /app/.deno && chown -R scriba:scriba /app

# Set environment variables with defaults
ENV PORT=8000
ENV DENO_DIR="/app/.deno"

# Switch to non-root user
USER scriba

# Expose port
EXPOSE 8000

# Run the application
CMD ["deno", "run", "-A", "main.ts"]