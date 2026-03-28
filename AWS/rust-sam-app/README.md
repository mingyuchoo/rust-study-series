# rust-sam-app

SAM CLI로 생성한 Rust 기반 서버리스 애플리케이션입니다. AWS Lambda 함수와 API Gateway API를 포함합니다.

## 프로젝트 구조

- `rust_app/Cargo.toml` - 프로젝트 설정 파일
- `rust_app/src/get_hello_world.rs` - GET 요청 Lambda 함수
- `rust_app/src/post_hello_world.rs` - POST 요청 Lambda 함수
- `template.yaml` - AWS 리소스 정의 템플릿

## 주요 의존성

| 패키지 | 버전 | 설명 |
|--------|------|------|
| `lambda_http` | 0.13.0 | Lambda HTTP 이벤트 처리 |
| `lambda_runtime` | 0.13.0 | Lambda 런타임 |
| `serde` | 1.0.214 | 직렬화/역직렬화 |
| `serde_json` | 1.0.132 | JSON 처리 |
| `tokio` | 1.44.2 | 비동기 런타임 |
| `tracing` | 0.1.40 | 구조화 로깅 |

## 사전 요구사항

- Rust 1.66.0 이상
- SAM CLI - [설치 가이드](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
- Docker - [설치 가이드](https://hub.docker.com/search/?type=edition&offering=community)
- [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) (크로스 컴파일용)

## 배포 방법

```bash
sam build
sam deploy
```

또는 가이드 모드로 배포:

```bash
sam deploy --guided
```

## 로컬 테스트

### 함수 직접 호출

```bash
sam local invoke HelloWorldFunction --event events/event-name.json
```

### API Gateway 에뮬레이션

```bash
sam local start-api
```

```bash
curl -X GET http://localhost:3000/hello?name=Matthew
```

## 로그 확인

```bash
sam logs -n HelloWorldFunction --stack-name rust-sam-app --tail
```

## 테스트

```bash
cargo test
```

## 정리 (삭제)

```bash
sam delete
```

## 참고 자료

- [AWS SAM 개발자 가이드](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/what-is-sam.html)
- [AWS Serverless Application Repository](https://aws.amazon.com/serverless/serverlessrepo/)
