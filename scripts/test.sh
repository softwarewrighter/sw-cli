#!/usr/bin/env bash
set -e

echo "Running tests for sw-cli workspace..."
echo "======================================"
echo

echo "Running tests for sw-cli..."
cargo test -p sw-cli

echo
echo "Running tests for sw-cli-macros..."
cargo test -p sw-cli-macros

echo
echo "Running tests for demo-cli..."
cargo test -p demo-cli

echo
echo "Running cargo fmt check..."
cargo fmt --all -- --check

echo
echo "Running clippy..."
cargo clippy --all -- -D warnings

echo
echo "✓ All tests and checks passed!"
