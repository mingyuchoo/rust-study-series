# README

## Run Rust code using `rust-script`

### Install

```bash
cargo install rust-script
```

### Run

```bash
rust-script <rust-filename>.rs
```

## Run Rust code using Shebang

### Example code

```rust
#!/usr/bin/env rust-script
//! ```carg
//! [dependencies]
//! ```

fn main() {
    println!("Hello, World!");
}
```

```bash
chmod +x example.rs

./example.rs
```

## Run Rust code using `cargo-eval`

### Install

```bash
cargo install cargo-eval

cargo eval <rust-filename>rs
```
