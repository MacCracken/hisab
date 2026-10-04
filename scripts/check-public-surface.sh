#!/usr/bin/env bash
# check-public-surface.sh — prove that hisab's `public` annotations describe a
# COMPLETE and EXACT surface, by flipping every module `private` in a scratch
# copy and compiling against it the way the suites and a consumer would.
#
# Added 3.1.0 (the `pub fn` half of the public/private surface). In 3.1.0 no
# module was `private`, so `public` was a documented no-op — the suites compiled
# byte-identical with and without it. Since 3.3.0 the shipped bundle is private
# (src/visibility.cyr, claim 5), so `public` is the boundary a consumer sees. The
# 35 modules themselves are still unflipped, and the suites include them one file
# at a time, so for the suites it still changes nothing. That is exactly why a
# gate is needed: nothing in an ordinary build can tell a complete surface from
# an incomplete one. A new cross-module reference to an unannotated helper
# compiles in the suites, and in the private bundle too, because the bundle is
# one file (claim 1). This gate's naming scan and per-module flip are what
# refuse it; nothing else in the tree flips the modules.
#
# Six claims, each fail-closed:
#   0. NAMING COMPLETENESS — every top-level fn / struct / enum / var in the
#      real tree whose name does not start with `_` carries `public`, and no `_`
#      name does (3.3.0: a `public _helper` reached from another module passed
#      every claim). The convention IS the surface; this is the claim a dropped
#      marker violates. src/visibility.cyr may declare nothing at all.
#      And every MEMBER of a non-public enum starts with `_`: in Cyrius "enum
#      constants and type names carry no visibility; they are always public"
#      (cyrius-guide, Visibility), so `private` cannot hide them and the name is
#      the only signal left. Found in 3.3.0, when the tree's first non-public enum
#      (`_RenderLayout`) leaked `FLOAT_RENDER_BUF` through claim 3.
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
#      every probe. (Enum members are not probed here: the language makes every
#      one reachable, which is why claim 0 checks their names instead.)
#      KNOWN_LEAKS (below) is the allowlist for pinned compiler defects; it is
#      EMPTY since the 6.6.4 pin (3.1.1) and any entry added to it FAILS this
#      gate the day upstream fixes the defect it names.
#   4. The real consumers in examples/*.cyr compile clean under the flip.
#   5. THE SHIPPED BUNDLE IS PRIVATE (3.3.0). `src/visibility.cyr` is the first [lib]
#      module and holds one `private` line, so the committed dist/hisab.cyr carries
#      the marker ahead of every declaration, and a consumer reading a non-public
#      global from THAT file -- not the scratch flip -- is refused. And no
#      non-public declaration directly follows a `public enum`: cycc 6.6.2/6.6.3
#      export it (3.3.0 shipped `_SYM_EPS` in that slot, readable under 6.6.3).
#      The modules themselves stay unflipped so the suites keep white-box access;
#      claims 1-4 flip them in a scratch copy to prove the boundaries anyway.
#   All claims read declarations through ONE scanner (scan.py, written below)
#   that strips comments and strings and tracks brace depth, and the module list
#   is every .cyr path in [lib], cross-checked against the bundle's module count.
#
# ⛔ WHY THE PROBES ARE CALLS AND NOT `&name`: on cycc 6.6.2 and 6.6.3 a
# file-private fn was reachable from another file through address-of — `&_helper`
# compiled and the pointer ran through callptr/fncallN — while a direct call was
# refused, so `&name` proved nothing in either direction. Filed upstream, now at
# cyrius/docs/development/issues/archived/2026-09-13-hisab-private-fn-reachable-via-address-of.md,
# and FIXED in 6.6.4 (eleven resolution paths now check). hisab took the fix in
# 3.1.1 and has no issue file of its own for it: CHANGELOG.md [3.1.1] and the
# 6.6.4 entry of docs/development/dependency-watch.md record it. The probes STAY
# calls: a call is the shape a consumer writes, and a gate that only worked from
# 6.6.4 up would be blind on every pin below it.
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
# Mutation-proven at 3.3.0, each in a scratch copy of the repo:
#   - `src/visibility.cyr` without its `private` line, bundle regenerated
#       -> the bundle-marker count above fails (35 lines for 35 + 1);
#   - `src/visibility.cyr` moved second in [lib], bundle regenerated
#       -> claim 5 fails: marker at line 131, first declaration at 47;
#   - `enum _ProbeLeak { PROBE_LEAK_MEMBER = 7; }` appended to calc.cyr
#       -> claim 0 fails on the member.
# ⛔ AND AN ADVERSARIAL REVIEW THEN PASSED THE WHOLE GATE SEVEN WAYS, each a
# non-`_` name readable from the private bundle with every claim green. Re-run
# against the scanner, all seven now fail claim 0, with the reason named:
#   - `enum _ProbeLeak { PROBE_LEAK_MEMBER; }` (no `= value`);
#   - `enum _ProbeLeak {   # sizes {bytes}` with the member on the next line;
#   - `var _probe_pad = 0; enum _ProbeLeak { ... }` (not at column 0);
#   - a [lib] entry `"src/ext/leakmod.cyr",` and one `"src/leakmod.cyr",  # c`;
#   - a fn and an enum appended to src/visibility.cyr;
#   - `public fn _sym_is_zero` reached from symbolic_ext.
#   And `var _adj_probe` inserted after `public enum ExprTag` fails claim 5.
#
# Nothing under the repo is modified: everything happens in a mktemp copy.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

T="$(mktemp -d)"
trap 'rm -rf "$T"' EXIT
tar --exclude=.git --exclude=build -cf - . | tar -C "$T" -xf -
cd "$T"

# ── the shared scanner ───────────────────────────────────────────────────────
# ⛔ 3.3.0 — EVERY CLAIM USED TO READ DECLARATIONS WITH ITS OWN LINE REGEX, and an
# adversarial review planted shapes that passed all six claims: an enum member with
# no `= value` (`enum _X { A; }`, the first form cyrius-guide shows), a `}` inside a
# comment in an enum body, a declaration that does not start its line
# (`var _p = 0; enum _X { ... }`), a [lib] entry in a subdirectory or with a
# trailing comment, and a declaration added to src/visibility.cyr. Each left a
# non-`_` enum member readable from the private bundle with the gate green. One
# scanner now strips comments and string literals (strings hold braces: LaTeX),
# tracks brace depth, and reports every depth-0 declaration wherever it sits.
# [measured: scratchpad mutants rv_noeq / rv_brace / rv_sameline / rv_msub / rv_mcom / rv_mvis, 2026-09-30]
cat > scan.py <<'SCANEOF'
import re, sys, json

# The attribute tokens the cyrius guide lists ("A `#` is not always a comment").
ATTR = re.compile(r'#(?:inline|naked|pure|io|alloc|must_use|regalloc|deprecated|assert|pe_import)(?![A-Za-z0-9_])|#derive\([^)\n]*\)')

def strip(src):
    """Blank out comments and string-literal contents, keeping every newline."""
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if c == '"':
            out.append('"'); i += 1
            while i < n and src[i] != '"':
                if src[i] == '\\' and i + 1 < n:
                    out.append('  ' if src[i + 1] != '\n' else ' \n'); i += 2; continue
                out.append('\n' if src[i] == '\n' else ' '); i += 1
            if i < n: out.append('"'); i += 1
        elif c == '#':
            # ⛔ 3.3.1 — AN ATTRIBUTE IS NOT A COMMENT. `#inline fn f(j) { ... }` is one
            # declaration: cycc reads `#inline` as a token and keeps lexing the line.
            # Blanking to end of line made geo_diff's 18 `#inline` _GeoJet accessors
            # invisible, and claim 3's non-public probe count fell 511 -> 493 with the
            # gate green. Blank only the attribute token; the rest of the line is code.
            # [measured: probe counts before/after this fix, 2026-09-30]
            m = ATTR.match(src, i)
            if m:
                out.append(' ' * (m.end() - i)); i = m.end()
                continue
            while i < n and src[i] != '\n':
                out.append(' '); i += 1
        else:
            out.append(c); i += 1
    return ''.join(out)

DECL = re.compile(r'(?<![A-Za-z0-9_])(public\s+)?(fn|var|struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)')

def scan(path):
    """Every depth-0 declaration in `path`, in order."""
    text = strip(open(path, encoding='utf-8', errors='replace').read())
    decls, depth, i, n = [], 0, 0, len(text)
    while i < n:
        c = text[i]
        if c == '{': depth += 1; i += 1; continue
        if c == '}': depth -= 1; i += 1; continue
        if depth == 0:
            m = DECL.match(text, i)
            if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == '_')):
                pub, kind, name = bool(m.group(1)), m.group(2), m.group(3)
                line = text.count('\n', 0, i) + 1
                d = {'file': path, 'line': line, 'kind': kind, 'name': name, 'public': pub}
                j = m.end()
                if kind == 'fn':
                    lp = text.index('(', j); rp = text.index(')', lp)
                    params = text[lp + 1:rp].strip()
                    d['arity'] = 0 if params == '' else len([p for p in params.split(',') if p.strip()])
                elif kind in ('struct', 'enum'):
                    lb = text.index('{', j); rb = text.index('}', lb)
                    parts = [x.strip() for x in text[lb + 1:rb].split(';')]
                    names = []
                    for x in parts:
                        mm = re.match(r'([A-Za-z_][A-Za-z0-9_]*)', x)
                        if mm: names.append(mm.group(1))
                    d['members'] = names
                    i = rb + 1; decls.append(d); continue
                decls.append(d); i = m.end(); continue
        i += 1
    return decls

def lib_modules(manifest):
    """Every .cyr path in the [lib] section, however it is written."""
    sec = re.split(r'(?m)^\[lib\]\s*$', open(manifest).read(), maxsplit=1)
    if len(sec) < 2: return []
    body = re.split(r'(?m)^\[', sec[1], maxsplit=1)[0]
    body = '\n'.join(l.split('#', 1)[0] for l in body.split('\n'))
    return re.findall(r'"([^"]+\.cyr)"', body)

if __name__ == '__main__':
    mode = sys.argv[1]
    if mode == 'lib':
        print('\n'.join(lib_modules(sys.argv[2])))
    elif mode == 'decls':
        for p in sys.argv[2:]:
            for d in scan(p): print(json.dumps(d))
SCANEOF

# Every [lib] module, read the way distlib reads them rather than with a line regex.
# src/visibility.cyr is the bundle's `private` marker, not a math module: it is
# checked by claim 5 and is not flipped (it would only carry a second marker).
LIBMODS=$(python3 scan.py lib cyrius.cyml)
MODULES=$(echo "$LIBMODS" | grep -v '^src/visibility\.cyr$')
n_lib=$(echo "$LIBMODS" | grep -c . || true)
echo "$LIBMODS" | grep -qx 'src/visibility.cyr' || { echo "FAIL: src/visibility.cyr is not a [lib] module"; exit 1; }
n_mod=0
for m in $MODULES; do
    [ -f "$m" ] || { echo "FAIL: [lib] names $m, which does not exist"; exit 1; }
    { echo "private"; echo; cat "$m"; } > "$m.tmp" && mv "$m.tmp" "$m"
    n_mod=$((n_mod + 1))
done
[ "$n_mod" -ge 30 ] || { echo "FAIL: only $n_mod [lib] modules found — manifest parse broke"; exit 1; }

cyrius distlib >/dev/null 2>&1 || { echo "FAIL: cyrius distlib failed on the flipped copy"; exit 1; }
n_priv=$(grep -c '^private$' dist/hisab.cyr || true)
[ "$n_priv" -eq $((n_mod + 1)) ] || { echo "FAIL: bundle carries $n_priv 'private' lines for $n_mod modules + visibility.cyr — src/visibility.cyr lost its marker, or distlib did not pass them through"; exit 1; }
# The bundle must hold exactly the modules checked here, so a module the scanner
# could not see cannot ride into the bundle unexamined.
n_hdr=$(grep -cE '^# --- [^ ]+\.cyr ---$' dist/hisab.cyr || true)
[ "$n_hdr" -eq "$n_lib" ] || { echo "FAIL: the bundle holds $n_hdr modules but [lib] parsed to $n_lib — a module would go unchecked"; exit 1; }

fail=0
echo "=== public-surface gate: $n_mod modules flipped private in $T ==="

# ── 0. naming completeness (real tree, before the flip touched anything) ─────
# Four rules over every depth-0 declaration of every [lib] module:
#   a non-`_` name carries `public`;  a `_` name never does (3.3.0 — a
#   `public _helper` reached from another module passed every claim, the exact
#   3.x pattern); every member of a NON-public enum is `_`-named (enum constants
#   carry no visibility in Cyrius, so the name is the only signal left); and
#   src/visibility.cyr holds no declarations at all.
unmarked=$(python3 - "$REPO_ROOT" $LIBMODS <<'PYEOF2'
import sys, os
sys.path.insert(0, os.getcwd())
from scan import scan
root, mods = sys.argv[1], sys.argv[2:]
for m in mods:
    for d in scan(os.path.join(root, m)):
        where = f"{m}:{d['line']}"
        if m == 'src/visibility.cyr':
            print(f"{where}: {d['kind']} {d['name']} declared in the marker module"); continue
        under = d['name'].startswith('_')
        if not under and not d['public']:
            print(f"{where}: {d['kind']} {d['name']} has no `public`")
        if under and d['public']:
            print(f"{where}: {d['kind']} {d['name']} is `_`-named AND `public` — promote it under a real name")
        if d['kind'] == 'enum' and not d['public']:
            for mem in d['members']:
                if not mem.startswith('_'):
                    print(f"{where}: member {mem} of non-public enum {d['name']} (enum constants are always public)")
PYEOF2
)
if [ -n "$unmarked" ]; then
    echo "  0. naming completeness: FAIL"
    echo "$unmarked" | head -20 | sed 's/^/       /'
    fail=1
else
    echo "  0. naming completeness: ok (every non-underscore declaration is public, no underscore one is, non-public enum members are underscored)"
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
import sys, os
T = sys.argv[1]; mods = sys.argv[2:]
sys.path.insert(0, T)
from scan import scan
pub_calls, pub_reads, priv_calls, priv_reads = [], [], [], []
n_pub_fn = n_pub_struct = n_pub_var = n_pub_enum = 0
for m in mods:
    for d in scan(os.path.join(T, m)):
        pub, kind, name = d['public'], d['kind'], d['name']
        if kind == 'fn':
            (pub_calls if pub else priv_calls).append(f"{name}({', '.join(['0'] * d['arity'])});")
            if pub: n_pub_fn += 1
        elif kind == 'var':
            (pub_reads if pub else priv_reads).append(f"var r_{name} = {name};")
            if pub: n_pub_var += 1
        elif kind == 'struct':
            for f in d['members']:
                (pub_calls if pub else priv_calls).append(f"{name}_{f}(0);")
                (pub_calls if pub else priv_calls).append(f"{name}_set_{f}(0, 0);")
            if pub: n_pub_struct += 1
        elif kind == 'enum':
            # Members of a NON-public enum are not probed: enum constants carry no
            # visibility in Cyrius, so a refusal can never be expected (claim 0
            # checks that they are `_`-named instead).
            if pub:
                for mem in d['members']:
                    pub_reads.append(f"var r_{mem} = {mem};")
                n_pub_enum += 1
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
# ⛔ 3.3.1 — THESE FLOORS WERE 500 AND 300, so the 18 probes lost to `#inline` (see
# strip()) could not trip them: 511 -> 493 passed green. They are now the live
# populations. Raise them when the surface grows; lower them only with a reason in
# the commit, the same discipline as check-constants.sh's POPULATION_FLOOR.
PUB_PROBE_FLOOR=846    # re-derived 2026-09-30, 3.3.1; 846 in 3.3.4: public f64_tan retired (ganita 1.2.11 has one)
PRIV_PROBE_FLOOR=520   # re-derived 2026-09-30, 3.3.2 - 19 dead non-public declarations removed (D063, D075, D095)
                       # 492 -> 520 in 3.3.3: the release's new private helpers and constants (re-derived 2026-10-01)
[ "$n_pub_probes" -ge "$PUB_PROBE_FLOOR" ] || { echo "FAIL: only $n_pub_probes public probes generated, floor $PUB_PROBE_FLOOR — a declaration stopped being seen, or the generator is broken"; exit 1; }
[ "$n_priv_probes" -ge "$PRIV_PROBE_FLOOR" ] || { echo "FAIL: only $n_priv_probes private probes generated, floor $PRIV_PROBE_FLOOR — a declaration stopped being seen, or the generator is broken"; exit 1; }

out=$(cyrius check --with-deps consumer_public.cyr 2>&1 || true)
v=$(echo "$out" | grep -c "is private to its file" || true)
e=$(echo "$out" | grep -c '^error' || true)
if [ "$v" -ne 0 ] || [ "$e" -ne 0 ]; then
    echo "  2. external reachability: FAIL — $v of $n_pub_probes public probes refused, $e error line(s)"
    echo "$out" | grep -E '^error' | head -20 | sed 's/^/       /'
    fail=1
else
    echo "  2. external reachability: ok ($n_pub_probes probes over $((n_pub_fn + n_pub_struct + n_pub_var + n_pub_enum)) public declarations: $n_pub_fn fn, $n_pub_struct struct (accessors probed), $n_pub_var var, $n_pub_enum enum (members probed) — all reachable from a consumer)"
fi

# ⛔ KNOWN LEAKS — pinned COMPILER defects, each keyed to its upstream filing. The
# exactness claim expects exactly these to be accepted; if one is REFUSED the
# upstream fix has landed and this gate FAILS until the entry is removed, so a
# pinned defect inverts into a tripwire for its own repair (2.20.0's rule).
#   EMPTY since 3.1.1 (cycc 6.6.4). The one entry it ever held:
#   _ad_pow — the declaration right after `public enum AdPowLimit` in autodiff.cyr.
#     A `public enum` leaked its marker onto the next top-level declaration
#     (cycc 6.6.2 and 6.6.3). Filed upstream, now at
#     cyrius/docs/development/issues/archived/2026-09-13-hisab-public-enum-leaks-onto-next-declaration.md,
#     fixed in 6.6.4 (`public` arms the marker only for a token that can carry
#     it), and this gate reported it REFUSED on the pin bump — the inversion
#     working as designed. `_SYM_EPS` (symbolic.cyr, after `public enum ExprTag`)
#     was the masked second instance while it was itself public. 3.3.0 made it
#     private and so, under 6.6.3, LEAKED it from the shipped bundle -- invisible
#     here because this gate runs the repo's pin. ExprTag now sits above a public
#     fn, and claim 5 fails any bundle where a non-public declaration follows a
#     `public enum`, which closes the class on 6.6.2/6.6.3 without needing them.
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

# ── 5. the SHIPPED bundle is private ─────────────────────────────────────────
# Three checks on the committed dist/hisab.cyr, not on the scratch flip:
#   the `private` marker precedes the first declaration;
#   a consumer reading a non-public global from it is refused;
#   no non-public declaration directly follows a `public enum` -- cycc 6.6.2 and
#   6.6.3 hand a `public enum`'s marker to the next declaration, and consumers
#   compile the bundle under their own pins (six on 6.6.3 as of 3.3.0).
shipped="$REPO_ROOT/dist/hisab.cyr"
# `grep -m1`, never `grep | head -1`: under `set -o pipefail` head closing early
# SIGPIPEs grep and the whole gate exits 141 with no message (it did, once).
first_priv=$(grep -m1 -n '^private$' "$shipped" | cut -d: -f1 || true)
read -r first_decl enum_leaks < <(python3 - "$shipped" <<'PYEOF5'
import sys, os
sys.path.insert(0, os.getcwd())
from scan import scan
ds = scan(sys.argv[1])
first = ds[0]['line'] if ds else 0
bad = [f"{a['name']}->{b['name']}@{b['line']}" for a, b in zip(ds, ds[1:])
       if a['kind'] == 'enum' and a['public'] and not b['public']]
print(first, ','.join(bad) or '-')
PYEOF5
)
probe_var=$(grep -m1 '^var r_' consumer_private.cyr | sed -E 's/^var r_[A-Za-z0-9_]+ = ([A-Za-z0-9_]+);/\1/')
cp "$shipped" shipped_hisab.cyr
printf 'include "shipped_hisab.cyr"\nvar r_probe = %s;\nfn _shipped_probe() { return 0; }\n' "$probe_var" > shipped_probe.cyr
sout=$(cyrius check --with-deps shipped_probe.cyr 2>&1 || true)
if [ -z "$first_priv" ] || [ -z "$first_decl" ] || [ "$first_decl" = "0" ] || [ "$first_priv" -gt "$first_decl" ]; then
    echo "  5. shipped bundle private: FAIL — dist/hisab.cyr has no \`private\` line ahead of its first declaration (line ${first_priv:-none} vs ${first_decl:-none}); is src/visibility.cyr first in [lib]?"
    fail=1
elif ! echo "$sout" | grep -q "'$probe_var' is private"; then
    echo "  5. shipped bundle private: FAIL — a consumer read the non-public global \`$probe_var\` from the committed bundle"
    fail=1
elif [ "$enum_leaks" != "-" ]; then
    echo "  5. shipped bundle private: FAIL — a non-public declaration follows a \`public enum\` ($enum_leaks); cycc 6.6.2/6.6.3 export it"
    fail=1
else
    echo "  5. shipped bundle private: ok (marker at line $first_priv, first declaration at $first_decl; reading \`$probe_var\` from the committed bundle is refused; every \`public enum\` is followed by a public declaration)"
fi

if [ "$fail" -ne 0 ]; then
    echo "=== public-surface gate: FAIL ==="; exit 1
fi
echo "=== public-surface gate: ok — the public surface is complete and exact, and the shipped bundle enforces it ==="
