#!/usr/bin/env bash
# Service Scaffolding Smoke Test
# Validates: T013 from tasks.md

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

TEST_NAME="Service Scaffolding"

main() {
	echo "Running: $TEST_NAME"

	setup_test_dir
	cd "$TEST_DIR"

	local template_id="official/blank-workspace"

	echo "Creating workspace..."
	if ! nfw new TestWorkspace --template "$template_id" --no-input 2>&1; then
		log_fail "nfw new command failed"
		exit 1
	fi

	cd "$TEST_DIR/TestWorkspace"

	echo "Adding service..."
	# Use --template flag as per actual CLI syntax
	if ! nfw add service TestService --template official/dotnet-service --no-input 2>&1; then
		log_fail "nfw add service command failed"
		exit 1
	fi

	assert_dir_exists "$TEST_DIR/TestWorkspace/src/TestService" "Service directory"

	log_pass "$TEST_NAME"
	exit 0
}

main "$@"
