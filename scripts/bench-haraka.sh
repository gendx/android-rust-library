#!/bin/bash

set -eux

export CARGO_UNSTABLE_SPARSE_REGISTRY=true

cd build/
git clone https://github.com/gendx/haraka-rs
cd haraka-rs/

RUSTFLAGS='-C target-feature=+aes' cargo +nightly bench --target aarch64-linux-android
RUSTFLAGS='-C target-feature=+aes' cargo +nightly bench --target armv7-linux-androideabi
cargo +nightly bench --target aarch64-linux-android
cargo +nightly bench --target armv7-linux-androideabi
