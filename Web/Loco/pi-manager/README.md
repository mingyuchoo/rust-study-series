# PI Manager 🎯

**Performance Indicator Manager** - [Loco](https://loco.rs) 프레임워크 기반 성과지표 관리 시스템

## 프로젝트 소개

PI Manager는 조직의 성과지표(Performance Indicators)를 체계적으로 관리하는 웹 애플리케이션입니다. 투입-과정-산출-결과의 논리 모델을 기반으로 성과를 다차원적으로 측정하고, AI 어시스턴트를 통해 지표 설정을 지원합니다.

### 주요 기능

- ✅ **성과지표 관리**: 연도별 성과지표 생성, 수정, 조회, 삭제
- 📊 **다차원 지표 체계**: 투입-과정-산출-결과 4단계 지표
- 🧮 **자동 점수 계산**: 가중평균 기반 성과 점수 자동 산출
- 📈 **시각화 대시보드**: Chart.js 기반 게이지 차트 및 상세 통계
- 🤖 **AI 어시스턴트**: Azure OpenAI 기반 지표 제안 기능
- 🔐 **JWT 인증**: 사용자 인증 및 권한 관리
- 🌐 **다국어 지원**: fluent-templates 기반 i18n
- 📧 **이메일 인증**: 회원가입 이메일 인증 및 비밀번호 재설정

## 빠른 시작

### 사전 요구사항

- Rust 1.70 이상
- SQLite (기본) 또는 PostgreSQL

### 설치 및 실행

```bash
# 개발 서버 시작
cargo loco start

# 브라우저에서 접속
# http://localhost:5150
```

서버가 시작되면 다음과 같은 Loco 로고와 함께 실행됩니다:

```text
                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs

environment: development
   database: automigrate
     logger: debug
compilation: debug
      modes: server

listening on http://localhost:5150
```

### 기타 명령어

```bash
# 테스트 실행
cargo test

# 특정 테스트 실행
cargo test test_can_create_indicator

# 마이그레이션 생성
cargo loco generate migration <name>

# 모델 생성
cargo loco generate model <name> <field:type>...

# 컨트롤러 생성
cargo loco generate controller <name>

# 프로덕션 빌드
cargo build --release
```

## 아키텍처 개요

PI Manager는 **Loco SaaS 프레임워크**를 기반으로 구축되었으며, 논리 모델(Logic Model) 기반의 성과평가 체계를 구현합니다.

### 도메인 모델

```text
PerformanceIndicator (성과지표)
├── InputIndices (투입지표)      - 투입되는 자원, 인력, 예산
├── ProcessIndices (과정지표)    - 진행 과정, 활동, 프로세스
├── OutputIndices (산출지표)     - 직접적인 결과물, 생산량
└── OutcomeIndices (결과지표)    - 최종 성과, 영향력, 변화
```

각 하위 지표는 `performance_indicator_id`로 상위 성과지표와 연결되며, 다음 정보를 포함합니다:

- **name**: 지표명
- **description**: 상세 설명
- **target_value**: 목표값
- **actual_value**: 실제값
- **weight**: 가중치 (점수 계산 시 사용)

### 점수 계산 로직

성과 점수는 다음 공식으로 자동 계산됩니다:

```text
각 지표 유형별 점수 = Σ [(실제값 / 목표값) × 100 × 가중치] / Σ 가중치
최종 점수 = (투입점수 + 과정점수 + 산출점수 + 결과점수) / 지표 유형 수
```

## API 엔드포인트

### 성과지표 API

```http
GET    /api/performance-indicators           - 전체 성과지표 조회
POST   /api/performance-indicators           - 성과지표 생성
GET    /api/performance-indicators/{id}      - 상세 조회 (하위지표 포함)
PUT    /api/performance-indicators/{id}      - 성과지표 수정
DELETE /api/performance-indicators/{id}      - 성과지표 삭제
GET    /api/performance-indicators/{id}/score - 점수만 조회
```

### 하위지표 API

각 지표 유형(input, process, output, outcome)은 동일한 CRUD 패턴을 따릅니다:

```http
GET    /api/{type}-indices/by-indicator/{pi_id}  - 성과지표별 조회
POST   /api/{type}-indices                        - 지표 생성
PUT    /api/{type}-indices/{id}                   - 지표 수정
DELETE /api/{type}-indices/{id}                   - 지표 삭제
```

### 인증 API

```http
POST   /api/auth/register                     - 사용자 등록
GET    /api/auth/verify/{token}               - 이메일 인증
POST   /api/auth/login                        - 로그인 (JWT 발급)
POST   /api/auth/forgot                       - 비밀번호 재설정 요청
POST   /api/auth/reset                        - 비밀번호 재설정
GET    /api/auth/current                      - 현재 사용자 정보 (JWT 필요)
POST   /api/auth/magic-link                   - 매직링크 요청
GET    /api/auth/magic-link/{token}           - 매직링크 인증
```

### AI 어시스턴트 API

```http
POST   /api/ai/chat                           - 지표 제안 요청
```

**요청 예시:**

```json
{
  "message": "교육 프로그램 운영과 관련된 지표를 만들고 싶어요",
  "indicator_type": "input"
}
```

**응답 예시:**

```json
{
  "message": "교육 프로그램 운영을 위한 투입지표 3개를 제안드립니다.",
  "suggestions": [
    {
      "name": "교육 강사 인력",
      "description": "프로그램 운영에 투입된 강사 수",
      "target_value": 10.0,
      "actual_value": 0.0,
      "weight": 0.3
    }
  ]
}
```

### 페이지 (서버 사이드 렌더링)

```http
GET    /                                       - 대시보드 (성과지표 목록)
GET    /indicators/new                         - 성과지표 생성 폼
GET    /indicators/{id}                        - 성과지표 상세 페이지
```

## 프로젝트 구조

```text
pi-manager/
├── src/
│   ├── app.rs                    # 앱 진입점, 라우팅, 훅
│   ├── controllers/              # HTTP 핸들러
│   │   ├── auth.rs               # 인증 API
│   │   ├── performance_indicators.rs
│   │   ├── indices.rs            # 하위지표 CRUD
│   │   ├── ai_assistant.rs       # AI 어시스턴트
│   │   └── pages.rs              # SSR 페이지
│   ├── models/                   # 도메인 모델 + ORM
│   │   ├── performance_indicators.rs
│   │   ├── input_indices.rs
│   │   ├── process_indices.rs
│   │   ├── output_indices.rs
│   │   ├── outcome_indices.rs
│   │   ├── users.rs
│   │   └── _entities/            # SeaORM 자동 생성 (수정 금지)
│   ├── views/                    # JSON 응답 포맷터
│   ├── workers/                  # 백그라운드 작업
│   │   └── download.rs
│   ├── initializers/             # 앱 초기화
│   │   ├── view_engine.rs        # Tera 템플릿 엔진
│   │   └── seed_data.rs          # 개발 환경 시드 데이터
│   ├── mailers/                  # 이메일 발송
│   ├── tasks/                    # CLI 작업
│   └── fixtures/                 # 시드 데이터 (YAML)
├── migration/                    # SeaORM 마이그레이션
│   └── src/
│       ├── m20220101_000001_users.rs
│       ├── m20240101_000002_performance_indicators.rs
│       ├── m20240101_000003_input_indices.rs
│       ├── m20240101_000004_process_indices.rs
│       ├── m20240101_000005_output_indices.rs
│       └── m20240101_000006_outcome_indices.rs
├── assets/
│   ├── views/                    # Tera 템플릿
│   │   ├── base.html             # 기본 레이아웃
│   │   ├── dashboard/            # 대시보드 페이지
│   │   └── home/
│   ├── static/                   # 정적 파일
│   └── i18n/                     # 다국어 파일
├── config/                       # 환경별 설정
│   ├── development.yaml          # 개발 환경
│   ├── test.yaml                 # 테스트 환경
│   └── production.yaml           # 프로덕션 환경
├── tests/                        # 통합 테스트
│   ├── models/
│   └── requests/
├── Cargo.toml
└── README.md
```

## 기술 스택

### 코어 프레임워크

- **[Loco](https://loco.rs) 0.16** - Rails-like Rust 웹 프레임워크
- **[Axum](https://github.com/tokio-rs/axum) 0.8** - 웹 서버
- **[Tokio](https://tokio.rs) 1.45** - 비동기 런타임

### 데이터베이스

- **[SeaORM](https://www.sea-ql.org/SeaORM/) 1.1** - ORM
- **SQLite** (기본) / **PostgreSQL** 지원

### 템플릿 & UI

- **[Tera](https://tera.netlify.app/)** - 서버 사이드 템플릿 엔진
- **[fluent-templates](https://github.com/XAMPPRocky/fluent-templates) 0.13** - i18n 다국어 지원
- **[Chart.js](https://www.chartjs.org/) 4.4** - 게이지 차트 시각화

### 인증 & 보안

- **JWT** - JSON Web Token 기반 인증
- **bcrypt** - 비밀번호 암호화

### AI & HTTP

- **Azure OpenAI API** - AI 어시스턴트 기능
- **[reqwest](https://github.com/seanmonstar/reqwest) 0.12** - HTTP 클라이언트

### 개발 & 테스트

- **[rstest](https://github.com/la10736/rstest) 0.25** - 파라미터화된 테스트
- **[insta](https://github.com/mitsuhiko/insta) 1.34** - 스냅샷 테스트
- **[serial_test](https://github.com/palfrey/serial_test) 3.1** - 직렬 테스트 실행

### 기타 유틸리티

- **serde 1** - 직렬화/역직렬화
- **chrono 0.4** - 날짜/시간
- **uuid 1.6** - UUID 생성
- **validator 0.20** - 데이터 유효성 검증
- **regex 1.11** - 정규식

## 설정

### 환경변수

데이터베이스 연결 및 AI 기능을 위한 환경변수를 설정할 수 있습니다:

```bash
# 데이터베이스 (선택사항, 기본값: SQLite)
export DATABASE_URL="sqlite://pi-manager_development.sqlite?mode=rwc"
# 또는 PostgreSQL
export DATABASE_URL="postgresql://user:password@localhost/pi-manager"

# 데이터베이스 연결 설정
export DB_CONNECT_TIMEOUT=500
export DB_IDLE_TIMEOUT=500
export DB_MIN_CONNECTIONS=1
export DB_MAX_CONNECTIONS=10

# Azure OpenAI (AI 어시스턴트 기능용)
export AZURE_OPENAI_API_KEY="your-api-key"
export AZURE_OPENAI_ENDPOINT="https://your-endpoint.openai.azure.com"
export AZURE_OPENAI_API_VERSION="2024-02-15-preview"
export AZURE_OPENAI_DEPLOYMENT_NAME="gpt-4"
```

### 설정 파일

**config/development.yaml** (개발 환경):

```yaml
server:
  port: 5150

database:
  uri: sqlite://pi-manager_development.sqlite?mode=rwc
  enable_logging: false
  connect_timeout: 500
  auto_migrate: true

auth:
  jwt:
    secret: 4iotyVoXxFYGdQiP84O5
    expiration: 604800  # 7일

mailer:
  smtp:
    host: localhost
    port: 1025
```

**config/test.yaml** (테스트 환경):

```yaml
server:
  port: 5150

database:
  uri: sqlite://pi-manager_test.sqlite?mode=rwc
  enable_logging: false
  dangerously_truncate: true
  dangerously_recreate: true

workers:
  mode: ForegroundBlocking

mailer:
  stub: true
```

## 개발 가이드

### 새로운 모델 추가

1. 마이그레이션 생성:

   ```bash
   cargo loco generate migration create_my_table
   ```

2. `migration/src/mXXXXXXXX_create_my_table.rs` 편집

3. 마이그레이션 실행:

   ```bash
   cargo loco db migrate
   ```

4. 엔티티 생성:

   ```bash
   cargo loco generate model MyModel field1:string field2:int
   ```

### 새로운 컨트롤러 추가

```bash
cargo loco generate controller my_feature
```

생성된 파일:

- `src/controllers/my_feature.rs` - 컨트롤러 로직
- `tests/requests/my_feature.rs` - 통합 테스트

`src/app.rs`의 `routes()` 메서드에 라우트 등록:

```rust
.nest("/api/my-feature", controllers::my_feature::routes())
```

### 시드 데이터 추가

`src/fixtures/` 디렉토리에 YAML 파일 생성:

```yaml
# src/fixtures/performance_indicators.yaml
- name: "교육 프로그램 운영"
  description: "연간 교육 프로그램 운영 성과"
  year: 2024
  target_value: 100.0
  actual_value: 85.0
  unit: "점"
  status: "Active"
```

개발 환경에서 자동으로 로드됩니다 (`SeedDataInitializer` 참고).

## 테스트

### 전체 테스트 실행

```bash
cargo test
```

### 특정 모듈 테스트

```bash
# 모델 테스트
cargo test --test models

# 요청 테스트
cargo test --test requests

# 특정 테스트
cargo test test_can_calculate_score
```

### 테스트 커버리지

주요 테스트 파일:

- `tests/models/performance_indicators.rs` - 도메인 로직 테스트
- `tests/models/indices.rs` - 하위지표 CRUD 테스트
- `tests/requests/auth.rs` - 인증 API 테스트
- `tests/requests/performance_indicators.rs` - 성과지표 API 테스트

### 스냅샷 테스트

insta를 사용한 스냅샷 테스트:

```bash
# 스냅샷 검토
cargo insta review

# 스냅샷 승인
cargo insta accept
```

## 배포

### 프로덕션 빌드

```bash
cargo build --release
```

### 환경 설정

`config/production.yaml` 파일을 프로덕션 환경에 맞게 설정하고, 환경변수를 설정합니다:

```bash
export LOCO_ENV=production
export DATABASE_URL="postgresql://user:password@db-host/pi-manager"
export JWT_SECRET="your-secure-secret-key"
```

### 실행

```bash
./target/release/pi-manager start
```

## 라이선스

이 프로젝트는 Loco 프레임워크의 SaaS starter를 기반으로 구축되었습니다.

## 참고 문서

- [Loco 공식 문서](https://loco.rs/docs/)
- [Loco 빠른 시작](https://loco.rs/docs/getting-started/tour/)
- [Loco 완전 가이드](https://loco.rs/docs/getting-started/guide/)
- [SeaORM 문서](https://www.sea-ql.org/SeaORM/docs/index)
- [Tera 템플릿 문서](https://tera.netlify.app/docs/)

## 기여

이슈나 개선 사항이 있다면 자유롭게 제안해 주세요!
