#!/bin/sh
docker run \
    --rm \
    -it \
    --cap-drop=all \
    --security-opt no-new-privileges \
    --read-only \
    -u dev \
    --memory=3g \
    --memory-swap=3g \
    --memory-swappiness=0 \
    --env USER=dev \
    --tmpfs /tmp:size=16m \
    --tmpfs /home/dev/build:exec,size=512m \
    --tmpfs /home/dev/.gradle:exec,size=512m \
    --tmpfs /home/dev/.android:size=1m \
    --tmpfs /home/dev/.cargo/registry:size=32m \
    --tmpfs /home/dev/.cargo/git:size=1m \
    --tmpfs /home/dev/.rustup/toolchains:exec,size=1m \
    --device /dev/bus/usb \
    android-rust-simd
