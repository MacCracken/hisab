#!/usr/bin/env bash
# hisab 3.4.1: cyrius.cyml declares [deps.sakshi] (tag 2.5.10), but dist/hisab.cyr makes no sakshi_ call. A consumer
# that also needs the sakshi stdlib leaf — anything that uses lib/log.cyr or sandhi — gets "refusing to overwrite
# stdlib leaf 'sakshi' … declare it in [deps] stdlib OR depend on the package, not both" on every resolve, and so
# does every consumer of that consumer (ushma 2.0.0 port, M0 probe 10: ushma's deps, distlib --all, distlib --check
# and each of four consumers' resolves). The copies are byte-identical at the pin (sha256 8a819554…) and the stdlib
# one wins, so it is noise, but it can only be silenced in hisab. Exits 1 while the warning appears.
set -u
d=$(mktemp -d)
cd "$d" || exit 2
mkdir -p src
cat > cyrius.cyml <<'EOF'
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
git = "https://github.com/MacCracken/hisab.git"
tag = "3.4.1"
modules = ["dist/hisab.cyr"]
EOF
printf '# Nothing to run: the warning is the resolve'"'"'s.\nfn main(): i64 { return 0; }\n\nvar r = main();\nsys_exit_group(r);\n' > src/main.cyr
out=$(cyrius deps 2>&1)
printf '%s\n' "$out" | grep -E 'sakshi' | head -3
printf '%s\n' "$out" | grep -q "refusing to overwrite stdlib leaf 'sakshi'" && exit 1
exit 0
