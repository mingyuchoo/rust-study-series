# SPEC-003: 세계 시계 웹 프론트엔드

## 메타데이터
- SPEC ID: SPEC-003
- PRD: PRD-003
- 작성일: 2026-03-31
- 상태: Implemented

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-003 | FR-1 | 루트 경로(/)에서 HTML 페이지 제공 |
| PRD-003 | FR-2 | 웹 페이지에서 모든 도시의 현재 시간을 실시간 표시 |
| PRD-003 | FR-3 | 웹 페이지에서 도시 추가 |
| PRD-003 | FR-4 | 웹 페이지에서 도시 삭제 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | GET / 요청 시 200 OK + text/html 응답 | FR-1 | tests/test_frontend.rs | PASS |
| TC-2  | HTML에 시계 표시 영역 포함 | FR-1, FR-2 | tests/test_frontend.rs | PASS |
| TC-3  | HTML에 도시 추가 폼 포함 | FR-1, FR-3 | tests/test_frontend.rs | PASS |
| TC-4  | HTML에 API 호출 JavaScript 포함 | FR-2, FR-3, FR-4 | tests/test_frontend.rs | PASS |
| TC-5  | HTML에 삭제 기능 JavaScript 포함 | FR-4 | tests/test_frontend.rs | PASS |
| TC-6  | 기존 API 엔드포인트 영향 없음 확인 | FR-1 | tests/test_frontend.rs | PASS |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 파일 | 심볼 (함수/클래스) | 관련 FR |
|------|-------------------|--------|
| src/web.rs | index_html | FR-1, FR-2, FR-3, FR-4 |
| src/web.rs | INDEX_HTML | FR-1, FR-2, FR-3, FR-4 |
| src/web.rs | create_router (/ 경로 추가) | FR-1 |

## 개요
웹 서버의 루트 경로(`/`)에서 세계 시계 HTML 페이지를 제공한다. HTML은 Rust 바이너리에 문자열로 내장되며, 기존 REST API를 fetch로 호출하여 동작한다.

## 기술 설계

### 아키텍처
```
브라우저 -> GET / -> index_html() -> 내장 HTML 문자열 응답
         -> fetch /api/clocks -> get_clocks() (기존)
         -> fetch POST /api/cities -> add_city() (기존)
         -> fetch DELETE /api/cities/{name} -> remove_city() (기존)
```

### API / 인터페이스

#### GET /
- 응답: 200 OK, Content-Type: text/html
- Body: 세계 시계 SPA HTML (JavaScript 포함)

#### web.rs 추가 함수
```rust
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index_html))       // 신규
        .route("/api/clocks", get(get_clocks))
        .route("/api/cities", get(list_cities).post(add_city))
        .route("/api/cities/{name}", delete(remove_city))
        .with_state(state)
}

async fn index_html() -> impl IntoResponse;
```

### HTML 페이지 구성
- 헤더: 제목 "World Clock"
- 시계 영역: `id="clocks"` — 도시별 시간 카드 목록
- 추가 폼: `id="add-form"` — 도시명, 타임존 입력 + 추가 버튼
- 에러 메시지: `id="error-message"`
- JavaScript:
  - `fetchClocks()`: GET /api/clocks → 시계 카드 렌더링
  - `addCity()`: POST /api/cities → 성공 시 목록 갱신, 에러 시 메시지 표시
  - `removeCity(name)`: DELETE /api/cities/{name} → 목록 갱신
  - `setInterval(fetchClocks, 1000)`: 1초마다 자동 갱신

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | GET / 정상 응답 | GET / | 200 OK, content-type에 text/html 포함 | integration | FR-1 |
| TC-2 | HTML에 시계 표시 영역 포함 | GET / 응답 body | `id="clocks"` 포함 | integration | FR-1, FR-2 |
| TC-3 | HTML에 도시 추가 폼 포함 | GET / 응답 body | `id="add-form"` 포함 | integration | FR-1, FR-3 |
| TC-4 | HTML에 API 호출 JS 포함 | GET / 응답 body | `fetchClocks`, `/api/clocks`, `/api/cities` 포함 | integration | FR-2, FR-3, FR-4 |
| TC-5 | HTML에 삭제 기능 JS 포함 | GET / 응답 body | `removeCity` 포함 | integration | FR-4 |
| TC-6 | 기존 API 영향 없음 | GET /api/clocks | 200 OK, JSON 배열 | integration | FR-1 |

## 구현 가이드
- 파일 위치: `src/web.rs` (확장)
- HTML은 `const INDEX_HTML: &str` 또는 `include_str!`로 내장
- 의존성: 추가 없음 (axum의 `Html` 응답 타입 사용)
- 주의사항: 기존 라우터에 `GET /` 경로만 추가, 기존 API 경로 영향 없음

## 완료 정의 (Definition of Done)
- [ ] 모든 테스트 케이스 통과
- [ ] 모든 FR에 대해 최소 1개 이상의 TC가 존재
- [ ] 추적성 매트릭스에 빈 항목 없음
- [ ] cargo clippy 경고 0개
- [ ] cargo fmt --check 통과
