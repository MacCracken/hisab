#!/usr/bin/env bash
# Verify every hand-encoded IEEE-754 hex constant in src/ against the value
# stated in its own comment.
#
# WHY THIS EXISTS
# ---------------
# hisab encodes f64 constants as raw i64 bit patterns (`var X = 0x3FF0...;`)
# because Cyrius has no float literals. The value is therefore unreadable at
# the point of use, and the adjacent comment is the only statement of intent.
# The 2026-08-03 audit found SEVEN constant tables where the bit pattern did
# not encode the commented value -- including four published numerical-method
# tableaux (Dormand-Prince, BDF-4, Yoshida-4, Gauss-Legendre-5), each of which
# was consequently falsified by its own mathematical invariant. DOPRI45 was not
# a consistent integrator at any order.
#
# The defect class is mechanical (hand-typing 16 hex digits), so the guard is
# mechanical too. Every constant is decoded and compared to its comment; any
# disagreement beyond the precision the comment itself claims is an error.
#
# COMMENT GRAMMAR ACCEPTED
#   # 0.2                          plain decimal
#   # 4/3                          exact rational
#   # 44/45 = 0.97777...           exact rational + truncated decimal
#   # 1/(2 - 2^(1/3)) = 1.3512...  expression + truncated decimal
#   # 2^-32                        expression
#   # 1e-12 as f64 bits            decimal + trailing prose
#   # ... nan / inf / sentinel     skipped (see SKIP_RE)
#
# When both an exact form and a decimal appear, the EXACT form is authoritative
# and the decimal is additionally cross-checked for self-consistency.
#
# Usage:  ./scripts/check-constants.sh [--verbose]
# Exit:   0 = all constants verified; 1 = at least one mismatch
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

VERBOSE=0
[ "${1:-}" = "--verbose" ] && VERBOSE=1

python3 - "$VERBOSE" <<'PYEOF'
import glob, re, struct, sys
from fractions import Fraction

verbose = sys.argv[1] == "1"

# var NAME = 0xHEX;  # comment
# The hex class MUST allow `_` digit separators: Cyrius accepts
# `0x4008_0000_0000_0000`, and 37 of the 148 declarations in src/ are written
# that way. The original class here was [0-9A-Fa-f]{1,16}, which silently did
# not match them -- so the gate skipped a quarter of the constants while
# printing a confident "verified" count. Found by the 2.7.0 re-audit.
DECL = re.compile(r'^\s*var\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(0[xX][0-9A-Fa-f_]{1,25})\s*;\s*#\s*(.*?)\s*$')
# Constants whose comment is prose, not a value.
SKIP_RE = re.compile(r'\b(nan|inf|sentinel|mask|bits? pattern|magic|seed|hash)\b', re.I)

def as_double(h):
    return struct.unpack('>d', struct.pack('>Q', int(h.replace('_', ''), 16) & 0xFFFFFFFFFFFFFFFF))[0]

def ulp(x):
    if x == 0 or x != x: return 5e-324
    b = struct.unpack('>Q', struct.pack('>d', abs(x)))[0]
    return abs(struct.unpack('>d', struct.pack('>Q', b + 1))[0] - abs(x))

import math

# Symbolic names the comments are allowed to use.
SYMS = {'pi': math.pi, 'PI': math.pi, 'tau': math.tau,
        'sqrt': math.sqrt, 'cbrt': lambda v: v ** (1.0 / 3.0),
        'exp': math.exp, 'ln': math.log, 'log': math.log}
ALPHA = re.compile(r'[A-Za-z_]+')

def eval_exact(expr):
    """Evaluate an arithmetic comment expression. Returns float or None."""
    e = expr.strip()
    if not re.search(r'\d', e):
        return None
    # Every alphabetic token must be a name we know, else this is prose.
    for tok in ALPHA.findall(e):
        if tok in SYMS: continue
        if re.fullmatch(r'[eE]', tok) and re.search(r'\d[eE][-+]?\d', e): continue  # 1e-12
        return None
    if not re.fullmatch(r'[-+*/^().0-9a-zA-Z_\s]+', e):
        return None
    # ^ is exponentiation in the comments, not xor
    e = e.replace('^', '**')
    try:
        v = eval(e, {"__builtins__": {}}, dict(SYMS))
        return float(v) if isinstance(v, (int, float)) else None
    except Exception:
        return None

def has_unknown_symbol(text):
    """True if the text mixes a number with an alphabetic token we cannot evaluate,
    so extracting a bare decimal from it would be a partial (wrong) match."""
    for tok in ALPHA.findall(text):
        if tok in SYMS: return True
        if re.fullmatch(r'[eE]', tok): continue
        return True
    return False

def exact_fraction(expr):
    """If the comment is a pure integer ratio a/b, return it as a Fraction."""
    m = re.fullmatch(r'\s*([-+]?\d+)\s*/\s*(\d+)\s*', expr)
    if not m: return None
    try: return Fraction(int(m.group(1)), int(m.group(2)))
    except ZeroDivisionError: return None

DEC = re.compile(r'[-+]?(?:\d+\.\d*|\.\d+|\d+)(?:[eE][-+]?\d+)?')

def decimal_tolerance(tok, truncated):
    """Absolute tolerance implied by how many digits the comment actually shows."""
    t = tok.lower()
    if 'e' in t:
        mant, _, exp = t.partition('e')
        exp = int(exp)
    else:
        mant, exp = t, 0
    mant = mant.lstrip('-+')
    frac = len(mant.split('.')[1]) if '.' in mant else 0
    # last shown digit sits at 10^(exp - frac)
    step = 10.0 ** (exp - frac)
    return step if truncated else 0.5 * step

results, errors, skipped = [], [], []

for path in sorted(glob.glob('src/*.cyr')):
    for lineno, line in enumerate(open(path, encoding='utf-8', errors='replace'), 1):
        m = DECL.match(line)
        if not m: continue
        name, hexlit, comment = m.group(1), m.group(2), m.group(3)
        if len(hexlit.replace('_', '')) - 2 < 12:   # short masks/flags, not f64 payloads
            continue
        actual = as_double(hexlit)
        site = f"{path}:{lineno} {name}"

        # +/-Infinity is checkable exactly, so verify rather than skip. These are
        # real sentinel values used by the geometry and spatial modules and a
        # wrong one would silently break every min/max seed that starts from them.
        cl = comment.strip().lower().lstrip('+')
        if cl in ('infinity', 'inf', '-infinity', '-inf'):
            want = float('-inf') if cl.startswith('-') else float('inf')
            ok = (actual == want)
            results.append(ok)
            if not ok:
                errors.append((site, comment, actual, want, 'infinity literal', float('inf')))
            elif verbose:
                print(f"  ok   {site:52s} = {actual!r} (infinity literal)")
            continue

        if SKIP_RE.search(comment):
            skipped.append((path, lineno, name, comment, 'prose comment')); continue

        # Split "EXACT = DECIMAL" if present; prefer the exact form.
        head, sep, tail = comment.partition('=')
        exact_src = head.strip() if sep else comment.strip()
        dec_src = tail.strip() if sep else comment.strip()
        truncated = '...' in comment or '…' in comment
        exact_src = exact_src.rstrip('.').strip()
        dec_src_clean = dec_src.replace('...', '').replace('…', '').strip()

        expected, tol, basis = None, None, None

        fr = exact_fraction(exact_src)
        if fr is not None:
            expected, basis = float(fr), f"exact {exact_src}"
            tol = 2 * ulp(expected)
        else:
            ev = eval_exact(exact_src)
            if ev is not None and (sep or not re.fullmatch(DEC.pattern, exact_src)):
                expected, basis = ev, f"expr {exact_src}"
                # A comment expression is re-evaluated here in a different order
                # than whatever produced the shipped bits (e.g. `1.0/2.4` in
                # float vs the correctly-rounded 5/12), so allow a few ulp.
                # Real transcription errors in this repo are >= 9e-6 relative --
                # five orders of magnitude clear of this bound.
                tol = 8 * ulp(expected)

        if expected is None:
            # Falling back to "pull the first decimal out of the comment" is only
            # safe when the comment is not an unevaluated symbolic expression --
            # otherwise `# -2*pi` would silently compare against -2.
            src_for_decimal = dec_src_clean if sep else comment
            # Strip trailing/parenthetical prose: "# 1e-4 (Armijo constant)",
            # "# 1e-12 as f64 bits". What remains must be a bare number.
            probe = re.sub(r'\([^)]*\)', ' ', src_for_decimal)
            probe = re.sub(r'\b(as f64 bits?|approx|approximately|about|~)\b', ' ', probe, flags=re.I)
            if has_unknown_symbol(probe):
                skipped.append((path, lineno, name, comment,
                                'symbolic expression not evaluable -- verify by hand')); continue
            d = DEC.search(dec_src_clean) or DEC.search(comment)
            if not d:
                skipped.append((path, lineno, name, comment, 'no parseable value')); continue
            tok = d.group(0)
            try: expected = float(tok)
            except ValueError:
                skipped.append((path, lineno, name, comment, 'unparseable number')); continue
            basis = f"decimal {tok}"
            tol = max(decimal_tolerance(tok, truncated), 4 * ulp(expected))

        diff = abs(actual - expected)
        ok = diff <= tol
        results.append(ok)
        if not ok:
            rel = diff / abs(expected) if expected else float('inf')
            errors.append((site, comment, actual, expected, basis, rel))
        elif verbose:
            print(f"  ok   {site:52s} = {actual!r} ({basis})")

        # Cross-check: when the comment gives BOTH an exact form and a decimal,
        # the decimal must also agree with the exact value it claims to expand.
        if sep and basis and not basis.startswith('decimal'):
            d = DEC.search(dec_src_clean)
            if d:
                try: shown = float(d.group(0))
                except ValueError: shown = None
                if shown is not None:
                    ctol = max(decimal_tolerance(d.group(0), truncated), 4 * ulp(expected))
                    if abs(shown - expected) > ctol:
                        errors.append((site + " [comment self-inconsistent]", comment,
                                       shown, expected, basis,
                                       abs(shown - expected) / abs(expected) if expected else float('inf')))
                        results.append(False)

total = len(results)
bad = len(errors)
print(f"=== constant check: {total - bad}/{total} verified, {len(skipped)} skipped ===")

if skipped and verbose:
    print("\n-- skipped (no machine-checkable value in comment) --")
    for p, l, n, c, why in skipped:
        print(f"  {p}:{l} {n}: {why} -- # {c}")

if errors:
    print(f"\n!! {bad} CONSTANT(S) DO NOT MATCH THEIR COMMENT\n")
    for site, comment, actual, expected, basis, rel in errors:
        print(f"  {site}")
        print(f"      comment : # {comment}")
        print(f"      basis   : {basis}")
        print(f"      encoded : {actual!r}")
        print(f"      expected: {expected!r}")
        print(f"      rel err : {rel:.3e}")
        try:
            print(f"      correct : 0x{struct.unpack('>Q', struct.pack('>d', expected))[0]:016X}")
        except Exception:
            pass
        print()
    print("A constant that disagrees with its own comment is a defect: the comment states")
    print("the intent and the bit pattern is what ships. Re-encode from the exact value.")
    sys.exit(1)

print("All hand-encoded f64 constants match their documented values.")
PYEOF
