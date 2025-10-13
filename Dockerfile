# Build stage
FROM rust:1.90-trixie AS builder

# Set working directory
WORKDIR /app

# Install system dependencies for building (including Node.js for DaisyUI)
RUN apt-get update && apt-get install -y \
    curl \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Copy NPM configs
COPY package.json package-lock.json ./

# Install NPM packages for CSS compilation
RUN npm install 

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy build script
COPY build.rs ./

# Copy source code and templates
COPY src/ ./src/
COPY templates/ ./templates/
COPY static/ ./static/

# Build the application in release mode
# This will also run build.rs which compiles Tailwind CSS
RUN cargo build --release

# Runtime stage - use Debian slim for smaller image size
FROM debian:trixie-slim

# Install runtime dependencies (curl for health check)
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd -g 1001 scriba && \
    useradd -u 1001 -g scriba -s /bin/sh -m scriba

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder --chown=scriba:scriba /app/target/release/scriba /app/scriba

# Copy static assets (including compiled CSS)
COPY --from=builder --chown=scriba:scriba /app/static /app/static

# Copy templates
COPY --from=builder --chown=scriba:scriba /app/templates /app/templates

# Create posts directory for volume mounting
RUN mkdir -p /app/posts && chown -R scriba:scriba /app

# Set environment variables with defaults
ENV BLOG_POSTS_DIR="/app/posts"

# Switch to non-root user
USER scriba

# Expose default port
EXPOSE 8000

# Run the application
CMD ["/app/scriba"]
