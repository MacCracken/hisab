#!/usr/bin/env bash
# check-result-migration.sh — guard the ONE failure mode the compiler cannot see
# during the 3.0.0 integer-code -> Result<T,E> migration.
#
# ⛔ WHY THIS EXISTS. `lib/result.cyr`'s header says the value form's diagnostic
# "is the migration tool: every stale site fails at its own line instead of
# miscompiling." That is true for BINDING sites (`var r = f();` is a hard error)
# and FALSE for ARGUMENT sites, which is where this library's tests live.
#
# A Result in argument position does not error: `rdx` never reaches a parameter,
# so the callee silently receives the TAG. Measured on cycc 6.6.2:
#     Ok  tag = 0        Err tag = 1        HSB_ERR_NONE = 0
# so after migrating `f`, an existing `assert_eq(f(...), HSB_ERR_NONE, "...")`
# becomes `assert_eq(0, 0)` and KEEPS PASSING WHILE TESTING NOTHING, while
# `assert_eq(f(...), <any other code>)` becomes `assert_eq(1, -N)` and fails
# loudly. The dangerous half is silent and it is the success path.
#
# Measured before the migration began: 98 test lines compare a fallible call
# against HSB_ERR_NONE, 100 against another code, 233 span lines.
#
# USAGE
#   scripts/check-result-migration.sh            # check every migrated function
#   scripts/check-result-migration.sh --selftest # prove the detector fires
#
# A function counts as MIGRATED when its body contains `return Ok(` or
# `return Err(`. For each, every call site outside its own definition must bind
# both halves (`var t, v = f(...)`) or propagate (`f(...)?`) — never sit in an
# argument position.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

python3 - "${1:-}" <<'PY'
import re, sys, glob, os

selftest = (len(sys.argv) > 1 and sys.argv[1] == "--selftest")

def migrated_fns(extra_src=None):
    fns = {}
    files = sorted(glob.glob("src/*.cyr"))
    for f in files:
        cur = None
        body = {}
        for l in open(f, errors="replace"):
            m = re.match(r'^(?:public )?fn ([A-Za-z_][A-Za-z_0-9]*)\(', l)
            if m:
                cur = m.group(1)
                body[cur] = []
            if cur:
                body[cur].append(l)
        for name, lines in body.items():
            txt = "".join(lines)
            if re.search(r'return\s+(Ok|Err)\s*\(', txt):
                fns[name] = f
    if extra_src:
        fns.update(extra_src)
    return fns

def scan(fns):
    """Return a list of (file, lineno, fnname, kind, text) offences."""
    bad = []
    if not fns:
        return bad
    alt = "|".join(sorted(map(re.escape, fns), key=len, reverse=True))
    call = re.compile(r'(?<![A-Za-z_0-9])(' + alt + r')\s*\(')
    for f in sorted(glob.glob("src/*.cyr") + glob.glob("tests/*") + glob.glob("examples/*.cyr")):
        if os.path.isdir(f):
            continue
        for i, l in enumerate(open(f, errors="replace"), 1):
            # ⚠ Skip comments. Without this the gate reports every prose mention
            # of a migrated function — 12 of the first 17 hits were comment lines,
            # which on a 500-site migration is enough noise to make the gate get
            # ignored, and a gate people learn to ignore is not a gate.
            if l.lstrip().startswith("#"):
                continue
            # ⚠ Blank out string literals before matching. Assertion MESSAGES
            # routinely name the function under test — "num_dct(n=0) still
            # rejects" — and matching inside them produced 12 false positives out
            # of 13 hits. A gate whose output is mostly noise gets skimmed, and a
            # skimmed gate is the one that misses the real entry.
            masked = re.sub(r'"(?:[^"\\]|\\.)*"', lambda mm: '"' + " " * (len(mm.group(0)) - 2) + '"', l)
            for m in call.finditer(masked):
                name = m.group(1)
                # its own definition is not a call site
                if re.match(r'^(?:public )?fn ' + re.escape(name) + r'\(', l):
                    continue
                before = masked[:m.start()].rstrip()
                # Accept: `var t, v = f(` / `... = f(` with a two-name bind, or `f(...)?`
                if re.search(r'var\s+[A-Za-z_][A-Za-z_0-9]*\s*,\s*[A-Za-z_][A-Za-z_0-9]*\s*=\s*$', before):
                    continue
                rest = masked[m.end():]
                # crude but adequate: a `?` right after the closing paren on this line
                if re.search(r'\)\s*\?', rest):
                    continue
                # ⭐ TAIL DELEGATION is correct and must not be flagged:
                # `return f(...);` forwards BOTH halves of the pair as this
                # function's own return value. Verified on cycc 6.6.2 against a
                # Result-returning callee, Ok and Err both propagating intact.
                # Flagging it would have made the gate cry wolf on 6 correct
                # sites, which is how a gate stops being read.
                if re.search(r'return\s*$', before):
                    continue
                kind = "argument-position" if before.endswith(("(", ",")) else "single-bind-or-bare"
                bad.append((f, i, name, kind, l.strip()[:100]))
    return bad

if selftest:
    # Plant a synthetic migrated fn and a synthetic offence; both must be seen.
    fake = {"__probe_fn__": "src/__probe__.cyr"}
    os.makedirs("tests", exist_ok=True)
    probe = "tests/__result_probe__.tcyr"
    with open(probe, "w") as fh:
        fh.write('assert_eq(__probe_fn__(1), HSB_ERR_NONE, "planted offence");\n')
        fh.write('var t, v = __probe_fn__(2);\n')
    try:
        found = scan(fake)
        offences = [b for b in found if b[0] == probe]
        ok = len(offences) == 1 and offences[0][3] == "argument-position"
        print("  selftest: planted 1 argument-position offence and 1 correct bind")
        print("  selftest: detector reported %d offence(s) in the probe" % len(offences))
        for o in offences:
            print("    %s:%d  %s  [%s]" % (o[0], o[1], o[2], o[3]))
        print("  selftest: %s" % ("PASS" if ok else "FAIL"))
        sys.exit(0 if ok else 1)
    finally:
        os.remove(probe)

fns = migrated_fns()
if not fns:
    print("=== result migration: 0 functions migrated yet, nothing to check ===")
    print("A function counts as migrated once its body returns Ok(...) or Err(...).")
    sys.exit(0)

bad = scan(fns)
print("=== result migration: %d migrated function(s), %d bad call site(s) ===" % (len(fns), len(bad)))
if bad:
    print()
    print("!! A Result in ARGUMENT position silently degrades to its TAG.")
    print("   Ok tag = 0 and HSB_ERR_NONE = 0, so `assert_eq(f(..), HSB_ERR_NONE)`")
    print("   KEEPS PASSING while testing nothing. Bind both halves instead:")
    print("       var tag, val = f(...);")
    print()
    for f, i, n, k, t in bad[:40]:
        print("  %s:%d  [%s]  %s" % (f, i, k, n))
        print("      %s" % t)
    if len(bad) > 40:
        print("  ... and %d more" % (len(bad) - 40))
    sys.exit(1)
print("Every call site of a migrated function binds both halves or propagates.")
PY
