#!/bin/bash

# set -eo pipefail

wasm-pack build -t web --release

npm i
npm run build

echo "WASM build completed"
