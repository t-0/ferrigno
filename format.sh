#! /usr/bin/env bash

find "src/rust" -name "*.rs" -exec rustfmt {} \;
