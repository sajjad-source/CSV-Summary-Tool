#!/usr/bin/env bash
set -euo pipefail

paths="${CRATE_PATHS:-.}"

for dir in ${paths//,/ }; do
    ver=$(grep -m1 '^version' "$dir/Cargo.toml" | cut -d '"' -f2)
    if [[ -z "${version:-}" ]]; then
        version="$ver"
    elif [[ "$ver" != "$version" ]]; then
        echo "All crates must share the same version" >&2
        exit 1
    fi
done

echo "version=$version" >> "$GITHUB_OUTPUT"
