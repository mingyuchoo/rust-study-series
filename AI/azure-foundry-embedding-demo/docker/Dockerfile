 # syntax=docker/dockerfile:1.7-labs

# ============================================
# 1단계: cargo-chef 사전 빌드 이미지
# ============================================
FROM rust:1.90-slim AS chef
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      build-essential \
      pkg-config \
      libssl-dev \
      ca-certificates && \
    cargo install cargo-chef --locked && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# ============================================
# 2단계: 레시피 생성
# ============================================
FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ============================================
# 3단계: 의존성 및 애플리케이션 빌드
# ============================================
FROM chef AS builder
WORKDIR /app

# 레시피 복사 및 의존성 빌드
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json

# 소스 코드 복사 및 빌드
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release && \
    cp /app/target/release/azure-foundry-embedding-demo /app/azure-foundry-embedding-demo

# ============================================
# 4단계: 최소 런타임 이미지
# ============================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      libssl3 \
      libsqlite3-0 \
      curl \
      binutils && \
    update-ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 빌드된 바이너리만 복사
COPY --from=builder /app/azure-foundry-embedding-demo /app/azure-foundry-embedding-demo

RUN mkdir -p /app/data && \
    strip /app/azure-foundry-embedding-demo

ENV DATABASE_URL=sqlite:///app/data/embeddings.db \
    SERVER_HOST=0.0.0.0 \
    SERVER_PORT=8000

EXPOSE 8000

CMD ["/app/azure-foundry-embedding-demo"]