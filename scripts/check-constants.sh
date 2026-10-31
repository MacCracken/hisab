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
# mechanical too. Every constant is decoded and compared to its comment, read
# exactly (3.3.7): an exact form or an untruncated decimal must ROUND TO the
# encoded double, bit for bit; a `...` decimal can only bound it.
#
# COMMENT GRAMMAR ACCEPTED
#   # 0.2                          plain decimal: its nearest double, exactly
#   # 4/3                          exact rational: its correctly rounded double
#   # 44/45 = 0.97777...           exact rational + truncated decimal
#   # 1/(2 - 2^(1/3)) = 1.3512...  expression + truncated decimal
#   # sqrt(5/pi)/4 = 0.3153915...  expression (+ - * / ^ ( ), sqrt cbrt exp ln log, pi tau)
#   # 2^-32                        expression
#   # 1e-12 as f64 bits            decimal + trailing prose
#   # 0.67560359...                truncated decimal: within its last digit (cannot pin)
#   # ... nan / inf / sentinel     skipped (see SKIP_RE)
#
# When both an exact form and a decimal appear, the EXACT form is authoritative
# and the decimal is additionally cross-checked for self-consistency. A decimal
# that has to pin the last bit needs about 17 significant digits; give the closed
# form when there is one.
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
DECL = re.compile(r'^\s*(?:public\s+)?var\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(0[xX][0-9A-Fa-f_]{1,25})\s*;\s*#\s*(.*?)\s*$')
# ⛔ 2.20.0 -- THE REGEX ABOVE REQUIRES THE COMMENT TO TRAIL, AND ONE DECLARATION
# DOCUMENTED ITSELF ON THE LINE ABOVE INSTEAD. `src/calc_ext.cyr`'s F64_1E_NEG30
# matched neither the verified nor the skipped path: it was INVISIBLE, so the gate
# reported a confident "158/158" over a population of 159. And it was
# mis-transcribed -- 9.99999999979426e-31 against a documented 1e-30. That is the
# same shape as the 2026-08-04 finding recorded above (a regex that silently
# skipped a quarter of the constants while printing a confident count), which is
# why the fallback below exists rather than a one-line fix to that declaration.
# ⚠ The fallback is deliberately narrow: it applies ONLY when the declaration has
# no trailing comment AND the immediately preceding line is a comment.
DECL_NC = re.compile(r'^\s*(?:public\s+)?var\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(0[xX][0-9A-Fa-f_]{1,25})\s*;\s*$')
COMMENT_ONLY = re.compile(r'^\s*#\s*(.*?)\s*$')
# ⭐ 3.3.7 -- AND THE HEX MEMBERS OF A MULTI-LINE ENUM. An enum member is an immediate where
# a `var` is a load (CLAUDE.md, "Enums for constants"), and 3.3.7 moved geo.cyr's `_GEO_*`
# constants and calc_integral_gauss5's nodes and weights into enums. A member is read only
# between an `enum NAME {` line and its closing `}`: the same `NAME = 0xHEX;  # comment`
# shape is an ordinary assignment in a function body.
ENUM_OPEN = re.compile(r'^\s*(?:public\s+)?enum\s+[A-Za-z_][A-Za-z0-9_]*\s*\{\s*(?:#.*)?$')
ENUM_DECL = re.compile(r'^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(0[xX][0-9A-Fa-f_]{1,25})\s*;\s*#\s*(.*?)\s*$')
# Constants whose comment is prose, not a value.
SKIP_RE = re.compile(r'\b(nan|inf|sentinel|mask|bits? pattern|magic|seed|hash)\b', re.I)

def as_double(h):
    return struct.unpack('>d', struct.pack('>Q', int(h.replace('_', ''), 16) & 0xFFFFFFFFFFFFFFFF))[0]

import math
from decimal import Decimal, localcontext

# ⛔ 3.3.7 — THE COMMENTS WERE READ IN DOUBLES, AND A DECIMAL WITHIN 4 ULP PASSED. An
# expression comment was eval()'d in f64 (`1/(2 - 2^(1/3))` rounded at every step),
# a ratio got 2 ulp, and a plain decimal got max(half its last digit, 4 ulp) -- so
# `_SH_Y20`, whose comment carries twenty correctly rounded digits, passed with +1, +3
# and +4 ulp planted on it and failed only at +5. A comment that pins the last bit
# must be read as exactly as it is written:
#   * a decimal is decoded EXACTLY (`Fraction`), and an untruncated one must round
#     to the encoded double -- its digits identify the bits, so a comment showing too
#     few of them fails and is given more (or an exact form);
#   * an exact form (a ratio, or an expression in + - * / ^, sqrt, cbrt, exp, ln, pi)
#     is evaluated in exact rationals, and in 120-digit decimals where it is
#     irrational, and the encoded double must be its correctly rounded value;
#   * a `...` comment declares that it is truncated, so it can only bound the value:
#     the encoded double must lie within one unit of the last shown digit plus half
#     an ulp of it. It cannot pin the bit, and the gate does not pretend it does.
# [measured: scratch census (3.3.7) of src/ under the old and the new rules, and +1/+3/+4/+5 ulp
#  planted on _SH_Y20 against both]
_PREC = 120
ALPHA = re.compile(r'[A-Za-z_]+')
_FUNCS = ('sqrt', 'cbrt', 'exp', 'ln', 'log')
_CONSTS = ('pi', 'PI', 'tau')
_TOKEN = re.compile(r'\s*(?:(\d+\.?\d*(?:[eE][-+]?\d+)?|\.\d+(?:[eE][-+]?\d+)?)|([A-Za-z_]+)|([-+*/^()]))')

def _dec(v):
    return Decimal(v.numerator) / Decimal(v.denominator) if isinstance(v, Fraction) else v

def _pi_dec():
    # Machin's formula, pi = 16 atan(1/5) - 4 atan(1/239), at the working precision.
    def atan_inv(n):
        x = Decimal(1) / n
        x2, term, s, k = x * x, x, x, 1
        eps = Decimal(10) ** -(_PREC + 8)
        while abs(term) > eps:
            term = -term * x2
            k += 2
            s += term / k
        return s
    return 16 * atan_inv(5) - 4 * atan_inv(239)

def _arith(op, a, b):
    if isinstance(a, Fraction) and isinstance(b, Fraction):
        if op == '+': return a + b
        if op == '-': return a - b
        if op == '*': return a * b
        return a / b
    a, b = _dec(a), _dec(b)
    if op == '+': return a + b
    if op == '-': return a - b
    if op == '*': return a * b
    return a / b

def _power(a, b):
    if isinstance(b, Fraction) and b.denominator == 1:
        n = int(b)
        if abs(n) > 4096: raise ValueError('exponent out of range')
        return a ** n if isinstance(a, Fraction) else _dec(a) ** n
    a, b = _dec(a), _dec(b)
    if a <= 0: raise ValueError('non-integer power of a non-positive base')
    return (b * a.ln()).exp()

def _call(name, a):
    if name == 'sqrt':
        if isinstance(a, Fraction) and a >= 0:
            rn, rd = math.isqrt(a.numerator), math.isqrt(a.denominator)
            if rn * rn == a.numerator and rd * rd == a.denominator: return Fraction(rn, rd)
        a = _dec(a)
        if a < 0: raise ValueError('sqrt of a negative')
        return a.sqrt()
    if name == 'cbrt':
        a = _dec(a)
        if a == 0: return Fraction(0)
        r = (abs(a).ln() / 3).exp()
        return -r if a < 0 else r
    if name == 'exp':
        return _dec(a).exp()
    a = _dec(a)                       # ln, log: natural log, as math.log was
    if a <= 0: raise ValueError('log of a non-positive')
    return a.ln()

class _Parser:
    """expr := term (('+'|'-') term)*   term := unary (('*'|'/') unary)*
       unary := ('-'|'+') unary | power  power := atom ('^' unary)?
       atom := number | const | func '(' expr ')' | '(' expr ')'
    Python's precedence with ^ for **: -2^2 = -4, 2^-52 and 2^3^2 = 2^9."""
    def __init__(self, text):
        self.toks, pos = [], 0
        text = text.rstrip()
        while pos < len(text):
            m = _TOKEN.match(text, pos)
            if not m or m.end() == pos: raise ValueError('untokenizable')
            self.toks.append(('num', m.group(1)) if m.group(1) else
                             ('name', m.group(2)) if m.group(2) else ('op', m.group(3)))
            pos = m.end()
        self.i = 0
    def peek(self):
        return self.toks[self.i] if self.i < len(self.toks) else (None, None)
    def take(self, kind=None, val=None):
        k, v = self.peek()
        if k is None or (kind and k != kind) or (val and v != val): raise ValueError('syntax')
        self.i += 1
        return v
    def expr(self):
        v = self.term()
        while self.peek() in (('op', '+'), ('op', '-')):
            v = _arith(self.take(), v, self.term())
        return v
    def term(self):
        v = self.unary()
        while self.peek() in (('op', '*'), ('op', '/')):
            v = _arith(self.take(), v, self.unary())
        return v
    def unary(self):
        if self.peek() in (('op', '-'), ('op', '+')):
            sign = self.take()
            v = self.unary()
            return _arith('-', Fraction(0), v) if sign == '-' else v
        return self.power()
    def power(self):
        v = self.atom()
        if self.peek() == ('op', '^'):
            self.take()
            v = _power(v, self.unary())
        return v
    def atom(self):
        k, v = self.peek()
        if k == 'num':
            self.take(); return Fraction(v)
        if k == 'name' and v in _CONSTS:
            self.take(); p = _pi_dec(); return 2 * p if v == 'tau' else p
        if k == 'name' and v in _FUNCS:
            self.take(); self.take('op', '(')
            a = self.expr(); self.take('op', ')')
            return _call(v, a)
        if (k, v) == ('op', '('):
            self.take(); a = self.expr(); self.take('op', ')')
            return a
        raise ValueError('syntax')

def eval_exact(expr):
    """Evaluate an arithmetic comment expression EXACTLY: a Fraction, or a Fraction
    holding a 120-digit approximation where the value is irrational. None for prose."""
    e = expr.strip()
    if not re.search(r'\d', e):
        return None
    # Every alphabetic token must be a name we know, else this is prose.
    for tok in ALPHA.findall(e):
        if tok in _FUNCS or tok in _CONSTS: continue
        if re.fullmatch(r'[eE]', tok) and re.search(r'\d[eE][-+]?\d', e): continue  # 1e-12
        return None
    try:
        with localcontext() as ctx:
            ctx.prec = _PREC + 10
            p = _Parser(e)
            v = p.expr()
            if p.i != len(p.toks): return None
            return v if isinstance(v, Fraction) else Fraction(v)
    except (ValueError, ZeroDivisionError, ArithmeticError):
        return None

def _bits(x):
    return struct.unpack('>Q', struct.pack('>d', x))[0]

def round_exact(v):
    """The correctly rounded (nearest, ties-to-even) double of an exact value, or None
    past the range. int / int true division is correctly rounded in CPython."""
    try:
        return v.numerator / v.denominator
    except OverflowError:
        return None

def ulp_distance(a, b):
    """Signed count of doubles from a to b (both finite)."""
    def key(x):
        k = _bits(x)
        return -(k & 0x7FFFFFFFFFFFFFFF) if k >> 63 else k
    return key(b) - key(a)

# ⛔ 3.3.3 — AT ±DBL_MAX THE STEP ABOVE IS +Inf, SO THE TOLERANCE WAS +Inf AND
# EVERY VALUE PASSED. 3.3.3 added the first constants there (_IVL_F64_MAX and
# _IVL_F64_NEG_MAX in interval.cyr); with _IVL_F64_MAX planted as
# 0x3FF0_0000_0000_0000 (1.0) under its own "(2 - 2^-52) * 2^1023" comment, the
# gate still printed "155/155 verified". At the top of the range the only gap is the
# one BELOW. (3.3.7 compares exact forms bit for bit, so only a `...` decimal still
# reads a gap, here.) [measured: scratch copy of the 3.3.3 tree, 2026-10-01]
def half_gap(x):
    """Half the wider of the gaps from x to its finite neighbours, exactly."""
    b = _bits(abs(x))
    up = struct.unpack('>d', struct.pack('>Q', b + 1))[0]
    lo = struct.unpack('>d', struct.pack('>Q', b - 1))[0] if b > 0 else 0.0
    gaps = [Fraction(abs(x)) - Fraction(lo)]
    if up != float('inf'): gaps.append(Fraction(up) - Fraction(abs(x)))
    return max(gaps) / 2

def sig21(v):
    """21 significant digits of an exact value, for the report."""
    with localcontext() as ctx:
        ctx.prec = 21
        return str(Decimal(v.numerator) / Decimal(v.denominator))

def has_unknown_symbol(text):
    """True if the text mixes a number with an alphabetic token we cannot evaluate,
    so extracting a bare decimal from it would be a partial (wrong) match."""
    for tok in ALPHA.findall(text):
        if tok in _FUNCS or tok in _CONSTS: return True
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

def decimal_step(tok):
    """The value of one unit in the last digit the token shows, exactly."""
    t = tok.lower()
    if 'e' in t:
        mant, _, exp = t.partition('e')
        exp = int(exp)
    else:
        mant, exp = t, 0
    mant = mant.lstrip('-+')
    frac = len(mant.split('.')[1]) if '.' in mant else 0
    # last shown digit sits at 10^(exp - frac)
    return Fraction(10) ** (exp - frac)

results, errors, skipped = [], [], []

for path in sorted(glob.glob('src/*.cyr')):
    prev_comment = None
    in_enum = False
    for lineno, line in enumerate(open(path, encoding='utf-8', errors='replace'), 1):
        if ENUM_OPEN.match(line):
            in_enum = True
            continue
        if in_enum and re.match(r'^\s*\}', line):
            in_enum = False
            continue
        m = DECL.match(line)
        if not m and in_enum:
            m = ENUM_DECL.match(line)
        if not m:
            # Fallback: a declaration documented on the line ABOVE. See DECL_NC.
            mn = DECL_NC.match(line)
            if mn and prev_comment:
                name, hexlit, comment = mn.group(1), mn.group(2), prev_comment
                m = None
            else:
                c = COMMENT_ONLY.match(line)
                prev_comment = c.group(1) if c else None
                continue
        else:
            name, hexlit, comment = m.group(1), m.group(2), m.group(3)
        prev_comment = None
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
                errors.append((site, comment, actual, want, 'infinity literal', 'exact', None))
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

        expected, basis, value, rule = None, None, None, None

        fr = exact_fraction(exact_src)
        if fr is not None:
            value, basis = fr, f"exact {exact_src}"
        else:
            ev = eval_exact(exact_src)
            if ev is not None and (sep or not re.fullmatch(DEC.pattern, exact_src)):
                value, basis = ev, f"expr {exact_src}"
        if value is not None:
            # ⛔ 3.3.1 -- THIS WAS 8 ULP, on the theory that a comment expression
            # re-evaluated in a different order (`1.0/2.4` in float vs the
            # correctly-rounded 5/12) lands a few ulp away. 8 of the 9 expression
            # constants sat at exactly 0 ulp; the ninth, color.cyr's _SRGB_INV_2_4,
            # was a 5-ulp transcription error the slack had been passing. 3.3.1 cut it
            # to 1 ulp, and 3.3.7 to none: the value is now exact, so the encoded
            # double must be its correctly rounded value.
            # [measured: per-constant ulp distance over src/, 2026-09-30 and 3.3.7]
            expected, rule = round_exact(value), 'correctly rounded'
            ok = expected is not None and _bits(actual) == _bits(expected)
        else:
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
            try: value = Fraction(tok)
            except ValueError:
                skipped.append((path, lineno, name, comment, 'unparseable number')); continue
            basis, expected = f"decimal {tok}", round_exact(value)
            # ⛔ 3.3.0 — A ONE- OR TWO-DIGIT COMMENT NAMES A ROUND VALUE, NOT A ROUNDED
            # DISPLAY: `_SYM_EPS` was 2^-50 = 8.88e-16 under `# 1e-15` and passed. 3.3.0
            # held short tokens to 4 ulp; 3.3.7 holds every untruncated token to its
            # nearest double (see the ⛔ 3.3.7 paragraph at eval_exact).
            if truncated:
                rule = 'inside its truncation (cannot pin the bit)'
                ok = abs(Fraction(actual) - value) <= decimal_step(tok) + half_gap(actual)
            else:
                rule = 'decoded exactly'
                ok = expected is not None and _bits(actual) == _bits(expected)

        results.append(ok)
        if not ok:
            errors.append((site, comment, actual, expected, basis, rule, value))
        elif verbose:
            print(f"  ok   {site:52s} = {actual!r} ({basis}; {rule})")

        # Cross-check: when the comment gives BOTH an exact form and a decimal,
        # the decimal must also agree with the exact value it claims to expand:
        # within one unit of its last digit when it says `...`, else a correctly
        # rounded display of the value, or the digits of the encoded double itself.
        if sep and basis and not basis.startswith('decimal'):
            d = DEC.search(dec_src_clean)
            if d:
                shown, step = Fraction(d.group(0)), decimal_step(d.group(0))
                if truncated:
                    cok = abs(shown - value) < step
                else:
                    rs = round_exact(shown)
                    cok = abs(shown - value) <= step / 2 or (rs is not None and _bits(rs) == _bits(actual))
                if not cok:
                    errors.append((site + " [comment self-inconsistent]", comment, round_exact(shown),
                                   expected, basis, 'the decimal must display the exact value', value))
                    results.append(False)

# ---------------------------------------------------------------------------
# DUPLICATE GLOBALS. Cyrius has ONE flat namespace and last-definition-wins, so
# two modules in the same bundle declaring the same unprefixed global silently
# resolve to whichever the bundler emits last. That is invisible until someone
# edits one copy and not the other -- at which point the value that ships is
# decided by manifest ORDER, not by the file being edited. Added 2.7.0 after
# F64_FIFTEEN was found declared in both calc.cyr and calc_ext.cyr.
#
# Two copies agreeing today is not safety, it is a coincidence with a deadline.
# Column 0 ONLY. Cyrius indents function bodies, so a leading-whitespace `var`
# is a LOCAL and shadows nothing outside its function.
# ⛔ 3.3.1 -- THIS ARM MATCHED NOTHING. The regex was `^var\s+([A-Z]...)`: it
# missed every `public var` (3.1.0 annotated them all) and every `_`-prefixed
# global, which is all 197 of the rest. 0 of 221 declarations matched, so it
# reported "no duplicates" over an empty set. cycc warns on a duplicate with a
# DIFFERENT value, which CI's duplicate-symbol step catches, but a same-value
# duplicate draws no diagnostic, and that is the case this arm exists for.
# [measured: regex replay over src/, 2026-09-30; cycc 6.6.12 duplicate probe]
GLOBAL_DECL = re.compile(r'^(?:public\s+)?var\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(\S.*?);')
seen_globals = {}
for path in sorted(glob.glob('src/*.cyr')):
    for lineno, line in enumerate(open(path, encoding='utf-8', errors='replace'), 1):
        m = GLOBAL_DECL.match(line)
        if not m:
            continue
        name, val = m.group(1), m.group(2).strip()
        seen_globals.setdefault(name, []).append((path, lineno, val))

# The same floor discipline as the constant population below: an arm that can be
# emptied by a mechanical edit while still printing "no duplicates" is the defect
# this block just had. Raise it when globals are added; lower it only with a
# reason in the commit (3.3.2 deletes the dead `_COL_*` globals and will).
GLOBAL_FLOOR = 218   # column-0 globals in src/; 221 -> 211 in 3.3.2: D063 deleted 10 dead _COL_* globals
                     # 211 -> 218 in 3.3.3: _IVL_F64_MAX/_IVL_F64_NEG_MAX (D021), four _GA_TOI_* (the
                     # time_of_impact normal), _NUMX_F64_INV_2P64 (num_sobol's 64-bit domain)
global_population = sum(len(v) for v in seen_globals.values())
if global_population < GLOBAL_FLOOR:
    print(f"\n!! DUPLICATE-GLOBAL SCAN SHRANK: {global_population} declarations seen, floor is {GLOBAL_FLOOR}.")
    print("   A global stopped matching GLOBAL_DECL -- the scan is blind to it. Check the")
    print("   declaration shape before touching the floor.")
    sys.exit(1)

dup_errors = []
for name, sites in sorted(seen_globals.items()):
    if len(sites) < 2:
        continue
    vals = {v for _, _, v in sites}
    where = ', '.join(f"{p}:{l}" for p, l, _ in sites)
    if len(vals) == 1:
        dup_errors.append((name, where, f"same value ({sites[0][2]}) -- still a hazard: an edit to one copy silently loses to bundle order"))
    else:
        dup_errors.append((name, where, "DIFFERENT VALUES: " + ' | '.join(sorted(vals))))

if dup_errors:
    print(f"\n!! {len(dup_errors)} DUPLICATE GLOBAL(S) ACROSS src/\n")
    for name, where, why in dup_errors:
        print(f"  {name}")
        print(f"      sites : {where}")
        print(f"      note  : {why}")
        print()
    print("Cyrius resolves these by bundle order, not by file. Declare each global once.")
    sys.exit(1)

total = len(results)
bad = len(errors)
print(f"=== constant check: {total - bad}/{total} verified, {len(skipped)} skipped ===")

if skipped and verbose:
    print("\n-- skipped (no machine-checkable value in comment) --")
    for p, l, n, c, why in skipped:
        print(f"  {p}:{l} {n}: {why} -- # {c}")

if errors:
    print(f"\n!! {bad} CONSTANT(S) DO NOT MATCH THEIR COMMENT\n")
    for site, comment, actual, expected, basis, rule, value in errors:
        print(f"  {site}")
        print(f"      comment : # {comment}")
        print(f"      basis   : {basis} ({rule})")
        print(f"      encoded : {actual!r}" + (f" = 0x{_bits(actual):016X}" if actual == actual else ""))
        if value is not None and value.denominator != 0 and not isinstance(expected, str):
            print(f"      exact   : {sig21(value)}")
        if expected is not None and expected == expected and abs(expected) != float('inf'):
            print(f"      expected: {expected!r} = 0x{_bits(expected):016X}")
            if actual == actual and abs(actual) != float('inf'):
                print(f"      distance: {ulp_distance(expected, actual):+d} ulp")
        print()
    print("A constant that disagrees with its own comment is a defect: the comment states")
    print("the intent and the bit pattern is what ships. Re-encode from the exact value.")
    sys.exit(1)

# ⛔ POPULATION FLOOR (3.1.0). This gate has now been caught TWICE reporting a
# confident N/N over a population that had silently shrunk: 2.20.0 found
# 158/158 over 159 (a comment on the line above the declaration), and the 3.1.0
# `public var` annotation dropped it to a GREEN 143/143 from 159 — the DECL
# regex began with `^\s*var`, so every annotated constant simply stopped being
# a constant as far as this gate knew. A gate whose population can shrink
# without failing is a gate that can be emptied by a mechanical edit. The floor
# is the population at the last audit; raise it when constants are added, never
# lower it without saying why in the commit.
POPULATION_FLOOR = 214   # 161 verified + 1 skipped; 164 -> 156 in 3.3.2: 8 of D063's 10 dead globals were hex constants
# 156 -> 162 in 3.3.3: the seven hex globals above (+7); D015 deleted solve_bicgstab's breakdown_tol (-1)
# 162 -> 214 in 3.3.7: the population had reached 212 without the floor following it; geo.cyr's
# _GEO_* constants and gauss5's nodes and weights became enum members (14 hex declarations out,
# 16 hex members in: two of them were aliases of error.cyr's public vars) [measured: this script's
# own count on the 3.3.7 tree, 211 verified + 3 skipped, 2026-10-31]
population = total + len(skipped)
if population < POPULATION_FLOOR:
    print(f"\n!! POPULATION SHRANK: {population} declarations seen, floor is {POPULATION_FLOOR}.")
    print("   A hand-encoded constant stopped matching DECL — the gate is blind to it, not")
    print("   the constant correct. Check the declaration shape (e.g. a new prefix) before")
    print("   touching the floor.")
    sys.exit(1)

print("All hand-encoded f64 constants match their documented values.")
PYEOF
