#! /usr/bin/env bash

RUST_TARGET_D=$(rustc -vV | awk '/^host:/ { print $2 }')
"target/${RUST_TARGET_D}/release/ferrigno" "${@}"
