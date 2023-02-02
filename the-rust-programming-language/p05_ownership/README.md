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
