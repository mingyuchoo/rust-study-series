# lambda-runtime-v08

Rust `lambda_runtime` 크레이트를 사용하는 AWS Lambda 함수 예제입니다.

## 참고 자료

- <https://crates.io/crates/lambda_runtime>

## 주요 의존성

| 패키지 | 버전 | 설명 |
|--------|------|------|
| `lambda_http` | 0.13.0 | Lambda HTTP 이벤트 처리 |
| `lambda_runtime` | 0.13.0 | Lambda 런타임 |
| `tokio` | 1.44.2 | 비동기 런타임 |
| `tracing` | 0.1.40 | 구조화 로깅 |
| `tracing-subscriber` | 0.3.18 | 로그 출력 |

## 사전 요구사항

### `cargo-lambda` 설치

- <https://www.cargo-lambda.info/guide/installation.html>

### Lambda 실행 방법

```bash
cargo lambda new <function-name>
cargo lambda watch
  $ # 다른 터미널을 열고 아래 명령 실행
  $ cargo lambda invoke --data-ascii '{"name":"World"}' <function-name>
cargo lambda build --release --output-format zip
```

### NixOS 환경

NixOS를 사용하는 경우 아래 설정을 따릅니다.

`/etc/nixos/configuration.nix`에 `cargo-lambda` 추가:

```nix
{ config, pkgs, ... }:
{
    users.users.{username} = {
        packages = with pkgs; [
            cargo-lambda
        ]
    }
}
```

Nix를 사용하는 경우:

```bash
nix-env -iA nixpkgs.cargo-lambda
```
