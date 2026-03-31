# SPEC-001: 세계 시계 핵심 기능

## 메타데이터
- SPEC ID: SPEC-001
- PRD: PRD-001
- 작성일: 2026-03-31
- 상태: Implemented

## 추적 정보

### 정방향 추적 (이 SPEC이 구현하는 요구사항)
| PRD | FR ID | 요구사항 |
|-----|-------|---------|
| PRD-001 | FR-1 | 여러 타임존의 현재 시간 표시 |
| PRD-001 | FR-2 | 도시/타임존 추가 |
| PRD-001 | FR-3 | 도시/타임존 삭제 |
| PRD-001 | FR-4 | 저장된 도시 목록 조회 |

### 역방향 추적 (이 SPEC을 검증하는 테스트)
| TC ID | 시나리오 | 검증 대상 FR | 테스트 파일 | 상태 |
|-------|---------|-------------|-----------|------|
| TC-1  | 유효한 타임존의 시간 표시 | FR-1 | tests/test_clock.rs | PASS |
| TC-2  | 잘못된 타임존 에러 처리 | FR-1 | tests/test_clock.rs | PASS |
| TC-3  | 여러 도시의 시간 포맷 출력 | FR-1 | tests/test_clock.rs | PASS |
| TC-4  | 빈 도시 목록 시 안내 메시지 | FR-1 | tests/test_clock.rs | PASS |
| TC-5  | 도시 추가 성공 | FR-2 | tests/test_config.rs | PASS |
| TC-6  | 중복 도시 추가 시 에러 | FR-2 | tests/test_config.rs | PASS |
| TC-7  | 잘못된 타임존으로 추가 시 에러 | FR-2 | tests/test_config.rs | PASS |
| TC-8  | 도시 삭제 성공 | FR-3 | tests/test_config.rs | PASS |
| TC-9  | 존재하지 않는 도시 삭제 시 에러 | FR-3 | tests/test_config.rs | PASS |
| TC-10 | 설정 파일 저장/로드 왕복 | FR-2, FR-3 | tests/test_config.rs | PASS |
| TC-11 | 설정 파일 없을 때 빈 설정 반환 | FR-1, FR-4 | tests/test_config.rs | PASS |
| TC-12 | 도시 목록 조회 | FR-4 | tests/test_config.rs | PASS |

### 구현 추적 (이 SPEC을 구현하는 코드)
| 파일 | 심볼 (함수/클래스) | 관련 FR |
|------|-------------------|--------|
| src/clock.rs | get_clock_display | FR-1 |
| src/clock.rs | format_clocks | FR-1 |
| src/config.rs | Config::load | FR-1, FR-4 |
| src/config.rs | Config::save | FR-2, FR-3 |
| src/config.rs | Config::add | FR-2 |
| src/config.rs | Config::remove | FR-3 |
| src/config.rs | default_config_path | FR-2, FR-3 |
| src/error.rs | AppError | FR-1, FR-2, FR-3 |
| src/cli.rs | Cli, Commands | FR-1, FR-2, FR-3, FR-4 |

## 개요
저장된 도시 목록의 현재 시간을 표시하고, 도시를 추가/삭제/조회하는 CLI 앱의 핵심 기능을 구현한다.

## 기술 설계

### 아키텍처
```
CLI (clap) -> main.rs -> clock.rs (시간 조회/포맷)
                      -> config.rs (설정 CRUD)
                      -> error.rs (에러 타입)
```

### API / 인터페이스

#### error.rs
```rust
#[derive(Debug, Error)]
pub enum AppError {
    UnknownTimezone(String),
    DuplicateCity(String),
    CityNotFound(String),
    Config(#[from] std::io::Error),
    Json(#[from] serde_json::Error),
}
```

#### clock.rs
```rust
pub struct ClockDisplay {
    pub city: String,
    pub timezone: String,
    pub time: String,
    pub utc_offset: String,
}

pub fn get_clock_display(city: &str, timezone: &str, now: DateTime<Utc>) -> Result<ClockDisplay, AppError>;
pub fn format_clocks(displays: &[ClockDisplay]) -> String;
```

#### config.rs
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CityEntry {
    pub name: String,
    pub timezone: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub cities: Vec<CityEntry>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, AppError>;
    pub fn save(&self, path: &Path) -> Result<(), AppError>;
    pub fn add(&mut self, entry: CityEntry) -> Result<(), AppError>;
    pub fn remove(&mut self, name: &str) -> Result<(), AppError>;
}

pub fn default_config_path() -> PathBuf;
```

#### cli.rs
```rust
#[derive(Parser)]
pub struct Cli {
    pub command: Option<Commands>,
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { name: String, timezone: String },
    Remove { name: String },
    List,
}
```

### 데이터 모델
설정 파일 (JSON):
```json
{
  "cities": [
    { "name": "Seoul", "timezone": "Asia/Seoul" },
    { "name": "New York", "timezone": "America/New_York" }
  ]
}
```

## 테스트 시나리오

| TC ID | 시나리오 | 입력 | 기대 결과 | 유형 | 검증 대상 FR |
|-------|---------|------|----------|------|-------------|
| TC-1 | 유효한 타임존의 시간 표시 | city="Seoul", tz="Asia/Seoul", now=고정시간 | ClockDisplay with time="+09:00 시간" | unit | FR-1 |
| TC-2 | 잘못된 타임존 에러 | city="Test", tz="Invalid/Zone" | AppError::UnknownTimezone | unit | FR-1 |
| TC-3 | 여러 도시의 시간 포맷 출력 | 2개 ClockDisplay | 테이블 형식 문자열 | unit | FR-1 |
| TC-4 | 빈 도시 목록 시 안내 메시지 | 빈 슬라이스 | "No cities configured." 포함 | unit | FR-1 |
| TC-5 | 도시 추가 성공 | CityEntry("Seoul", "Asia/Seoul") | cities.len() == 1 | unit | FR-2 |
| TC-6 | 중복 도시 추가 시 에러 | 같은 이름 2회 추가 | AppError::DuplicateCity | unit | FR-2 |
| TC-7 | 잘못된 타임존으로 추가 시 에러 | CityEntry("Test", "Invalid/Zone") | AppError::UnknownTimezone | unit | FR-2 |
| TC-8 | 도시 삭제 성공 | 추가 후 삭제 | cities.len() == 0 | unit | FR-3 |
| TC-9 | 존재하지 않는 도시 삭제 시 에러 | 빈 Config에서 삭제 | AppError::CityNotFound | unit | FR-3 |
| TC-10 | 설정 파일 저장/로드 왕복 | Config with 2 cities | 저장 후 로드 == 원본 | unit | FR-2, FR-3 |
| TC-11 | 설정 파일 없을 때 빈 설정 반환 | 존재하지 않는 경로 | Config::default() | unit | FR-1, FR-4 |
| TC-12 | 도시 목록 조회 | Config with 2 cities | 2개 도시 정보 포함 | unit | FR-4 |

## 구현 가이드
- 파일 위치: `src/error.rs`, `src/clock.rs`, `src/config.rs`, `src/cli.rs`, `src/main.rs`
- 의존성: chrono, chrono-tz, clap, serde, serde_json, thiserror, directories
- 주의사항: `get_clock_display`는 `DateTime<Utc>`를 외부에서 주입받아야 테스트 가능. `Config::load`는 파일 미존재 시 빈 Config 반환.

## 완료 정의 (Definition of Done)
- [ ] 모든 테스트 케이스 통과
- [ ] 모든 FR에 대해 최소 1개 이상의 TC가 존재
- [ ] 추적성 매트릭스에 빈 항목 없음
- [ ] cargo clippy 경고 0개
- [ ] cargo fmt --check 통과
