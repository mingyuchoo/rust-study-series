# Azure Bicep Lab - React + ASP.NET Core(F#) 통합 애플리케이션

Azure Container Apps에 배포 가능한 통합 React + ASP.NET Core(F#) 애플리케이션입니다. React 프론트엔드는 빌드 시 백엔드 `wwwroot/`로 출력되어 하나의 앱으로 동작합니다.

## 🏗️ 프로젝트 구조

```text
azure-bicep-fsharp-app/
├── infra/                          # Azure Bicep 인프라 템플릿 및 모듈
├── src/
│   ├── backend/                    # ASP.NET Core 9.0 (F#) 백엔드
│   │   ├── Program.fs              # 애플리케이션 진입점 (Minimal API)
│   │   ├── backend.fsproj          # F# 프로젝트 파일
│   │   ├── wwwroot/                # 프런트엔드 빌드 결과물 출력
│   │   ├── appsettings*.json       # 설정 파일
│   │   └── app.db                  # SQLite DB (로컬 개발용)
│   └── frontend/                   # React + TypeScript + Vite 프런트엔드
│       ├── src/                    # React 소스
│       ├── package.json            # Node.js 스크립트/의존성
│       └── vite.config.ts          # Vite 구성 (outDir, proxy 등)
├── azure.yaml                      # Azure Developer CLI 설정
└── README.md                       # 본 문서
```

## 🚀 빌드 및 실행

### 수동 빌드(로컬 개발용)

```bash
# 1) 프런트엔드 설치 및 빌드 (출력: src/backend/wwwroot)
cd src/frontend
pnpm install
pnpm run build

# 2) 백엔드 실행 (기본 포트: https://localhost:7000)
cd ../backend
cargo run
```

### 개발 모드(HMR + 프록시)

```bash
# 터미널 1: 백엔드 실행 (https://localhost:7000)
cd src/backend
cargo run

# 터미널 2: 프런트엔드 개발 서버 (http://localhost:5173)
cd src/frontend
pnpm run dev
```

프록시 설정은 `src/frontend/vite.config.ts`에서 `/api -> https://localhost:7000`으로 구성되어 있습니다.

## 🔌 API 엔드포인트 (Minimal API)

백엔드는 제품(Product) 관리용 엔드포인트를 제공합니다. 구현 파일: `src/backend/Program.fs`

- GET `/api/products` — 제품 목록 조회
- GET `/api/products/{id}` — 제품 단건 조회
- POST `/api/products` — 제품 생성
- PUT `/api/products/{id}` — 제품 수정
- DELETE `/api/products/{id}` — 제품 삭제

추가로 개발 환경에서 Swagger UI가 활성화됩니다: `https://localhost:7000/swagger`

## 🛠️ 기술 스택

### 백엔드

- Framework: ASP.NET Core 9.0
- Language: F#
- Features:
  - Minimal API + SQLite(Local)
  - 정적 파일 서빙(`wwwroot/`) 및 SPA Fallback(`MapFallbackToFile("index.html")`)
  - OpenAPI(Swagger) 문서화(개발 환경)

### 프런트엔드

- Framework: React 19 + TypeScript
- Build Tool: Vite 7 (Plugin: `@vitejs/plugin-react-swc`)
- Package Manager: pnpm
- Features:
  - 개발 프록시(`/api -> https://localhost:7000`)
  - 프로덕션 빌드 출력: `../backend/wwwroot`

## ☁️ Azure 배포(개요)

### Azure Developer CLI(azd) 초기화

```bash
mkdir ${PROJECT_NAME}
cd ${PROJECT_NAME}
azd init
```

프로젝트 루트의 `azure.yaml`을 통해 배포 구성이 관리됩니다. Bicep 템플릿은 `infra/` 디렉터리에 있습니다.

## 📝 Bicep 템플릿 빌드 예시

```bash
# Bicep 파일을 JSON으로 빌드
az bicep build --file ${PWD}/infra/main.bicep

# 빌드 결과를 stdout으로 출력
az bicep build --file ${PWD}/infra/main.bicep --stdout

# 특정 디렉토리에 빌드 결과 저장
az bicep build --file ${PWD}/infra/main.bicep --outdir ./output
```

## 🚦 시작하기

1. 저장소 클론

   ```bash
   git clone <repository-url>
   cd azure-bicep-fsharp-app
   ```

2. 프런트엔드 설치 및 빌드 → 백엔드 실행

   ```bash
   cd src/frontend && pnpm install && pnpm run build
   cd ../backend && cargo run
   ```

3. 브라우저에서 확인

   - 애플리케이션: `https://localhost:7000`
   - Swagger(UI): `https://localhost:7000/swagger`

4. Azure 배포(선택)

   ```bash
   azd up
   ```
