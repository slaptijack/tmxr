#!/bin/sh
set -e

echo "==> cargo fmt --check"
cargo fmt --all --check

echo "==> cargo clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "==> cargo test"
cargo test --all-targets --all-features

if command -v shellcheck >/dev/null 2>&1; then
	echo "==> shellcheck"
	shellcheck bin/*
else
	echo "==> shellcheck (skipped: not installed)"
fi
