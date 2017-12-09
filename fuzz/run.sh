#!/usr/bin/env bash

set -e

if [ ! -f run-fuzzer.sh ]; then
    wget https://github.com/rust-fuzz/targets/raw/master/run-fuzzer.sh -O run-fuzzer.sh
fi

crate="$(dirname $0)"
target="parse_document"
bash run-fuzzer.sh "$crate" "$target"
