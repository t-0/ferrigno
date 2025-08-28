#! /usr/bin/env bash


RUSTFLAGS="-Awarnings" cargo build || exit 1

if ! pushd tests
then
    printf "ERROR\n" 1>&2
    exit 1
fi
if ! RUST_BACKTRACE=1 ../target/debug/rlua -e"_U=true" all.lua
then
    printf "ERROR\n" 1>&2
    exit 1
fi
if ! popd
then
    printf "ERROR\n" 1>&2
    exit 1
fi
if ! git add .
then
    printf "ERROR\n" 1>&2
    exit 1
fi
