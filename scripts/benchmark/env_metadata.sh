#!/usr/bin/env bash
# Environment metadata collection for benchmark results
# Collects hardware and system information for benchmarking context

set -euo pipefail

collect_env_metadata() {
	local benchmark_bin="${1:-}"
	local cpu_cores ram_mb os disk_type cli_version

	cpu_cores=$(get_cpu_cores)
	ram_mb=$(get_ram_mb)
	os=$(get_os)
	disk_type=$(get_disk_type)
	cli_version=$(get_cli_version "$benchmark_bin")

	cat <<EOF
{
  "cpu_cores": $cpu_cores,
  "ram_mb": $ram_mb,
  "os": "$os",
  "disk_type": "$disk_type",
  "cli_version": "$cli_version"
}
EOF
}

get_cpu_cores() {
	local cores
	if command -v nproc &>/dev/null; then
		cores=$(nproc)
	elif command -v sysctl &>/dev/null; then
		cores=$(sysctl -n hw.ncpu 2>/dev/null || echo 1)
	else
		cores=1
	fi
	echo "${cores:-1}"
}

get_ram_mb() {
	local ram_mb
	if command -v free &>/dev/null; then
		ram_mb=$(free -m 2>/dev/null | awk '/^Mem:/{print $2}')
	elif command -v sysctl &>/dev/null; then
		ram_mb=$(sysctl -n hw.memsize 2>/dev/null | awk '{print int($1/1024/1024)}')
	elif [[ -f /proc/meminfo ]]; then
		ram_mb=$(awk '/^MemTotal:/{print int($2/1024)}' /proc/meminfo)
	else
		ram_mb=0
	fi
	echo "${ram_mb:-0}"
}

get_os() {
	local os
	if [[ -f /etc/os-release ]]; then
		os=$(awk -F= '/^ID=/{print $2}' /etc/os-release | tr -d '"' | tr ' ' '-')
	elif command -v uname &>/dev/null; then
		os=$(uname -s | tr '[:upper:]' '[:lower:]')
		os="${os}-$(uname -m)"
	else
		os="unknown"
	fi
	echo "${os:-unknown}"
}

get_disk_type() {
	local disk_type="unknown"

	# Try Linux sysfs first (handles both SATA and NVMe)
	for device in /sys/block/*; do
		local device_name="${device##*/}"

		# Skip loop devices and device mapper
		if [[ "$device_name" =~ ^(loop|dm-) ]]; then
			continue
		fi

		local rotational_file="$device/queue/rotational"
		if [[ -f "$rotational_file" ]]; then
			local rotational
			rotational=$(cat "$rotational_file" 2>/dev/null || echo "1")

			if [[ "$rotational" == "0" ]]; then
				# Check if it's NVMe
				if [[ "$device_name" =~ ^nvme ]]; then
					disk_type="nvme"
				else
					disk_type="ssd"
				fi
				break
			else
				disk_type="hdd"
				break
			fi
		fi
	done

	# Fallback to macOS diskutil
	if [[ "$disk_type" == "unknown" ]] && command -v diskutil &>/dev/null; then
		local protocol
		protocol=$(diskutil info / 2>/dev/null | awk '/Protocol:/{print tolower($2)}' || echo "unknown")
		case "$protocol" in
			*nvme*) disk_type="nvme" ;;
			*ssd*) disk_type="ssd" ;;
			*sata*) disk_type="hdd" ;;
		esac
	fi

	echo "$disk_type"
}

extract_semver() {
	echo "$1" | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+([-.][[:alnum:].]+)?' | head -n 1
}

get_cli_version() {
	local benchmark_bin="${1:-}"
	local output version

	if [[ -n "$benchmark_bin" && -x "$benchmark_bin" ]]; then
		if output=$("$benchmark_bin" --version 2>/dev/null); then
			version=$(extract_semver "$output")
			echo "${version:-unknown}"
			return
		fi

		if output=$("$benchmark_bin" version 2>/dev/null); then
			version=$(extract_semver "$output")
			echo "${version:-unknown}"
			return
		fi
	fi

	echo "unknown"
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
	collect_env_metadata
fi
