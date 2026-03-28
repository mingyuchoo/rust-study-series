# demo-rocket

Rocket 웹 프레임워크 + Diesel ORM + PostgreSQL + Askama 템플릿을 사용한 웹 API 예제입니다.

## 사전 준비

### Rust 설치

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rust nightly 컴파일러 설정:

```bash
rustup default nightly
```

### 시스템 라이브러리 설치

Ubuntu Linux 사용 시:

```bash
sudo apt install -y libpq-dev          # PostgreSQL
sudo apt install -y libmysqlclient-dev # MySQL
sudo apt install -y libsqlite3-dev     # SQLite
```

Fedora Linux 사용 시:

```bash
sudo dnf install -y postgresql-devel
```

macOS 사용 시:

```bash
brew install postgresql
```

### Diesel CLI 설치

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## 프로젝트 구조

```text
src/
└── main.rs    # 메인 진입점
```

## 주요 의존성

- `rocket` 0.5.1 (json, secrets 기능) - 웹 프레임워크
- `diesel` 2.2.4 (postgres, r2d2 기능) - ORM
- `askama` 0.12.1 + `askama_rocket` 0.12.0 - 템플릿 엔진
- `dotenvy` 0.15.7 - 환경 변수 관리
- `serde` 1.0.214 - 직렬화/역직렬화
- `npm_rs` 1.0.0 (빌드 의존성)

## 환경 설정

```bash
echo "export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres" > .envrc
# 또는
# .env.test 파일을 .env로 복사하세요
```

## 데이터베이스 마이그레이션

```bash
diesel setup
diesel migration generate <마이그레이션_이름>
# up.sql / down.sql 에 SQL 작성
diesel migration run
```

## 빌드 및 실행

```bash
cargo check
cargo run
```

## API 엔드포인트

```text
GET  /api/health                  # 헬스 체크
POST /api/posts                   # 게시물 생성
GET  /api/posts?<page>&<list>     # 게시물 목록 조회
```

## 팁

### watch 모드 실행

```bash
cargo install cargo-watch
cargo watch -x run
```

### curl 테스트

```bash
curl -X GET http://localhost:8000/api/posts?page=1&limit=10
```

## 참고 자료

- [rocket-template](https://github.com/UpsettingBoy/rocket-template)
