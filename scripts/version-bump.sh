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

# src/main.cyr hardcodes the string the CLI prints. Cyrius has no build-time
# string interpolation, so ${file:VERSION} cannot reach it and it was a manual
# edit this script did not make and no gate checked — the 2.9.1 → 2.9.2 bump
# left the CLI reporting 2.9.1. Rewritten here; CI's version-consistency step
# asserts it independently, so a hand-edit that misses it still fails the build.
if grep -qE '^\s*println\("hisab [0-9]+\.[0-9]+\.[0-9]+"\);' "$REPO_ROOT/src/main.cyr"; then
  sed -i -E "s/^(\s*)println\(\"hisab [0-9]+\.[0-9]+\.[0-9]+\"\);/\1println(\"hisab ${NEW_VERSION}\");/" \
    "$REPO_ROOT/src/main.cyr"
  echo "src/main.cyr: CLI version string -> $NEW_VERSION"
else
  echo "::warning:: src/main.cyr has no recognisable 'println(\"hisab X.Y.Z\");' — update it by hand"
fi

echo "VERSION: $NEW_VERSION"
echo "Next:"
echo "  1) Add a '## [${NEW_VERSION}] - $(date -u +%Y-%m-%d)' section in CHANGELOG.md"
echo "  2) Run 'cyrius distlib' — dist/hisab.cyr embeds the version in its header"
echo "  3) git commit, tag '$NEW_VERSION' (or 'v${NEW_VERSION}'), push tag"
