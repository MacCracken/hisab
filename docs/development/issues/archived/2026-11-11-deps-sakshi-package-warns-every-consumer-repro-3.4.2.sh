#!/usr/bin/env bash
# The sakshi repro (2026-11-11-deps-sakshi-package-warns-every-consumer-repro.sh, beside this) against a
# hisab working tree instead of a tag, so the fix can be checked before 3.4.2 is tagged: the same consumer
# (stdlib = [..., "sakshi", "log"]) with a path-only [deps.hisab]. Exits 1 while `cyrius deps` warns
# "refusing to overwrite stdlib leaf 'sakshi'", 0 when it does not.
#
#   docs/development/issues/archived/2026-11-11-deps-sakshi-package-warns-every-consumer-repro-3.4.2.sh [HISAB_TREE]
#
# HISAB_TREE defaults to the checkout this script sits in. Paired run, 2026-11-14 (cyrius 6.7.18): the 3.4.1
# tag's tree (git archive 3.4.1) exits 1 with the warning; the 3.4.2 tree exits 0, resolving 1 dep.
set -u
here=$(cd "$(dirname "$0")/../../../.." && pwd)
tree=$(cd "${1:-$here}" && pwd)
d=$(mktemp -d)
cd "$d" || exit 2
mkdir -p src
cat > cyrius.cyml <<EOT
[package]
name = "hisabconsumer"
version = "0.0.1"
language = "cyrius"
cyrius = "6.7.18"

[build]
entry = "src/main.cyr"
output = "build/hisabconsumer"

[deps]
stdlib = ["syscalls", "string", "alloc", "str", "fmt", "io", "vec", "sakshi", "log"]

[deps.hisab]
path = "$tree"
modules = ["dist/hisab.cyr"]
EOT
printf 'fn main(): i64 { return 0; }\n\nvar r = main();\nsys_exit_group(r);\n' > src/main.cyr
out=$(cyrius deps 2>&1)
printf '%s\n' "$out" | head -8
cd / && rm -rf "$d"
printf '%s\n' "$out" | grep -q "refusing to overwrite stdlib leaf 'sakshi'" && exit 1
exit 0
