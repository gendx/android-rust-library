#!/bin/bash

set -eux

export CARGO_UNSTABLE_SPARSE_REGISTRY=true

cd rustc-build/rust
rustup toolchain link stage1 build/x86_64-unknown-linux-gnu/stage1
cd ../..

cp -rf src/android-simd build/
cd build/android-simd
cargo fmt --check
cd ../..

set +x
echo "Benchmarking with nightly toolchain"
set -x
RUST_TOOLCHAIN=nightly ./build-relinked.sh

cd build/android-simd

# We need to disable LTO, otherwise we get a linker error ("duplicate symbol: rust_eh_personality")
RUSTFLAGS='-L /home/dev/build/relinked' cargo +nightly test  --features relink --target aarch64-linux-android --profile release-nolto || true
RUSTFLAGS='-L /home/dev/build/relinked' cargo +nightly bench --features relink --target aarch64-linux-android
cargo +nightly bench --target aarch64-linux-android
RUSTFLAGS='-C target-feature=+aes' cargo +nightly bench --target aarch64-linux-android

cd ../..

set +x
echo "Benchmarking with stage1 toolchain"
set -x
RUST_TOOLCHAIN=stage1 ./build-relinked.sh

cd build/android-simd

# Somehow --release works on stage1.
RUSTFLAGS='-L /home/dev/build/relinked' cargo +stage1 test  --features relink --target aarch64-linux-android --release || true
RUSTFLAGS='-L /home/dev/build/relinked' cargo +stage1 bench --features relink --target aarch64-linux-android
cargo +stage1 bench --target aarch64-linux-android
RUSTFLAGS='-C target-feature=+aes' cargo +stage1 bench --target aarch64-linux-android

cd ../..
