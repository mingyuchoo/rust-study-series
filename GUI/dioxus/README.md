# Dioxus

## 사전 준비사항

### Ubuntu Linux

```bash
$ sudo apt update
$ sudo apt install libwebkit2gtk-4.1-dev \
                   build-essential       \
                   pkg-config            \
                   libgtk-3-dev          \
                   libssl-dev            \
                   libsoup-3.0-dev       \
                   libxdo-dev
```

### Dioxus CLI 설치

```bash
$ cargo install cargo-binstall
$ cargo binstall dioxus-cli
```

## 빠른 시작

### 새 프로젝트 생성

```bash
$ dx new my-app
$ cd my-app
```

### 서버 실행

```bash
$ dx serve --platform desktop
```
