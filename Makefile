.PHONY: all build test clean lint fmt help

# Default target
all: build

# Build the workspace
build:
	@echo "Building nfw workspace..."
	cargo build --workspace

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

# Show help
help:
	@echo "Available targets:"
	@echo "  make all          - Build the project (default)"
	@echo "  make build        - Build the workspace"
	@echo "  make test         - Run all tests"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make lint         - Run linter"
	@echo "  make format       - Format code"
	@echo "  make help         - Show this help message"
