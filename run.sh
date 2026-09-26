#! /usr/bin/env bash


__run__() {
    unset -f __run__

    local __source_d
    __source_d="$(dirname "${BASH_SOURCE[0]}")"
    local __target_d
    __target_d="${__source_d}/target/$(rustc -vV | awk '/^host:/ { print $2 }')"
    local __ferrigno
    __ferrigno="${__target_d}/release/ferrigno"
    if [[ ! -x "${__ferrigno}" ]]
    then
        printf "ERROR: binary ferrigno does not exist\n" 1>&2
        return 1
    fi
    exec "${__ferrigno}" "${@}"
}


__run__ "${@}"
