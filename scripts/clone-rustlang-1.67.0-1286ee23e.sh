#!/bin/bash

set -eux

rm -Rf rustc-build/*
cd rustc-build
mkdir rust
cd rust
git init
git remote add origin https://github.com/rust-lang/rust
time git fetch --depth 1 origin 1286ee23e4e2dec8c1696d3d76c6b26d97bbcf82
time git checkout FETCH_HEAD
