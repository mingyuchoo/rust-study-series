#! /bin/bash

cargo install wasm-pack && \
wasm-pack build --target web --out-dir pkg
