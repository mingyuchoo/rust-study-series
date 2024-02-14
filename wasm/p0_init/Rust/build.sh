#! /bin/sh

rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release && \
cp target/wasm32-unknown-unknown/release/deps/*.wasm ../Node
