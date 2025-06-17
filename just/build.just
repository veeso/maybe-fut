# Build the crate
[group('build')]
build:
  cargo build
  # minimal tokio build
  cargo build --features tokio
  # full tokio build
  cargo build --all-features

# Run the tests
[group('build')]
test test_name="":
  cargo test-all {{test_name}}

# Run clippy
[group('build')]
lint:
  cargo lint

# Format code
[group('build')]
fmt:
  cargo +nightly fmt --all -- --check

# Check all the things
[group('build')]
check: build test lint fmt
