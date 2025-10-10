# Iced Counter

A simple counter application built with the Iced GUI framework for Rust. This project demonstrates basic GUI interactions with increment and decrement buttons.

## Features

- Interactive counter with increment/decrement buttons
- Clean, modern GUI using Iced framework
- Real-time value display

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

## Installation

1. Clone or create this project:
```shell
cargo new iced-app
cd iced-app
```

2. Add the Iced dependency:
```shell
cargo add iced
```

Or manually add to your `Cargo.toml`:
```toml
[dependencies]
iced = "0.13.1"
```

## How to run

```shell
cargo run
```

This will compile and launch the counter application window.

## Project Structure

- `src/main.rs` - Main application code with Counter struct and GUI logic
- `Cargo.toml` - Project configuration and dependencies

## Troubleshooting

### Linker Error: "invalid linker name in argument '-fuse-ld=mold'"

If you encounter this error during compilation, it means the `mold` linker is configured but not installed. Install it using:

**Fedora/RHEL:**
```shell
sudo dnf install mold
```

**Ubuntu/Debian:**
```shell
sudo apt install mold
```

**Arch Linux:**
```shell
sudo pacman -S mold
```

Alternatively, you can disable mold by removing any linker configuration from your Rust config files.

## References

- [Example snippets](https://redandgreen.co.uk/iced-rs-example-snippets/rust-programming/)
- [Iced Documentation](https://docs.rs/iced/)