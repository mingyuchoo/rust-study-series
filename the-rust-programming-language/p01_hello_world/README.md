# README
## How to build and run

```bash
rustup default stable
rustup update stable
rustc main.rs
./main
```

## How to use watch mode

### Install `cargo-watch` for watch mode

```bash
cargo install cargo-watch
```

### Run as watch mode with `cargo-watch`

```bash
# Run test only
cargo watch -x test

# Run check then tests
cargo watch -x check -x test

# Run run current application
cargo watch -x 'run --bin app'

# Run run with arguments
cargo watch -x 'run -- --some-arg'

# run an arbitrary command
cargo watch -- echo Hello world

# Run with features passed to carg
cargo watch --features "foo,bar"
```
