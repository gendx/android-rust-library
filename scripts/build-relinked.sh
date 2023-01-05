#!/bin/bash

set -eux

export CARGO_UNSTABLE_SPARSE_REGISTRY=true
export PATH=${PATH}:${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin
OBJDUMP=${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-objdump

cp -rf src/relinked build/
cd build/relinked

cargo fmt --check

cargo +${RUST_TOOLCHAIN} build --target aarch64-linux-android --release
${OBJDUMP} target/aarch64-linux-android/release/librelinked.a --syms | grep aes
${OBJDUMP} target/aarch64-linux-android/release/librelinked.a --syms | grep gf
#${OBJDUMP} target/aarch64-linux-android/release/librelinked.a --disassemble-symbols=aesenc_fallback 2> /dev/null
cp target/aarch64-linux-android/release/librelinked.a libfallback.a

RUSTFLAGS='-C target-feature=+aes' cargo +${RUST_TOOLCHAIN} build --target aarch64-linux-android --release
${OBJDUMP} target/aarch64-linux-android/release/librelinked.a --syms | grep aes
${OBJDUMP} target/aarch64-linux-android/release/librelinked.a --syms | grep gf
#${OBJDUMP} target/aarch64-linux-android/release/librelinked.a --disassemble-symbols=aesenc_simd 2> /dev/null
cp target/aarch64-linux-android/release/librelinked.a libsimd.a

rm -R target
