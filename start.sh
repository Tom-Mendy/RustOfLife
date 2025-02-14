#!/usr/bin/env bash

# Run the tests
. /etc/os-release
case "${ID}" in
nixos)
    nix develop --command bash -c 'cargo run'
    ;;
*)
    cargo run
    ;;
esac
