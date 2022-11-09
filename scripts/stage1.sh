#!/bin/bash

set -eux

cd rustc-build/rust

cd library/stdarch
git apply ~/stdarch.patch
cd ../..

CARGOFLAGS="--timings" ./x.py build --stage 1 \
    --target x86_64-unknown-linux-gnu \
    --target aarch64-linux-android \
    --target armv7-linux-androideabi \
    --target i686-linux-android \
    --target x86_64-linux-android
rustup toolchain link stage1 build/x86_64-unknown-linux-gnu/stage1
