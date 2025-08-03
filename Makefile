# Makefile for Rust project

# Binary name (optional, set if you want to run with `make run`)
BIN_NAME := xortrick

.PHONY: all build run test fmt lint clean release check

# Default target
all: build

# Build in debug mode
build:
	cargo build

# Build optimized release binary
release:
	cargo build --release

# Run the app (debug)
run:
	cargo run

# Run the app with args: make run ARGS="foo bar"
run-args:
	cargo run -- $(ARGS)

# Check code without building binaries
check:
	cargo check

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt --all

# Lint with clippy
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Clean target directory
clean:
	cargo clean
