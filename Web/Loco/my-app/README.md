# my-app

[Loco](https://loco.rs) 프레임워크 기반 SaaS 스타터 웹 애플리케이션입니다.
JWT 인증 기반의 `User` 모델과 프론트엔드 연동을 포함합니다.

## 주요 의존성

- `loco-rs` 0.15 - Rails 스타일 웹 프레임워크
- `axum` 0.8.1 - HTTP 서버
- `sea-orm` 1.1.0 (SQLite, PostgreSQL) - ORM
- `tokio` 1.33.0 - 비동기 런타임
- `serde` / `serde_json` - 직렬화
- `validator` 0.20 - 데이터 유효성 검증
- `uuid` 1.6.0 - UUID 생성
- `chrono` 0.4 - 날짜/시간

## 빠른 시작

```bash
cargo loco db seed
cargo loco start
```

서버는 `http://localhost:5150` 에서 실행됩니다.

## 프론트엔드

`frontend/` 디렉토리에 React + Rsbuild 기반 프론트엔드가 포함되어 있습니다.
자세한 내용은 [frontend/README.md](frontend/README.md)를 참조하세요.

## 설정

[config/development.yaml](config/development.yaml) 파일에서 프론트엔드 또는 서버 사이드 템플릿 렌더링을 선택할 수 있습니다.

## 참고 자료

- [Loco 빠른 시작 가이드](https://loco.rs/docs/getting-started/tour/)
- [Loco 완전 가이드](https://loco.rs/docs/getting-started/guide/)
