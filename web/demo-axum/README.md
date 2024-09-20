# README

## How to create a Axum project

```bash
$ cargo new <project-name>
$ cd <project-name>
$ cargo add tokio --features macros,rt-multi-thread
$ cargo add axum

# edit ./src/main.rs

$ cargo check
$ cargo test
$ cargo build --release
$ ./target/release/<project-name>
```
