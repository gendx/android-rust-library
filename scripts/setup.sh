#!/bin/bash

set -eux

ln -sf ~/rustup/toolchains/* .rustup/toolchains/
adb shell echo "Hello from Android device" || true
