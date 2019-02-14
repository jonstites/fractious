#!/bin/bash

main() {
    cargo clippy -- -D warnings    
    cargo build --target "$TARGET"
    cargo test --target "$TARGET"
}

main
