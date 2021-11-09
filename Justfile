# This help screen
show-help:
	just --list

# Test it was built ok
test:
	RUST_BACKTRACE=1 cargo test

# Test the markdown in the docs directory
specdown: build
	specdown run --temporary-workspace-dir --add-path "$PWD/target/release" "README.md"

# Build release version
build:
	cargo build --release

# Lint it
lint:
	cargo +nightly fmt --all -- --check
	cargo +nightly clippy --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::cargo
	cargo +nightly check
	cargo +nightly audit

# Format what can be formatted
fmt:
	cargo +nightly fix --allow-dirty --allow-staged
	cargo +nightly clippy --allow-dirty --allow-staged --fix -Z unstable-options --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::cargo -D clippy::nursery
	cargo +nightly fmt --all
	yamlfmt -w .github/*.yml .github/workflows/*.yml .*.yml

# Clean the build directory
clean:
	cargo clean
