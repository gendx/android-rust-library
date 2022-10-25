#!/bin/bash

set -eux

set +x
echo "Stripping Rust libraries..."
set -x
${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip ${HOME}/build/android-simd/target/aarch64-linux-android/release/libsimd.so
${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip ${HOME}/build/android-simd/target/armv7-linux-androideabi/release/libsimd.so
${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip ${HOME}/build/android-simd/target/i686-linux-android/release/libsimd.so
${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip ${HOME}/build/android-simd/target/x86_64-linux-android/release/libsimd.so

set +x
echo "Stripped size of libsimd.so"
echo "- aarch64 : $(stat -c%s ${HOME}/build/android-simd/target/aarch64-linux-android/release/libsimd.so) bytes"
echo "- armv7   : $(stat -c%s ${HOME}/build/android-simd/target/armv7-linux-androideabi/release/libsimd.so) bytes"
echo "- i686    : $(stat -c%s ${HOME}/build/android-simd/target/i686-linux-android/release/libsimd.so) bytes"
echo "- x86_64  : $(stat -c%s ${HOME}/build/android-simd/target/x86_64-linux-android/release/libsimd.so) bytes"
set -x
