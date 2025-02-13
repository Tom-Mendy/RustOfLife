#!/usr/bin/env bash

# Install the necessary tools
rustup component add llvm-tools-preview
cargo install grcov

rm -rf target/
rm default*.profraw

# Run the tests
. /etc/os-release
case "${ID}" in
nixos)
    nix develop --command bash -c 'RUSTFLAGS="-C instrument-coverage" cargo test --tests'
    ;;
*)
    RUSTFLAGS="-C instrument-coverage" cargo test --tests
    ;;
esac

# Generate the coverage report
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore '/*' -o ./target/debug/coverage/

# Open the coverage report in the browser
xdg-open ./target/debug/coverage/index.html

rm default*.profraw
