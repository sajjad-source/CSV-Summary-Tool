#!/usr/bin/env bash
set -euo pipefail

# increment_version.sh
# Usage:  ./increment_version.sh  <path/to/Cargo.toml>

toml="$1"

current=$(grep -m1 '^version' "$toml" | cut -d '"' -f2)
IFS='.' read -r major minor patch <<<"$current"
patch=$((patch + 1))
next="$major.$minor.$patch"

sed -i'' -E "0,/^version = \".*\"/s//version = \"$next\"/" "$toml"

echo "Bumped $current → $next"
echo "version=$next" >> "$GITHUB_OUTPUT"
