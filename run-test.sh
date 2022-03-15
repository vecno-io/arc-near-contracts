#!/bin/bash
set -e

cd ./arc-app && cargo test -- --nocapture && cd .. 
cd ./arc-core && cargo test -- --nocapture && cd .. 
cd ./arc-market && cargo test -- --nocapture && cd ..
