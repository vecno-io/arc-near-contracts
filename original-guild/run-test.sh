#!/bin/bash
set -e

cd ./guild-core && cargo test && cd .. 
cd ./guild-standard && cargo test && cd .. 
