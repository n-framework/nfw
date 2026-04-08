#!/usr/bin/env bash
# Smoke Test Runner: discovers and executes all smoke tests
# Usage: ./run_smoke_tests.sh [--verbose]
# Exit codes: 0 = all passed, 1 = one or more failed, 2 = environment error

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

VERBOSE="${VERBOSE:-false}"
if [[ "${1:-}" == "--verbose" ]]; then
	VERBOSE=true
fi

# Check prerequisites before running any tests
check_all_prerequisites

echo "Smoke Test Suite: Build & Test Workflows"
echo "========================================="

TOTAL=0
PASSED=0
FAILED=0
FAILURES=""

# Discover and run all *_test.sh files
for test_file in "$SCRIPT_DIR"/*_test.sh; do
	if [[ ! -f "$test_file" ]]; then
		continue
	fi

	TOTAL=$((TOTAL + 1))
	test_name=$(basename "$test_file" .sh)

	if $VERBOSE; then
		log_info "Running: $test_name"
	fi

	# Run test in subshell to isolate environment
	if bash "$test_file" 2>&1; then
		PASSED=$((PASSED + 1))
		if $VERBOSE; then
			log_pass "$test_name"
		fi
	else
		FAILED=$((FAILED + 1))
		FAILURES="${FAILURES}\n  [FAIL] $test_name"
		if ! $VERBOSE; then
			log_fail "$test_name"
		fi
	fi
done

echo "========================================="

if [[ "$FAILED" -eq 0 ]]; then
	echo "$PASSED/$TOTAL tests passed"
	exit 0
else
	echo "$PASSED/$TOTAL tests passed, $FAILED failed"
	if [[ -n "$FAILURES" ]]; then
		echo -e "Failures:$FAILURES"
	fi
	exit 1
fi
