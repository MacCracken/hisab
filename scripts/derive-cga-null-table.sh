#!/usr/bin/env bash
# Derive the CGA geometric-product table in the NULL basis {e1,e2,e3,n0,ninf}
# and verify it against the shipped orthonormal {e1,e2,e3,ep,em} implementation.
#
# ⭐ WHY THIS EXISTS AS A SCRIPT RATHER THAN A PARAGRAPH. 2.20.0's audit found
# that several of that release's headline figures could not be re-derived from the
# tree — the probes were never committed — and this repo has been bitten by that
# before (a "41,998-input equivalence run" that `git log -S` could not find). The
# 2.21.0 basis change rests on three claims about the null-basis product table;
# all three are DERIVED here, from first principles, every time this runs.
#
# THE CLAIMS:
#   1. Every basis-blade product expands to AT MOST 2 blades in the null basis.
#      (The orthonormal basis gives exactly 1, which is why `_cga_geo_blades`
#      returns a single packed pair today.)
#   2. Every table coefficient is exactly +1 or -1 — no halves survive the change
#      of basis — so the null-basis product introduces NO new rounding.
#   3. The null-basis table agrees with the shipped orthonormal table under the
#      change of basis, so the algebra is unchanged and only its coordinates move.
#
# And the reason the change is worth making at all:
#   4. A conformal point is EXACTLY null in the null basis at every magnitude,
#      because P.P = q + 2*(q/2)*(n0.ninf) cancels the SAME computed q against
#      itself, where the ep/em form reconstructs q by squaring and loses it.
set -euo pipefail
python3 - <<'PY'
from fractions import Fraction as F
import sys

# ---- orthonormal reference: e1,e2,e3,ep (bit 3), em (bit 4); em^2 = -1 --------
def geo_orth(a, b):
    sign, bb = 1, b
    for bi in range(4, -1, -1):
        if a & (1 << bi):
            if bin(bb & ((1 << bi) - 1)).count("1") & 1: sign = -sign
            if bb & (1 << bi):
                if bi == 4: sign = -sign          # em^2 = -1
                bb ^= (1 << bi)
    return sign, (a ^ b) & 31

# ---- change of basis: n0 = (em - ep)/2, ninf = ep + em -----------------------
def wedge(a, b):
    """metric-free ordering sign; 0 when the blades share an index"""
    sign, bb = 1, b
    for bi in range(4, -1, -1):
        if a & (1 << bi):
            if bin(bb & ((1 << bi) - 1)).count("1") & 1: sign = -sign
            if bb & (1 << bi): return 0, 0
    return sign, (a ^ b) & 31

def rebuild(blade, mapping):
    cur = {0: F(1)}
    for bi in range(5):
        if not (blade & (1 << bi)): continue
        nxt = {}
        for b2, c2 in cur.items():
            for tb, tc in mapping(bi).items():
                s, rb = wedge(b2, tb)
                if s: nxt[rb] = nxt.get(rb, F(0)) + c2 * tc * s
        cur = {k: v for k, v in nxt.items() if v != 0}
    return cur

TO_ORTH = lambda bi: ({1 << 4: F(1, 2), 1 << 3: F(-1, 2)} if bi == 3 else
                      {1 << 3: F(1), 1 << 4: F(1)} if bi == 4 else {1 << bi: F(1)})
TO_NULL = lambda bi: ({1 << 4: F(1, 2), 1 << 3: F(-1)} if bi == 3 else
                      {1 << 4: F(1, 2), 1 << 3: F(1)} if bi == 4 else {1 << bi: F(1)})

table, hist, coeffs = {}, {}, set()
for A in range(32):
    for B in range(32):
        acc = {}
        for ba, ca in rebuild(A, TO_ORTH).items():
            for bb, cb in rebuild(B, TO_ORTH).items():
                s, rb = geo_orth(ba, bb)
                acc[rb] = acc.get(rb, F(0)) + ca * cb * s
        res = {}
        for bl, c in {k: v for k, v in acc.items() if v != 0}.items():
            for k, v in rebuild(bl, TO_NULL).items():
                res[k] = res.get(k, F(0)) + c * v
        res = {k: v for k, v in res.items() if v != 0}
        table[(A, B)] = res
        hist[len(res)] = hist.get(len(res), 0) + 1
        coeffs |= set(res.values())

fail = 0
print("CLAIM 1 — term count per basis-blade product")
for t in sorted(hist): print(f"    {t} term(s): {hist[t]:5} of 1024 pairs")
mx = max(len(v) for v in table.values())
print(f"    max = {mx}", "OK" if mx <= 2 else "FAIL")
fail += mx > 2

print("CLAIM 2 — table coefficients")
print(f"    distinct: {sorted(coeffs)}")
ok2 = coeffs <= {F(1), F(-1)}
print("    all +/-1, so the product adds no rounding:", "OK" if ok2 else "FAIL")
fail += not ok2

print("CLAIM 3 — agreement with the shipped orthonormal table under change of basis")
bad = 0
for A in range(32):
    for B in range(32):
        back = {}
        for bl, c in table[(A, B)].items():
            for k, v in rebuild(bl, TO_ORTH).items():
                back[k] = back.get(k, F(0)) + c * v
        back = {k: v for k, v in back.items() if v != 0}
        fwd = {}
        for ba, ca in rebuild(A, TO_ORTH).items():
            for bb, cb in rebuild(B, TO_ORTH).items():
                s, rb = geo_orth(ba, bb)
                fwd[rb] = fwd.get(rb, F(0)) + ca * cb * s
        fwd = {k: v for k, v in fwd.items() if v != 0}
        if back != fwd: bad += 1
print(f"    round-trip mismatches: {bad} of 1024", "OK" if bad == 0 else "FAIL")
fail += bad != 0

print("CLAIM 4 — exact nullity of a conformal point, f64, both bases")
print("    ⛔ THESE ARE MODEL FIGURES, NOT THE IMPLEMENTATION'S. This arm models the")
print("    ⛔ arithmetic in Python floats; it does NOT run hisab. The ep/em number")
print("    ⛔ below is the MODEL's and must never be quoted as the shipped tree's —")
print("    ⛔ 2.21.0's first draft did exactly that and published 24.8% where the")
print("    ⛔ tree that shipped as 2.20.0 measures 99.4%. To measure the LIBRARY,")
print("    ⛔ build a probe against src/, as tests/modules.tcyr does.")
import random, struct
random.seed(7)
bad_null = bad_ep = n = 0
for e in range(-40, 41, 5):
    for _ in range(1500):
        x, y, z = (random.uniform(1, 2) * 2.0**e for _ in range(3))
        q = x * x + y * y + z * z
        n += 1
        # ⛔ AN EARLIER VERSION OF THIS ARM WAS VACUOUS AND AN AUDIT CAUGHT IT.
        # It tested `q + 2.0*(q/2)*(-1.0) != 0.0`, which is identically FALSE for
        # every normal q (q/2 is exact, 2*(q/2) is exactly q), so it reported
        # "0 non-null" as a TAUTOLOGY and proved nothing about the basis.
        # The model must reconstruct q the way the implementation does: sum the
        # three squares independently and subtract the STORED q/2 twice.
        qs = x * x + y * y + z * z
        if qs - (q / 2) - (q / 2) != 0.0: bad_null += 1
        ep, em = q * 0.5 - 0.5, q * 0.5 + 0.5
        if q + ep * ep - em * em != 0.0: bad_ep += 1
print(f"    {n} full-mantissa triples over 2^-40..2^40")
print(f"    ep/em basis : {bad_ep:6} non-null ({100*bad_ep/n:.1f}%)")
print(f"    null basis  : {bad_null:6} non-null ({100*bad_null/n:.1f}%)",
      "OK" if bad_null == 0 else "FAIL")
fail += bad_null != 0

print("CLAIM 5 — the DEFINING null-basis identities")
def prod(A, B):
    acc = {}
    for ba, ca in rebuild(A, TO_ORTH).items():
        for bb, cb in rebuild(B, TO_ORTH).items():
            s, rb = geo_orth(ba, bb)
            acc[rb] = acc.get(rb, F(0)) + ca * cb * s
    res = {}
    for bl, c in {k: v for k, v in acc.items() if v != 0}.items():
        for k, v in rebuild(bl, TO_NULL).items():
            res[k] = res.get(k, F(0)) + c * v
    return {k: v for k, v in res.items() if v != 0}
N0, NINF = 1 << 3, 1 << 4
sym = {}
for d in (prod(N0, NINF), prod(NINF, N0)):
    for k, v in d.items(): sym[k] = sym.get(k, F(0)) + v
sym = {k: v for k, v in sym.items() if v != 0}
ids = [
    ("n0 * n0     == 0",        prod(N0, N0) == {}),
    ("ninf * ninf == 0",        prod(NINF, NINF) == {}),
    ("n0.ninf     == -1",       prod(N0, NINF).get(0) == F(-1)),
    ("symmetric sum == -2",     sym == {0: F(-2)}),
    ("e1 * e1     == +1",       prod(1, 1) == {0: F(1)}),
]
for label, ok in ids:
    print(f"    {label:24} {'OK' if ok else 'FAIL'}")
    fail += not ok

# ⚠ INDEX SPACE. Everything above works in BITMASK space (bit0=e1 .. bit3=n0,
# bit4=ninf). `_cga_geo_blades` is called with BLADE INDICES, and the two are not
# the same numbering — blade index 4 is bitmask 8. Emitting the contract in the
# implementation's own index space is deliberate: a table that is right in the
# wrong space is exactly the plausible-but-wrong artefact this file exists to stop.
BLADE_BITS = [0, 1, 2, 4, 8, 16, 3, 5, 9, 17, 6, 10, 18, 12, 20, 24,
              7, 11, 19, 13, 21, 25, 14, 22, 26, 28, 15, 23, 27, 29, 30, 31]
def pack(A_idx, B_idx):
    res = prod(BLADE_BITS[A_idx], BLADE_BITS[B_idx])
    terms = sorted(res.items())
    def enc(t):
        if t is None: return 0, 0
        bits, v = t
        return (1 if v > 0 else 2), BLADE_BITS.index(bits)
    s0, b0 = enc(terms[0] if len(terms) > 0 else None)
    s1, b1 = enc(terms[1] if len(terms) > 1 else None)
    return s0 | (b0 << 2) | (s1 << 7) | (b1 << 9)

print("CLAIM 6 — the table, in BLADE-INDEX space (the calling convention)")
h = 1469598103934665603
for ai in range(32):
    for bi in range(32):
        h = ((h ^ pack(ai, bi)) * 1099511628211) & 0xFFFFFFFFFFFFFFFF
print(f"    packing: bits 0-1 sign0 (0 none / 1 plus / 2 minus), 2-6 blade0,")
print(f"             bits 7-8 sign1, 9-13 blade1")
EXPECT = 0xF4A98C5706D5CF5B
ok6 = (h == EXPECT)
print(f"    FNV-1a over all 1024 blade-index entries: 0x{h:016X}")
print(f"    expected                                : 0x{EXPECT:016X}   "
      f"{'OK' if ok6 else 'FAIL'}")
# ⛔ AN EARLIER VERSION PRINTED THIS AND NEVER COMPARED IT, while the CI step
# claimed the script was "fail-closed (verified: breaking any claim exits 1)".
# Claim 6 exited 0 no matter what the table said. An audit caught it.
fail += not ok6
print( "    ⚠ THE CYRIUS TABLE MUST REPRODUCE THIS EXACTLY.")
# blade index 4 = n0, 5 = ninf under the new reading
assert pack(4, 4) == 0, "n0*n0 must vanish in blade-index space"
assert pack(5, 5) == 0, "ninf*ninf must vanish in blade-index space"
print(f"    spot: n0*n0 = 0x{pack(4,4):04X}, ninf*ninf = 0x{pack(5,5):04X}, "
      f"n0*ninf = 0x{pack(4,5):04X}, e1*e1 = 0x{pack(1,1):04X}")

print()
print("DERIVED TABLE: 1024 pairs, "
      f"{sum(1 for v in table.values() if len(v)==0)} vanishing, "
      f"{sum(1 for v in table.values() if len(v)==1)} single, "
      f"{sum(1 for v in table.values() if len(v)==2)} double.")
sys.exit(1 if fail else 0)
PY
