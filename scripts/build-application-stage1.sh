#!/bin/bash

set -eux

./setup.sh
./clone-rustlang-1.67.0-1286ee23e.sh
./stage0.sh
./stage1.sh
./script-rust-stage1.sh
./script-java.sh
