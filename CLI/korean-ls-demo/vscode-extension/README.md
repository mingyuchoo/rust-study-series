# Korean Language VSCode 확장 프로그램

한국어 프로그래밍 언어를 위한 VSCode 확장 프로그램입니다.

## 개발

### 설치

```bash
npm install
```

### 컴파일

```bash
npm run compile
```

### Watch 모드

```bash
npm run watch
```

### 디버깅

1. VSCode에서 이 폴더 열기
2. F5 키를 눌러 Extension Development Host 실행

## 주요 의존성

- **vscode-languageclient ^9.0.1**: LSP 클라이언트
- **typescript ^5.0.0**: TypeScript 컴파일러
- **@types/vscode ^1.75.0**: VSCode API 타입 정의

## 지원 언어

- 언어 ID: `korean`
- 파일 확장자: `.kr`, `.한국어`
- 별칭: Korean, 한국어

## 요구사항

- Rust Language Server가 빌드되어 있어야 합니다
- 서버 경로: `../rust-server/target/release/korean-language-server`
