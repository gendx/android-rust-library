#!/bin/bash

set -eux

./script-rust-nightly-nostrip.sh
./strip-rust.sh
