#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CLI_PROJECT="$ROOT_DIR/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj"
ITERATIONS="${1:-5}"

if [[ ! -f "$CLI_PROJECT" ]]; then
  echo "CLI project not found: $CLI_PROJECT" >&2
  exit 1
fi

if ! [[ "$ITERATIONS" =~ ^[0-9]+$ ]] || [[ "$ITERATIONS" -le 0 ]]; then
  echo "Iterations must be a positive integer." >&2
  exit 2
fi

total_ms=0
for ((i = 1; i <= ITERATIONS; i += 1)); do
  start_ns="$(date +%s%N)"
  dotnet run --project "$CLI_PROJECT" -- --help >/dev/null
  end_ns="$(date +%s%N)"
  elapsed_ms=$(((end_ns - start_ns) / 1000000))
  total_ms=$((total_ms + elapsed_ms))
  echo "run #$i: ${elapsed_ms}ms"
done

average_ms=$((total_ms / ITERATIONS))
echo "average help startup: ${average_ms}ms"
