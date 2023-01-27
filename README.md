# rust-setup-series

## Prerequisite

### For Ubuntu

```
$ sudo apt update
$ sudo apt install -y musl-tools
```

## How to install Rustup

- <https://rustup.rs/>

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup default stable
$ rustup update stable
```
## How to code foramting and linting

### Install component

```bash
$ rustup component add rustfmt
$ rustup component add clippy
```
### Run

```bash
$ cargo fmt
$ cargo clippy --fix
```

## use Watch mode for Actix-web

### Install

```bash
cargo install cargo-watch
```

### Run

```bash
cargo watch -x 'run --bin app'
```
