FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && \
	mv ./target/release/hello-rocket ./hello-rocket

# Runtime image
FROM ubuntu:22.04

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/hello-rocket /app/hello-rocket

# Run the app
CMD ./hello-rocket
