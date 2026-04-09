#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=packages/acore-scripts/src/logger.sh
source "${SCRIPT_DIR}/../../../packages/acore-scripts/src/logger.sh"

REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
cd "$REPO_ROOT"

acore_log_section "🔍 Linting markdown files with markdownlint-cli2..."

if ! command -v bun &> /dev/null; then
	acore_log_warning "bun not found, skipping markdown linting"
	exit 0
fi

mapfile -t markdown_files < <(fd -e md -t f . "$REPO_ROOT")
if [ ${#markdown_files[@]} -eq 0 ]; then
	acore_log_warning "No markdown files found."
	exit 0
fi

if ! bun pm ls 2> /dev/null | grep -q markdownlint-cli2; then
	acore_log_info "Installing markdownlint-cli2..."
	if ! bun add -g markdownlint-cli2; then
    acore_log_error "Failed to install markdownlint-cli2"
    acore_log_error "Please check your network connection and bun permissions"
    exit 1
fi
fi

bunx markdownlint-cli2 --fix "${markdown_files[@]}"

acore_log_success "✨ Markdown linting complete!"
