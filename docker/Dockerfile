# =============================================================================
# Builder Stage - Build Rust binary with all features
# =============================================================================
FROM rust:1.91-bookworm AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        cmake \
        g++ \
        pkg-config \
        libssl-dev \
        && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace manifests for dependency caching
COPY Cargo.toml Cargo.lock ./

# Copy only the crates needed for the build
COPY crates/crawlberg/ crates/crawlberg/
COPY crates/crawlberg-cli/ crates/crawlberg-cli/

# Replace glob workspace members with only the crates we need
RUN sed -i 's|^members = .*|members = ["crates/crawlberg", "crates/crawlberg-cli"]|' Cargo.toml && \
    sed -i '/^exclude/,/^\]/d' Cargo.toml

# Build release binary with all features
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    cargo build --release --package crawlberg-cli \
        --features "crawlberg-cli/api,crawlberg-cli/mcp,crawlberg/api,crawlberg/mcp,crawlberg/warc,crawlberg/ai,crawlberg/tracing,crawlberg/browser,crawlberg/interact" && \
    cp target/release/crawlberg /build/crawlberg && \
    strip /build/crawlberg

# =============================================================================
# Runtime Stage - Chainguard base with Chromium for browser automation
# =============================================================================
FROM debian:bookworm-slim

LABEL org.opencontainers.image.source="https://github.com/xberg-io/crawlberg"
LABEL org.opencontainers.image.description="Crawlberg — high-performance web crawler with browser automation, API server, and MCP"
LABEL org.opencontainers.image.licenses="MIT"

WORKDIR /app

# Install runtime deps: Chromium + fonts + curl for healthcheck
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        chromium \
        fonts-liberation \
        fonts-noto-color-emoji \
        && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/crawlberg /usr/local/bin/crawlberg

# Chromium flags: set CRAWLBERG_DISABLE_GPU=false to enable GPU acceleration
# (useful when running with --gpus or in GPU-enabled environments)
ARG DISABLE_GPU=true
RUN groupadd -r crawlberg && \
    useradd -r -g crawlberg -d /app -s /sbin/nologin crawlberg && \
    chown -R crawlberg:crawlberg /app

ENV RUST_LOG=info \
    CHROME_BIN=/usr/bin/chromium \
    CRAWLBERG_DISABLE_GPU=${DISABLE_GPU} \
    CHROMIUM_FLAGS="--no-sandbox --disable-dev-shm-usage"

USER crawlberg

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["curl", "-f", "http://localhost:3000/health"]

# Entrypoint: crawlberg binary. Override CMD for CLI usage.
# API server: docker run crawlberg (default)
# CLI:        docker run crawlberg scrape https://example.com
# MCP:        docker run crawlberg mcp
ENTRYPOINT ["crawlberg"]
CMD ["serve", "--host", "0.0.0.0", "--port", "3000"]
