#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

git -C "${PROJECT_ROOT}" submodule update --init --recursive --quiet 2>/dev/null || true

cd "${PROJECT_ROOT}"
cargo fetch
