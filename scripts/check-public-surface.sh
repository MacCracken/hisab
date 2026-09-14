#!/usr/bin/env bash
# check-public-surface.sh — prove that hisab's `public` annotations describe a
# COMPLETE and EXACT surface, by flipping every module `private` in a scratch
# copy and compiling against it the way the suites and a consumer would.
#
# Added 3.1.0 (the `pub fn` half of the public/private surface). In the shipped
# tree no module is `private`, so `public` is a documented no-op — the suites
# compile byte-identical with and without it. That is exactly why a gate is
# needed: nothing in an ordinary build can tell a complete surface from an
# incomplete one, and a new cross-module reference to an unannotated helper
# would be found only at the 4.0.0 flip, by whoever flips it.
#
# Five claims, each fail-closed:
#   0. NAMING COMPLETENESS — every top-level fn / struct / enum / var in the
#      real tree whose name does not start with `_` carries `public`. The
#      convention IS the surface; this is the claim a dropped marker violates.
#   1. INTERNAL COMPLETENESS — with `private` at the top of all 35 modules, a
#      probe that `include`s each module AS ITS OWN FILE (the way tests/*.tcyr
#      and examples/*.cyr do) compiles with ZERO 'is private to its file'
#      errors: every cross-module reference inside hisab names a `public` item.
#      ⛔ NOT the bundle. `dist/hisab.cyr` is ONE file and `private` is
#      per-file, so inside the bundle every "cross-module" call is an in-file
#      call and can never be refused — the first draft of this claim checked
#      the flipped bundle, stayed green with `_perm` unmarked, and was proving
#      nothing. The bundle's 35 `private` lines only ever face a consumer.
#   2. EXTERNAL REACHABILITY — a generated consumer that CALLS every `public fn`
#      (arity read from the declaration, zero arguments, compile-only), calls
#      every derived accessor of every `public struct`, and reads every `public
#      var` and every member of every `public enum`, compiles with ZERO
#      violations against the flipped bundle.
#   3. EXACTNESS (anti-vacuous) — a second generated consumer that calls every
#      NON-public top-level fn and reads every non-public global is refused on
#      every probe. KNOWN_LEAKS (below) is the allowlist for pinned compiler
#      defects; it is EMPTY since the 6.6.4 pin (3.1.1) and any entry added to
#      it FAILS this gate the day upstream fixes the defect it names.
#   4. The real consumers in examples/*.cyr compile clean under the flip.
#
# ⛔ WHY THE PROBES ARE CALLS AND NOT `&name`: on cycc 6.6.2 and 6.6.3 a
# file-private fn was reachable from another file through address-of — `&_helper`
# compiled and the pointer ran through callptr/fncallN — while a direct call was
# refused, so `&name` proved nothing in either direction. Filed upstream as
# cyrius/docs/development/issues/2026-09-13-hisab-private-fn-reachable-via-address-of.md
# and FIXED in 6.6.4 (eleven resolution paths now check; hisab closed it in
# 3.1.1, see docs/development/issues/archived/2026-09-13-cyrius-private-fn-
# reachable-via-address-of.md). The probes STAY calls: a call is the shape a
# consumer writes, and a gate that only worked from 6.6.4 up would be blind on
# every pin below it.
#
# Mutation-proven at 3.1.0 (each mutant verified installed, then restored):
#   - drop `public` from `_perm` (calc.cyr; reached from calc_ext + noise_simplex)
#       -> claim 1 fails, 42 sites;
#   - drop `public` from `hvec3_new`        -> claim 0 fails (and 1: 54 sites, 4: 20 sites);
#   - a `_`-named fn placed directly after `public enum HsbError`
#       -> claim 3 fails: UNEXPECTED leak (the compiler defect, caught);
#   - a public declaration inserted between `AdPowLimit` and `_ad_pow`
#       -> claim 3 fails: known leak now REFUSED (the allowlist inverts).
#   ⭐ 3.1.1: that inversion FIRED FOR REAL on the 6.6.4 pin bump — claim 3
#   reported `_ad_pow` refused before any source changed, which is how the
#   upstream fix was detected here rather than read off a changelog.
#
# Nothing under the repo is modified: everything happens in a mktemp copy.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

T="$(mktemp -d)"
trap 'rm -rf "$T"' EXIT
tar --exclude=.git --exclude=build -cf - . | tar -C "$T" -xf -
cd "$T"

MODULES=$(grep -E '^\s*"src/[a-z_0-9]+\.cyr",?\s*$' cyrius.cyml | tr -d ' ",')
n_mod=0
for m in $MODULES; do
    { echo "private"; echo; cat "$m"; } > "$m.tmp" && mv "$m.tmp" "$m"
    n_mod=$((n_mod + 1))
done
[ "$n_mod" -ge 30 ] || { echo "FAIL: only $n_mod [lib] modules found — manifest parse broke"; exit 1; }

cyrius distlib >/dev/null 2>&1 || { echo "FAIL: cyrius distlib failed on the flipped copy"; exit 1; }
n_priv=$(grep -c '^private$' dist/hisab.cyr || true)
[ "$n_priv" -eq "$n_mod" ] || { echo "FAIL: bundle carries $n_priv 'private' lines for $n_mod modules — distlib did not pass them through"; exit 1; }

fail=0
echo "=== public-surface gate: $n_mod modules flipped private in $T ==="

# ── 0. naming completeness (real tree, before the flip touched anything) ─────
unmarked=$(cd "$REPO_ROOT" && grep -nE '^(fn|struct|enum|var) [A-Za-z][A-Za-z0-9_]*' $MODULES || true)
if [ -n "$unmarked" ]; then
    echo "  0. naming completeness: FAIL — non-underscore top-level declaration(s) without \`public\`:"
    echo "$unmarked" | head -20 | sed 's/^/       /'
    fail=1
else
    echo "  0. naming completeness: ok (every non-underscore top-level declaration carries public)"
fi

# ── 1. internal completeness, per-file includes ─────────────────────────────
{ for m in $MODULES; do echo "include \"$m\""; done; echo 'fn _internal_probe() { return 0; }'; } > internal_probe.cyr
out=$(cyrius check --with-deps internal_probe.cyr 2>&1 || true)
v=$(echo "$out" | grep -c "is private to its file" || true)
if [ "$v" -ne 0 ] || ! echo "$out" | grep -q '^ok:'; then
    echo "  1. internal completeness: FAIL — $v cross-module reference(s) to a non-public item"
    echo "$out" | grep "is private" | sed -E "s/.*'([^']+)' is private.*/\1/" | sort | uniq -c | sort -rn | head -20 | sed 's/^/       /'
    fail=1
else
    echo "  1. internal completeness: ok (0 violations across $n_mod modules included as separate files)"
fi

# ── 2 + 3. generate the two consumers from the (flipped) sources ─────────────
python3 - "$T" $MODULES <<'EOF'
import re, sys
T = sys.argv[1]; mods = sys.argv[2:]
DIR = re.compile(r'#(must_use|pure|inline|io|alloc|deprecated|derive)\b')
pub_calls, pub_reads, priv_calls, priv_reads = [], [], [], []
n_pub_fn = n_pub_struct = n_pub_var = n_pub_enum = 0
for m in mods:
    lines = open(m).read().split('\n')
    i = 0
    while i < len(lines):
        l = lines[i]
        mm = re.match(r'^(public )?(fn|struct|enum|var)\s+([A-Za-z_][A-Za-z0-9_]*)', l)
        if not mm:
            i += 1; continue
        pub, kind, name = bool(mm.group(1)), mm.group(2), mm.group(3)
        if kind == 'fn':
            sig = l
            while '(' in sig and ')' not in sig.split('(', 1)[1]:
                i += 1; sig += ' ' + lines[i].strip()
            params = sig.split('(', 1)[1].split(')', 1)[0].strip()
            arity = 0 if params == '' else len([p for p in params.split(',') if p.strip()])
            call = f"{name}({', '.join(['0'] * arity)});"
            (pub_calls if pub else priv_calls).append(call)
            if pub: n_pub_fn += 1
        elif kind == 'var':
            (pub_reads if pub else priv_reads).append(f"var r_{name} = {name};")
            if pub: n_pub_var += 1
        elif kind == 'struct':
            body = l
            while '}' not in body:
                i += 1; body += ' ' + lines[i]
            fields = [f.strip() for f in body.split('{', 1)[1].split('}', 1)[0].split(';') if f.strip()]
            for f in fields:
                (pub_calls if pub else priv_calls).append(f"{name}_{f}(0);")
                (pub_calls if pub else priv_calls).append(f"{name}_set_{f}(0, 0);")
            if pub: n_pub_struct += 1
        elif kind == 'enum':
            # one-line and multi-line bodies alike: read to the closing brace, never past it
            body = l
            while '}' not in body:
                i += 1; body += ' ' + lines[i]
            inner = body.split('{', 1)[1].split('}', 1)[0]
            for em in re.finditer(r'(?<![A-Za-z0-9_])([A-Za-z_][A-Za-z0-9_]*)\s*=', inner):
                (pub_reads if pub else priv_reads).append(f"var r_{em.group(1)} = {em.group(1)};")
            if pub: n_pub_enum += 1
        i += 1
def emit(path, calls, reads):
    with open(path, 'w') as f:
        f.write('include "dist/hisab.cyr"\n')
        f.write('\n'.join(reads) + '\n')
        f.write('fn _surface_probe() {\n' + '\n'.join('    ' + c for c in calls) + '\n    return 0;\n}\n')
emit(f'{T}/consumer_public.cyr', pub_calls, pub_reads)
emit(f'{T}/consumer_private.cyr', priv_calls, priv_reads)
open(f'{T}/probe_counts', 'w').write(f"{len(pub_calls) + len(pub_reads)} {len(priv_calls) + len(priv_reads)} {n_pub_fn} {n_pub_struct} {n_pub_var} {n_pub_enum}\n")
EOF
read -r n_pub_probes n_priv_probes n_pub_fn n_pub_struct n_pub_var n_pub_enum < probe_counts
[ "$n_pub_probes" -gt 500 ] || { echo "FAIL: only $n_pub_probes public probes generated — the generator is broken, not the surface"; exit 1; }
[ "$n_priv_probes" -gt 300 ] || { echo "FAIL: only $n_priv_probes private probes generated — the generator is broken, not the surface"; exit 1; }

out=$(cyrius check --with-deps consumer_public.cyr 2>&1 || true)
v=$(echo "$out" | grep -c "is private to its file" || true)
e=$(echo "$out" | grep -c '^error' || true)
if [ "$v" -ne 0 ] || [ "$e" -ne 0 ]; then
    echo "  2. external reachability: FAIL — $v of $n_pub_probes public probes refused, $e error line(s)"
    echo "$out" | grep -E '^error' | head -20 | sed 's/^/       /'
    fail=1
else
    echo "  2. external reachability: ok ($n_pub_probes probes: $n_pub_fn fn, $n_pub_struct struct, $n_pub_var var, $n_pub_enum enum — all reachable from a consumer)"
fi

# ⛔ KNOWN LEAKS — pinned COMPILER defects, each keyed to its upstream filing. The
# exactness claim expects exactly these to be accepted; if one is REFUSED the
# upstream fix has landed and this gate FAILS until the entry is removed, so a
# pinned defect inverts into a tripwire for its own repair (2.20.0's rule).
#   EMPTY since 3.1.1 (cycc 6.6.4). The one entry it ever held:
#   _ad_pow — the declaration right after `public enum AdPowLimit` in autodiff.cyr.
#     A `public enum` leaked its marker onto the next top-level declaration
#     (cycc 6.6.2 and 6.6.3). Filed as
#     cyrius/docs/development/issues/2026-09-13-hisab-public-enum-leaks-onto-next-declaration.md,
#     fixed in 6.6.4 (`public` arms the marker only for a token that can carry
#     it), and this gate reported it REFUSED on the pin bump — the inversion
#     working as designed. `_SYM_EPS` (symbolic.cyr, after `public enum ExprTag`)
#     was the masked second instance; it is public by its own marker.
KNOWN_LEAKS=""
out=$(cyrius check --with-deps consumer_private.cyr 2>&1 || true)
python3 - "$T" "$out" "$KNOWN_LEAKS" <<'EOF' > exactness_report
import re, sys
T, out, known = sys.argv[1], sys.argv[2], set(sys.argv[3].split())
refused = set(re.findall(r"'([^']+)' is private", out))
probed = []
for l in open(f'{T}/consumer_private.cyr'):
    l = l.strip()
    if l.startswith('var r_'):
        probed.append(l.split(' = ')[1].rstrip(';'))
    elif l and not l.startswith(('include', 'fn ', 'return', '}')):
        probed.append(l.split('(')[0])
leaked = [n for n in probed if n not in refused]
unexpected = [n for n in leaked if n not in known]
stale = [n for n in known if n in refused]
print(len(probed), len(refused), ' '.join(unexpected) or '-', ' '.join(stale) or '-')
EOF
read -r n_probed n_refused unexpected stale < exactness_report
if [ "$unexpected" != "-" ]; then
    echo "  3. exactness: FAIL — $n_refused of $n_probed non-public probes refused; UNEXPECTED leak(s): $unexpected"
    fail=1
elif [ "$stale" != "-" ]; then
    echo "  3. exactness: FAIL — known leak(s) now REFUSED: $stale — upstream fixed it; remove the KNOWN_LEAKS entry and its filing pointer"
    fail=1
else
    echo "  3. exactness: ok ($n_refused of $n_probed non-public probes refused; known compiler leak(s) still present: ${KNOWN_LEAKS:-none})"
fi

# ── 4. real consumers under the flip ─────────────────────────────────────────
for ex in examples/*.cyr; do
    [ -f "$ex" ] || continue
    out=$(cyrius check --with-deps "$ex" 2>&1 || true)
    v=$(echo "$out" | grep -c "is private to its file" || true)
    if [ "$v" -ne 0 ] || ! echo "$out" | grep -q '^ok:'; then
        echo "  4. $ex under the flip: FAIL — $v violation(s)"; echo "$out" | grep -E '^error' | head -5 | sed 's/^/       /'; fail=1
    else
        echo "  4. $ex under the flip: ok"
    fi
done

if [ "$fail" -ne 0 ]; then
    echo "=== public-surface gate: FAIL ==="; exit 1
fi
echo "=== public-surface gate: ok — the public surface is complete and exact under a full private flip ==="
