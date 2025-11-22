#!/usr/bin/env bash
set -e

echo "Running demo-cli example..."
echo "============================"
echo

echo "1. Running without arguments:"
echo "   $ demo-cli"
echo
cargo run -q -p demo-cli

echo
echo "2. Running with -V flag (version info):"
echo "   $ demo-cli -V"
echo
cargo run -q -p demo-cli -- -V

echo
echo "3. Running with --version flag:"
echo "   $ demo-cli --version"
echo
cargo run -q -p demo-cli -- --version
