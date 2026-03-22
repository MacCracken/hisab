#!/usr/bin/env bash
# version-bump.sh — Update the version across Cargo.toml and VERSION file.
#
# Usage: ./scripts/version-bump.sh <new-version>
# Example: ./scripts/version-bump.sh 0.2.0

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <new-version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION="$1"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Validate version format (semver)
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
    echo "Error: version must be semver (e.g., 0.2.0 or 0.2.0-rc1)"
    exit 1
fi

echo "Bumping version to $NEW_VERSION"

# Update VERSION file
echo "$NEW_VERSION" > "$REPO_ROOT/VERSION"
echo "  Updated VERSION"

# Update workspace Cargo.toml
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$REPO_ROOT/Cargo.toml"
echo "  Updated Cargo.toml"

# Verify
echo ""
echo "Done. New version: $(cat "$REPO_ROOT/VERSION")"
echo "Run 'cargo check --workspace' to verify."
