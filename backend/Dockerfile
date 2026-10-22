# Stage 1: Build the application
FROM rust:1.97 AS builder

WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Enforce SQLx offline mode so it reads from the json file
ENV SQLX_OFFLINE=true

# Build the application in release mode
RUN cargo build --release

# Stage 2: Run the application
FROM debian:bookworm-slim

WORKDIR /app

# Copy the compiled binary from the builder stage
# NOTE: Replace "my_rust_app" with the actual binary name from your Cargo.toml
COPY --from=builder /usr/src/app/target/release/habit-tracking-backend .

# Set the startup command
CMD ["./habit-tracking-backend"]