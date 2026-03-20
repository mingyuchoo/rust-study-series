# RAG AI 웹 서비스 (Svelte 버전)

React 기반 `frontend`를 SvelteKit + Svelte 5로 포팅한 프론트엔드입니다.

## 기술 스택

| 항목 | 기술 |
|------|------|
| 프레임워크 | SvelteKit 2 + Svelte 5 (runes) |
| 언어 | TypeScript 5 |
| 빌드 도구 | Vite 6 |
| 런타임 | Bun 1.0+ |
| HTTP 클라이언트 | 네이티브 fetch (래퍼) |

## 디렉토리 구조

```
frontend/
├── src/
│   ├── app.html                          # HTML 진입점
│   ├── app.css                           # 전역 스타일
│   ├── lib/
│   │   ├── services/                     # API 호출 함수
│   │   │   ├── api.ts                    # fetch 래퍼 (인터셉터)
│   │   │   ├── auth.ts                   # 인증 API
│   │   │   ├── chat.ts                   # 채팅 API
│   │   │   ├── vectorSearch.ts           # 벡터 검색 API
│   │   │   ├── graphSearch.ts            # 그래프 검색 API
│   │   │   ├── reindex.ts                # 재인덱싱 API
│   │   │   └── health.ts                 # 헬스 체크 API
│   │   ├── stores/
│   │   │   └── auth.ts                   # 인증 상태 관리 (Svelte Store)
│   │   └── types/
│   │       └── api.ts                    # API 요청/응답 타입 정의
│   └── routes/                           # 파일 기반 라우팅
│       ├── +layout.svelte                # 레이아웃 (네비게이션 + 인증 가드)
│       ├── +layout.ts                    # SPA 모드 설정
│       ├── +page.svelte                  # 홈
│       ├── login/+page.svelte            # 로그인
│       ├── chat/+page.svelte             # 통합 질의응답
│       ├── vector-search/+page.svelte    # 벡터 검색
│       ├── graph-search/+page.svelte     # 그래프 검색
│       ├── reindex/+page.svelte          # 재인덱싱 관리
│       └── health/+page.svelte           # 헬스 체크
├── svelte.config.js                      # SvelteKit 설정
├── vite.config.ts                        # Vite 설정
├── tsconfig.json                         # TypeScript 설정
└── package.json                          # 의존성 및 스크립트
```

## 시작하기

```bash
# 의존성 설치
bun install

# 개발 서버 실행 (localhost:5173)
bun dev

# 타입 체크
bun run check

# 프로덕션 빌드
bun run build

# 빌드 결과 미리보기
bun run preview

# 코드 포매팅
bun run format
```

## React → Svelte 주요 변환 사항

| React | Svelte 5 |
|-------|----------|
| `useState` | `$state` rune |
| `useEffect` | `$effect` rune / `onMount` |
| `useContext` + Provider | `writable` store |
| React Router | SvelteKit 파일 기반 라우팅 |
| Fluent UI 컴포넌트 | CSS 기반 스타일링 |
| Axios 인터셉터 | 네이티브 fetch 래퍼 |
| `PrivateRoute` | `+layout.svelte` 인증 가드 |

## 라우팅

| 경로 | 설명 | 인증 필요 |
|------|------|-----------|
| `/` | 홈 페이지 | 아니오 |
| `/login` | 로그인 | 아니오 |
| `/chat` | 통합 질의응답 | 예 |
| `/vector-search` | 벡터 검색 | 예 |
| `/graph-search` | 그래프 검색 | 예 |
| `/reindex` | 재인덱싱 관리 | 예 |
| `/health` | 시스템 상태 | 아니오 |

## 보안 참고사항

- 토큰은 `localStorage`에 저장됩니다 (`rag_tokens_v1` 키).
- 운영 환경에서는 **HttpOnly 쿠키** 사용을 권장합니다.
- 401 응답 시 자동 토큰 갱신 로직이 포함되어 있습니다.
