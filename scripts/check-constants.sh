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
#   # 2.0, the scale it leaves     a value at the head of a longer comment: that value is checked
#   # the sign-bit mask            prose, no number in it: skipped (see SKIP_RE); a number beside
#                                  those words is a value, and is checked or fails
#
# When both an exact form and a decimal appear, the EXACT form is authoritative
# and the decimal is additionally cross-checked for self-consistency. A decimal
# that has to pin the last bit needs about 17 significant digits; give the closed
# form when there is one.
#
# Usage:  ./scripts/check-constants.sh [--verbose]
#         ./scripts/check-constants.sh --selftest     plant every shape below, check each verdict
#         ./scripts/check-constants.sh --root DIR     check DIR/src/*.cyr with no floors (selftest)
# Exit:   0 = all constants verified; 1 = at least one mismatch
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SELF="$REPO_ROOT/scripts/check-constants.sh"
cd "$REPO_ROOT"

VERBOSE=0
FLOORS=1
case "${1:-}" in
    --verbose) VERBOSE=1 ;;
    --root) cd "$2"; FLOORS=0 ;;
    --selftest)
        # ⭐ 3.4.1 SYM4 -- A GATE THAT CANNOT FAIL IS NOT A GATE, AND THIS ONE HAS BEEN GREEN OVER A
        # WRONG CONSTANT FOUR TIMES. Every shape it reads, and every shape it refuses, is planted
        # here, wrong and right, and each verdict is checked. 2.0 + 1 ulp under a `# 2.0` comment.
        exec python3 - "$SELF" <<'SELFEOF'
import os, subprocess, sys, tempfile
gate = sys.argv[1]
BAD = '0x4000_0000_0000_0001'
OK = '0x4000_0000_0000_0000'
# (name, source with %s for the hex literal, wrong plant fires, right plant passes)
SHAPES = [
    ('`;` member',                'enum _ZA {\n    _ZA_X = %s;   # 2.0\n}\n', 1, 1),
    ('`,` member',                'enum _ZA {\n    _ZA_X = %s,   # 2.0\n    _ZA_Y = 1\n}\n', 1, 1),
    ('last member, no `;`',       'enum _ZA {\n    _ZA_X = %s   # 2.0\n}\n', 1, 1),
    ('one-line enum',             'enum _ZA { _ZA_X = %s; }   # 2.0\n', 1, 1),
    ('member on the `{` line',    'enum _ZA { _ZA_X = %s;   # 2.0\n    _ZA_Y = 1;\n}\n', 1, 1),
    ('member on the `}` line',    'enum _ZA {\n    _ZA_Y = 1;\n    _ZA_X = %s; }   # 2.0\n', 1, 1),
    ('comment on the line above', 'enum _ZA {\n    # 2.0\n    _ZA_X = %s;\n}\n', 1, 1),
    ('public enum',               'public enum _ZA {\n    _ZA_X = %s;   # 2.0\n}\n', 1, 1),
    ('`{` on the next line',      'enum _ZA\n{\n    _ZA_X = %s;   # 2.0\n}\n', 1, 1),
    ('`{` next line, typed',      'enum _ZA: i64\n{\n    _ZA_X = %s;   # 2.0\n}\n', 1, 1),
    ('a comment before `{`',      'enum _ZA   # the scale\n{\n    _ZA_X = %s;   # 2.0\n}\n', 1, 1),
    ('value on the next line',    'enum _ZA {\n    _ZA_X =\n      %s;   # 2.0\n}\n', 1, 1),
    ('`{` inside a comment',      'enum _ZA {  # { open\n    _ZA_X = %s;   # 2.0\n}\n', 1, 1),
    ('`}` inside a comment',      'enum _ZA {\n    _ZA_Y = 1;   # } not the end\n    _ZA_X = %s;   # 2.0\n}\n', 1, 1),
    ('enum inside a fn',          'fn _zf() {\n    enum _ZA {\n        _ZA_X = %s;   # 2.0\n    }\n    return _ZA_X;\n}\n', 1, 1),
    ('second enum on the line',   'enum _ZB { _ZB_Y = 1; } enum _ZA { _ZA_X = %s; }   # 2.0\n', 1, 0),
    ('two hex members, one line', 'enum _ZA { _ZA_X = %s, _ZA_Y = 0x4000_0000_0000_0000 }   # 2.0\n', 1, 0),
    ('value is an expression',    'enum _ZA {\n    _ZA_X = %s | 1;   # 2.0\n}\n', 1, 0),
    ('no comment',                'enum _ZA {\n    _ZA_X = %s;\n}\n', 1, 0),
    ('comment with no value',     'enum _ZA {\n    _ZA_X = %s;   # the scale\n}\n', 1, 0),
    ('value, the word mask',      'enum _ZA {\n    _ZA_X = %s;   # 2.0, the scale the mask leaves\n}\n', 1, 1),
    ('value, the word inf',       'enum _ZA {\n    _ZA_X = %s;   # 2.0, finite (not inf)\n}\n', 1, 1),
    ('value, the word sentinel',  'enum _ZA {\n    _ZA_X = %s;   # 2.0: a sentinel\n}\n', 1, 1),
    ('a number past the head',    'enum _ZA {\n    _ZA_X = %s;   # the mask for the scale 2.0\n}\n', 1, 0),
    ('var, the word mask',        'var _ZA_X = %s;   # 2.0, the scale the mask leaves\n', 1, 1),
    ('var, the word hash',        'var _ZA_X = %s;   # 2.0 (the hash seed)\n', 1, 1),
]
# Prose: bits, not a value, in a comment with no value: skipped, whatever the bits.
PROSE = [
    ('a mask',            'enum _ZA {\n    _ZA_X = %s;   # the sign-bit mask\n}\n'),
    ('an integer',        'enum _ZA {\n    _ZA_X = %s;   # an integer sentinel\n}\n'),
    ('a NaN, with `_m3`', 'enum _ZA {\n    _ZA_X = %s;   # a NaN\'s bits, never a result of `_m3_wscale`\n}\n'),
]
def gate_rc(src):
    with tempfile.TemporaryDirectory() as d:
        os.mkdir(d + '/src')
        with open(d + '/src/zz.cyr', 'w') as f: f.write(src)
        r = subprocess.run(['bash', gate, '--root', d], capture_output=True, text=True)
        return r.returncode, r.stdout
bad = 0
n = 0
for name, src, fires, passes in SHAPES:
    rc, out = gate_rc(src % BAD)
    n += 1
    if (rc != 0) != bool(fires):
        print('  selftest: %s: the wrong constant %s' % (name, 'passed' if rc == 0 else 'fired unexpectedly'))
        bad += 1
    rc, out = gate_rc(src % OK)
    n += 1
    if (rc == 0) != bool(passes):
        print('  selftest: %s: the right constant gave rc %d' % (name, rc))
        bad += 1
for name, src in PROSE:
    rc, out = gate_rc(src % BAD)
    n += 1
    if rc != 0 or ' 1 skipped' not in out:
        print('  selftest: %s: not skipped as prose (rc %d)' % (name, rc))
        bad += 1
# An `enum` the parser cannot open fails the gate whatever follows it.
rc, out = gate_rc('enum _ZA\nvar _ZB = 1;\n')
n += 1
if rc == 0:
    print('  selftest: an `enum` with no body passed')
    bad += 1
# A word in a comment or a string is not an enum.
rc, out = gate_rc('# enum in a comment\nfn _zf() { return "enum"; }\n')
n += 1
if rc != 0:
    print('  selftest: an `enum` in a comment or string fired')
    bad += 1
print('  selftest: %d verdicts, %d wrong: %s' % (n, bad, 'PASS' if bad == 0 else 'FAIL'))
sys.exit(1 if bad else 0)
SELFEOF
        ;;
esac

python3 - "$VERBOSE" "$FLOORS" <<'PYEOF'
import glob, re, struct, sys
from fractions import Fraction

verbose = sys.argv[1] == "1"
floors = sys.argv[2] == "1"

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
# between an `enum NAME {` and its closing `}`: the same `NAME = 0xHEX;  # comment` shape is
# an ordinary assignment in a function body.
# ⛔ 3.4.1 SYM3 -- AND IT READ ONE SHAPE OF MEMBER, SO THE OTHERS WERE INVISIBLE. The member
# regex wanted `NAME = 0xHEX;  # comment` on a line of its own between an `enum NAME {` line
# and a `}` line. cycc also takes a member closed by `,`, the last member with no separator,
# and members on the `{` or `}` line -- a one-line enum among them -- and none of those was
# seen: complex.cyr's _CX_DIV_FOUR (the last member, no `;`) and autodiff.cyr's _AD_UNSEEN
# (`enum _AdSweep { _AD_UNSEEN = 0x7FF4_0000_0000_0A0D }`) were neither verified nor skipped,
# and the count stayed green. An enum body is now read from `{` to `}`, across lines, split
# into members at `;` and `,` (but see ⛔ 3.4.1 SYM4 below: not yet as cycc reads it).
# Every member whose value holds a hex
# literal of 12 digits or more is an f64 constant, and it must be verified against its
# comment -- the comment after it on its line when it is the line's only member, else the
# comment line just above -- or carry a comment SKIP_RE names as prose (a mask, a NaN, a
# sentinel). Any other such member FAILS the gate: one sharing its line with another member
# (a trailing comment cannot say whose it is), a value that is not a bare hex literal, no
# comment, or a comment with no value in it. A var that cannot be read is still skipped, as
# before; an enum member is not.
# [measured: scratch copy of the 3.4.1 SYM3 tree, a wrong constant planted in each shape, 2026-10-31]
# ⛔ 3.4.1 SYM4 -- AND "AS cycc READS IT" WAS FALSE, THREE WAYS, AND A WRONG _CX_DIV_SAFE PASSED. The
# scan was line by line, so `{` on the line after `enum NAME` opened nothing and a second enum on a
# line was read as the first's trailing text: cycc takes both, and the members were invisible, a
# population short of its floor by less than the floor's slack. And a comment was prose by its
# WORDS, so `# 2^1022, the scale the mask leaves` skipped a wrong value. With the braces moved down
# for _CxDivWide and _CxExt and _CX_DIV_SAFE planted as 2^1023 under `# 2^1022`, the gate reported
# 299/299 verified. Now the file is read as tokens, comments and strings out: EVERY `enum` token must
# open a body the parser reads (it fails otherwise, which closes the class and not the instances),
# wherever its `{` is, and each member takes the comment of the line it ends on, shared with
# nothing else on that line. A comment is prose only when it holds NO VALUE: the words no longer
# exempt anything beside a number, and a value at the head of a longer comment is verified (see
# value_head). [measured: ./scripts/check-constants.sh --selftest, and the two plants on the tree]
ENUM_OPEN = re.compile(r'enum\s+[A-Za-z_][A-Za-z0-9_]*\s*(?::\s*[A-Za-z_][A-Za-z0-9_]*\s*)?\{')
ENUM_TOKEN = re.compile(r'(?<![A-Za-z0-9_])enum(?![A-Za-z0-9_])')
ENUM_MEMBER = re.compile(r'^([A-Za-z_][A-Za-z0-9_]*)\s*(?:=\s*(.*?))?$', re.S)
HEX_BARE = re.compile(r'^0[xX][0-9A-Fa-f_]{1,25}$')
HEX_ANY = re.compile(r'0[xX][0-9A-Fa-f_]+')
# Constants whose comment is prose, not a value: one of these words and no number standing on its
# own. A digit inside a name (`_m3_wscale`) is not a number.
SKIP_RE = re.compile(r'\b(nan|inf|sentinel|mask|bits? pattern|magic|seed|hash)\b', re.I)
NUMBER = re.compile(r'(?<![A-Za-z0-9_.])[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?(?![A-Za-z0-9_])')
# Where a value at the head of a longer comment ends: "2.0, the scale", "2^1022: the bound".
HEAD_END = re.compile(r'\s*[,:;]|\s+\(|\s+--+\s|\s+—\s')

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
# 3.4.1 SYM3: enum members whose hex f64 the gate could not read -- each one fails it.
undecodable = []
enum_seen = 0

def hex_payload(h):
    """A hex literal long enough to be an f64 payload, not a short mask or flag."""
    return len(h.replace('_', '')) - 2 >= 12

def value_head(comment):
    """The value a comment opens with, when it opens with one and has more to say: `2.0` from
    "2.0, the scale the mask leaves", `inf` from "inf: the exponent field". None when it does not
    (the comment's own grammar above then reads it whole, an `=` included)."""
    text = comment.strip()
    if '=' in text:
        return None
    m = HEAD_END.search(text)
    h = text[:m.start()].strip() if m else text
    if h.lower().lstrip('+-') in ('inf', 'infinity'):
        return h
    if h and '...' not in h and '…' not in h and eval_exact(h) is not None:
        return h
    return None

def check_one(path, lineno, name, hexlit, comment, in_enum):
    """Verify one hex declaration against its comment: into results / errors / skipped, or,
    for an enum member whose comment holds no value, into undecodable."""
    actual = as_double(hexlit)
    site = f"{path}:{lineno} {name}"

    def skip(why):
        if in_enum and why != 'prose comment':
            undecodable.append((path, lineno, name, f"its comment has no value to check ({why}): # {comment}"))
        else:
            skipped.append((path, lineno, name, comment, why))

    # +/-Infinity is checkable exactly, so verify rather than skip. These are
    # real sentinel values used by the geometry and spatial modules and a
    # wrong one would silently break every min/max seed that starts from them.
    head = value_head(comment)
    cl = (head if head is not None else comment).strip().lower().lstrip('+')
    if cl in ('infinity', 'inf', '-infinity', '-inf'):
        want = float('-inf') if cl.startswith('-') else float('inf')
        ok = (actual == want)
        results.append(ok)
        if not ok:
            errors.append((site, comment, actual, want, 'infinity literal', 'exact', None))
        elif verbose:
            print(f"  ok   {site:52s} = {actual!r} (infinity literal)")
        return

    if head is not None and head != comment.strip():
        comment = head
    elif SKIP_RE.search(comment) and not NUMBER.search(comment):
        skip('prose comment'); return

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
            skip('symbolic expression not evaluable -- verify by hand'); return
        d = DEC.search(dec_src_clean) or DEC.search(comment)
        if not d:
            skip('no parseable value'); return
        tok = d.group(0)
        try: value = Fraction(tok)
        except ValueError:
            skip('unparseable number'); return
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

def split_code(lines):
    """(code, comments): one string of the file's code, its comments and strings blanked and its
    newlines kept, and each line's comment. A `#` in a string or a character literal is neither."""
    out, cmts = [], []
    for line in lines:
        code, i, n, q = [], 0, len(line), None
        cmt = None
        while i < n:
            c = line[i]
            if q:
                if c == '\\':
                    code.append('  '); i += 2; continue
                if c == q: q = None
                code.append(' ')
            elif c == '"' or c == "'":
                q = c
                code.append(' ')
            elif c == '#':
                cmt = line[i + 1:].strip()
                break
            else:
                code.append(c)
            i += 1
        out.append(''.join(code).rstrip('\n'))
        cmts.append(cmt)
    return '\n'.join(out), cmts

def scan_enums(path, lines):
    """Read every enum as cycc does, and return the line numbers (1-based) they cover."""
    global enum_seen
    code, cmts = split_code(lines)
    covered = set()
    def line_at(pos):
        return code.count('\n', 0, pos) + 1
    at = 0
    members = []
    while True:
        t = ENUM_TOKEN.search(code, at)
        if not t:
            break
        m = ENUM_OPEN.match(code, t.start())
        if not m:
            undecodable.append((path, line_at(t.start()), 'enum',
                                "an `enum` this gate could not open: nothing in it is checked"))
            at = t.end()
            continue
        close = code.find('}', m.end())
        if close < 0:
            undecodable.append((path, line_at(t.start()), 'enum', "an enum with no closing `}`"))
            break
        covered.update(range(line_at(t.start()), line_at(close) + 1))
        pos = m.end()
        for piece in re.split(r'([;,])', code[m.end():close]):
            start = pos
            pos += len(piece)
            if piece in (';', ',') or not piece.strip():
                continue
            lead = len(piece) - len(piece.lstrip())
            q = ' '.join(piece.split())
            members.append((line_at(start + lead), line_at(start + len(piece.rstrip()) - 1), q))
        at = close + 1
    by_line = {}
    for first, last, q in members:
        by_line.setdefault(last, []).append(q)
    for first, last, q in members:
        if not any(hex_payload(h) for h in HEX_ANY.findall(q)):
            continue
        enum_seen += 1
        mm = ENUM_MEMBER.match(q)
        name = mm.group(1) if mm else q
        if not mm or mm.group(2) is None or not HEX_BARE.match(mm.group(2)):
            undecodable.append((path, last, name, f"its value is not a bare hex literal: {q}"))
            continue
        if len(by_line[last]) > 1:
            undecodable.append((path, last, name, "it shares its line with another member, so the "
                                "line's comment cannot say which one it documents: give it a line of its own"))
            continue
        cmt = cmts[last - 1]
        if not cmt and first >= 2 and not lines[first - 2].split('#')[0].strip():
            cmt = cmts[first - 2]
        if not cmt:
            undecodable.append((path, last, name, "it has no comment, on its line or the line above"))
            continue
        check_one(path, last, name, mm.group(2), cmt, True)
    return covered

for path in sorted(glob.glob('src/*.cyr')):
    prev_comment = None
    lines = open(path, encoding='utf-8', errors='replace').readlines()
    in_enum = scan_enums(path, lines)
    for lineno, line in enumerate(lines, 1):
        if lineno in in_enum:
            prev_comment = None
            continue
        m = DECL.match(line)
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
        if not hex_payload(hexlit):   # short masks/flags, not f64 payloads
            continue
        check_one(path, lineno, name, hexlit, comment, False)

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
GLOBAL_FLOOR = 327   # column-0 globals in src/; 221 -> 211 in 3.3.2: D063 deleted 10 dead _COL_* globals
                     # 211 -> 218 in 3.3.3: _IVL_F64_MAX/_IVL_F64_NEG_MAX (D021), four _GA_TOI_* (the
                     # time_of_impact normal), _NUMX_F64_INV_2P64 (num_sobol's 64-bit domain)
                     # 218 -> 321 in 3.4.1 SYM4: the floor had not followed the population, this
                     # script's own count on the merged 3.4.1 tree [measured: 2026-11-01]
                     # 321 -> 320 in 3.4.1 INS6: einsum's static scratch retired _EINSUM_ARENA_CAP and
                     # _einsum_arena for _einsum_top (an array has no `=`, so _EINSUM_SCRATCH is not counted)
                     # 320 -> 327 at the 3.4.1 close: the last passes' new globals (_GA_REFUSED and the
                     # ball-by-the-rim reading), this script's own count on the merged tree [measured: 2026-11-01]
global_population = sum(len(v) for v in seen_globals.values())
if floors and global_population < GLOBAL_FLOOR:
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
print(f"=== constant check: {total - bad}/{total} verified, {len(skipped)} skipped "
      f"({enum_seen} hex enum members read, {len(undecodable)} unreadable) ===")

if undecodable:
    print(f"\n!! {len(undecodable)} ENUM MEMBER(S) HOLD A HEX f64 THIS GATE CANNOT CHECK\n")
    for p, l, n, why in undecodable:
        print(f"  {p}:{l} {n}: {why}")
    print("\nAn enum member is checked or the gate fails (3.4.1 SYM3): write it as `NAME = 0xHEX;  # value`")
    print("on a line of its own (the `;` is optional on the last member), or name it prose in its comment")
    print("(a mask, a NaN, a sentinel) when it holds bits rather than a value.")

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
if undecodable:
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
# 214 -> 305 in 3.4.1 SYM3, and two more floors beside it: the VERIFIED count, so a constant
# cannot move from verified to skipped (a comment edited into prose) unnoticed, and the count
# of hex enum members read, so the enum parser cannot lose a shape the way the member regex did
# (see the ⛔ 3.4.1 SYM3 paragraph at ENUM_START). [measured: this script's own count on the 3.4.1
# SYM3 tree, 294 verified + 11 skipped, 80 hex enum members, 2026-11-01]
# 305 -> 340, 294 -> 321, 80 -> 105 in 3.4.1 SYM4: the floors had fallen 10 enum members behind the
# tree, room for the 7 a plant hid, so each is the count again. [measured: this script's own count
# on the merged 3.4.1 tree, 321 verified + 19 skipped, 105 hex enum members, 2026-11-01]
POPULATION_FLOOR = 346
VERIFIED_FLOOR = 324
ENUM_MEMBER_FLOOR = 108
population = total + len(skipped)
if not floors:
    sys.exit(0)
if population < POPULATION_FLOOR:
    print(f"\n!! POPULATION SHRANK: {population} declarations seen, floor is {POPULATION_FLOOR}.")
    print("   A hand-encoded constant stopped matching DECL — the gate is blind to it, not")
    print("   the constant correct. Check the declaration shape (e.g. a new prefix) before")
    print("   touching the floor.")
    sys.exit(1)
if total - bad < VERIFIED_FLOOR:
    print(f"\n!! VERIFIED COUNT SHRANK: {total - bad} constants verified, floor is {VERIFIED_FLOOR}.")
    print("   A constant the gate used to check is now skipped or unseen. Restore a comment that")
    print("   states its value before touching the floor.")
    sys.exit(1)
if enum_seen < ENUM_MEMBER_FLOOR:
    print(f"\n!! ENUM MEMBERS SHRANK: {enum_seen} hex enum members read, floor is {ENUM_MEMBER_FLOOR}.")
    print("   An enum member stopped being read -- check the enum's shape before touching the floor.")
    sys.exit(1)

print("All hand-encoded f64 constants match their documented values.")
PYEOF
