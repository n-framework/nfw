.PHONY: all build build-release test clean lint fmt help smoke-tests benchmark

# Default target
all: build

# Build the workspace
build:
	@echo "Building nfw workspace..."
	cargo build --workspace

# Build the workspace (release)
build-release:
	@echo "Building nfw workspace (release)..."
	cargo build --workspace --release

# Run all tests
test:
	@echo "Running nfw workspace tests..."
	cargo test --workspace

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run linter
lint:
	bash scripts/lint.sh

# Format code
format:
	bash scripts/format.sh

# Run smoke tests
smoke-tests:
	@echo "Running smoke tests..."
	bash tests/smoke/run_smoke_tests.sh

# Run benchmark harness
benchmark: build-release
	@echo "Running performance benchmark..."
	BENCHMARK_BIN=target/release/nframework-nfw-cli bash scripts/benchmark/run_benchmark.sh

# Show help
help:
	@echo "Available targets:"
	@echo "  make all          - Build the project (default)"
	@echo "  make build        - Build the workspace"
	@echo "  make build-release - Build the workspace (release)"
	@echo "  make test         - Run all tests"
	@echo "  make smoke-tests  - Run CLI smoke tests"
	@echo "  make benchmark    - Run performance benchmark"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make lint         - Run linter"
	@echo "  make format       - Format code"
	@echo "  make help         - Show this help message"
