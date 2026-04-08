#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=packages/acore-scripts/src/logger.sh
source "${SCRIPT_DIR}/../../../packages/acore-scripts/src/logger.sh"

REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
cd "$REPO_ROOT"

acore_log_section "🦀 Linting Rust code with cargo clippy..."

fd -t f Cargo.toml . "$REPO_ROOT" | while read -r cargo_file; do
	cargo_dir="$(dirname "$cargo_file")"
	acore_log_info "📋 Linting $(basename "$cargo_dir")..."
	(cd "$cargo_dir" && cargo clippy -- -D warnings)
done

acore_log_info "▶️ Running cargo machete (unused dependency check)..."
cargo machete

acore_log_success "✨ Rust linting complete!"
