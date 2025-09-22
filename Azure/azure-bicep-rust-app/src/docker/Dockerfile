# Use rust:alpine3.22 as base image
FROM rust:alpine3.22

# Install necessary packages for building
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev

# Set working directory
WORKDIR /app

# Copy the backend source code
COPY ../backend .

# Build the application in release mode
RUN cargo build --release

# Expose the default port
EXPOSE 8080

# Run the application
CMD ["./target/release/backend"]