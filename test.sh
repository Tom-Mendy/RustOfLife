#!/usr/bin/env bash

# Run the tests
cargo test

# Generate the coverage report
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore '/*' -o ./target/debug/coverage/

# Open the coverage report in the browser
xdg-open ./target/debug/coverage/index.html
