#!/bin/bash

set -eux

./setup.sh
./script-rust-nightly.sh
./script-java.sh
