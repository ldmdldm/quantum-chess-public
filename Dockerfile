# Build stage
FROM rust:1.70-slim AS builder
WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy cargo configuration files
COPY Cargo.toml Cargo.lock ./

# Create dummy source file to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled application
COPY --from=builder /app/target/release/quantum-chess .

# Set environment variables
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
ENV LOG_LEVEL=info

# Create non-root user
RUN useradd -m quantum
USER quantum

# Expose port
EXPOSE 8080

# Run the application
CMD ["./quantum-chess"]
