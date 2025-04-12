# README


## Prerequisites

For Ubuntu Linux

```bash
$ sudo apt update
$ sudo apt install libwebkit2gtk-4.1-dev \
                   build-essential \
                   pkg-config \
                   libgtk-3-dev \
                   libssl-dev \
                   libsoup-3.0-dev \
                   libxdo-dev
```

```bash
$ cargo install cargo-binstall
$ cargo binstall dioxus-cli
```

## Quick start

### Create a new project

```bash
$ dx new my-app
$ cd my-app
```

### Start server

```bash
$ dx serve --platform desktop
```
