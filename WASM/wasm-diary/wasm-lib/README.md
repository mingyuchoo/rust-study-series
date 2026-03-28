# wasm-lib

wasm-diary 프로젝트의 Rust WASM 라이브러리입니다. 다이어리 비즈니스 로직을 구현합니다.

## 주요 기능

- 다이어리 엔트리 관리 (CRUD)
- 사용자 관리 (RBAC - 관리자/일반 사용자)
- 회원가입/로그인 (SHA-256 비밀번호 해싱)
- 감정(8종) 및 날씨(6종) 기록
- 키워드 검색 및 필터링
- 통계 계산

## 주요 의존성

| 패키지 | 버전 | 설명 |
|--------|------|------|
| `wasm-bindgen` | 0.2.95 | WebAssembly-JavaScript 간 통신 |
| `serde` / `serde_json` | 1 | JSON 직렬화 |
| `argon2` | 0.5 | 비밀번호 해싱 |
| `uuid` | 1 | UUID 생성 |
| `js-sys` | 0.3 | JavaScript API 바인딩 |

## WASM 빌드

```bash
wasm-pack build
```

## 테스트

```bash
cargo test
```

## watch 모드 사용

### `cargo-watch` 설치

```bash
cargo install cargo-watch
```

### watch 모드 실행

```bash
# 테스트만 실행
cargo watch -x test

# check 후 테스트 실행
cargo watch -x check -x test
```
