<p align="center">
  <a href="https://github.com/mingyuchoo/rust-study-series/blob/main/LICENSE"><img alt="license" src="https://img.shields.io/github/license/mingyuchoo/rust-study-series"/></a>
  <a href="https://github.com/mingyuchoo/rust-study-series/issues"><img alt="Issues" src="https://img.shields.io/github/issues/mingyuchoo/rust-study-series?color=appveyor" /></a>
  <a href="https://github.com/mingyuchoo/rust-study-series/pulls"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/mingyuchoo/rust-study-series?color=appveyor" /></a>
</p>

# rust-study-series

## For NixOS

edit `/etc/nixos/configuration.nix` in root account

```nix
{ config, pkgs, ... }:
{
  users.users.<user_name> = {
  rustup
  }
}
```

run following commands to install stable toolchain in user account

```bash
rustup default stable
rustup component add rls # or `llvm`
rustup component add rust-analysis
rustup component add rust-analyzer
```

run following command to install cargo tools in user account

```bash
cargo install cargo-lambda
cargo install cargo-modules
cargo install cargo-tauri
cargo install cargo-watch
```

## For Nix

```bash
$ sh <(curl -L https://nixos.org/nix/install) --daemon
# or
$ sh <(curl -L https://nixos.org/nix/install) --no-daemon

nix-channel --add https://nixos.org/channels/nixpkgs-unstable nixpkgs
nix-channel --update
```

### How to make dev. environment

```bash
$ nix develop
```

## For Ubuntu

```bash
$ sudo apt update
$ sudo apt install -y musl-tools
```

### How to install Rustup

- <https://rustup.rs/>

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup default stable
$ rustup update stable
```
### Install component

```bash
$ rustup component list
$ rustup component add cargo
$ rustup component add clippy
$ rustup component add llvm-tools
$ rustup component add rls
$ rustup component add rust-analysis
$ rustup component add rust-analyzer
$ rustup component add clippy
```
## Formatting & linting

```bash
$ cargo fmt
$ cargo clippy --fix
```

## How to use watch mode

### Install `cargo-watch` for watch mode

```bash
$ cargo install cargo-watch
```

### Run as watch mode with `cargo-watch`

```bash
# Run test only
$ cargo watch -x test

# Run check then tests
$ cargo watch -x check -x test

# Watch changes in on the `src` and clear the console and then run
$ cargo watch -c -w src -x run

# Run run current application
$ cargo watch -x 'run --bin app'

# Run run with arguments
$ cargo watch -x 'run -- --some-arg'

# run an arbitrary command
$ cargo watch -- echo Hello world

# Run with features passed to carg
$ cargo watch --features "foo,bar"
```

## How to see module structures

### Install `cargo-module`

```bash
$ cargo install cargo-modules
```

### Get structure of modules in your crate

```bash
$ cargo modules generate tree --types
```
