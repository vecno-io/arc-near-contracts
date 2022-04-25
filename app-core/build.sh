#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

mkdir -p ../build
cp ../target/wasm32-unknown-unknown/release/app_core.wasm ../build/app-core.wasm
