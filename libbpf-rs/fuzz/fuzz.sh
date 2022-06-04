#!/bin/sh

export RUSTFLAGS="-Clinker=clang" CC="clang" CFLAGS="-g -fsanitize=address -fsanitize-coverage=inline-8bit-counters,trace-cmp,edge,pc-table"
cargo +nightly -Z sparse-registry fuzz build
