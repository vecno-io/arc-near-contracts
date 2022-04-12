#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

mkdir -p ../build
cp ../target/wasm32-unknown-unknown/release/arc_actor.wasm ../build/arc-actor.wasm
