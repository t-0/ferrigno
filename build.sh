#! /usr/bin/env bash


__build_inner__() {
    unset -f __build_inner__

    local __target_d
    __target_d=$(rustc -vV | awk '/^host:/ { print $2 }')
    if [[ -z "${__target_d}" ]]
    then
        printf "ERROR: could not determine host triple from rustc\n" 1>&2
        return 1
    fi

    local __it
    for __it in dev release
    do
        if ! cargo build --target "${__target_d}" --profile "${__it}" --features full
        then
            printf "ERROR: cargo build --profile %s failed\n" "${__it}" 1>&2
            return 1
        fi
    done
}

__build__() {
    unset -f __build__

    local __repo_d
    __repo_d="$(dirname "${BASH_SOURCE[0]}")"
    if pushd "${__repo_d}" >/dev/null
    then
        local __ret
        __build_inner__ "${@}"
        __ret=$?
        if ! popd >/dev/null
        then
            printf "ERROR: could not pop directory\n" 1>&2
        fi
        return "${__ret}"
    else
        unset -f __build_inner__

        printf "ERROR: could not pushd directory\n" 1>&2
        return 1
    fi
}


__build__ "${@}"
