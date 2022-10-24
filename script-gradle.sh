#!/bin/bash

set -eu

echo "Running gradle build..."
set -x

echo "sdk.dir=${ANDROID_HOME}" > local.properties
gradle build

set +x
echo "Signing release APK..."
set -x
${ANDROID_HOME}/build-tools/30.0.3/apksigner \
    sign \
    --in app/build/outputs/apk/release/app-release-unsigned.apk \
    --out app/build/outputs/apk/release/app-release.apk \
    --ks $HOME/.android/debug.keystore \
    --ks-key-alias androiddebugkey \
    --ks-pass pass:android \
    --key-pass pass:android

set +x
echo "Checking ZIP-alignment..."
set -x
${ANDROID_HOME}/build-tools/30.0.3/zipalign \
    -c -p 4 \
    app/build/outputs/apk/release/app-release.apk

mv app/build/outputs/apk/release/app-release-unsigned.apk ${HOME}/build/android-simd-app-release-unsigned.apk
mv app/build/outputs/apk/release/app-release.apk ${HOME}/build/android-simd-app-release.apk
mv app/build/outputs/apk/debug/app-debug.apk ${HOME}/build/android-simd-app-debug.apk

unzip -lv ${HOME}/build/android-simd-app-release-unsigned.apk | grep -E "(lib|classes|files)"
unzip -lv ${HOME}/build/android-simd-app-release.apk | grep -E "(lib|classes|files)"
unzip -lv ${HOME}/build/android-simd-app-debug.apk | grep -E "(lib|classes|files)"

set +x
echo "Clearing up some space..."
set -x

rm -R ${HOME}/.gradle/*
