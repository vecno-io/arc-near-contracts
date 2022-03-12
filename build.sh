#!/bin/bash
set -e

cd ./arc-app && ./build.sh && cd .. 
cd ./arc-core && ./build.sh && cd .. 
cd ./arc-market && ./build.sh && cd ..
