# postgres-postgres-init

`postgres` 크레이트를 사용한 PostgreSQL 직접 연결 예제 프로젝트입니다.

## 사전 준비

### Fedora Linux 사용 시

```bash
sudo dnf install postgresql-devel
```

## 프로젝트 구조

```text
src/
└── main.rs    # 메인 진입점
```

## 주요 의존성

- `postgres` 0.19.9 - PostgreSQL 네이티브 클라이언트
- `dotenv` 0.15.0 - 환경 변수 관리

## 환경 설정

`.env` 파일을 생성하고 데이터베이스 연결 정보를 설정하세요:

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres
```

## 빌드 및 실행

```bash
cargo check
cargo run
```

## 참고 자료

- <https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html>
