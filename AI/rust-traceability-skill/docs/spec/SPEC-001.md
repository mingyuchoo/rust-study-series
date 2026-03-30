# SPEC-001: 블로그 게시글 CRUD API

## 메타데이터
- SPEC ID: SPEC-001
- PRD: PRD-001
- 작성일: 2026-03-31
- 상태: Implemented

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-001 | FR-1 | 게시글 생성 |
| PRD-001 | FR-2 | 게시글 단건 조회 |
| PRD-001 | FR-3 | 게시글 목록 조회 |
| PRD-001 | FR-4 | 게시글 수정 |
| PRD-001 | FR-5 | 게시글 삭제 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | 게시글 생성 성공 | FR-1 | tests/test_blog_api.rs | PASS |
| TC-2  | 제목 누락 시 생성 실패 | FR-1 | tests/test_blog_api.rs | PASS |
| TC-3  | 게시글 단건 조회 성공 | FR-2 | tests/test_blog_api.rs | PASS |
| TC-4  | 존재하지 않는 게시글 조회 시 404 | FR-2 | tests/test_blog_api.rs | PASS |
| TC-5  | 게시글 목록 조회 (빈 목록) | FR-3 | tests/test_blog_api.rs | PASS |
| TC-6  | 게시글 목록 조회 (복수 건) | FR-3 | tests/test_blog_api.rs | PASS |
| TC-7  | 게시글 수정 성공 | FR-4 | tests/test_blog_api.rs | PASS |
| TC-8  | 존재하지 않는 게시글 수정 시 404 | FR-4 | tests/test_blog_api.rs | PASS |
| TC-9  | 게시글 삭제 성공 | FR-5 | tests/test_blog_api.rs | PASS |
| TC-10 | 존재하지 않는 게시글 삭제 시 404 | FR-5 | tests/test_blog_api.rs | PASS |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 파일 | 심볼 (함수/클래스) | 관련 FR |
|------|-------------------|--------|
| src/model.rs | Post, CreatePostRequest, UpdatePostRequest | FR-1, FR-2, FR-3, FR-4, FR-5 |
| src/store.rs | BlogStore, create_post, get_post, list_posts, update_post, delete_post | FR-1, FR-2, FR-3, FR-4, FR-5 |
| src/handler.rs | app, create_post, get_post, list_posts, update_post, delete_post | FR-1, FR-2, FR-3, FR-4, FR-5 |

## 개요
Axum 기반 REST API로 블로그 게시글의 CRUD 기능을 제공한다. 도메인 로직(순수)과 HTTP 핸들러(부수 효과)를 분리하여 테스트 용이성을 확보한다.

## 기술 설계

### 아키텍처
```
src/
├── main.rs          # 서버 진입점 (부수 효과)
├── model.rs         # 도메인 모델 (순수)
├── store.rs         # 인메모리 저장소 (부수 효과)
└── handler.rs       # HTTP 핸들러 (부수 효과)
```

계층:
- model: Post, CreatePostRequest, UpdatePostRequest 등 순수 데이터 구조
- store: Arc<RwLock<HashMap>> 기반 인메모리 저장소. CRUD 연산 제공.
- handler: Axum 핸들러 함수. store를 State로 주입받아 HTTP 요청/응답 변환.

### API / 인터페이스

| 메서드 | 경로 | 요청 본문 | 응답 | 상태 코드 |
|--------|------|----------|------|----------|
| POST   | /posts | `{ "title": string, "content": string }` | `Post` | 201 Created |
| GET    | /posts/:id | - | `Post` | 200 OK / 404 |
| GET    | /posts | - | `Post[]` | 200 OK |
| PUT    | /posts/:id | `{ "title": string, "content": string }` | `Post` | 200 OK / 404 |
| DELETE | /posts/:id | - | - | 204 No Content / 404 |

### 데이터 모델

```rust
struct Post {
    id: u64,
    title: String,
    content: String,
    created_at: String,   // ISO 8601
    updated_at: String,   // ISO 8601
}

struct CreatePostRequest {
    title: String,
    content: String,
}

struct UpdatePostRequest {
    title: String,
    content: String,
}
```

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | 게시글 생성 성공 | POST /posts `{"title":"제목","content":"본문"}` | 201, id/created_at 자동 부여 | integration | FR-1 |
| TC-2 | 제목 누락 시 생성 실패 | POST /posts `{"content":"본문"}` | 422 Unprocessable Entity | integration | FR-1 |
| TC-3 | 게시글 단건 조회 성공 | GET /posts/1 (존재하는 ID) | 200, 해당 Post 반환 | integration | FR-2 |
| TC-4 | 존재하지 않는 게시글 조회 | GET /posts/9999 | 404 Not Found | integration | FR-2 |
| TC-5 | 빈 목록 조회 | GET /posts (게시글 없음) | 200, 빈 배열 | integration | FR-3 |
| TC-6 | 복수 건 목록 조회 | GET /posts (게시글 2개 생성 후) | 200, 2개 Post 배열 | integration | FR-3 |
| TC-7 | 게시글 수정 성공 | PUT /posts/1 `{"title":"수정","content":"수정"}` | 200, 수정된 Post, updated_at 변경 | integration | FR-4 |
| TC-8 | 존재하지 않는 게시글 수정 | PUT /posts/9999 `{"title":"x","content":"x"}` | 404 Not Found | integration | FR-4 |
| TC-9 | 게시글 삭제 성공 | DELETE /posts/1 (존재하는 ID) | 204 No Content | integration | FR-5 |
| TC-10 | 존재하지 않는 게시글 삭제 | DELETE /posts/9999 | 404 Not Found | integration | FR-5 |

## 구현 가이드
- 파일 위치: `src/model.rs`, `src/store.rs`, `src/handler.rs`, `src/main.rs`
- 의존성: axum, tokio, serde, serde_json, chrono (또는 time)
- 주의사항: RwLock을 사용하여 동시성 안전하게 처리. Store는 Clone 가능하도록 Arc로 감싸기.

## 완료 정의 (Definition of Done)
- [ ] 모든 테스트 케이스 통과
- [ ] 모든 FR에 대해 최소 1개 이상의 TC가 존재
- [ ] 추적성 매트릭스에 빈 항목 없음
- [ ] 코드 리뷰 완료
