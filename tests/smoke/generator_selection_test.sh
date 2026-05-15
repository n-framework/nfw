#!/usr/bin/env bash
# Generator Selection Smoke Test (Non-Interactive Mode)
# Validates: T011 from tasks.md

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

TEST_NAME="Generator Selection (non-interactive)"

main() {
	echo "Running: $TEST_NAME"

	setup_test_dir
	cd "$TEST_DIR"

	local generator_id="official/blank-workspace"

	echo "Testing: nfw new TestWorkspace --generator $generator_id --no-input"

	if nfw new TestWorkspace --generator "$generator_id" --no-input 2>&1; then
		:
	else
		log_fail "nfw new command failed"
		exit 1
	fi

	assert_dir_exists "$TEST_DIR/TestWorkspace" "Workspace directory"
	assert_file_exists "$TEST_DIR/TestWorkspace/nfw.yaml" "Workspace config"

	local generator_in_config
	generator_in_config=$(grep -o 'generator:.*' "$TEST_DIR/TestWorkspace/nfw.yaml" || echo "")
	if [[ -n "$generator_in_config" ]]; then
		log_pass "Generator identifier recorded in configuration"
	else
		log_fail "Generator identifier not found in configuration"
		exit 1
	fi

	log_pass "$TEST_NAME"
	exit 0
}

main "$@"
