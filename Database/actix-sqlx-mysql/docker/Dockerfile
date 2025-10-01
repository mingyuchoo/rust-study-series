# ============================================
# 1단계: 의존성 캐싱을 위한 플래너 스테이지
# ============================================
FROM rust:1.90-slim-bullseye as planner

WORKDIR /app

# cargo-chef 설치 (의존성 캐싱 도구)
RUN cargo install cargo-chef

# 프로젝트 파일 복사
COPY . .

# 의존성 레시피 생성
RUN cargo chef prepare --recipe-path recipe.json

# ============================================
# 2단계: 의존성 빌드 스테이지
# ============================================
FROM rust:1.90-slim-bullseye as cacher

WORKDIR /app

# cargo-chef 설치
RUN cargo install cargo-chef

# OpenSSL 빌드 의존성 설치 (sqlx native-tls 빌드용)
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 레시피 파일 복사
COPY --from=planner /app/recipe.json recipe.json

# 의존성만 빌드 (코드 변경 시 캐시 재사용)
RUN cargo chef cook --release --recipe-path recipe.json

# ============================================
# 3단계: 애플리케이션 빌드 스테이지
# ============================================
FROM rust:1.90-slim-bullseye as builder

WORKDIR /app

# 캐시된 의존성 복사
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# 소스 코드 복사
COPY . .

# OpenSSL 빌드 의존성 설치 (sqlx native-tls 빌드용)
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 애플리케이션 빌드 (의존성은 이미 빌드됨)
RUN cargo build --release --bin actix-sqlx-mysql

# ============================================
# 4단계: 최종 런타임 스테이지
# ============================================
FROM debian:bullseye-slim as runtime

WORKDIR /app

# 런타임 의존성 설치 (OpenSSL 1.1 포함)
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl1.1 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 빌드된 바이너리 복사
COPY --from=builder /app/target/release/actix-sqlx-mysql /usr/local/bin/actix-sqlx-mysql

# 환경 변수 설정
ENV HOST=0.0.0.0
ENV PORT=8000
ENV RUST_LOG=info

# 포트 노출
EXPOSE 8000

# 헬스체크 설정
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# 애플리케이션 실행
CMD ["actix-sqlx-mysql"]
