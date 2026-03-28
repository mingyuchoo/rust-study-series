# Scripts

Rust 스크립트 실행 방법을 소개합니다.

## `rust-script`를 사용하여 Rust 코드 실행

### 설치

```bash
cargo install rust-script
```

### 실행

```bash
rust-script <rust-filename>.rs
```

## Shebang을 사용하여 Rust 코드 실행

### 예제 코드

```rust
#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! serde = {version = "1.0", features = ["derive"]}
//! serde_json = "1.0"
//! ```

fn main() {
    println!("Hello, World!");
}
```

```bash
chmod +x example.rs

./example.rs
```

## `cargo-eval`을 사용하여 Rust 코드 실행

### 설치

```bash
cargo install cargo-eval

cargo eval <rust-filename>.rs
```

## 포함된 예제

- `example.rs` - serde를 사용한 JSON 직렬화 예제 (rust-script shebang 방식)
