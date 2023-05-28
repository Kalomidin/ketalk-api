FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cp .env.prod .env
RUN cargo build --release --bin ketalk

# We do not need the Rust toolchain to run the binary!
FROM --platform=linux/amd64 debian AS runtime
WORKDIR /app

# install postgresql and openssl for the runtime
RUN apt update -y
RUN apt install -y openssl libssl-dev
RUN apt-get install -y libpq5

# Copy the binary from the builder stage
# Note: binary can not create database. It only runs migrations
COPY --from=builder /app/target/release/ketalk /app
COPY --from=builder /app/.env /app
COPY --from=builder /app/migrations /app


ENV PORT 8080

ENTRYPOINT ["/app/ketalk"]