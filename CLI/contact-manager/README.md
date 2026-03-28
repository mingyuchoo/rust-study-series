# 연락처 관리 CLI (Contact Manager)

CSV 파일 기반의 연락처 관리 CLI 애플리케이션입니다.

## 프로젝트 구조

```
contact-manager/
├── Cargo.toml      # 프로젝트 설정 및 의존성
└── src/
    └── main.rs     # 애플리케이션 진입점
```

## 주요 의존성

- **structopt 0.3.26**: CLI 인자 파싱
- **thiserror 2.0.0**: 에러 타입 정의

**작성자**: Mingyu Choo (Rust Edition 2024)

## 빌드 방법

```bash
cargo check
cargo build --profile dev     # 개발용
cargo build --profile release # 릴리즈용
```

## 실행 방법

```bash
cargo run -- --help
contact_manager 0.1.0
project: contact manager

USAGE:
    concat_manager.exe [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               verbose

OPTIONS:
    -d <data-file>         [default: data.csv]

SUBCOMMANDS:
    add
    edit
    help      Prints this message or the help of the given subcommand(s)
    list
    remove
    search
```

## 주요 기능

- **add**: 새 연락처 추가
- **edit**: 기존 연락처 수정
- **list**: 전체 연락처 목록 조회
- **remove**: 연락처 삭제
- **search**: 연락처 검색
- **-d** 옵션으로 데이터 파일 지정 (기본값: `data.csv`)
