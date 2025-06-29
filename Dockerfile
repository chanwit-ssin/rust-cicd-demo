# Stage 1: Build the application
FROM rust:1.78 as builder

# Create a new empty shell project
WORKDIR /usr/src/rust-ping-pong

# Copy over your manifests
COPY Cargo.toml Cargo.lock ./

# Copy over your source code
COPY src ./src

# Build for release
RUN cargo build --release

# Stage 2: Create the final, minimal image
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/rust-ping-pong/target/release/rust-ping-pong /usr/local/bin/rust-ping-pong

# Expose the port the app runs on
EXPOSE 3000

# Set the startup command to run the binary
CMD ["rust-ping-pong"]