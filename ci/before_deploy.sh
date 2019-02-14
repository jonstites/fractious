#!/bin/bash

main() {
    set -ex
    cargo build --target "$TARGET" --release
    
    local tmpdir="$(mktemp -d)"
    local name="${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"
    local staging="${tmpdir}/${name}"
    local out_dir="$(pwd)/deployment"

    mkdir -p out_dir
    mkdir -p ${staging}
    
    cp target/$TARGET/release/fractious $staging/
    cp {README.md,LICENSE-MIT,LICENSE-APACHE} $staging

    (cd "$tmpdir" && tar czf "$out_dir/${name}.tar.gz" "${name}")
    rm -rf $tmpdir
    set +ex
}

main
