.PHONY: all build build-release test clean lint fmt help smoke-tests setup

# Default target
all: build

# Setup development environment
setup:
	@echo "Setting up nfw development environment..."
	bash scripts/setup.sh

# Build the workspace
build:
	@echo "Building nfw workspace..."
	bash scripts/build.sh

# Build the workspace (release)
build-release:
	@echo "Building nfw workspace (release)..."
	cargo build --workspace --release

# Run all tests
test:
	@echo "Running nfw workspace tests..."
	bash scripts/test.sh

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	bash scripts/build.sh clean

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


# Show help
help:
	@echo "Available targets:"
	@echo "  make all          - Build the project (default)"
	@echo "  make setup        - Setup development environment"
	@echo "  make build        - Build the workspace"
	@echo "  make build-release - Build the workspace (release)"
	@echo "  make test         - Run all tests"
	@echo "  make smoke-tests  - Run CLI smoke tests"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make lint         - Run linter"
	@echo "  make format       - Format code"
	@echo "  make help         - Show this help message"
