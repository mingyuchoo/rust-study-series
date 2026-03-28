# Azure Bicep Lab - React + Rust(Actix-web) 통합 애플리케이션

Azure Container Apps에 배포 가능한 통합 React + Rust(Actix-web) 애플리케이션입니다. React 프론트엔드는 빌드 시 백엔드 `wwwroot/`로 출력되어 하나의 앱으로 동작합니다.

## 프로젝트 구조

```text
azure-bicep-rust-app/
├── infra/                          # Azure Bicep 인프라 템플릿 및 모듈
├── src/
│   ├── backend/                    # Rust(Actix-web) 백엔드
│   │   ├── src/                    # Rust 소스 코드
│   │   ├── Cargo.toml             # Rust 프로젝트 설정
│   │   ├── Makefile.toml          # cargo-make 태스크
│   │   ├── docs/                  # 문서
│   │   └── todos.db               # SQLite DB (로컬 개발용)
│   └── frontend/                   # React + TypeScript + Vite 프런트엔드
│       ├── src/                    # React 소스
│       ├── package.json            # Node.js 스크립트/의존성
│       └── vite.config.ts          # Vite 구성 (outDir, proxy 등)
├── azure.yaml                      # Azure Developer CLI 설정
└── README.md                       # 본 문서
```

## 빌드 및 실행

### 수동 빌드(로컬 개발용)

```bash
# 1) 프런트엔드 설치 및 빌드 (출력: src/backend/wwwroot)
cd src/frontend
pnpm install
pnpm run build

# 2) 백엔드 실행 (기본 포트: http://localhost:8080)
cd ../backend
cargo run
```

### 개발 모드(HMR + 프록시)

```bash
# 터미널 1: 백엔드 실행
cd src/backend
cargo run

# 터미널 2: 프런트엔드 개발 서버 (http://localhost:5173)
cd src/frontend
pnpm run dev
```

프록시 설정은 `src/frontend/vite.config.ts`에서 `/api -> http://localhost:8080`으로 구성되어 있습니다.

## API 엔드포인트

백엔드는 TODO 관리용 REST API를 제공합니다.

- GET `/api/todos` -- TODO 목록 조회
- POST `/api/todos` -- TODO 생성
- PUT `/api/todos/{id}` -- TODO 수정
- DELETE `/api/todos/{id}` -- TODO 삭제

Swagger UI: `http://localhost:8080/swagger-ui/`

## 기술 스택

### 백엔드

- Framework: Actix-web 4.9
- Language: Rust (edition 2024)
- Features:
  - REST API + SQLite (sqlx)
  - 정적 파일 서빙(`wwwroot/`) 및 SPA Fallback
  - OpenAPI(Swagger) 문서화 (utoipa + utoipa-swagger-ui)
  - UUID 기반 엔티티 식별
  - 날짜/시간 추적 (chrono)

### 프런트엔드

- Framework: React 19 + TypeScript
- Build Tool: Vite (Plugin: `@vitejs/plugin-react-swc`)
- Package Manager: pnpm
- Features:
  - 개발 프록시(`/api -> http://localhost:8080`)
  - 프로덕션 빌드 출력: `../backend/wwwroot`

## Azure 배포(개요)

### Azure Developer CLI(azd) 초기화

```bash
mkdir ${PROJECT_NAME}
cd ${PROJECT_NAME}
azd init
```

프로젝트 루트의 `azure.yaml`을 통해 배포 구성이 관리됩니다. Bicep 템플릿은 `infra/` 디렉터리에 있습니다.

## Bicep 템플릿 빌드 예시

```bash
# Bicep 파일을 JSON으로 빌드
az bicep build --file ${PWD}/infra/main.bicep

# 빌드 결과를 stdout으로 출력
az bicep build --file ${PWD}/infra/main.bicep --stdout

# 특정 디렉토리에 빌드 결과 저장
az bicep build --file ${PWD}/infra/main.bicep --outdir ./output
```

## 시작하기

1. 저장소 클론

   ```bash
   git clone <repository-url>
   cd azure-bicep-rust-app
   ```

2. 프런트엔드 설치 및 빌드 -> 백엔드 실행

   ```bash
   cd src/frontend && pnpm install && pnpm run build
   cd ../backend && cargo run
   ```

3. 브라우저에서 확인

   - 애플리케이션: `http://localhost:8080`
   - Swagger(UI): `http://localhost:8080/swagger-ui/`

4. Azure 배포(선택)

   ```bash
   azd up
   ```
