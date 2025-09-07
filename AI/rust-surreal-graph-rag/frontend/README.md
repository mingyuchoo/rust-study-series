# RAG AI 웹 서비스 (프론트엔드)

> TypeScript, pnpm, Vite, React 18, Axios, Fluent UI v8 기반의 클라이언트

## 기능

- 로그인/로그아웃/내 정보(me)
- 통합 질의응답(/api/chat/ask)
- 벡터 검색(/api/search/vector)
- 헬스 체크(/api/health)

## 사전 준비

- Node.js 18+
- pnpm 8+
- 백엔드 서버 실행 및 CORS 허용 상태

## 환경 변수

`.env` 또는 `.env.local` 파일에 다음 항목을 설정하십시오.

```bash
VITE_API_BASE_URL=http://localhost:4000
```

## 설치 및 실행

```bash
pnpm install
pnpm dev
```

## 빌드

```bash
pnpm build
pnpm preview
```

## 구조

- `src/services/*`: Axios 기반 API 호출 래퍼와 인터셉터
- `src/store/auth.tsx`: 인증 컨텍스트 및 토큰 저장/관리(localStorage)
- `src/pages/*`: 페이지 컴포넌트 (Login, Chat, VectorSearch, Health)
- `src/components/*`: 공용 컴포넌트 및 내비게이션 바

## 보안 주의

- 토큰은 브라우저 저장소(localStorage)에 저장됩니다. 운영 환경에서는 보안 요구사항에 따라 저장소 전략과 SameSite/HttpOnly 쿠키 등으로 전환을 고려하십시오.

## 라이선스

- 사내/프로젝트 정책을 따르십시오.
