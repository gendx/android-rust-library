#!/bin/bash

set -eux

cp -r tools/flamedisk rustc-build/

cd rustc-build
git clone https://github.com/brendangregg/FlameGraph

cd flamedisk
cargo fmt --check
cargo run -- > ../du.samples
cd ..

./FlameGraph/flamegraph.pl --title "Disk usage" --countname "bytes" --nametype "File:" --colors mem du.samples > du.svg
