# Contributing to File Converter App

이 문서는 File Converter App 프로젝트에 기여하는 방법을 설명합니다.

## 개발 환경 설정

### 필수 요구사항

- Rust 1.70 이상
- Cargo
- Git

### 저장소 클론

```bash
git clone <repository-url>
cd file-converter-app
```

### 의존성 설치

Rust 프로젝트는 Cargo가 자동으로 의존성을 관리하므로, 별도의 설치 과정이 필요하지 않습니다. 첫 빌드 시 자동으로 다운로드됩니다.

## 개발 워크플로우

### 1. 브랜치 생성

```bash
git checkout -b feature/your-feature-name
```

### 2. 코드 작성

프로젝트는 Cargo workspace로 구성되어 있습니다:

- `plugin-manager/`: 핵심 비즈니스 로직
- `plugin-interface/`: 플러그인 인터페이스
- `database-manager/`: 데이터베이스 관리
- `app-gui/`: GUI 애플리케이션
- `plugins/`: 플러그인 구현체

### 3. 코드 포맷팅

```bash
# 전체 프로젝트 포맷팅
cargo fmt --all

# 특정 크레이트만 포맷팅
cargo fmt -p plugin-manager
```

### 4. 린트 실행

```bash
# Clippy 실행
cargo clippy --all-targets --all-features -- -D warnings

# 특정 크레이트만 검사
cargo clippy -p plugin-manager
```

### 5. 테스트 실행

```bash
# 전체 테스트
cargo test --workspace

# 특정 크레이트 테스트
cargo test -p plugin-manager

# 통합 테스트만 실행
cargo test -p plugin-manager --test integration_test

# 테스트 출력 표시
cargo test -- --nocapture
```

### 6. 빌드 확인

```bash
# 개발 빌드
cargo build --workspace

# 릴리스 빌드
cargo build --workspace --release
```

### 7. 실행 및 테스트

```bash
# GUI 애플리케이션 실행
cargo run -p app-gui

# 로그 레벨 설정하여 실행
RUST_LOG=debug cargo run -p app-gui
```

## 코딩 가이드라인

### Rust 스타일 가이드

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)를 따릅니다
- `cargo fmt`의 기본 설정을 사용합니다
- `cargo clippy`의 경고를 모두 해결합니다

### 네이밍 컨벤션

- **타입**: `PascalCase` (예: `PluginRegistry`, `ConversionEngine`)
- **함수/변수**: `snake_case` (예: `convert_file`, `plugin_name`)
- **상수**: `SCREAMING_SNAKE_CASE` (예: `MAX_FILE_SIZE`)
- **모듈**: `snake_case` (예: `plugin_interface`)

### 에러 처리

- `thiserror`를 사용하여 커스텀 에러 타입 정의
- `anyhow`를 사용하여 애플리케이션 레벨 에러 처리
- `Result<T, E>`를 명시적으로 반환
- `unwrap()`이나 `expect()` 사용 최소화

### 문서화

- 모든 public API에 문서 주석 작성
- 예제 코드 포함 권장
- 복잡한 로직에는 인라인 주석 추가

```rust
/// 파일을 변환합니다.
///
/// # Arguments
///
/// * `input_path` - 입력 파일 경로
/// * `output_format` - 출력 형식
/// * `options` - 변환 옵션
///
/// # Returns
///
/// 변환 결과를 담은 `ConversionResult`
///
/// # Errors
///
/// 파일을 읽을 수 없거나 변환에 실패하면 에러를 반환합니다.
///
/// # Example
///
/// ```no_run
/// use plugin_manager::ConversionEngine;
/// 
/// let engine = ConversionEngine::new(registry);
/// let result = engine.convert_file(&input, &format, "plugin", &options)?;
/// ```
pub fn convert_file(
    &self,
    input_path: &Path,
    output_format: &FileFormat,
    plugin_name: &str,
    options: &ConversionOptions,
) -> Result<ConversionResult> {
    // 구현
}
```

## 테스트 작성

### 단위 테스트

각 모듈의 `tests` 모듈에 작성:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // 테스트 코드
        assert_eq!(expected, actual);
    }
}
```

### 통합 테스트

`tests/` 디렉토리에 작성:

```rust
// plugin-manager/tests/integration_test.rs
use plugin_manager::*;

#[test]
fn test_end_to_end_conversion() {
    // 통합 테스트 코드
}
```

### 테스트 커버리지

- 핵심 비즈니스 로직은 반드시 테스트 작성
- 에러 케이스도 테스트에 포함
- 엣지 케이스 고려

## 플러그인 개발

새로운 플러그인을 개발하려면:

1. `plugins/` 디렉토리에 새 크레이트 생성
2. `Plugin` 트레이트 구현
3. `create_plugin()` 함수 추가
4. 테스트 작성
5. README.md 작성

자세한 내용은 [플러그인 개발 가이드](plugins/text-converter/README.md)를 참조하세요.

## 커밋 메시지 가이드라인

### 형식

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat`: 새로운 기능
- `fix`: 버그 수정
- `docs`: 문서 변경
- `style`: 코드 포맷팅, 세미콜론 누락 등
- `refactor`: 코드 리팩토링
- `test`: 테스트 추가 또는 수정
- `chore`: 빌드 프로세스, 도구 설정 등

### Scope

- `core`: 핵심 시스템
- `gui`: GUI 모듈
- `database`: 데이터베이스 모듈
- `plugin`: 플러그인 관련
- `docs`: 문서

### 예시

```
feat(core): Add batch conversion support

Implement batch_convert method in ConversionEngine to process
multiple files sequentially. Each file is converted independently,
and errors in one file don't affect others.

Closes #123
```

## Pull Request 프로세스

1. **브랜치 업데이트**: main 브랜치의 최신 변경사항을 가져옵니다
   ```bash
   git checkout main
   git pull origin main
   git checkout your-branch
   git rebase main
   ```

2. **테스트 실행**: 모든 테스트가 통과하는지 확인
   ```bash
   cargo test --workspace
   ```

3. **린트 확인**: Clippy 경고가 없는지 확인
   ```bash
   cargo clippy --all-targets --all-features
   ```

4. **포맷팅**: 코드가 올바르게 포맷되었는지 확인
   ```bash
   cargo fmt --all -- --check
   ```

5. **PR 생성**: 변경사항을 설명하는 PR 생성
   - 무엇을 변경했는지
   - 왜 변경했는지
   - 어떻게 테스트했는지

6. **리뷰 대응**: 리뷰어의 피드백에 응답하고 필요한 수정 진행

## 이슈 리포팅

버그를 발견하거나 기능을 제안하려면:

### 버그 리포트

다음 정보를 포함해주세요:

- 버그 설명
- 재현 단계
- 예상 동작
- 실제 동작
- 환경 정보 (OS, Rust 버전 등)
- 에러 메시지 또는 로그

### 기능 제안

다음 정보를 포함해주세요:

- 기능 설명
- 사용 사례
- 예상되는 이점
- 가능한 구현 방법

## 질문하기

질문이 있으면:

1. 먼저 [README.md](README.md)와 관련 문서를 확인하세요
2. 기존 이슈를 검색하여 같은 질문이 있는지 확인하세요
3. 새로운 이슈를 생성하여 질문하세요

## 라이선스

기여한 코드는 프로젝트의 MIT 라이선스 하에 배포됩니다.

## 감사합니다!

File Converter App 프로젝트에 기여해 주셔서 감사합니다! 🎉
