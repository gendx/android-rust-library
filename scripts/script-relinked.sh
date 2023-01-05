#!/bin/bash

set -eux

export CARGO_UNSTABLE_SPARSE_REGISTRY=true

RUST_TOOLCHAIN=nightly ./build-relinked.sh

cp -rf src/android-simd build/
cd build/android-simd

cargo fmt --check

export PATH=${PATH}:${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin
RUSTFLAGS='-L /home/dev/build/relinked' cargo +nightly build --features relink --target aarch64-linux-android --release
cargo +nightly build --target armv7-linux-androideabi --release
cargo +nightly build --target i686-linux-android --release
cargo +nightly build --target x86_64-linux-android --release

cd ../..

set +x
echo "Raw size of libsimd.so"
echo "- aarch64 : $(stat -c%s ${HOME}/build/android-simd/target/aarch64-linux-android/release/libsimd.so) bytes"
echo "- armv7   : $(stat -c%s ${HOME}/build/android-simd/target/armv7-linux-androideabi/release/libsimd.so) bytes"
echo "- i686    : $(stat -c%s ${HOME}/build/android-simd/target/i686-linux-android/release/libsimd.so) bytes"
echo "- x86_64  : $(stat -c%s ${HOME}/build/android-simd/target/x86_64-linux-android/release/libsimd.so) bytes"
set -x

./strip-rust.sh

set +x
echo "Cleaning up some space..."
set -x

rm build/relinked/*.a
