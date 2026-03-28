# rdkafka-v028-simple

`rdkafka` 크레이트를 사용한 Kafka 프로듀서/컨슈머 예제 프로젝트입니다.

## 사전 준비

### Ubuntu 사용 시

```bash
sudo apt install -y cmake
sudo apt install -y libssl-dev
sudo apt install -y libsasl2-dev
```

### Kafka Docker 컨테이너 실행

- <https://github.com/mingyuchoo/docker-composes/tree/main/kafka>

## 프로젝트 구조

```text
src/
├── producer.rs    # Kafka 프로듀서
└── consumer.rs    # Kafka 컨슈머
build.rs           # 빌드 스크립트
```

## 주요 의존성

- `rdkafka` 0.32.2 (`cmake-build`, `sasl`, `ssl` 기능)

## 빌드 방법

```bash
cargo build --profile dev     # 개발용
cargo build --profile release # 릴리즈용
```

## 실행 방법

### 프로듀서 실행

```bash
cargo run --bin producer
```

### 컨슈머 실행

```bash
cargo run --bin consumer
```

## 참고 자료

- <https://github.com/confluentinc/examples>
- <https://dev.to/abhirockzz/getting-started-with-kafka-and-rust-part-1-4hkb>
