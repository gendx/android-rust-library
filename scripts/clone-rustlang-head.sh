#!/bin/bash

set -eux

rm -Rf rustc-build/*
cd rustc-build
time git clone --depth=1 https://github.com/rust-lang/rust
