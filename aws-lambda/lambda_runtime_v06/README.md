# How to build AWS Lambda for Rust

## Prerequisite

```bash
sudo apt update
sudo apt install -y musl-tools

# Install rustup here

rustup default stable
rustup target add x86_64-unknown-linux-musl
cargo install cargo-lambda
```

## Create a project for Lambda for Rust

```bash
cargo lambda new <project-name>
cd <project-name>
```

## Improve your code

Change your Cargo.toml and *.rs

## Run your Lambda function locally

```bash
cargo lambda watch
```

## Test your Lambda

```bash
# for test
cargo lambda invoke --data-ascii '{"command": "hi"}'
```

## Build your Lambda for deploy

```bash
cargo lambda build --release --target x86_64-unknown-linux-musl
```

## Deploy your Lambda to AWS

```bash
cargo lambda deploy
```

## References

- <https://www.cargo-lambda.info/>
