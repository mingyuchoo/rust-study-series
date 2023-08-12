# p16_crates_io

## How to create rust workspace

```bash
rustup default stable
rustup update stable

mkdir <workspace_name>
cd <workspace_name>
touch Cargo.toml
```

edit `<workspace_name>/Cargo.toml`

```toml
[workspace]

members = [
  "<binary_crate_name>",
]
```

add a binary crate

```bash
cargo new <binary_crate_name>
```

edit `<workspace_name>/Cargo.toml`

```toml
[workspace]

members = [
  "<binary_crate_name>",
  "<library_crate_name>", # add HERE
]
```

add a library crate

```bash
cargo new <library_crate_name> --lib
```

## How to build

```bash
cargo clean
cargo fmt
cargo test
cargo doc  # cargo doc --open
cargo build --profile dev     # for development
cargo build --profile release # for release
cargo run -p adder # or cargo run
```

## How to use watch mode

### Install `cargo-watch` for watch mode

```bash
cargo install cargo-watch
```

### Run as watch mode with `cargo-watch`

```bash
# Run test only
$ cargo watch -x test

# Run check then tests
$ cargo watch -x check -x test

# Run run current application
cargo watch -x 'run --bin app'

# Run run with arguments
$ cargo watch -x 'run -- --some-arg'

# run an arbitrary command
$ cargo watch -- echo Hello world

# Run with features passed to carg
$ cargo watch --features "foo,bar"
```

## Important

**Disallow hyphens in Rust crate names, but continue allowing them in Cargo packages.**

-<https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md>
