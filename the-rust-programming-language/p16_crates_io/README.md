# p16_crates_io

## How to create rust workspace

```bash
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
cargo build
cargo build --release
cargo run -p adder # or cargo run
```

## Important

**Disallow hyphens in Rust crate names, but continue allowing them in Cargo packages.**

-<https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md>
