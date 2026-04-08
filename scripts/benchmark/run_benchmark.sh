#!/usr/bin/env bash
# Benchmark Harness: measures CLI generation performance
# Usage: ./run_benchmark.sh [--iterations N] [--test NAME] [--help]
#
# Tests:
#   workspace_creation  - Measures `nfw new` end-to-end
#   service_creation    - Measures `nfw add service` end-to-end
#   combined            - Measures workspace + service creation (SC-001 target)
#
# Output: JSON to stdout and saved to benchmark-results.json

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

source "$SCRIPT_DIR/env_metadata.sh"

# Simple logging functions (self-contained)
log_info() {
	echo "[INFO] $1"
}

log_success() {
	echo "[SUCCESS] $1"
}

log_error() {
	echo "[ERROR] $1" >&2
}

log_header() {
	echo "=================================="
	echo "$1"
	echo "=================================="
}

log_divider() {
	echo "----------------------------------"
}

# ============================================================================
# Configuration
# ============================================================================

DEFAULT_ITERATIONS=10
DEFAULT_TEST="combined"
TARGET_MS=1000
OUTPUT_FILE="benchmark-results.json"

# ============================================================================
# Argument parsing
# ============================================================================

ITERATIONS="${BENCHMARK_ITERATIONS:-$DEFAULT_ITERATIONS}"
TEST_NAME="${BENCHMARK_TEST:-$DEFAULT_TEST}"
BENCHMARK_BIN_PATH=""
BENCHMARK_STATE_DIR=""
BENCHMARK_HOME=""
BENCHMARK_TEMPLATE_SOURCE_PATH=""

resolve_benchmark_bin() {
	if [[ -n "${BENCHMARK_BIN:-}" ]]; then
		local configured_bin="$BENCHMARK_BIN"
		if [[ "$configured_bin" != /* ]]; then
			configured_bin="$WORKSPACE_ROOT/$configured_bin"
		fi

		if [[ -x "$configured_bin" ]]; then
			echo "$configured_bin"
			return
		fi

		log_error "BENCHMARK_BIN is not executable: $configured_bin"
		exit 1
	fi

	local candidates=(
		"$WORKSPACE_ROOT/target/release/nframework-nfw-cli"
		"$WORKSPACE_ROOT/target/release/nfw"
		"$WORKSPACE_ROOT/target/debug/nframework-nfw-cli"
		"$WORKSPACE_ROOT/target/debug/nfw"
	)

	local candidate
	for candidate in "${candidates[@]}"; do
		if [[ -x "$candidate" ]]; then
			echo "$candidate"
			return
		fi
	done

	log_error "Could not find CLI binary. Build first with: cargo build --workspace --release"
	exit 1
}

resolve_template_source_path() {
	local configured_source="${BENCHMARK_TEMPLATE_SOURCE:-}"
	local candidate=""

	if [[ -n "$configured_source" ]]; then
		if [[ "$configured_source" == /* ]]; then
			candidate="$configured_source"
		else
			candidate="$WORKSPACE_ROOT/$configured_source"
		fi
	else
		candidate="$WORKSPACE_ROOT/../nfw-templates"
	fi

	if [[ ! -d "$candidate" ]]; then
		log_error "Template source path does not exist: $candidate"
		exit 1
	fi

	(
		cd "$candidate"
		pwd
	)
}

setup_benchmark_environment() {
	BENCHMARK_TEMPLATE_SOURCE_PATH=$(resolve_template_source_path)
	BENCHMARK_STATE_DIR=$(mktemp -d "${TMPDIR:-/tmp}/nfw-bench-env-XXXXXX")
	BENCHMARK_HOME="$BENCHMARK_STATE_DIR/home"
	local config_dir="$BENCHMARK_HOME/.nfw"

	mkdir -p "$config_dir"

	cat >"$config_dir/sources.yaml" <<EOF
sources:
  - name: official
    url: "file://$BENCHMARK_TEMPLATE_SOURCE_PATH"
    enabled: true
EOF
}

run_nfw_command() {
	HOME="$BENCHMARK_HOME" "$BENCHMARK_BIN_PATH" "$@"
}

usage() {
	echo "Usage: $0 [OPTIONS]"
	echo ""
	echo "Options:"
	echo "  --iterations N   Number of benchmark iterations (default: $DEFAULT_ITERATIONS)"
	echo "  --test NAME      Test to run: workspace_creation, service_creation, combined (default: $DEFAULT_TEST)"
	echo "  --help           Show this help message"
	echo ""
	echo "Output: JSON to stdout and saved to $OUTPUT_FILE"
}

while [[ $# -gt 0 ]]; do
	case $1 in
	--iterations)
		if [[ $# -lt 2 ]]; then
			log_error "Missing value for --iterations"
			exit 1
		fi
		ITERATIONS="$2"
		shift 2
		;;
	--test)
		if [[ $# -lt 2 ]]; then
			log_error "Missing value for --test"
			exit 1
		fi
		TEST_NAME="$2"
		shift 2
		;;
	--help)
		usage
		exit 0
		;;
	*)
		log_error "Unknown option: $1"
		usage
		exit 1
		;;
	esac
done

if ! [[ "$ITERATIONS" =~ ^[1-9][0-9]*$ ]]; then
	log_error "Invalid --iterations value: $ITERATIONS (must be a positive integer)"
	exit 1
fi

# ============================================================================
# Timing helpers
# ============================================================================

# Returns current time in nanoseconds
now_ns() {
	date +%s%N
}

# Converts nanoseconds to milliseconds
ns_to_ms() {
	echo $(( $1 / 1000000 ))
}

# ============================================================================
# Benchmark functions
# ============================================================================

benchmark_workspace_creation() {
	local test_dir
	test_dir=$(mktemp -d "${TMPDIR:-/tmp}/nfw-bench-XXXXXX")

	(
		cd "$test_dir"
		trap 'rm -rf "$test_dir" 2>/dev/null || true' EXIT
		start=$(now_ns)
		run_nfw_command new BenchmarkWorkspace --template official/blank-workspace --no-input >/dev/null 2>&1 || return 1
		end=$(now_ns)

		echo $((end - start))
	)
}

benchmark_service_creation() {
	local workspace_dir
	workspace_dir=$(mktemp -d "${TMPDIR:-/tmp}/nfw-bench-svc-XXXXXX")

	(
		cd "$workspace_dir"
		trap 'rm -rf "$workspace_dir" 2>/dev/null || true' EXIT
		run_nfw_command new BenchmarkWorkspace --template official/blank-workspace --no-input >/dev/null 2>&1 || return 1

		cd "$workspace_dir/BenchmarkWorkspace" || return 1
		start=$(now_ns)
		run_nfw_command add service BenchmarkService --template official/dotnet-service --no-input >/dev/null 2>&1 || return 1
		end=$(now_ns)

		echo $((end - start))
	)
}

benchmark_combined() {
	local workspace_dir
	workspace_dir=$(mktemp -d "${TMPDIR:-/tmp}/nfw-bench-combined-XXXXXX")

	(
		cd "$workspace_dir"
		trap 'rm -rf "$workspace_dir" 2>/dev/null || true' EXIT
		start=$(now_ns)
		run_nfw_command new BenchmarkWorkspace --template official/blank-workspace --no-input >/dev/null 2>&1 || return 1

		cd "$workspace_dir/BenchmarkWorkspace" || return 1
		run_nfw_command add service BenchmarkService --template official/dotnet-service --no-input >/dev/null 2>&1 || return 1
		end=$(now_ns)

		echo $((end - start))
	)
}

# ============================================================================
# Statistics computation
# ============================================================================

compute_statistics() {
	local -a values=("$@")
	local count=${#values[@]}

	if [[ $count -eq 0 ]]; then
		echo '{"median_ms":0,"p95_ms":0,"min_ms":0,"max_ms":0,"mean_ms":0}'
		return
	fi

	# Sort values
	local sorted=($(printf '%s\n' "${values[@]}" | sort -n))

	local min_ms=${sorted[0]}
	local max_ms=${sorted[$((count - 1))]}

	# Mean
	local sum=0
	for v in "${sorted[@]}"; do
		sum=$((sum + v))
	done
	local mean_ms=$((sum / count))

	# Median
	local mid=$((count / 2))
	local median_ms
	if [[ $((count % 2)) -eq 0 ]]; then
		median_ms=$(((sorted[mid-1] + sorted[mid]) / 2))
	else
		median_ms=${sorted[mid]}
	fi

	# P95
	local p95_index=$(((count * 95 + 99) / 100 - 1))
	if [[ $p95_index -ge $count ]]; then
		p95_index=$((count - 1))
	fi
	local p95_ms=${sorted[$p95_index]}

	echo "{\"median_ms\":$median_ms,\"p95_ms\":$p95_ms,\"min_ms\":$min_ms,\"max_ms\":$max_ms,\"mean_ms\":$mean_ms}"
}

# ============================================================================
# Main execution
# ============================================================================

run_benchmark() {
	trap 'if [[ -n "$BENCHMARK_STATE_DIR" ]]; then rm -rf "$BENCHMARK_STATE_DIR" 2>/dev/null || true; fi' EXIT
	BENCHMARK_BIN_PATH=$(resolve_benchmark_bin)
	setup_benchmark_environment
	log_info "Using CLI binary: $BENCHMARK_BIN_PATH"
	log_info "Using template source: file://$BENCHMARK_TEMPLATE_SOURCE_PATH"
	log_header "Benchmark: $TEST_NAME ($ITERATIONS iterations)"

	declare -a durations_ms=()

	for i in $(seq 1 $ITERATIONS); do
		log_info "Iteration $i/$ITERATIONS..."

		local ns_result=""
		case $TEST_NAME in
		workspace_creation)
			if ! ns_result=$(benchmark_workspace_creation); then
				log_error "Iteration $i failed: workspace_creation command failed"
				exit 1
			fi
			;;
		service_creation)
			if ! ns_result=$(benchmark_service_creation); then
				log_error "Iteration $i failed: service_creation command failed"
				exit 1
			fi
			;;
		combined)
			if ! ns_result=$(benchmark_combined); then
				log_error "Iteration $i failed: combined command failed"
				exit 1
			fi
			;;
		*)
			log_error "Unknown test: $TEST_NAME"
			exit 1
			;;
		esac

		local ms_result=0
		ms_result=$(ns_to_ms "$ns_result")
		durations_ms+=("$ms_result")

		log_info "  Result: ${ms_result}ms"
	done

	# Compute statistics
	local stats_json
	stats_json=$(compute_statistics "${durations_ms[@]}")

	# Determine pass/fail
	local p95_ms
	p95_ms=$(echo "$stats_json" | grep -o '"p95_ms":[0-9]*' | cut -d: -f2)
	local passed="false"
	if [[ "$p95_ms" -le "$TARGET_MS" ]]; then
		passed="true"
	fi

	# Collect environment metadata
	local env_json
	env_json=$(collect_env_metadata "$BENCHMARK_BIN_PATH")

	# Build result JSON
	local timestamp
	timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

	local results_json="["
	for i in "${!durations_ms[@]}"; do
		if [[ $i -gt 0 ]]; then
			results_json+=","
		fi
		results_json+="{\"iteration\":$((i + 1)),\"duration_ms\":${durations_ms[$i]}}"
	done
	results_json+="]"

	local full_json
	full_json=$(
		cat <<EOF
{
  "test_name": "$TEST_NAME",
  "iterations": $ITERATIONS,
  "results": $results_json,
  "statistics": $stats_json,
  "target_ms": $TARGET_MS,
  "passed": $passed,
  "environment": $env_json,
  "timestamp": "$timestamp"
}
EOF
	)

	# Output
	echo "$full_json"
	echo "$full_json" >"$OUTPUT_FILE"

	log_divider

	if [[ "$passed" == "true" ]]; then
		log_success "PASSED: p95=${p95_ms}ms <= target=${TARGET_MS}ms"
	else
		log_error "FAILED: p95=${p95_ms}ms > target=${TARGET_MS}ms"
	fi

	log_info "Results saved to $OUTPUT_FILE"
}

run_benchmark
