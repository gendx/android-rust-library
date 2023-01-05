#!/bin/bash

set -eu

echo "####################"
echo "# android-runner.sh invoked with: $@"
echo "####################"

# The binary to upload and run is the first argument.
BINARY_PATH="$1"
BINARY=`basename ${BINARY_PATH}`
# Remove the first parameter.
shift

# Push the test binary on the device via ADB.
adb push "${BINARY_PATH}" "/data/local/tmp/$BINARY"
adb shell "chmod 755 /data/local/tmp/$BINARY"

# Run the test binary, forwarding the remaining parameters, so that benchmarks,
# test filtering, etc. work.
adb shell "/data/local/tmp/$BINARY $@"

# Cleanup.
adb shell "rm /data/local/tmp/$BINARY"
