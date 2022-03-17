#!/bin/bash
set -e

cd ./arc-app && cargo test && cd .. 
cd ./arc-core && cargo test && cd .. 
cd ./arc-market && cargo test && cd ..
