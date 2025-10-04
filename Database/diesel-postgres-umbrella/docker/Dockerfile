# 프로젝트 이름을 ARG로 받아서 사용
ARG PROJECT_NAME=diesel-postgres-umbrella
# 빌드와 실행을 동일한 이미지에서 진행
FROM rust:1.86-slim AS builder

# Diesel(Postgres) 빌드에 필요한 패키지 설치
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config libssl-dev libpq-dev build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1) 의존성 빌드 캐시 최적화: 매니페스트만 먼저 복사 후 더미 소스로 선빌드
COPY Cargo.toml Cargo.toml
COPY persistence/Cargo.toml persistence/Cargo.toml
COPY runner/Cargo.toml runner/Cargo.toml

# 더미 소스 파일 생성으로 의존성만 빌드
RUN mkdir -p persistence/src runner/src \
    && echo "pub fn placeholder() {}" > persistence/src/lib.rs \
    && echo "fn main() {}" > runner/src/main.rs

# 워크스페이스 의존성만 먼저 빌드 (runner 타겟 기준)
RUN cargo build -p runner --release

# 2) 실제 소스 복사 후 최종 빌드
COPY . .
RUN cargo build -p runner --release

# -------- Runtime stage --------
FROM debian:bookworm-slim AS runtime

# Diesel(Postgres) 런타임 의존성
RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq5 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# runner 바이너리만 복사
COPY --from=builder /app/target/release/runner /app/runner

ENV RUST_LOG=info

# CLI 앱이므로 포트 노출 불필요
CMD ["/app/runner"]
