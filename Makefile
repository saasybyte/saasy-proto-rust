.PHONY: build check clippy clippy-strict fmt clean

# Build the crate (triggers proto generation via build.rs)
build:
	cargo build

# Fast compilation check (no codegen)
check:
	cargo check

# Run clippy lints
clippy:
	cargo clippy

# Run clippy lints (strict, fails on warnings)
clippy-strict:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Clean build artifacts
clean:
	cargo clean
