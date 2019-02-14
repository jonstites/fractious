#!/bin/bash

export PATH="$PATH:$HOME/.cargo/bin"

set -ex

configure_cargo() {
    local gcc_suffix=""
    if [ -n "$GCC_VERSION" ]; then
        gcc_suffix="-$GCC_VERSION"
    fi
    local gcc="${prefix}gcc${gcc_suffix}"

    # information about the cross compiler
    "${gcc}" -v

    # tell cargo which linker to use for cross compilation
    mkdir -p .cargo
    cat >>.cargo/config <<EOF
[target.$TARGET]
linker = "${gcc}"
EOF
}


main() {
    rustup self update
    rustup target add $TARGET
    rustup component add clippy
    configure_cargo
}

main
