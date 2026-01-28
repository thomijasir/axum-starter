
FROM rust:1.93 as builder

WORKDIR /app
COPY . .
# Build the binary
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev && cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
# Copy the binary from the builder stage
COPY --from=builder /app/target/release/axum-boilerplate /app/api
# Install necessary runtime dependencies
RUN apt-get update && apt install -y openssl libpq5
# Add execute permissions to the binary
RUN chmod +x /app/api
# Create a volume for the public directory
RUN mkdir -p /app/public
# Clean up
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

EXPOSE 3099
CMD ["./api"]
