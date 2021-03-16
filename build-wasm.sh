#! /bin/bash

set -e
set -o pipefail

cargo build --all

wasm-pack build pips-wasm --out-dir pkg

# add the .wasm extension to the first line to make
# the angular compiler happy
sed -e '1 s/pips_wasm_bg"/pips_wasm_bg.wasm"/' \
    -i pips-wasm/pkg/pips_wasm.js

# make sure all files are included in package.json
pushd pips-wasm/pkg > /dev/null

PACKAGE_JSON="$(cat package.json)"
FILES=$(ls *.ts *.js *.wasm)
for file in $FILES; do
	# make sure all input files are included in package.json ".files"
	if [ "$(jq ".files | all(. != \"$file\")" <<<"$PACKAGE_JSON")" == "true" ]; then
		PACKAGE_JSON="$(jq ".files |= . + [\"$file\"]" <<<"$PACKAGE_JSON")"
	fi
done

echo "$PACKAGE_JSON" > package.json

popd > /dev/null
