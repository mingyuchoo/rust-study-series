# Dockerfile - 멀티스테이지 빌드

# Stage 1) 빌드 단계: Rust 툴체인 포함 이미지 사용
FROM rust:1.91-slim AS builder

WORKDIR /app

# 의존성 캐시 최적화를 위해 Cargo 매니페스트만 먼저 복사
COPY Cargo.toml ./
# Cargo.lock이 있으면 함께 복사(없으면 무시)
# hadolint ignore=DL3059
COPY Cargo.lock* ./

# 더미 main.rs 생성 후 의존성만 미리 빌드하여 캐시 레이어 확보
RUN mkdir -p src \
 && echo "fn main() { println!(\"build deps cache\"); }" > src/main.rs \
 && cargo build --release \
 && rm -rf src

# 실제 소스 복사 및 빌드
COPY src ./src
RUN cargo build --release

# Stage 2) 런타임 단계: 슬림 베이스 이미지 사용
FROM debian:bookworm-slim AS runtime

# CA 루트 인증서 설치(HTTPS 호출용)
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Stage 3) 런타임 단계: 빌더에서 바이너리만 추출 (패키지명과 동일: caching_with_redis)
COPY --from=builder /app/target/release/caching_with_redis /usr/local/bin/caching_with_redis

# 기본 환경 변수 설정
ENV REDIS_URL=redis://redis:6379 \
    CACHE_TTL=3600 \
    RUST_LOG=info

# 네트워크 포트가 필요 없다면 EXPOSE 생략 가능(예시로 남김)
EXPOSE 8080

# 기본 실행 커맨드(도움말 확인을 위해 빈 실행 또는 status 사용 가능)
ENTRYPOINT ["/usr/local/bin/caching_with_redis"]
CMD ["status"]
