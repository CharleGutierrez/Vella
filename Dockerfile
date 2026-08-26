# Build stage
FROM rust:1.75-slim-bookworm AS builder
WORKDIR /usr/src/vella

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/vella*

# Build actual code
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies (e.g., OpenSSL)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /usr/src/vella/target/release/vella /usr/local/bin/vella

# Expose Vella's default Edge Gateway port
EXPOSE 8080

# Run the God-Tier OS
CMD ["vella"]
