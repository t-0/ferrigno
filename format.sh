#! /usr/bin/env bash

find src -name "*.rs" -exec rustfmt {} \;
