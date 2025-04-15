# Use an official Rust image as a parent image
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Install mold linker
RUN apt-get update && apt-get install -y mold

# Set mold as the linker and set the target CPU to x86_64-v3
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold -C target-cpu=x86_64-v3"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin forum_meucci

# Use a minimal base image for the final stage
FROM debian:bookworm-slim AS runtime

# Install curl
RUN apt-get update && apt-get install -y curl

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/forum_meucci ./bin/forum_meucci

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
ENTRYPOINT ["./bin/forum_meucci"]