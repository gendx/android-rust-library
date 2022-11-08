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
    --env "DISPLAY=unix${DISPLAY}" \
    --tmpfs /tmp:size=16m \
    --tmpfs /home/dev/build:exec,size=512m \
    --tmpfs /home/dev/.gradle:exec,size=512m \
    --tmpfs /home/dev/.android:size=3g \
    --tmpfs /home/dev/.cargo/registry:size=16m \
    --volume /tmp/.X11-unix:/tmp/.X11-unix \
    --device /dev/dri \
    --device /dev/kvm \
    android-rust-simd
