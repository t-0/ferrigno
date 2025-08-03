#! /usr/bin/env bash


RUSTFLAGS="-Awarnings" cargo build || exit 1

pushd tests || exit 1
../target/debug/rlua -e"_U=true" all.lua || exit 1
popd || exit 1
