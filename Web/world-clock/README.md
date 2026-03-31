# World Clock

여러 타임존의 현재 시간을 CLI, REST API, 웹 브라우저에서 확인할 수 있는 도구입니다.

## 기능

### CLI
- 저장된 도시들의 현재 시간을 테이블 형식으로 표시
- 도시/타임존 추가 및 삭제
- 저장된 도시 목록 조회
- JSON 기반 설정 파일 (OS별 표준 경로)
- `--config` 옵션으로 설정 파일 경로 지정 가능

### 웹 서비스 (REST API)
- `GET /api/clocks` — 모든 도시의 현재 시간 조회 (JSON)
- `POST /api/cities` — 도시 추가
- `DELETE /api/cities/{name}` — 도시 삭제
- `GET /api/cities` — 도시 목록 조회
- 동시 요청 안전 (RwLock 기반 공유 상태)

### 웹 프론트엔드
- `http://localhost:3000/` 접속 시 세계 시계 웹 페이지 제공
- 도시별 시간을 카드 형태로 표시, 1초마다 자동 갱신
- 웹 페이지에서 도시 추가/삭제 가능
- 반응형 디자인 (모바일/데스크톱)
- 외부 의존성 없는 단일 HTML (서버 내장)

### 추적성 맵 그래프
- `http://localhost:3000/trace` 접속 시 추적성 시각화 페이지 제공
- PRD → FR → SPEC → TC → CODE 정방향 추적 그래프
- CODE → TC → SPEC → FR → PRD 역방향 추적 그래프
- 탭으로 정방향/역방향 전환
- 노드 호버 시 상세 정보 툴팁
- `GET /api/trace` — 추적성 데이터 JSON API

## 설치

```bash
cargo install --path .
```

## 사용법

### CLI

```bash
# 저장된 도시들의 현재 시간 표시
world-clock

# 도시 추가
world-clock add Seoul Asia/Seoul
world-clock add "New York" America/New_York
world-clock add London Europe/London

# 도시 삭제
world-clock remove Seoul

# 저장된 도시 목록 조회
world-clock list

# 설정 파일 경로 지정
world-clock --config ./my-config.json add Tokyo Asia/Tokyo
```

### 웹 서비스

```bash
# 기본 포트(3000)로 웹 서버 시작
world-clock serve

# 포트 지정
world-clock serve --port 8080
```

서버 시작 후 브라우저에서 `http://localhost:3000/` 에 접속하면 세계 시계 웹 페이지를 사용할 수 있습니다.

#### API 예시

```bash
# 모든 도시의 현재 시간 조회
curl http://localhost:3000/api/clocks

# 도시 추가
curl -X POST http://localhost:3000/api/cities \
  -H "Content-Type: application/json" \
  -d '{"name":"Seoul","timezone":"Asia/Seoul"}'

# 도시 삭제
curl -X DELETE http://localhost:3000/api/cities/Seoul

# 도시 목록 조회
curl http://localhost:3000/api/cities
```

#### API 응답 예시

```json
// GET /api/clocks
[
  {
    "city": "Seoul",
    "timezone": "Asia/Seoul",
    "time": "2026-03-31 15:30:00",
    "utc_offset": "+09:00"
  },
  {
    "city": "New York",
    "timezone": "America/New_York",
    "time": "2026-03-31 02:30:00",
    "utc_offset": "-04:00"
  }
]

// GET /api/cities
[
  { "name": "Seoul", "timezone": "Asia/Seoul" },
  { "name": "New York", "timezone": "America/New_York" }
]
```

## CLI 출력 예시

```
  City      Timezone          Time                 UTC Offset
  ──────────────────────────────────────────────────────────────
  Seoul     Asia/Seoul        2026-03-31 15:30:00  +09:00
  New York  America/New_York  2026-03-31 02:30:00  -04:00
  London    Europe/London     2026-03-31 07:30:00  +01:00
```

## 타임존

IANA 타임존 데이터베이스 형식을 사용합니다. 예시:

| 도시 | 타임존 |
|------|--------|
| 서울 | Asia/Seoul |
| 도쿄 | Asia/Tokyo |
| 뉴욕 | America/New_York |
| 런던 | Europe/London |
| 파리 | Europe/Paris |
| 시드니 | Australia/Sydney |

전체 타임존 목록: [IANA Time Zone Database](https://www.iana.org/time-zones)

## 설정 파일

설정은 JSON 파일로 OS별 표준 설정 디렉토리에 저장됩니다:

| OS | 경로 |
|----|------|
| Windows | `%APPDATA%\world-clock\config.json` |
| macOS | `~/Library/Application Support/world-clock/config.json` |
| Linux | `~/.config/world-clock/config.json` |

## 개발

```bash
# 빌드
cargo build

# 테스트
cargo test

# 린트
cargo clippy

# 포매팅
cargo fmt
```

## 프로젝트 구조

```
world-clock/
├── src/
│   ├── main.rs       # 진입점
│   ├── lib.rs        # 라이브러리 루트
│   ├── cli.rs        # CLI 명령어 정의 (clap)
│   ├── clock.rs      # 시간 조회/포맷 핵심 로직
│   ├── config.rs     # 설정 파일 CRUD
│   ├── error.rs      # 에러 타입
│   └── web.rs        # 웹 서비스 (REST API + 프론트엔드 + 추적성 그래프)
├── tests/
│   ├── test_clock.rs    # 시계 기능 테스트
│   ├── test_config.rs   # 설정 기능 테스트
│   ├── test_web.rs      # REST API 테스트
│   ├── test_frontend.rs # 웹 프론트엔드 테스트
│   └── test_trace.rs    # 추적성 그래프 테스트
├── docs/
│   ├── prd/          # 요구사항 문서
│   ├── spec/         # 기술 명세서
│   ├── registry.json # 추적 레지스트리
│   └── traceability-matrix.md
├── Cargo.toml
└── README.md
```

## 기술 스택

- **언어**: Rust (2024 edition)
- **CLI**: clap
- **웹 프레임워크**: axum
- **비동기 런타임**: tokio
- **시간 처리**: chrono, chrono-tz
- **직렬화**: serde, serde_json

## 라이선스

MIT
