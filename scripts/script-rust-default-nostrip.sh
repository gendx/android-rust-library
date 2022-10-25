#!/bin/bash

set -eux

cp -rf src/android-simd build/
cd build/android-simd

export PATH=${PATH}:${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release

cd ../..

set +x
echo "Raw size of libsimd.so"
echo "- aarch64 : $(stat -c%s ${HOME}/build/android-simd/target/aarch64-linux-android/release/libsimd.so) bytes"
echo "- armv7   : $(stat -c%s ${HOME}/build/android-simd/target/armv7-linux-androideabi/release/libsimd.so) bytes"
echo "- i686    : $(stat -c%s ${HOME}/build/android-simd/target/i686-linux-android/release/libsimd.so) bytes"
echo "- x86_64  : $(stat -c%s ${HOME}/build/android-simd/target/x86_64-linux-android/release/libsimd.so) bytes"

echo "Clearing up some space..."
set -x

rm -Rf ${HOME}/.cargo/registry/*
