# React Frontend - Azure Bicep Lab

이 디렉토리는 Azure Bicep Lab 프로젝트의 React 프론트엔드 부분입니다. ASP.NET Core 백엔드와 통합되어 하나의 애플리케이션으로 배포됩니다.

## 🛠️ 기술 스택

- **Framework**: React 19.1.1 with TypeScript
- **Build Tool**: Vite 7.1.6  
- **Compiler**: SWC (Speedy Web Compiler)
- **Package Manager**: pnpm

## 📁 프로젝트 구조

```
src/frontend/
├── src/
│   ├── App.tsx           # 메인 React 컴포넌트
│   ├── App.css           # 스타일시트
│   └── main.tsx          # 애플리케이션 진입점
├── public/               # 정적 파일
├── package.json          # Node.js 의존성
├── vite.config.ts        # Vite 설정 (백엔드 통합)
└── tsconfig.json         # TypeScript 설정
```

## 🚀 개발 및 빌드

### 개발 서버 실행

```bash
# 의존성 설치
pnpm install

# 개발 서버 시작 (http://localhost:5173)
pnpm run dev
```

### 프로덕션 빌드

```bash
# 빌드 (결과물은 ../backend/wwwroot에 생성)
pnpm run build

# 빌드 결과 미리보기
pnpm run preview
```

## 🔧 Vite 설정 특징

`vite.config.ts`에서 다음과 같이 설정되어 있습니다:

- **빌드 출력**: `../backend/wwwroot`로 설정하여 ASP.NET Core와 통합
- **개발 프록시**: `/api` 요청을 `https://localhost:5173`으로 프록시
- **SWC 컴파일러**: 빠른 빌드를 위해 SWC 사용

## 🌐 API 통합

프론트엔드는 다음 백엔드 API와 통신합니다:

- `GET /api/health` - 헬스 체크
- `GET /api/data` - 샘플 데이터 조회

개발 모드에서는 Vite 프록시를 통해, 프로덕션에서는 동일한 도메인에서 API에 접근합니다.

## 📦 배포

이 React 앱은 빌드 시 자동으로 ASP.NET Core 백엔드의 `wwwroot` 디렉토리에 배포되어 하나의 통합된 애플리케이션으로 동작합니다.

전체 애플리케이션 빌드는 루트 디렉토리에서 실행하세요:

```bash
# 루트 디렉토리에서
./build.sh  # 또는 build.bat, build.ps1
```
