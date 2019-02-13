#!/bin/bash

main() {
    cargo build --target "$TARGET"
    cargo test --target "$TARGET"
}

main
