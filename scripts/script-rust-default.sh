#!/bin/bash

set -eux

./script-rust-default-nostrip.sh
./strip-rust.sh
