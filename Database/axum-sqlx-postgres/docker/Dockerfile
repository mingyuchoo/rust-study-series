######
# Rust multi-stage build for axum-sqlx-postgres
######

# 1) Builder stage
FROM rust:1.81 as builder
WORKDIR /app

# 캐시 최적화를 위해 먼저 매니페스트 파일만 복사
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# 빈 소스 트리를 만들어 의존성만 미리 다운로드
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# 실제 소스 복사 후 빌드
COPY . .
RUN cargo build --release

# 2) Runtime stage
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 애플리케이션 바이너리 복사 (패키지명 = 바이너리명 가정)
COPY --from=builder /app/target/release/axum-sqlx-postgres /usr/local/bin/app

# 런타임 환경변수 (옵션)
ENV RUST_LOG=info

# 앱 포트 노출 (src/main.rs는 8000 포트 사용)
EXPOSE 8000

# 실행 커맨드
CMD ["/usr/local/bin/app"]
