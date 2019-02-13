#!/bin/bash

main() {

    cargo build --target "$TARGET" --release
    
    local tmpdir="$(mktemp -d)"
    local name="${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"
    local staging="${tmpdir}/${name}"

    cp target/$TARGET/release/fractious $staging/
    cp {README.md, LICENSE-MIT, LICENSE-APACHE} $staging

    (cd "$tmpdir" && tar czf "$(pwd)/deployment/${name}.tar.gz" "${name}")
    rm -rf $tmpdir
}

main
