# foundry-cli

Foundry Local을 사용하는 CLI 애플리케이션 예제입니다.

## 실행

```bash
cargo run -p foundry-cli
```

또는

```bash
cargo make run
```

## 로그 레벨 설정

```bash
# 디버그 로그 활성화
RUST_LOG=debug cargo run -p foundry-cli

# Info 레벨 (기본값)
RUST_LOG=info cargo run -p foundry-cli
```

## 코드 구조

이 CLI는 `foundry-client`와 `foundry-types` 라이브러리를 사용하여 Foundry Local API와 상호작용합니다.

주요 기능:
- Foundry Local 클라이언트 초기화
- 채팅 완성 요청 생성 및 전송
- 응답 처리 및 출력
