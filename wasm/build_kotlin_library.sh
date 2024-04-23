#!/bin/sh

set -eo pipefail

wasm-pack build -t web --release

npm i
npm run build
