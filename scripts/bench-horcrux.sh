#!/bin/bash

set -eux

export CARGO_UNSTABLE_SPARSE_REGISTRY=true

cd rustc-build/rust
rustup toolchain link stage1 build/x86_64-unknown-linux-gnu/stage1
cd ../..

cd build/
git clone https://github.com/gendx/horcrux
cd horcrux

cd horcrux
echo "Testing fallback implementation"
cargo +nightly test --target aarch64-linux-android --release -- test::gf064
cargo +nightly bench --target aarch64-linux-android -- bench_mul
cargo +nightly bench --target aarch64-linux-android -- bench_invert
cargo +nightly bench --target aarch64-linux-android -- compact::bench_split_10
cargo +nightly bench --target aarch64-linux-android -- compact::bench_reconstruct_10

echo "Testing static CPU feature detection"
RUSTFLAGS='-C target-feature=+aes' cargo +nightly test --target aarch64-linux-android --release -- test::gf064
RUSTFLAGS='-C target-feature=+aes' cargo +nightly bench --target aarch64-linux-android -- bench_mul
RUSTFLAGS='-C target-feature=+aes' cargo +nightly bench --target aarch64-linux-android -- bench_invert
RUSTFLAGS='-C target-feature=+aes' cargo +nightly bench --target aarch64-linux-android -- compact::bench_split_10
RUSTFLAGS='-C target-feature=+aes' cargo +nightly bench --target aarch64-linux-android -- compact::bench_reconstruct_10
cd ..

git reset --hard HEAD
git apply ~/horcrux.dynamic_detect.patch

cd horcrux
echo "Testing dynamic CPU feature detection"
cargo +stage1 test --target aarch64-linux-android --release -- test::gf064
cargo +stage1 bench --target aarch64-linux-android -- bench_mul
cargo +stage1 bench --target aarch64-linux-android -- bench_invert
cargo +stage1 bench --target aarch64-linux-android -- compact::bench_split_10
cargo +stage1 bench --target aarch64-linux-android -- compact::bench_reconstruct_10
cd ..
