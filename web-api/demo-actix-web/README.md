# README

## How to create a Rust project

```bash
cargo new {project-name}
cd {project-name}
cargo add actix-web
cargo add clap --features=derive

# edit ./src/main.rs

cargo check
cargo test
cargo run -- --port 8080
cargo build --release
./target/release/{project-name}.exe --port 8080
```
