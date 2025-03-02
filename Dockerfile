# Use an official Rust image as the build stage
FROM rust:1.85.0 AS builder

# Set the working directory
WORKDIR /app

# Copy the project files
COPY . .

# Build the project in release mode
RUN cargo build --release

# Use a smaller runtime image
FROM rust:1.76-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-algorithms .

# Expose the gRPC port
EXPOSE 50051

# Run the binary
CMD ["./rust-algorithms"]