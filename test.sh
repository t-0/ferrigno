#! /usr/bin/env bash

RUST_TARGET_D=$(rustc -vV | awk '/^host:/ { print $2 }')
for RUST_PROFILE in debug release
do
    if [[ "$RUST_PROFILE" == "release" ]]
    then
        if ! RUSTFLAGS="-Awarnings" CARGO_TARGET_DIR="target/${RUST_TARGET_D}" cargo build --release
        then
            printf "ERROR\n" 1>&2
            exit 1
        fi
    else
        if ! RUSTFLAGS="-Awarnings" CARGO_TARGET_DIR="target/${RUST_TARGET_D}" cargo build
        then
            printf "ERROR\n" 1>&2
            exit 1
        fi
    fi
    if ! pushd "src/lua/tests"
    then
        printf "ERROR\n" 1>&2
        exit 1
    fi
    if ! RUST_BACKTRACE=1 "../../../target/${RUST_TARGET_D}/${RUST_PROFILE}/ferrigno" --bare -e"_U=true" all.lua
    then
        printf "ERROR\n" 1>&2
        exit 1
    fi
    if ! popd
    then
        printf "ERROR\n" 1>&2
        exit 1
    fi
    if ! RUST_BACKTRACE=1 "./target/${RUST_TARGET_D}/${RUST_PROFILE}/ferrigno" --bare -e"_U=true" "@tests/all.lua"
    then
        printf "ERROR\n" 1>&2
        exit 1
    fi
    if ! git add .
    then
        printf "ERROR\n" 1>&2
        exit 1
    fi
done
