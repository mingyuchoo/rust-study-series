# SaaS 프론트엔드

Loco SaaS 스타터의 프론트엔드 애플리케이션입니다.

## 기술 스택

- [TypeScript](https://www.typescriptlang.org/) - 타입 안전한 JavaScript
- [Rsbuild](https://rsbuild.dev/) - Rust 기반 웹 빌드 도구
- [Biome](https://biomejs.dev/) - Rust 기반 포매터 및 린터
- [React](https://reactjs.org/) - UI 라이브러리
- [React Router DOM](https://reactrouter.com/) - 클라이언트 사이드 라우팅

## 개발 환경 설정

### 1. 패키지 설치

```bash
pnpm install
```

### 2. 개발 모드 실행

```bash
pnpm dev
```

개발 서버가 시작되며 브라우저에서 자동으로 열립니다.

### 3. 빌드

```bash
pnpm build
```

빌드 후 `dist` 폴더가 생성되며, Loco 서버(`cargo loco start`)에서 정적 파일로 제공됩니다.

### 4. 코드 린트

```bash
pnpm lint
```
