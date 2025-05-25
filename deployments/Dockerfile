# Build stage
FROM rust:1.86-slim-bookworm AS builder

# Install required system dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy only Cargo.toml Cargo.loc first
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src

# Copy actual source code and Cargo.lock (if it exists)
COPY src ./src

# Build the actual application
RUN cargo build --release --bin notify

# Runtime stage - using distroless for minimal size with SSL support
FROM gcr.io/distroless/cc-debian12

# Copy the binary
COPY --from=builder /usr/src/app/target/release/notify /app/notify

# Expose port
EXPOSE 9001

# Run the binary (distroless has no shell)
ENTRYPOINT ["/app/notify"]
