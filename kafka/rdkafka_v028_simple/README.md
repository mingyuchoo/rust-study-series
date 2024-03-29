# README
## Prerequisite

### For Ubuntu

```bash
sudo apt install -y cmake
sudo apt install -y libssl-dev
sudo apt install -y libsasl2-dev
```

### Run Kafka Docker Container

- <https://github.com/mingyuchoo/docker-composes/tree/main/kafka>

## How to build

```bash
cargo build --profile dev     # for development
cargo build --profile release # for release
```

## How to run

### Run a producer

```bash
cargo run --bin producer
```

### Run a consumer

```bash
cargo run --bin consumer
```

## References

- <https://github.com/confluentinc/examples>
- <https://dev.to/abhirockzz/getting-started-with-kafka-and-rust-part-1-4hkb>
