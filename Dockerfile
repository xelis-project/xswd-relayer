# Multi-stage build for xswd-relayer
# Stage 1: Build the Rust binary
# Using Rust 1.91+ for Vec::get_disjoint_mut support (required by xelis-blockchain)
FROM rust:1.91-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the release binary
RUN cargo build --release

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 relayer && \
    mkdir -p /app/logs && \
    chown -R relayer:relayer /app

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/xswd-relayer /usr/local/bin/xswd-relayer

# Set ownership
RUN chown relayer:relayer /usr/local/bin/xswd-relayer

# Switch to non-root user
USER relayer

# Expose WebSocket port
EXPOSE 8080

# Health check endpoint (if /metrics is enabled)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD timeout 2 bash -c "</dev/tcp/127.0.0.1/8080" || exit 1

# Default command - can be overridden with docker run args
CMD ["xswd-relayer", \
     "--bind-address", "0.0.0.0:8080", \
     "--log-level", "info", \
     "--disable-interactive-mode", \
     "--disable-ascii-art"]
