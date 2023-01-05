#!/bin/sh

mkdir -p rustc-build
chown 1000:1000 rustc-build
PWD=`pwd`

docker run \
    --rm \
    -it \
    --cap-drop=all \
    --security-opt no-new-privileges \
    --read-only \
    -u dev \
    --cpus 8 \
    --memory=5g \
    --memory-swap=5g \
    --memory-swappiness=0 \
    --env USER=dev \
    --env "DISPLAY=unix${DISPLAY}" \
    --tmpfs /tmp:size=256m \
    --tmpfs /home/dev/build:exec,size=512m \
    --tmpfs /home/dev/.gradle:exec,size=512m \
    --tmpfs /home/dev/.android:size=3g \
    --tmpfs /home/dev/.cargo/registry:size=512m \
    --tmpfs /home/dev/.cargo/git:size=1m \
    --tmpfs /home/dev/.rustup/toolchains:exec,size=1m \
    --volume /tmp/.X11-unix:/tmp/.X11-unix \
    --volume ${PWD}/rustc-build:/home/dev/rustc-build \
    --device /dev/bus/usb \
    --device /dev/dri \
    --device /dev/kvm \
    android-rust-simd
