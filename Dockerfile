# ─────────────────────────────────────────────────────────────────────────────
# Stage 1 — Builder
#   Uses the official Rust image to compile a fully-static binary.
#   We use "cargo-chef" pattern manually for better layer caching:
#   dependencies are compiled before our own source code, so Docker
#   only re-runs the expensive dep build when Cargo.toml changes.
# ─────────────────────────────────────────────────────────────────────────────
FROM rust:1.85-slim AS builder

WORKDIR /app

# 1. Copy only the manifest files first (cache layer for dependencies)
COPY Cargo.toml ./

# 2. Create a dummy main so cargo can build deps without our real code
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 3. Build dependencies only — this layer is cached unless Cargo.toml changes
RUN cargo build --release && rm -rf src

# 4. Now copy real source and build the actual binary
COPY src ./src

# Touch main.rs so cargo knows it changed (avoids cache hit on binary)
RUN touch src/main.rs && cargo build --release

# ─────────────────────────────────────────────────────────────────────────────
# Stage 2 — Runtime
#   Tiny Debian-slim image. We only copy the compiled binary — no Rust
#   toolchain, no source code, no build artifacts.  Result: ~20 MB image.
# ─────────────────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# Install only the minimum OS libs needed at runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/docker-cicd-demo .

# Render / Docker will set PORT; default 3000 for local runs
ENV PORT=3000
EXPOSE 3000

# Health check — Docker polls this every 30 s
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:$PORT/health || exit 1

CMD ["./docker-cicd-demo"]