# Windows Foundry Local 예제 (Rust)

Foundry Local Rust SDK를 사용하여 로컬에서 AI 모델과 상호작용하는 간단한 예제입니다.

## 필수 조건

- Rust 1.70.0 이상
- Foundry Local이 설치되어 있고 PATH에서 사용 가능해야 함

## Windows Foundry Local 설치하기

```bash
winget install Microsoft.FoundryLocal
foundry model run phi-4-mini
```

## 샘플 실행하기

1. Foundry Local이 설치되어 있는지 확인하세요
2. 샘플을 실행하세요:

```bash
cargo run
```

## 이 샘플이 하는 일

1. FoundryLocalManager 인스턴스를 생성합니다
2. Foundry Local 서비스가 아직 실행 중이 아니라면 시작합니다
3. phi-4-mini 모델을 다운로드하고 로드합니다
4. OpenAI 호환 API를 사용하여 모델에 프롬프트를 전송합니다
5. 모델의 응답을 표시합니다

## 코드 구조

- `src/main.rs` - 메인 애플리케이션 코드
- `Cargo.toml` - 프로젝트 구성 및 의존성
