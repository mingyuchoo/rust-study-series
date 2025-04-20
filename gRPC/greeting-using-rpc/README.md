# README

## Prerequisites

On Ubuntu Linux

```bssh
sudo apt-get install protobuf-compiler
```


## working process

```bash
cargo new {project-name}
cd {project-name}
touch README.md
mkdir proto data
cargo add tokio --features tokio/macros,rt-multi-thread
cargo add tonic-build --build
cargo add tonic
cargo add prost
cargo add prost-types
touch build.rs
```
## How to run

### Run Server

```bash
cargo run --bin server
```

### Run Client

```bash
cargo run --bin client
```
