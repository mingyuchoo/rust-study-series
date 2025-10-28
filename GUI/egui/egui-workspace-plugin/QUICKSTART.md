# Quick Start Guide

File Converter App을 빠르게 시작하는 방법입니다.

## 5분 안에 시작하기

### 1. 필수 요구사항 확인

Rust가 설치되어 있는지 확인:

```bash
rustc --version
```

설치되어 있지 않다면 [rustup.rs](https://rustup.rs/)에서 설치하세요.

### 2. 프로젝트 빌드

```bash
# 릴리스 빌드 (권장)
cargo build --release

# 또는 개발 빌드 (빠르지만 최적화되지 않음)
cargo build
```

### 3. 애플리케이션 실행

```bash
# 릴리스 버전 실행
cargo run -p gui --release

# 또는 개발 버전 실행
cargo run -p gui
```

### 4. 첫 번째 변환 실행

1. **파일 선택**: "파일 선택..." 버튼 클릭
2. **텍스트 파일 선택**: `.txt` 파일 선택
3. **출력 형식 선택**: "Plain Text" 선택
4. **변환 실행**: "변환 시작" 버튼 클릭
5. **결과 확인**: 변환된 파일이 생성됩니다

## 주요 기능 둘러보기

### 변환 탭

- 단일 또는 다중 파일 선택
- 출력 형식 선택
- 변환 진행 상태 확인

### 이력 탭

- 최근 변환 작업 조회
- 변환 상세 정보 확인
- 성공/실패 상태 확인

### 설정 탭

- 기본 출력 디렉토리 설정
- 테마 변경 (Light/Dark/System)
- 언어 설정

## 일반적인 사용 시나리오

### 시나리오 1: 단일 파일 변환

```
1. 변환 탭 선택
2. "파일 선택..." 클릭
3. 파일 하나 선택
4. 출력 형식 선택
5. "변환 시작" 클릭
```

### 시나리오 2: 여러 파일 일괄 변환

```
1. 변환 탭 선택
2. "파일 선택..." 클릭
3. Ctrl/Cmd를 누른 채로 여러 파일 선택
4. 출력 형식 선택
5. "변환 시작" 클릭
6. 진행률 바에서 전체 진행 상태 확인
```

### 시나리오 3: 텍스트 인코딩 변환

```
1. 변환 탭 선택
2. 텍스트 파일 선택
3. "Plain Text" 출력 형식 선택
4. 변환 옵션에서 대상 인코딩 지정 (예: UTF-8, EUC-KR)
5. "변환 시작" 클릭
```

## 문제 해결

### 애플리케이션이 시작되지 않음

```bash
# 의존성 확인
cargo check --workspace

# 클린 빌드
cargo clean
cargo build --release
```

### 플러그인이 로드되지 않음

```bash
# 플러그인 빌드 확인
cargo build -p text-converter --lib

# 로그 확인
RUST_LOG=debug cargo run -p gui
```

### 변환이 실패함

1. 이력 탭에서 에러 메시지 확인
2. 입력 파일 권한 확인
3. 출력 디렉토리 쓰기 권한 확인
4. 로그 확인: `RUST_LOG=debug cargo run -p gui`

## 다음 단계

- [전체 문서 읽기](README.md)
- [플러그인 개발하기](plugins/text-converter/README.md)
- [프로젝트에 기여하기](CONTRIBUTING.md)

## 도움이 필요하신가요?

- 버그 리포트: GitHub Issues
- 기능 제안: GitHub Issues
- 질문: GitHub Discussions

즐거운 파일 변환 되세요! 🚀
