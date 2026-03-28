# diesel-mysql-init

Diesel ORM과 MySQL을 사용한 기본 CRUD 예제 프로젝트입니다.

## 사전 준비

Diesel ORM 사용을 위한 `diesel_cli` 설치

```bash
cargo install diesel_cli --no-default-features --features mysql
```

## 프로젝트 구조

```text
src/
├── bin/
│   ├── show_posts.rs      # 게시물 조회
│   ├── write_post.rs      # 게시물 작성
│   ├── publish_post.rs    # 게시물 공개
│   └── delete_post.rs     # 게시물 삭제
├── lib.rs                 # 라이브러리 진입점
└── models.rs / schema.rs  # 모델 및 스키마
```

## 주요 의존성

- `diesel` 2.2.5 (MySQL 기능)
- `dotenvy` 0.15.7

## 환경 설정

```bash
echo DATABASE_URL=mysql://root:root@localhost:3306/root > .env
# 또는
# .env.test 파일을 .env로 복사하세요
```

## 마이그레이션 생성 및 실행

```bash
diesel setup
diesel migration generate <마이그레이션_이름>
# up.sql / down.sql 에 SQL 작성
diesel migration run
```

## 빌드 확인

```bash
cargo check
```

## 실행 방법

```bash
cargo run --bin show_posts
cargo run --bin write_post
cargo run --bin publish_post
cargo run --bin delete_post
```
