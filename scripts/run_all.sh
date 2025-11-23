#!/usr/bin/env bash
# Run all Cargo examples in this crate.
# Always sets INSTA_UPDATE=always and runs all examples sequentially.
# Continues on failures and prints a summary at the end.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

export INSTA_UPDATE=always

shopt -s nullglob
examples=(examples/*.rs)
shopt -u nullglob

if [[ ${#examples[@]} -eq 0 ]]; then
  echo "No example files found in ./examples."
  exit 0
fi

count=0
failures=()
echo "Discovered ${#examples[@]} example(s). Running them sequentially..."
for f in "${examples[@]}"; do
  name="$(basename "${f%.rs}")"
  echo
  echo "==> Running example: ${name}"
  if cargo run --example "${name}"; then
    echo "OK: ${name}"
  else
    echo "FAIL: ${name}"
    failures+=("${name}")
  fi
  ((count++))
done

echo
if [[ ${#failures[@]} -gt 0 ]]; then
  echo "Completed ${count}/${#examples[@]} example(s). Failures: ${#failures[@]}"
  printf '  - %s\n' "${failures[@]}"
  exit 1
else
  echo "Completed ${count}/${#examples[@]} example(s). All succeeded."
fi
