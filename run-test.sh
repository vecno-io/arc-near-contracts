#!/bin/bash
set -e

cd ./standard && cargo test && cd .. 

cd ./arc-actor && cargo test && cd .. 
cd ./arc-guild && cargo test && cd ..
