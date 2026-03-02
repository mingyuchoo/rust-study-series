#!/usr/bin/env bash

rustup target add wasm32-unknown-unknown

cargo install wasm-pack && \
wasm-pack build --target web --out-dir pkg
