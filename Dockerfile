# Git Friends Dockerfile
# Multi-stage build for efficient production images

# Build stage
FROM rust:1.86-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app gitfriends

# Set working directory
WORKDIR /app

# Copy built binaries from builder stage
COPY --from=builder /app/target/release/gf-server ./bin/
COPY --from=builder /app/target/release/gf-irc ./bin/
COPY --from=builder /app/target/release/gf-hook ./bin/
COPY --from=builder /app/target/release/gf-tester ./bin/

# Copy default configuration
COPY git-friends.toml.example ./config/git-friends.toml

# Create directories for configuration and data
RUN mkdir -p /app/config /app/data /app/logs && \
    chown -R gitfriends:gitfriends /app

# Switch to app user
USER gitfriends

# Expose ports
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Default command (can be overridden)
CMD ["./bin/gf-server", "--bind", "0.0.0.0:8080"]
