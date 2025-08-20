# Use the official Rust image as the base image
FROM rust:1.86.0 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release --bin tutor-service

# Use a smaller base image for the final image
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/tutor-service /app/tutor-service

# Expose the port (assuming your Actix Web server runs on port 8080)
EXPOSE 8080

# Run the application
CMD ["./tutor-service"]