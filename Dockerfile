# syntax=docker/dockerfile:1

# ---------------------------------
# Development Stage (hot reload)
# ---------------------------------
FROM rust:1.91 AS development

WORKDIR /app

# System deps
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for hot reload
RUN cargo install cargo-watch

RUN cargo build || true

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]