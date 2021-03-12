#! /bin/bash

set -e
set -o pipefail

cargo build --all

wasm-pack build pips-wasm --out-dir pkg

# add the .wasm extension to the first line to make
# the angular compiler happy
sed -e '1 s/pips_wasm_bg"/pips_wasm_bg.wasm"/' \
    -i pips-wasm/pkg/pips_wasm.js
