# diesel-sqlite-init

Diesel ORM과 SQLite를 사용한 기본 CRUD 예제 프로젝트입니다.

## 사전 준비

Diesel ORM 사용을 위한 `diesel_cli` 설치

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

## 프로젝트 구조

```text
src/
├── bin/
│   ├── show_posts.rs      # 게시물 조회
│   ├── show_drafts.rs     # 초안 조회
│   ├── write_post.rs      # 게시물 작성
│   ├── publish_post.rs    # 게시물 공개
│   └── delete_post.rs     # 게시물 삭제
├── lib.rs                 # 라이브러리 진입점
└── models.rs / schema.rs  # 모델 및 스키마
```

## 주요 의존성

- `diesel` 2.2.5 (SQLite 기능, `returning_clauses_for_sqlite_3_35`)
- `dotenvy` 0.15.7
- `rustsqlite` 0.32.1 (bundled)

## 환경 설정

```bash
echo DATABASE_URL=mydb.sqlite3 > .env
# 또는
# .env.test 파일을 .env로 복사하세요
```

## 마이그레이션 생성 및 실행

```bash
diesel setup
diesel migration generate <마이그레이션_이름>
```

`up.sql` 예시:

```sql
CREATE TABLE posts (
  id INTEGER     NOT NULL PRIMARY KEY AUTOINCREMENT,
  title TEXT     NOT NULL,
  body  TEXT     NOT NULL,
  published BOOL NOT NULL DEFAULT FALSE
);
```

`down.sql` 예시:

```sql
DROP TABLE posts;
```

마이그레이션 실행:

```bash
diesel migration run
```

## 빌드 확인

```bash
cargo check
```

## 실행 방법

```bash
cargo run --bin show_posts
cargo run --bin show_drafts
cargo run --bin write_post
cargo run --bin publish_post
cargo run --bin delete_post
```
