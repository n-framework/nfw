#!/usr/bin/env bash
# Error Path Smoke Test
# Validates that CLI handles error conditions gracefully

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

TEST_NAME="Error Path Handling"

main() {
    echo "Running: $TEST_NAME"

    setup_test_dir
    cd "$TEST_DIR"

    local test_passed=0
    local test_failed=0

    # Test 1: Invalid template should fail gracefully
    echo "Test 1: Invalid template identifier"
    if nfw new TestWorkspace --template "nonexistent/template" --no-input 2>&1 | grep -q "template"; then
        log_pass "Invalid template rejected with appropriate error message"
        ((test_passed++))
    else
        log_fail "Invalid template did not produce expected error"
        ((test_failed++))
    fi

    # Test 2: Missing .NET SDK should be detected
    echo "Test 2: Missing .NET SDK detection"
    # Temporarily hide dotnet from PATH
    local original_path="$PATH"
    PATH=$(echo "$PATH" | sed 's|/usr/local/bin:/usr/bin:/bin||g')

    if nfw new TestWorkspace --template "official/blank-workspace" --no-input 2>&1 | grep -iqE "(dotnet|\.net|sdk)"; then
        log_pass "Missing .NET SDK detected appropriately"
        ((test_passed++))
    else
        log_fail "Missing .NET SDK not detected"
        ((test_failed++))
    fi

    PATH="$original_path"

    # Test 3: Invalid workspace name should be rejected
    echo "Test 3: Invalid workspace name"
    if nfw new "" --template "official/blank-workspace" --no-input 2>&1 | grep -iqE "(invalid|name|required)"; then
        log_pass "Empty workspace name rejected"
        ((test_passed++))
    else
        log_fail "Empty workspace name not rejected"
        ((test_failed++))
    fi

    # Test 4: Permission issues during workspace creation
    echo "Test 4: Permission handling"
    local restricted_dir="$TEST_DIR/restricted"
    mkdir -p "$restricted_dir"
    chmod 000 "$restricted_dir"

    if nfw new TestWorkspace --template "official/blank-workspace" --no-input --cwd "$restricted_dir" 2>&1 | grep -iqE "(permission|denied|access)"; then
        log_pass "Permission errors handled gracefully"
        ((test_passed++))
    else
        log_fail "Permission errors not handled properly"
        ((test_failed++))
    fi

    chmod 755 "$restricted_dir"

    # Test 5: Template cache empty scenario
    echo "Test 5: Empty template cache"
    # This test assumes we can clear the template cache
    # If nfw templates cache is cleared, should provide helpful error
    log_info "Template cache test - verify error message quality"

    # Summary
    echo "=================================="
    echo "Error Path Tests Summary:"
    echo "  Passed: $test_passed"
    echo "  Failed: $test_failed"
    echo "=================================="

    if [[ $test_failed -eq 0 ]]; then
        log_pass "$TEST_NAME"
        exit 0
    else
        log_fail "$TEST_NAME - $test_failed test(s) failed"
        exit 1
    fi
}

main "$@"
