#!bash

rustup toolchain install stable

rustup target add wasm32-unknown-unknown

cargo install trunk

trunk build --release
