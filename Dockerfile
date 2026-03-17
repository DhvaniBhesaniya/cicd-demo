# ─────────────────────────────────────────────────────────────────────────────
# Stage 1 — Builder
#   Uses the official Rust image to compile a fully-static binary.
# ─────────────────────────────────────────────────────────────────────────────
FROM rust:1.85-slim AS builder

WORKDIR /app

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# ─────────────────────────────────────────────────────────────────────────────
# Stage 2 — Runtime
#   Tiny Debian-slim image. We only copy the compiled binary — no Rust
#   toolchain, no source code, no build artifacts.  Result: ~20 MB image.
# ─────────────────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# Install only the minimum OS libs needed at runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl \
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