#!/bin/bash

set -euxo pipefail

cargo build

gcc \
    -I. \
    ./demo.c \
    -L./target/debug \
    -l:libcalc_ffi.a \
    -lpthread \
    -ldl \
    -o demo
