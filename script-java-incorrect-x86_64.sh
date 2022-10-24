#!/bin/bash

set -eu

echo "Size of libsimd.so"
echo "- aarch64 : $(stat -c%s ${HOME}/build/android-simd/target/aarch64-linux-android/release/libsimd.so) bytes"
echo "- armv7   : $(stat -c%s ${HOME}/build/android-simd/target/armv7-linux-androideabi/release/libsimd.so) bytes"
echo "- i686    : $(stat -c%s ${HOME}/build/android-simd/target/i686-linux-android/release/libsimd.so) bytes"
echo "- x86_64  : $(stat -c%s ${HOME}/build/android-simd/target/x86_64-linux-android/release/libsimd.so) bytes"
set -x

cp -rf src/MyRustSimdApplication build/
cd build/MyRustSimdApplication

cd app/src/main
rm -Rf jniLibs
mkdir jniLibs
mkdir -p jniLibs/arm64-v8a
mkdir -p jniLibs/armeabi-v7a
mkdir -p jniLibs/x86
mkdir -p jniLibs/x64

ln -sf ${HOME}/build/android-simd/target/aarch64-linux-android/release/libsimd.so jniLibs/arm64-v8a/libsimd.so
ln -sf ${HOME}/build/android-simd/target/armv7-linux-androideabi/release/libsimd.so jniLibs/armeabi-v7a/libsimd.so
ln -sf ${HOME}/build/android-simd/target/i686-linux-android/release/libsimd.so jniLibs/x86/libsimd.so
ln -sf ${HOME}/build/android-simd/target/x86_64-linux-android/release/libsimd.so jniLibs/x64/libsimd.so
cd ../../..

../../script-gradle.sh
