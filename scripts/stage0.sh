#!/bin/bash

set -eux

cd rustc-build/rust
cp ~/rustbuild-config.toml config.toml
CARGOFLAGS="--timings" ./x.py build
rustup toolchain link stage0 build/x86_64-unknown-linux-gnu/stage0
