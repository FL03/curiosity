#!/usr/bin/env bash
rustup update
rustup default nightly
rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
rustup component add clippy rustfmt --toolchain nightly