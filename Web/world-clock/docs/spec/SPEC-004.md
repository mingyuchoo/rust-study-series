# SPEC-004: 추적성 맵 그래프 시각화

## 메타데이터
- SPEC ID: SPEC-004
- PRD: PRD-004
- 작성일: 2026-03-31
- 상태: Implemented

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-004 | FR-1 | 추적성 데이터를 JSON API로 제공 |
| PRD-004 | FR-2 | 추적성 그래프 웹 페이지 제공 |
| PRD-004 | FR-3 | 정방향 추적 그래프 표시 |
| PRD-004 | FR-4 | 역방향 추적 그래프 표시 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | GET /api/trace 정상 JSON 응답 | FR-1 | tests/test_trace.rs | PASS |
| TC-2  | GET /api/trace 응답에 trace_map 포함 | FR-1 | tests/test_trace.rs | PASS |
| TC-3  | GET /trace 200 OK + text/html 응답 | FR-2 | tests/test_trace.rs | PASS |
| TC-4  | HTML에 그래프 SVG 컨테이너 포함 | FR-2, FR-3 | tests/test_trace.rs | PASS |
| TC-5  | HTML에 정방향 추적 렌더링 JS 포함 | FR-3 | tests/test_trace.rs | PASS |
| TC-6  | HTML에 역방향 추적 렌더링 JS 포함 | FR-4 | tests/test_trace.rs | PASS |
| TC-7  | HTML에 방향 전환 탭 포함 | FR-3, FR-4 | tests/test_trace.rs | PASS |
| TC-8  | 기존 API 영향 없음 확인 | FR-2 | tests/test_trace.rs | PASS |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 파일 | 심볼 (함수/클래스) | 관련 FR |
|------|-------------------|--------|
| src/web.rs | get_trace | FR-1 |
| src/web.rs | trace_html | FR-2, FR-3, FR-4 |
| src/web.rs | TRACE_HTML | FR-2, FR-3, FR-4 |
| src/web.rs | AppState::registry_path | FR-1 |
| src/web.rs | create_router (/trace, /api/trace 경로) | FR-1, FR-2 |

## 개요
registry.json의 trace_map을 JSON API로 제공하고, 이를 SVG 그래프로 시각화하는 HTML 페이지를 서버에 내장한다.

## 기술 설계

### 아키텍처
```
브라우저 -> GET /trace -> trace_html() -> 내장 HTML
         -> GET /api/trace -> get_trace() -> registry.json 읽기 -> JSON 응답
```

### API / 인터페이스

#### GET /api/trace
- 응답: 200 OK, Content-Type: application/json
- Body: registry.json의 내용 (entries + trace_map)

#### GET /trace
- 응답: 200 OK, Content-Type: text/html
- Body: 추적성 그래프 HTML (SVG + JavaScript)

#### web.rs 추가
```rust
pub fn create_router(state: Arc<AppState>) -> Router {
    // 기존 라우트 + 추가:
    .route("/trace", get(trace_html))
    .route("/api/trace", get(get_trace))
}

async fn get_trace(State(state): State<Arc<AppState>>) -> impl IntoResponse;
async fn trace_html() -> Html<&'static str>;
```

### AppState 확장
registry.json 경로를 AppState에 추가하여 런타임에 읽을 수 있도록 한다.
```rust
pub struct AppState {
    pub config: RwLock<Config>,
    pub config_path: PathBuf,
    pub registry_path: PathBuf,
}
```

### HTML 그래프 페이지 구성
- 헤더: "Traceability Map"
- 탭: `id="tab-forward"` (정방향), `id="tab-reverse"` (역방향)
- SVG 컨테이너: `id="trace-graph"`
- JavaScript:
  - `fetchTrace()`: GET /api/trace → 데이터 파싱
  - `renderForward(data)`: 정방향 그래프 SVG 렌더링
  - `renderReverse(data)`: 역방향 그래프 SVG 렌더링
- 노드 레이어: PRD(보라) → FR(파랑) → SPEC(초록) → TC(노랑) → CODE(빨강)
- 엣지: 계층 간 연결선

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | GET /api/trace 정상 응답 | GET /api/trace | 200 OK, JSON | integration | FR-1 |
| TC-2 | 응답에 trace_map 포함 | GET /api/trace body | `trace_map` 키 포함 | integration | FR-1 |
| TC-3 | GET /trace HTML 응답 | GET /trace | 200 OK, text/html | integration | FR-2 |
| TC-4 | HTML에 SVG 컨테이너 | GET /trace body | `id="trace-graph"` 포함 | integration | FR-2, FR-3 |
| TC-5 | HTML에 정방향 JS | GET /trace body | `renderForward` 포함 | integration | FR-3 |
| TC-6 | HTML에 역방향 JS | GET /trace body | `renderReverse` 포함 | integration | FR-4 |
| TC-7 | HTML에 방향 전환 탭 | GET /trace body | `tab-forward`, `tab-reverse` 포함 | integration | FR-3, FR-4 |
| TC-8 | 기존 API 영향 없음 | GET /api/clocks | 200 OK, JSON | integration | FR-2 |

## 구현 가이드
- 파일 위치: `src/web.rs` (확장)
- AppState에 `registry_path` 필드 추가
- 의존성: 추가 없음
- 주의사항: registry.json은 매 요청마다 파일에서 읽음 (최신 데이터 반영)

## 완료 정의 (Definition of Done)
- [ ] 모든 테스트 케이스 통과
- [ ] 모든 FR에 대해 최소 1개 이상의 TC가 존재
- [ ] 추적성 매트릭스에 빈 항목 없음
- [ ] cargo clippy 경고 0개
- [ ] cargo fmt --check 통과
