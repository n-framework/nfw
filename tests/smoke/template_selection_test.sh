#!/usr/bin/env bash
# Template Selection Smoke Test (Non-Interactive Mode)
# Validates: T011 from tasks.md

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

TEST_NAME="Template Selection (non-interactive)"

main() {
	echo "Running: $TEST_NAME"

	setup_test_dir
	cd "$TEST_DIR"

	local template_id="official/blank-workspace"

	echo "Testing: nfw new TestWorkspace --template $template_id --no-input"

	if nfw new TestWorkspace --template "$template_id" --no-input 2>&1; then
		:
	else
		log_fail "nfw new command failed"
		exit 1
	fi

	assert_dir_exists "$TEST_DIR/TestWorkspace" "Workspace directory"
	assert_file_exists "$TEST_DIR/TestWorkspace/nfw.yaml" "Workspace config"

	local template_in_config
	template_in_config=$(grep -o 'template:.*' "$TEST_DIR/TestWorkspace/nfw.yaml" || echo "")
	if [[ -n "$template_in_config" ]]; then
		log_pass "Template identifier recorded in configuration"
	else
		log_fail "Template identifier not found in configuration"
		exit 1
	fi

	log_pass "$TEST_NAME"
	exit 0
}

main "$@"
