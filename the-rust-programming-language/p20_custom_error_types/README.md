# README
## How to build and run

```bash
rustup default stable
rustup update stable

cargo check
cargo bulid --profile dev # for development
cargo build --profile release # for release
cargo test
cargo run --bin main
```

## Optional commands

### Update dependencies using in project

```bash
cargo update
```

### Create HTML document and Open it with Web browser

```bash
cargo doc --no-deps --open  # `target/doc/<package_name>/index.html`
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
