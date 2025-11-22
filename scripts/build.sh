#!/usr/bin/env bash
set -e

echo "Building sw-cli workspace..."
echo "=============================="
echo

echo "Building main crate (sw-cli)..."
cargo build -p sw-cli

echo
echo "Building macro crate (sw-cli-macros)..."
cargo build -p sw-cli-macros

echo
echo "Building demo example (demo-cli)..."
cargo build -p demo-cli

echo
echo "✓ All builds completed successfully!"
