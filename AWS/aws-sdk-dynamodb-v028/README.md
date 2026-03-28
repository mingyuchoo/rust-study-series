# aws-sdk-dynamodb-v028

AWS DynamoDB SDK를 사용하는 Rust 프로젝트입니다.

## 프로젝트 설명

- AWS 계정의 DynamoDB 테이블을 조회합니다.

## 주요 의존성

| 패키지 | 버전 | 설명 |
|--------|------|------|
| `aws-config` | 1.5.10 | AWS SDK 설정 |
| `aws-sdk-dynamodb` | 1.53.0 | DynamoDB SDK |
| `tokio` | 1.44.2 | 비동기 런타임 |

## 사전 요구사항

- AWS 계정 및 자격 증명 설정 (`~/.aws/credentials`)
- Rust 툴체인

## 실행 방법

```bash
cargo run
```
