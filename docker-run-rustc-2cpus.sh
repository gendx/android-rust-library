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
    --cpus 2 \
    --memory=2g \
    --memory-swap=2g \
    --memory-swappiness=0 \
    --env USER=dev \
    --tmpfs /tmp:size=256m \
    --tmpfs /home/dev/.cargo/registry:size=512m \
    --tmpfs /home/dev/.rustup/toolchains:exec,size=1m \
    --volume ${PWD}/rustc-build:/home/dev/rustc-build \
    android-rust-simd
