#!/usr/bin/env bash
# Workspace Generation Smoke Test
# Validates: T012 from tasks.md

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

TEST_NAME="Workspace Generation"

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
	assert_dir_exists "$TEST_DIR/TestWorkspace/src" "src/ directory"
	assert_file_exists "$TEST_DIR/TestWorkspace/nfw.yaml" "nfw.yaml config"

	log_pass "$TEST_NAME"
	exit 0
}

main "$@"
