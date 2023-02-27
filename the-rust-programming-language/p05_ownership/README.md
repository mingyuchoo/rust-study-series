# p05_owership

## How to build and run

```bash
rustup default stable
rustup update stable

cargo check
cargo bulid
cargo test
cargo run
```

## Optional commands

### Update dependencies using in project

```bash
cargo update
```

### Create HTML document and Open it with Web browser

```bash
cargo doc --open
```


## Ownership Rules

First, let’s take a look at the ownership rules.
Keep these rules in mind as we work through the examples that illustrate them:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

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
