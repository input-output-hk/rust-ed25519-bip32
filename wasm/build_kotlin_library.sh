#!/bin/sh

set -eo pipefail

wasm-pack build -t nodejs

npm i
npm run build
