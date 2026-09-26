#! /usr/bin/env bash


__test_inner__() {
    unset -f __test_inner__

    local __target_d
    __target_d=$(rustc -vV | awk '/^host:/ { print $2 }')
    if [[ -z "${__target_d}" ]]
    then
        printf "ERROR: could not determine host triple from rustc\n" 1>&2
        return 1
    fi

    local __it
    for __it in debug release
    do
        local -a __cargo_args=(--target "${__target_d}")
        [[ "${__it}" == "release" ]] && __cargo_args+=(--release)
        if ! cargo test "${__cargo_args[@]}"
        then
            printf "ERROR: cargo test failed\n" 1>&2
            return 1
        fi
        local __ferrigno
        __ferrigno="${PWD}/target/${__target_d}/${__it}/ferrigno"
        if ! (cd "src/rust/ferrigno/lua/tests" && RUST_BACKTRACE=1 "${__ferrigno}" --bare -e"_U=true" all.lua)
        then
            printf "ERROR: lua tests failed (%s)\n" "$__it" 1>&2
            return 1
        fi
        if ! RUST_BACKTRACE=1 "${__ferrigno}" --bare -e"_U=true" "@tests/all.lua"
        then
            printf "ERROR: @tests/all.lua failed (%s)\n" "$__it" 1>&2
            return 1
        fi
    done
    return 0
}

__test__() {
    unset -f __test__

    local __repo_d
    __repo_d="$(dirname "${BASH_SOURCE[0]}")"
    if ! "${__repo_d}/build.sh"
    then
        unset -f __test_inner__
        return 1
    fi
    if ! pushd "${__repo_d}" >/dev/null
    then
        unset -f __test_inner__
        printf "ERROR: could not pushd directory\n" 1>&2
        return 1
    fi
    local __ret
    __test_inner__ "${@}"
    __ret=$?
    if ! popd >/dev/null
    then
        printf "ERROR: could not pop directory\n" 1>&2
    fi
    return "${__ret}"
}


__test__ "${@}"
