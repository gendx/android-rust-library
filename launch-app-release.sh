#!/bin/bash

set -ux

adb logcat -c

adb uninstall com.example.myrustapplication
adb install ${HOME}/build/android-simd-app-release.apk
adb shell monkey -p com.example.myrustapplication 1

sleep 2

adb logcat -d | grep MyRustSimdApplication
