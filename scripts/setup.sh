#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

source "${PROJECT_ROOT}/packages/acore-scripts/src/logger.sh"

if ! git -C "${PROJECT_ROOT}" submodule update --init --recursive --quiet; then
    acore_log_error "Failed to initialize submodules"
    acore_log_error "Please check your network connection and git credentials"
    exit 1
fi

cd "${PROJECT_ROOT}"
cargo fetch

acore_log_success "✅ Setup complete!"
