#!/bin/bash

set -eux

rm -rf ${HOME}/.android/*
cp -r ${HOME}/android/* ${HOME}/.android/
${ANDROID_HOME}/emulator/emulator -verbose -avd test_avd &
