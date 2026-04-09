#!/usr/bin/env bash
# Shared smoke test utility functions
# Used by all smoke test scripts under tests/smoke/

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${NFW_REPO_ROOT:-$(cd "$SCRIPT_DIR/../../../../" && pwd)}"

# Add default nfw build location to PATH
export PATH="$REPO_ROOT/target/debug:$PATH"

# Simple logging functions (self-contained, no external dependencies)
log_info() {
	echo "[INFO] $1"
}

log_success() {
	echo "[SUCCESS] $1"
}

log_error() {
	echo "[ERROR] $1" >&2
}

log_pass() {
	echo "[PASS] $1"
}

log_fail() {
	echo "[FAIL] $1"
}

# ============================================================================
# Temporary directory management (FR-005, FR-006)
# ============================================================================

setup_test_dir() {
	TEST_DIR=$(mktemp -d "${TMPDIR:-/tmp}/nfw-smoke-XXXXXX")
	trap cleanup_test_dir EXIT INT TERM
	log_info "Test directory: $TEST_DIR"
}

cleanup_test_dir() {
	if [[ -n "${TEST_DIR:-}" && -d "${TEST_DIR:-}" ]]; then
		if rm -rf "$TEST_DIR" 2>/dev/null; then
			log_info "Cleaned up test directory: $TEST_DIR"
		else
			log_error "Failed to clean up test directory: $TEST_DIR"
			log_error "Manual cleanup may be required"
		fi
	fi
}

# ============================================================================
# Prerequisite checks (T015)
# ============================================================================

check_cli_installed() {
	log_info "Checking for nfw in PATH..."
	log_info "PATH=$PATH"
	log_info "REPO_ROOT=$REPO_ROOT"
	log_info "Checking for binary at: $REPO_ROOT/target/debug/nfw"
	if [[ -f "$REPO_ROOT/target/debug/nfw" ]]; then
		log_info "Found nfw file: $(file "$REPO_ROOT/target/debug/nfw")"
	elif [[ -L "$REPO_ROOT/target/debug/nfw" ]]; then
		log_info "Found nfw symlink: $(readlink -f "$REPO_ROOT/target/debug/nfw")"
	else
		log_info "No nfw binary found in target/debug/"
		ls -la "$REPO_ROOT/target/debug/" | grep -E "nfw|nframework" || log_info "No nfw-related files found"
	fi
	if ! command -v nfw &>/dev/null; then
		log_error "nfw CLI not found in PATH"
		log_error "Expected at: $REPO_ROOT/target/debug/nfw or \$NFW_REPO_ROOT/target/debug/nfw"
		log_error "command -v nfw output: $(command -v nfw 2>&1 || echo 'not found')"
		exit 2
	fi
	log_success "nfw CLI found: $(command -v nfw)"
}

check_template_cache() {
	if ! nfw templates &>/dev/null 2>&1; then
		log_error "Template cache is empty or inaccessible"
		exit 2
	fi
}

check_dotnet_sdk() {
	if ! command -v dotnet &>/dev/null; then
		log_error ".NET SDK not found in PATH"
		exit 2
	fi
}

check_make() {
	if ! command -v make &>/dev/null; then
		log_error "make not found in PATH"
		exit 2
	fi
}

check_all_prerequisites() {
	log_info "Checking prerequisites..."
	check_cli_installed
	check_template_cache
	check_make
	log_success "All prerequisites satisfied"
}

# ============================================================================
# Assertion helpers
# ============================================================================

assert_dir_exists() {
	local path="$1"
	local label="${2:-$path}"
	if [[ ! -d "$path" ]]; then
		log_fail "$label: directory does not exist at '$path'"
		log_info "Parent directory exists: $([[ -d "$(dirname "$path")" ]] && echo "yes" || echo "no")"
		log_info "Current directory: $(pwd)"
		return 1
	fi
	log_pass "$label: directory exists"
}

assert_file_exists() {
	local path="$1"
	local label="${2:-$path}"
	if [[ ! -f "$path" ]]; then
		log_fail "$label: file does not exist at $path"
		return 1
	fi
	log_pass "$label: file exists"
}

assert_file_contains() {
	local file="$1"
	local pattern="$2"
	local label="${3:-$pattern}"
	if ! grep -q "$pattern" "$file" 2>/dev/null; then
		log_fail "$label: pattern '$pattern' not found in $file"
		if [[ "${VERBOSE:-false}" == true ]]; then
			log_info "Contents of $file (first 10 lines):"
			head -10 "$file" | while IFS= read -r line; do
				log_info "  $line"
			done
		fi
		return 1
	fi
	log_pass "$label: found in $file"
}

assert_exit_code() {
	local actual="$1"
	local expected="${2:-0}"
	local label="${3:-exit code}"
	if [[ "$actual" -ne "$expected" ]]; then
		log_fail "$label: expected $expected, got $actual"
		return 1
	fi
	log_pass "$label: $actual (expected $expected)"
}

assert_cleaned_up() {
	local path="$1"
	local label="${2:-$path}"
	if [[ -e "$path" ]]; then
		log_fail "$label: path still exists after cleanup"
		return 1
	fi
	log_pass "$label: cleaned up successfully"
}
