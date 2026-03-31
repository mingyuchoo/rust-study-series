# SPEC-002: 세계 시계 웹 서비스 REST API

## 메타데이터
- SPEC ID: SPEC-002
- PRD: PRD-002
- 작성일: 2026-03-31
- 상태: Implemented

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-002 | FR-1 | REST API로 모든 도시의 현재 시간 조회 |
| PRD-002 | FR-2 | REST API로 도시 추가 |
| PRD-002 | FR-3 | REST API로 도시 삭제 |
| PRD-002 | FR-4 | REST API로 도시 목록 조회 |
| PRD-002 | FR-5 | 웹 서버 시작/종료 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | GET /api/clocks 정상 응답 | FR-1 | tests/test_web.rs | PASS |
| TC-2  | GET /api/clocks 빈 목록 시 빈 배열 반환 | FR-1 | tests/test_web.rs | PASS |
| TC-3  | POST /api/cities 도시 추가 성공 (201) | FR-2 | tests/test_web.rs | PASS |
| TC-4  | POST /api/cities 중복 도시 (409) | FR-2 | tests/test_web.rs | PASS |
| TC-5  | POST /api/cities 잘못된 타임존 (400) | FR-2 | tests/test_web.rs | PASS |
| TC-6  | DELETE /api/cities/{name} 삭제 성공 (204) | FR-3 | tests/test_web.rs | PASS |
| TC-7  | DELETE /api/cities/{name} 존재하지 않는 도시 (404) | FR-3 | tests/test_web.rs | PASS |
| TC-8  | GET /api/cities 도시 목록 조회 | FR-4 | tests/test_web.rs | PASS |
| TC-9  | serve 명령어로 서버 시작 | FR-5 | tests/test_web.rs | PASS |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 파일 | 심볼 (함수/클래스) | 관련 FR |
|------|-------------------|--------|
| src/web.rs | create_router | FR-1, FR-2, FR-3, FR-4 |
| src/web.rs | get_clocks | FR-1 |
| src/web.rs | list_cities | FR-4 |
| src/web.rs | add_city | FR-2 |
| src/web.rs | remove_city | FR-3 |
| src/web.rs | AppState | FR-1, FR-2, FR-3, FR-4 |
| src/cli.rs | Commands::Serve | FR-5 |
| src/main.rs | run (serve 분기) | FR-5 |

## 개요
axum 기반 HTTP 웹 서버를 추가하여 세계 시계 기능을 REST API로 제공한다. 기존 도메인 로직을 재사용한다.

## 기술 설계

### 아키텍처
```
CLI (clap) -> main.rs -> Commands::Serve -> web.rs (axum 서버)
                                              -> handlers: get_clocks, add_city, remove_city, list_cities
                                              -> AppState (Arc<RwLock<Config>> + config_path)
                      -> clock.rs (기존, 재사용)
                      -> config.rs (기존, 재사용)
                      -> error.rs (확장: HTTP 에러 응답)
```

### API / 인터페이스

#### REST API 엔드포인트

##### GET /api/clocks
- 응답: 200 OK
```json
[
  {
    "city": "Seoul",
    "timezone": "Asia/Seoul",
    "time": "2026-03-31 15:30:00",
    "utc_offset": "+09:00"
  }
]
```
- 도시가 없을 때: 200 OK, `[]`

##### POST /api/cities
- 요청 Body:
```json
{
  "name": "Seoul",
  "timezone": "Asia/Seoul"
}
```
- 성공: 201 Created, 추가된 도시 정보 반환
- 중복 도시: 409 Conflict, `{"error": "이미 존재하는 도시: Seoul"}`
- 잘못된 타임존: 400 Bad Request, `{"error": "알 수 없는 타임존: Invalid/Zone"}`

##### DELETE /api/cities/{name}
- 성공: 204 No Content
- 미존재: 404 Not Found, `{"error": "존재하지 않는 도시: Berlin"}`

##### GET /api/cities
- 응답: 200 OK
```json
[
  {
    "name": "Seoul",
    "timezone": "Asia/Seoul"
  }
]
```

#### 공유 상태
```rust
pub struct AppState {
    pub config: RwLock<Config>,
    pub config_path: PathBuf,
}
```

#### CLI 확장
```rust
pub enum Commands {
    // ... 기존 명령어
    Serve {
        #[arg(long, default_value = "3000")]
        port: u16,
    },
}
```

#### web.rs 함수 시그니처
```rust
pub fn create_router(state: Arc<AppState>) -> Router;
pub async fn get_clocks(State(state): State<Arc<AppState>>) -> impl IntoResponse;
pub async fn list_cities(State(state): State<Arc<AppState>>) -> impl IntoResponse;
pub async fn add_city(State(state): State<Arc<AppState>>, Json(body): Json<CityEntry>) -> impl IntoResponse;
pub async fn remove_city(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> impl IntoResponse;
```

### 데이터 모델
기존 `CityEntry`, `Config`, `ClockDisplay` 를 재사용한다.
- `ClockDisplay`에 `Serialize` derive 추가 필요.

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | GET /api/clocks 정상 응답 | 도시 2개 등록된 상태 | 200, 2개 시간 JSON | integration | FR-1 |
| TC-2 | GET /api/clocks 빈 목록 | 도시 없는 상태 | 200, `[]` | integration | FR-1 |
| TC-3 | POST /api/cities 성공 | `{"name":"Seoul","timezone":"Asia/Seoul"}` | 201, 도시 정보 | integration | FR-2 |
| TC-4 | POST /api/cities 중복 | 이미 존재하는 도시 | 409, 에러 메시지 | integration | FR-2 |
| TC-5 | POST /api/cities 잘못된 타임존 | `{"name":"Test","timezone":"Invalid/Zone"}` | 400, 에러 메시지 | integration | FR-2 |
| TC-6 | DELETE /api/cities/{name} 성공 | 존재하는 도시명 | 204 | integration | FR-3 |
| TC-7 | DELETE /api/cities/{name} 미존재 | 없는 도시명 | 404, 에러 메시지 | integration | FR-3 |
| TC-8 | GET /api/cities 목록 조회 | 도시 2개 등록된 상태 | 200, 2개 도시 JSON | integration | FR-4 |
| TC-9 | serve CLI 명령어 파싱 | `serve --port 8080` | port=8080 파싱 성공 | unit | FR-5 |

## 구현 가이드
- 파일 위치: `src/web.rs` (신규), `src/cli.rs` (확장), `src/main.rs` (확장), `src/error.rs` (확장)
- 의존성: axum, tokio, tower-http (CORS)
- 주의사항:
  - `ClockDisplay`에 `Serialize` derive 추가
  - `AppError`에 `IntoResponse` 구현으로 HTTP 에러 응답 매핑
  - 설정 변경 시 파일에도 반영 (persist)
  - `RwLock`으로 동시성 안전 보장

## 완료 정의 (Definition of Done)
- [ ] 모든 테스트 케이스 통과
- [ ] 모든 FR에 대해 최소 1개 이상의 TC가 존재
- [ ] 추적성 매트릭스에 빈 항목 없음
- [ ] cargo clippy 경고 0개
- [ ] cargo fmt --check 통과
