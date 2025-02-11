# Use an official Rust image as a parent image
FROM rust:latest AS builder

# Install mold linker
RUN apt-get update && apt-get install -y mold

# Set mold as the linker
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Set the working directory
WORKDIR /usr/src/app

# Copy the files into the container
COPY . .

# Create the bin directory and build the Rust project with mold linker
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:bookworm-slim AS runtime

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/forum_meucci ./bin/forum_meucci

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
CMD ["./bin/forum_meucci"]