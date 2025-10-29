# 한국어 Language Server

한국어 프로그래밍 언어를 위한 Language Server Protocol (LSP) 구현입니다.

## 개요

이 프로젝트는 한국어 키워드를 사용하는 프로그래밍 언어를 위한 Language Server를 제공합니다. VSCode 확장 프로그램과 함께 사용하여 한국어 코드 작성 시 자동완성, 호버 정보, 진단 기능을 제공합니다.

## 기능

- **자동완성**: 한국어 키워드 자동완성 지원
- **호버 정보**: 키워드에 마우스를 올리면 상세 정보 표시
- **진단**: 코드 작성 시 실시간 오류 및 경고 표시
- **문서 동기화**: 파일 열기, 변경, 저장, 닫기 이벤트 처리

## 지원 키워드

- `함수` - 함수 선언
- `변수` - 변수 선언
- `만약` - 조건문
- `아니면` - else 조건
- `반복` - 반복문
- `출력` - 콘솔 출력
- `참` / `거짓` - 불린 값

## 프로젝트 구조

```
.
├── rust-server/           # Rust Language Server
│   ├── src/
│   │   └── main.rs       # Language Server 구현
│   └── Cargo.toml        # Rust 의존성
│
└── vscode-extension/      # VSCode 확장 프로그램
    ├── src/
    │   └── extension.ts  # 확장 프로그램 진입점
    ├── package.json      # 확장 프로그램 설정
    └── tsconfig.json     # TypeScript 설정
```

## 설치 및 빌드

### 필요 사항

- Rust (최신 stable 버전)
- Node.js 및 npm
- VSCode

### 1. Rust Language Server 빌드

```bash
cd rust-server
cargo build --release
```

빌드된 바이너리는 `rust-server/target/release/korean-language-server`에 생성됩니다.

### 2. VSCode 확장 프로그램 빌드

```bash
cd vscode-extension
npm install
npm run compile
```

컴파일된 파일은 `vscode-extension/out/` 디렉터리에 생성됩니다.

### 3. 확장 프로그램 설치

VSCode에서 개발 모드로 실행:
1. VSCode에서 `vscode-extension` 폴더 열기
2. F5 키를 눌러 Extension Development Host 실행

또는 확장 프로그램 패키징:
```bash
cd vscode-extension
npm install -g @vscode/vsce
vsce package
```

## 사용법

1. `.kr` 또는 `.한국어` 확장자를 가진 파일 생성
2. 한국어 키워드 입력 시 자동완성 제안 확인
3. 키워드에 마우스를 올려 상세 정보 확인

## 기술 스택

- **Language Server**: Rust + tower-lsp
- **VSCode Extension**: TypeScript + vscode-languageclient
- **프로토콜**: Language Server Protocol (LSP)

## 개발

Language Server는 stdin/stdout을 통해 JSON-RPC 메시지를 주고받습니다. VSCode 확장 프로그램이 클라이언트 역할을 하며, Rust로 작성된 서버와 통신합니다.

## 라이선스

MIT
