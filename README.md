# pips

Project for analyzing dice roll formulas in TTRPGs.

## Tools

- [rustc/cargo](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [node/npm](https://nodejs.org/en/)
- [cargo-release](https://docs.rs/crate/cargo-release/)

## Usage

```sh
# Build pips
cargo build --all

# Test
cargo test

# Build pips-wasm
./build-wasm.sh
```

## Publishing

```sh
# Publish pips crate
cargo release <level>

# OR Publish both pips crate and pips-wasm package
cargo release <level> --workspace

# push changes
git push
git push --tags
```

For details on running the website, see [pips-web/README.md](pips-web/README.md).
