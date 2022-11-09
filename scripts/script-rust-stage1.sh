#!/bin/bash

set -eux

./script-rust-stage1-nostrip.sh
./strip-rust.sh
