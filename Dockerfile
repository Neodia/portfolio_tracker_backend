# Build stage
FROM rust:1.95-slim AS builder
WORKDIR /app

ENV SQLX_OFFLINE=true

# Cache dependencies separately from source code
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Build the actual binary
COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/portfolio_tracker_backend /usr/local/bin/

CMD ["portfolio_tracker_backend"]