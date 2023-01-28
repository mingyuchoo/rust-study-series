<p align="center">
  <a href="https://github.com/mingyuchoo/rust-study-series/issues"><img alt="Issues" src="https://img.shields.io/github/issues/mingyuchoo/rust-study-series?color=appveyor" /></a>
  <a href="https://github.com/mingyuchoo/rust-study-series/pulls"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/mingyuchoo/rust-study-series?color=appveyor" /></a>
</p>

# rust-study-series

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
