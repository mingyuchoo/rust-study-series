# 빌드 가이드

## 빠른 시작

### 전체 빌드 (Makefile 사용)

```bash
# 전체 빌드
make all

# 또는 개별 빌드
make build-server    # Rust 서버만
make build-extension # VSCode 확장만
```

### 수동 빌드

#### 1. Rust Language Server

```bash
cd rust-server
cargo build --release
```

빌드 결과: `rust-server/target/release/korean-language-server`

#### 2. VSCode Extension

```bash
cd vscode-extension
npm install
npm run compile
```

빌드 결과: `vscode-extension/out/extension.js`

## 개발 모드

### Rust 서버 개발

```bash
cd rust-server
cargo watch -x run  # cargo-watch 설치 필요
```

### VSCode 확장 개발

```bash
cd vscode-extension
npm run watch
```

그리고 VSCode에서:
1. `vscode-extension` 폴더 열기
2. F5 키로 Extension Development Host 실행

## 테스트

```bash
# Rust 테스트
cd rust-server
cargo test

# 전체 테스트 (Makefile)
make test
```

## 정리

```bash
make clean
```

## 문제 해결

### Rust 서버가 빌드되지 않는 경우

```bash
cd rust-server
cargo clean
cargo build --release
```

### VSCode 확장이 서버를 찾지 못하는 경우

서버 경로 확인:
```bash
ls -la rust-server/target/release/korean-language-server
```

경로가 올바른지 `vscode-extension/src/extension.ts` 확인
