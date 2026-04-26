#!/usr/bin/env bash
# Bump the project version. cyrius.cyml pulls the version via
# ${file:VERSION} interpolation, so VERSION is the single source
# of truth — no manifest edit needed. CHANGELOG.md still requires
# a manual section header for the new version.
set -euo pipefail

[ $# -ne 1 ] && { echo "Usage: $0 <semver>"; exit 1; }
NEW_VERSION="$1"

echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$' || {
  echo "ERROR: '$NEW_VERSION' is not semver x.y.z"; exit 1; }

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "$NEW_VERSION" > "$REPO_ROOT/VERSION"

if ! grep -q '^version = "${file:VERSION}"' "$REPO_ROOT/cyrius.cyml"; then
  echo "::warning:: cyrius.cyml does not use \${file:VERSION} — manifest version may drift"
fi

echo "VERSION: $NEW_VERSION"
echo "Next:"
echo "  1) Add a '## [${NEW_VERSION}] - $(date -u +%Y-%m-%d)' section in CHANGELOG.md"
echo "  2) git commit, tag '$NEW_VERSION' (or 'v${NEW_VERSION}'), push tag"
