#!/usr/bin/env bash
# check-heap-globals.sh — no module global keeps a pointer into the bump heap across calls.
#
# WHY THIS EXISTS
# A caller's alloc_reset() zeroes the first heap chunk and hands it out again (lib/alloc.cyr),
# so a pointer a module global kept from an earlier call makes the next call read garbage, or
# write over the caller's new data. hisab's rule since 3.3.5 (the note above `_COL_O2D_BUF` in
# src/collision_mesh.cyr) is that no such pointer is kept. Three modules broke it for releases,
# each found by hand: `_COL_O2D_BUF` (through 3.3.4), `_CGA_NULL_TBL` (through 3.4.0) and
# `_einsum_arena` (through 3.4.0, SIGSEGV on the next einsum). Nothing looked. This looks.
#
# THE RULE IT CHECKS
# Every module-level var in src/ (main.cyr, the smoke binary, is not library code) that is
# written a value which CAN be a pointer into the heap or the frame, at any depth of any
# function, in an initializer or in a depth-0 statement, must either be cleared later in the
# same function with no `return` in between, or be named in ALLOW below with why it is safe.
# A value can be a pointer when it is:
#   - the result of a call that can reach alloc(): the closure over every fn in src/ and lib/,
#     so arena_*, vec_new and any hisab helper count, however deep the alloc() sits;
#   - the result of an indirect call (fncall0..N, callptr), or of load64 (a stored pointer);
#   - the result of a helper that returns an indirect call's result whole, to a fixed point;
#   - a parameter (the caller's pointer), or the address of a local (the frame);
#   - a local any of those reaches, by assignment or by any arithmetic on it, to a fixpoint;
#   - a global already flagged, or a call of a helper that hands an argument back.
# f64_* returns a double whatever its arguments are, so a call of one is a number. A store
# through `&G`, or through a local holding `&G`, is a write to G. A scalar global whose address
# is taken at all fails, and so does an array's handed to a call, since the gate cannot follow
# what the callee stores there.
#
# THE ALLOWLIST (ALLOW, below). One entry per global:
#   why      what makes it safe. Required, and read by people: a stale pointer is safe only
#            as long as its reasoning holds, so write what holds it.
#   sites    every fn that may write it such a value. A new writer fails the gate.
#   readers  pointers only: EXACTLY the fns that read it. A new reader, or one gone, fails the
#            gate: a kept pointer is safe only because each read follows a write in the same
#            call, and a new reader needs that proved again.
#   through  optional: no public fn may reach `sites` or `readers` except through one of
#            these (the call graph is walked with them removed, `&fn` handed to one of them
#            included).
# An entry that matches nothing live fails too, so the list cannot rot.
#
# WHAT IT CANNOT SEE. A pointer stored through the address a helper RETURNS (`return &G`), or held
# in a local of a name it also declares in another block (a local shadows the global through the
# whole function). It reads what a global IS WRITTEN; a read before a write inside `readers` is the
# suites' to catch: tests/modules_c.tcyr, group "3.4.1 INS6: a caller's alloc_reset() between
# calls, module by module", calls every module's public entries after a reset over a filled heap,
# and sets the collision queries' kept pointers to an address nothing maps between queries.
#
# USAGE
#   scripts/check-heap-globals.sh             check src/; exit 1 on anything unexplained
#   scripts/check-heap-globals.sh --census    every global written at run time, with its verdict
#   scripts/check-heap-globals.sh --selftest  plant every shape in a scratch dir, check each verdict
# Exit: 0 = clean; 1 = an unexplained write, a stale or empty entry, a reader or path that moved,
# or a population floor failed (the scan reached too little to be trusted); 2 = bad option.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

python3 - "${1:-}" <<'PY'
import glob, os, re, sys, tempfile

mode = sys.argv[1] if len(sys.argv) > 1 else ""
if mode not in ("", "--census", "--selftest"):
    print("unknown option: %s" % mode, file=sys.stderr)
    sys.exit(2)

# ── the allowlist ────────────────────────────────────────────────────────────────────────
_O = ["_gjk_intersect_3d_o", "_gjk_epa_3d_o", "_mpr_intersect_o", "_mpr_penetration_o", "_toi_o"]
_NUM = "a number, not a pointer: "
ALLOW = {
    # Pointers. Each of these holds an address at some point of a query or a build.
    "_COL_BI_BUF": {
        "why": "A public path enters through _col_scratch_call, which points it at a buffer in its own "
               "frame and clears it on return, so the `== 0` branch that allocates never runs there. "
               "It runs only for a suite's direct call of a predicate; the next builder overwrites it.",
        "sites": ["_col_bi_ready"], "readers": ["_col_bi_ready", "_col_bi_slot"],
        "through": ["_col_scratch_call"]},
    "_COL_O2D_BUF": {
        "why": "As _COL_BI_BUF: _col_scratch_call's frame on every public path, and the allocating "
               "branch runs only for a suite's direct call of the predicate.",
        "sites": ["_col_o2d_exact_raw"], "readers": ["_col_o2d_exact_raw"],
        "through": ["_col_scratch_call"]},
    "_GA_KC_F": {
        "why": "The caller's support function, compared with the argument and never called through, "
               "under _GA_KC_P's lifetime.",
        "sites": ["_ga_canon"], "readers": ["_ga_kc_take", "_ga_mink_scaled"]},
    "_GA_KC_P": {
        "why": "One support point, heap. _ga_canon clears it first, every public query runs _ga_canon "
               "before its first support, and that support takes and clears it. Left set only where a "
               "query ends before any support (a refused allocation); the next _ga_canon clears it "
               "before any read.",
        "sites": ["_ga_canon"], "readers": ["_ga_kc_take"]},
    "_GA_KC_S": {
        "why": "The caller's shape pointer, compared and never followed, under _GA_KC_P's lifetime.",
        "sites": ["_ga_canon"], "readers": ["_ga_kc_take"]},
    "_GA_MINK_AXES": {
        "why": "A heap vector for the one _gjk_intersect_core inside _ga_aspect_confirms, which clears it "
               "on the next line. The five entries save it on entry and put back what they found: 0 "
               "after a top-level call, and the outer query's value after a nested one.",
        "sites": _O,
        "readers": _O + ["_gjk_mink_support", "_ga_far_note", "_ga_far_sep", "_ga_mink_scaled",
                         "_ga_tet_margin", "_ga_pm_ready", "_epa_seed_portal", "_ga_sup_scale",
                         "_ga_proof_margin", "_epa_rf_hm", "_epa_core_retry", "_epa_round_core"]},
    "_GA_MS_PA": {
        "why": "The last support's point of A, heap, left behind by design. Every read follows a support "
               "of the same query: _ga_tet_margin, _epa_rf_hm and _epa_rf_crease on the next line, "
               "_ga_ms_size from _ga_far_note and _ga_far_sep on a value a support produced and from "
               "_ga_order_near only on a contact. A query's first action is a support, so a stale "
               "value is overwritten before any read.",
        "sites": ["_gjk_mink_support", "_ga_mink_scaled"],
        "readers": ["_ga_ms_size", "_ga_tet_margin", "_epa_rf_hm", "_epa_rf_crease"]},
    "_GA_MS_PB": {
        "why": "As _GA_MS_PA, the point of B.",
        "sites": ["_gjk_mink_support", "_ga_mink_scaled"],
        "readers": ["_ga_ms_size", "_ga_tet_margin", "_epa_rf_hm", "_epa_rf_crease"]},
    "_GA_NEAR_TETRA": {
        "why": "A heap tetrahedron left behind by design. _toi_near_point clears it on entry and "
               "_ga_inside_proven, the only reader, reads it straight after that call.",
        "sites": ["_toi_near_point"], "readers": ["_ga_inside_proven"]},
    "_GA_NEAR_V": {
        "why": "A heap point left behind by design. _ga_inside_proven clears it on entry, and its one "
               "reader, _ga_near_ring, runs only straight after that call (_ga_cycle_proof).",
        "sites": ["_ga_inside_proven"], "readers": ["_ga_near_ring"]},
    "_GA_PM": {
        "why": "The query's portal memo, heap. Each entry clears it on entry and puts back what it found "
               "(_ga_cache_save, _ga_cache_restore, with no return between them), so it is 0 after a "
               "top-level call, and nothing reaches it from a public fn outside those entries.",
        "sites": ["_epa_seed_portal", "_ga_cache_restore", "_epa_core_retry"],
        "readers": ["_ga_pm_ready", "_epa_seed_portal", "_ga_cache_save", "_epa_core_retry"],
        "through": _O},
    "_GA_PORTAL_SEPD": {
        "why": "A heap direction left behind by design, with _GA_PORTAL_APART. _gjk_core_dir_3d follows it "
               "only where _gjk_portal_certain just returned 2, that is APART = 1, which this query's "
               "portal set together with it. _epa_seed_portal only copies it to and from the query's "
               "memo.",
        "sites": ["_epa_seed_portal", "_epa_seed_portal_x"],
        "readers": ["_gjk_core_dir_3d", "_epa_seed_portal"]},
    "_GA_RF_P": {
        "why": "A heap support point left behind by design. _epa_rf_hm writes it and reads it on the next "
               "line; _epa_rf_crease reads it straight after _epa_rf_h, which calls _epa_rf_hm.",
        "sites": ["_epa_rf_hm"], "readers": ["_epa_rf_hm", "_epa_rf_crease"]},
    "_GA_SNAP_SMX": {
        "why": "_GA_MINK_AXES as a memo key: compared, never followed, and only while _GA_SNAP_SM is not "
               "0. _ga_cache_restore puts back what the entry found, 0 at top level.",
        "sites": ["_ga_sup_scale", "_ga_cache_restore"], "readers": ["_ga_sup_scale", "_ga_cache_save"],
        "through": _O},
    # Numbers: values the patterns cannot tell from a pointer.
    "_CGA_NT": {"why": _NUM + "14 bits of a packed sign and blade pair, in a static array.",
                "sites": ["_cga_build_null_tbl"]},
    "_CGA_SC_I": {"why": _NUM + "a blade index in a static array.", "sites": ["_cga_scalar_pairs"]},
    "_CGA_SC_J": {"why": _NUM + "a blade index in a static array.", "sites": ["_cga_scalar_pairs"]},
    "_CGA_SC_S": {"why": _NUM + "a sign in a static array.", "sites": ["_cga_scalar_pairs"]},
    "_CGA_XACC": {"why": _NUM + "32-bit limbs of the exact norm's accumulator, in a static array.",
                  "sites": ["_cga_xadd", "_cga_nsq_exact"]},
    "_GA_UX": {"why": _NUM + "the static +x direction, three doubles, handed to a support function to read.",
               "sites": ["_ga_canon"]},
    "_GRAD3_TABLE": {"why": _NUM + "gradient components, doubles, in a static array.", "sites": ["_grad3_set"]},
    "_lext_dz_stat": {"why": _NUM + "a static counter of lifts.", "sites": ["_lext_dz_lift"]},
    "_CGA_NULL_TBL": {"why": _NUM + "the built flag _cga_build_null_tbl returns (0 or 1); the table is the "
                                    "static _CGA_NT.", "sites": ["_cga_null_tbl"]},
    "_GA_CF_DONE": {"why": _NUM + "a flag, put back from the entry's frame block.",
                    "sites": ["_ga_cache_restore"]},
    "_GA_CF_S": {"why": _NUM + "the scale, a double, put back from the frame block.",
                 "sites": ["_ga_cache_restore"]},
    "_GA_DL_SM": {"why": _NUM + "a double, put back from the frame block.", "sites": ["_ga_cache_restore"]},
    "_GA_DL_SMS": {"why": _NUM + "a double, put back from the frame block.", "sites": ["_ga_cache_restore"]},
    "_GA_EPA_PROVEN": {"why": _NUM + "a verdict, 0 or 1 (_epa_contains, or a local flag).",
                       "sites": ["_epa_refine"]},
    "_GA_MINK_REDO": {"why": _NUM + "a magnitude's bit pattern.", "sites": ["_ga_redo_note"]},
    "_GA_MINK_S": {"why": _NUM + "the call's scale, a double.",
                   "sites": ["_ga_mink_scaled"]},
    "_GA_PM_ON": {"why": _NUM + "a flag, put back from the frame block.",
                  "sites": ["_ga_cache_restore"]},
    "_GA_PORTAL_APART": {"why": _NUM + "a flag, read back from the query's memo.", "sites": ["_epa_seed_portal"]},
    "_GA_PORTAL_NUDGED": {"why": _NUM + "a flag, read back from the query's memo.",
                          "sites": ["_epa_seed_portal"]},
    "_GA_RF_GL": {"why": _NUM + "a double: the length of a crease's chord, _epa_rf_crease's l1.",
                  "sites": ["_epa_rf_crease"]},
    "_GA_SNAP_JUMPS": {"why": _NUM + "a flag: put back from the frame block by _ga_cache_restore.",
                       "sites": ["_ga_cache_restore"]},
    "_GA_SNAP_SM": {"why": _NUM + "a double: _ga_sup_scale's reading, put back from the frame block.",
                    "sites": ["_ga_sup_scale", "_ga_cache_restore"]},
    "_GA_SNAP_SMS": {"why": _NUM + "a double, the scale that reading was taken under.",
                     "sites": ["_ga_cache_restore"]},
    "_GA_TETRA_GAP": {"why": _NUM + "a double: the least of four gaps, or read back from the query's memo.",
                      "sites": ["_epa_tetra_ok", "_epa_seed_portal"]},
    "_GA_TETRA_LMAX": {"why": _NUM + "a double, read back from the query's memo.", "sites": ["_epa_seed_portal"]},
    "_GA_TETRA_NAN": {"why": _NUM + "a flag, read back from the query's memo.", "sites": ["_epa_seed_portal"]},
    "_einsum_top": {"why": _NUM + "a byte count into the static _EINSUM_SCRATCH.", "sites": ["_einsum_take"]},
}

# The population floors: a scan that reaches little reports little. They sit under the 3.4.1
# tree (330 module vars, 1,783 fns, 64 written at run time, 202 writes, a closure of 915 fns).
FLOOR_VARS, FLOOR_FNS, FLOOR_WRITTEN, FLOOR_WRITES, FLOOR_CLOSURE = 300, 1500, 50, 170, 800

# ── the scanner ──────────────────────────────────────────────────────────────────────────
# The attribute tokens the cyrius guide lists: `#inline fn f(x) { ... }` is a declaration, so
# a `#` that opens one is not a comment. Same list as scripts/check-public-surface.sh.
ATTR = re.compile(r'#(?:inline|naked|pure|io|alloc|must_use|regalloc|deprecated|assert|pe_import)'
                  r'(?![A-Za-z0-9_])|#derive\([^)\n]*\)')
CHAR = re.compile(r"'(?:\\.|[^'\\\n])'")
NAME = r'[A-Za-z_][A-Za-z0-9_]*'
TOP = re.compile(r'(?<![A-Za-z0-9_])(public\s+)?(fn|var|struct|enum|union|type)\s+(' + NAME + r')')
CALL = re.compile(r'(?<![A-Za-z0-9_])(' + NAME + r')\s*\(')
ASSIGN = re.compile(r'(?<![A-Za-z0-9_.])(' + NAME + r'(?:\s*,\s*' + NAME + r')*)\s*=(?![=])')
INDIRECT = re.compile(r'^(?:fncall[0-9]+|callptr[0-9]*)$')
KEYWORDS = {"if", "elif", "while", "for", "return", "var", "else", "break", "continue"}

def strip(src):
    """Blank comments and string contents, keeping every newline (a `}` in either is not a brace)."""
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if c == '"':
            out.append('"')
            i += 1
            while i < n and src[i] != '"':
                if src[i] == '\\' and i + 1 < n:
                    out.append('  ' if src[i + 1] != '\n' else ' \n')
                    i += 2
                    continue
                out.append('\n' if src[i] == '\n' else ' ')
                i += 1
            if i < n:
                out.append('"')
                i += 1
        elif c == "'":
            m = CHAR.match(src, i)
            if m:
                out.append(' ' * (m.end() - i))
                i = m.end()
            else:
                out.append(c)
                i += 1
        elif c == '#':
            m = ATTR.match(src, i)
            if m:
                out.append(' ' * (m.end() - i))
                i = m.end()
                continue
            while i < n and src[i] != '\n':
                out.append(' ')
                i += 1
        else:
            out.append(c)
            i += 1
    return ''.join(out)

def closes(t, i, op, cl):
    d = 0
    while i < len(t):
        if t[i] == op:
            d += 1
        elif t[i] == cl:
            d -= 1
            if d == 0:
                return i
        i += 1
    return -1

def parse(path, fns, globs, top):
    """Every depth-0 fn and var of `path`, wherever on a line it sits, `public` or not; a depth-0
    statement that declares nothing goes to `top`."""
    t = strip(open(path, encoding='utf-8', errors='replace').read())
    i, n = 0, len(t)
    while i < n:
        if t[i].isspace():
            i += 1
            continue
        m = TOP.match(t, i)
        if m:
            kind, name = m.group(2), m.group(3)
            line = t.count('\n', 0, i) + 1
            if kind == 'fn':
                lp = t.index('(', m.end() - 1)
                rp = closes(t, lp, '(', ')')
                params = [p.split(':')[0].strip() for p in t[lp + 1:rp].split(',') if p.strip()]
                j = rp
                while t[j] not in '{;':
                    j += 1
                if t[j] == ';':
                    i = j + 1
                    continue
                k = closes(t, j, '{', '}')
                fns.setdefault(name, []).append({'file': path, 'line': line, 'params': params,
                                                 'body': t[j:k + 1], 'first': t.count('\n', 0, j) + 1,
                                                 'public': bool(m.group(1))})
                i = k + 1
                continue
            if kind in ('struct', 'enum', 'union'):
                j = t.index('{', m.end())
                i = closes(t, j, '{', '}') + 1
                continue
            if kind == 'type':
                i = t.index(';', i) + 1
                continue
            e = t.index(';', i)
            decl = ' '.join(t[m.start(2):e].split())
            head = decl.split('=')[0]
            names = re.findall(NAME, re.sub(r':[^,]*', '', head[4:].split('[')[0]))
            init = decl.split('=', 1)[1].strip() if '=' in decl else ''
            for nm in names:
                globs.setdefault(nm, []).append({'file': path, 'line': line, 'array': '[' in head,
                                                 'init': init, 'public': bool(m.group(1))})
            i = e + 1
            continue
        # `include "..."` and `private` end at no `;`: read as a statement they swallow the next fn.
        m = re.compile(r'include\s*"[^"\n]*"|private(?![A-Za-z0-9_])').match(t, i)
        if m:
            i = m.end()
            continue
        e, b = t.find(';', i), t.find('{', i)
        if b != -1 and (e == -1 or b < e):
            k = closes(t, b, '{', '}')
            top.append((path, t.count('\n', 0, i) + 1, t[i:k + 1]))
            i = k + 1
        elif e == -1:
            break
        else:
            top.append((path, t.count('\n', 0, i) + 1, t[i:e + 1]))
            i = e + 1

def calls_of(body):
    return set(m.group(1) for m in CALL.finditer(body)) - KEYWORDS

def analyse(src_files, lib_files):
    """The index: fns, globals, depth-0 statements, the alloc closure, the fns that hand an argument back."""
    fns, globs, top, lfns = {}, {}, [], {}
    for f in lib_files:
        parse(f, lfns, {}, [])
    for f in src_files:
        parse(f, fns, globs, top)
    graph = {}
    for tbl in (lfns, fns):
        for n, ds in tbl.items():
            s = graph.setdefault(n, set())
            for d in ds:
                s |= calls_of(d['body'])
    # alloc_via and realloc_via reach the allocator through its vtable, an indirect call: vec_new
    # and every str_* go that way, and a closure seeded on alloc alone misses them.
    reach = {'alloc', 'alloc_via', 'realloc_via'}
    grew = True
    while grew:
        grew = False
        for n, s in graph.items():
            if n not in reach and s & reach:
                reach.add(n)
                grew = True
    handback = set()
    for tbl in (lfns, fns):
        for n, ds in tbl.items():
            for d in ds:
                ps = '|'.join(re.escape(p) for p in d['params'] if p)
                if ps and re.search(r'(?<![A-Za-z0-9_])return\s+(?:' + ps + r')\s*[;+-]', d['body']):
                    handback.add(n)
    # A helper that returns a callback's answer passes on whatever the callback made, and it need
    # not reach alloc() itself: _ga_sup tests a support function's result for 0 and returns it.
    # Only what the helper returns whole counts: a callback answer it reads through (load64) or
    # adds to is its own number. To a fixed point, so a wrapper of a wrapper counts.
    relay = set()
    grew = True
    while grew:
        grew = False
        for tbl in (lfns, fns):
            for n, ds in tbl.items():
                if n in relay or n.startswith('f64_'):
                    continue
                for d in ds:
                    def origin(c):
                        return INDIRECT.match(c) or c in relay
                    names = [m.group(1) for m in re.finditer(r'(?<![A-Za-z0-9_.])(' + NAME + r')\s*=\s*(' + NAME
                                                             + r')\s*\(', d['body']) if origin(m.group(2))]
                    rets = re.findall(r'(?<![A-Za-z0-9_])return\s+(' + NAME + r')\s*([;(])', d['body'])
                    if any((o == '(' and origin(x)) or (o == ';' and x in names) for x, o in rets):
                        relay.add(n)
                        grew = True
                        break
    return {'fns': fns, 'globs': globs, 'top': top, 'reach': reach, 'handback': handback, 'relay': relay}

def whole_call(e):
    """`f(...)` when the whole of e is one call: (f, the text between its parentheses)."""
    m = re.match(r'(' + NAME + r')\s*\(', e)
    if not m:
        return None
    lp = m.end() - 1
    if closes(e, lp, '(', ')') != len(e) - 1:
        return None
    return m.group(1), e[lp + 1:-1]

def pointer_like(expr, known, ix):
    """Can this value be a pointer into the heap or the frame? `known` holds the names that can."""
    e = expr.strip()
    while e.startswith('(') and closes(e, 0, '(', ')') == len(e) - 1:
        e = e[1:-1].strip()
    def source(c):
        return c in ix['reach'] or c in ix['relay'] or INDIRECT.match(c) or c == 'load64'
    wc = whole_call(e)
    if wc:
        f, args = wc
        if f.startswith('f64_'):
            return False
        if source(f):
            return True
        if any(source(c) for c in calls_of(args)):
            return True
        if f in ix['handback']:
            return any(pointer_like(a, known, ix) for a in re.split(r',(?![^()]*\))', args) if a.strip())
        return False
    # An expression: a double from f64_* and a number from a helper fall out, then any source
    # call or any name that can be a pointer left in it decides (`p + 8`, `8 + p`, `p | 1`).
    while True:
        m = CALL.search(e)
        while m and source(m.group(1)):
            return True
        if not m:
            break
        rp = closes(e, m.end() - 1, '(', ')')
        if rp == -1:
            break
        e = e[:m.start()] + '0' + e[rp + 1:]
    return any(x in known for x in re.findall(NAME, e))

def locals_of(de):
    b = de['body']
    names = set(de['params'])
    for m in re.finditer(r'(?<![A-Za-z0-9_])var\s+(' + NAME + r'(?:\s*,\s*' + NAME + r')*)', b):
        names |= set(re.findall(NAME, m.group(1)))
    return names

def assignments(body):
    """(names, offset of the statement, right-hand side) for each `a = e;`, `var a = e;`, `a, b = e;`."""
    out = []
    for m in ASSIGN.finditer(body):
        e = body.find(';', m.end())
        if e == -1:
            continue
        names = [x for x in re.findall(NAME, m.group(1)) if x != 'var']
        if body[:m.start()].rstrip().endswith('var') and not names:
            continue
        out.append((names, m.start(), ' '.join(body[m.end():e].split())))
    return out

def reading_mask(body, name):
    """Offsets where `name` is read: every mention that is not an assignment's left side."""
    lhs = set()
    for names, off, _ in assignments(body):
        if name in names:
            for m in re.finditer(r'(?<![A-Za-z0-9_])' + re.escape(name) + r'(?![A-Za-z0-9_])',
                                 body[off:body.index('=', off)]):
                lhs.add(off + m.start())
    return [m.start() for m in re.finditer(r'(?<![A-Za-z0-9_.])' + re.escape(name) + r'(?![A-Za-z0-9_])', body)
            if m.start() not in lhs]

def census(ix, allow=None):
    """Every run-time write to a module global: (global, fn, file, line, rhs, pointer-like, cleared)."""
    globs, fns = ix['globs'], ix['fns']
    # A global ALLOW calls a number is no source of pointers: what is read from it is a number.
    numbers = set(g for g, e in (allow or {}).items() if 'readers' not in e)
    bodies = []
    for n, ds in fns.items():
        for d in ds:
            bodies.append((n, d))
    for path, line, text in ix['top']:
        bodies.append(("<depth 0>", {'file': path, 'line': line, 'params': [], 'body': text, 'first': line}))
    flagged_g = set()
    writes = {}
    grew = True
    while grew:
        grew = False
        writes = {}
        for n, d in bodies:
            b = d['body']
            locs = locals_of(d)
            arrs = set(re.findall(r'(?<![A-Za-z0-9_])var\s+(' + NAME + r')\s*(?::\s*\w+\s*)?\[', b))
            known = set(d['params']) | arrs | ((flagged_g - numbers) - locs)
            asg = assignments(b)
            again = True
            while again:
                again = False
                for names, _, rhs in asg:
                    for x in names:
                        if x in locs and x not in known and pointer_like(rhs, known, ix):
                            known.add(x)
                            again = True
            aliases = {}
            for names, _, rhs in asg:
                m = re.fullmatch(r'&\s*(' + NAME + r')(?:\s*[+-].*)?', rhs)
                if m and m.group(1) in globs and m.group(1) not in locs:
                    for x in names:
                        if x in locs:
                            aliases[x] = m.group(1)
            for names, off, rhs in asg:
                for x in names:
                    if x in globs and x not in locs:
                        p = pointer_like(rhs, known, ix)
                        w = (x, n, d['file'], d['first'] + b.count('\n', 0, off), rhs, p, cleared(b, off, x))
                        writes.setdefault(x, []).append(w)
                        if p and x not in flagged_g:
                            flagged_g.add(x)
                            grew = True
            # A store through the global's address, or a local holding it.
            for m in re.finditer(r'(?<![A-Za-z0-9_])store(?:8|16|32|64)\s*\(', b):
                lp = m.end() - 1
                rp = closes(b, lp, '(', ')')
                inner = b[lp + 1:rp]
                d0, parts, cut = 0, [], 0
                for i, c in enumerate(inner):
                    d0 += (c == '(') - (c == ')')
                    if c == ',' and d0 == 0:
                        parts.append(inner[cut:i])
                        cut = i + 1
                parts.append(inner[cut:])
                if len(parts) != 2:
                    continue
                gm = re.search(r'&\s*(' + NAME + r')', parts[0])
                g = gm.group(1) if gm and gm.group(1) in globs and gm.group(1) not in locs else None
                am = re.match(r'\s*(' + NAME + r')', parts[0])
                if g is None and am and am.group(1) in aliases:
                    g = aliases[am.group(1)]
                if g is not None and pointer_like(parts[1], known, ix):
                    rhs = ' '.join(parts[1].split())
                    writes.setdefault(g, []).append((g, n, d['file'], d['first'] + b.count('\n', 0, m.start()),
                                                     rhs, True, False))
                    flagged_g.add(g)
            # A global's address that leaves the function: the gate cannot follow what a callee
            # stores there. Any address of a scalar; an array's, or a local holding it, handed to
            # a call that is not a load or a store through it.
            for m in re.finditer(r'(?<![A-Za-z0-9_&])&\s*(' + NAME + r')', b):
                g = m.group(1)
                if g in globs and g not in locs and not globs[g][0]['array']:
                    w = (g, n, d['file'], d['first'] + b.count('\n', 0, m.start()), '&' + g + ' taken', True, False)
                    writes.setdefault(g, []).append(w)
            for m in CALL.finditer(b):
                if re.match(r'(?:load|store)(?:8|16|32|64)$', m.group(1)) or m.group(1) in KEYWORDS:
                    continue
                rp = closes(b, m.end() - 1, '(', ')')
                args = b[m.end():rp]
                for x in re.findall(r'&\s*(' + NAME + r')', args) + [a for a in re.findall(NAME, args) if a in aliases]:
                    g = aliases.get(x, x)
                    if g in globs and globs[g][0]['array'] and (x in aliases or x not in locs):
                        w = (g, n, d['file'], d['first'] + b.count('\n', 0, m.start()),
                             '&' + g + ' handed to ' + m.group(1), True, False)
                        writes.setdefault(g, []).append(w)
    for g, ds in globs.items():
        if ds[0]['init'] and pointer_like(ds[0]['init'], flagged_g, ix):
            writes.setdefault(g, []).append((g, "<initializer>", ds[0]['file'], ds[0]['line'], ds[0]['init'],
                                             True, False))
    return writes

def cleared(body, off, name):
    """1 where `name = 0;` runs on every path after this write: it follows with no `return`
    between, in this block or one enclosing it, never in a block opened after the write (an
    `else`, or a later `if`)."""
    tail = body[off + 1:]
    c = re.search(r'(?<![A-Za-z0-9_])' + re.escape(name) + r'\s*=\s*0\s*;', tail)
    r = re.search(r'(?<![A-Za-z0-9_])return(?![A-Za-z0-9_])', tail)
    if c is None or (r is not None and r.start() < c.start()):
        return False
    depth, low = 0, 0
    for ch in tail[:c.start()]:
        depth += (ch == '{') - (ch == '}')
        low = min(low, depth)
    return depth == low

def public_fns(ix):
    return set(n for n, ds in ix['fns'].items() if any(d['public'] for d in ds))

def reaches_public(ix, targets, through):
    """Public fns that reach a target without passing a `through` fn: a call or a `&fn` of one is
    an edge, and whatever is handed to a `through` fn is that fn's, not the caller's."""
    edges = {}
    tset = '|'.join(re.escape(t) for t in through)
    for n, ds in ix['fns'].items():
        if n in through:
            continue
        s = set()
        for d in ds:
            b = d['body']
            if tset:
                while True:
                    m = re.search(r'(?<![A-Za-z0-9_])(?:' + tset + r')\s*\(', b)
                    if not m:
                        break
                    rp = closes(b, m.end() - 1, '(', ')')
                    b = b[:m.start()] + '0' + b[rp + 1:]
            s |= calls_of(b)
            s |= set(m.group(1) for m in re.finditer(r'&\s*(' + NAME + r')', b))
        edges[n] = s
    pub = public_fns(ix)
    seen, stack = set(pub), list(pub)
    while stack:
        for y in edges.get(stack.pop(), ()):
            if y not in seen and y in ix['fns']:
                seen.add(y)
                stack.append(y)
    return sorted(set(targets) & seen)

def check(ix, allow, floors=True, quiet=False):
    """The verdict lines; an empty list is a clean tree."""
    bad = []
    writes = census(ix, allow)
    nw = sum(len(v) for v in writes.values())
    if floors:
        for what, have, need in (("module vars", len(ix['globs']), FLOOR_VARS), ("fns", len(ix['fns']), FLOOR_FNS),
                                 ("globals written", len(writes), FLOOR_WRITTEN), ("writes", nw, FLOOR_WRITES),
                                 ("fns in the alloc closure", len(ix['reach']), FLOOR_CLOSURE)):
            if have < need:
                bad.append("only %d %s (floor %d): the scan is not reaching src/" % (have, what, need))
    used = set()
    for g, ws in sorted(writes.items()):
        for (x, fn, f, line, rhs, ptr, clr) in ws:
            if not ptr or clr:
                continue
            e = allow.get(g)
            if e is None or fn not in e['sites']:
                bad.append("%s:%d  %s = %s  (in %s): can be a heap pointer, is not cleared before the "
                           "function returns, and ALLOW does not name this write" % (f, line, g, rhs[:60], fn))
            else:
                used.add((g, fn))
    for g, e in sorted(allow.items()):
        if len(e.get('why', '')) < 20:
            bad.append("ALLOW %s: no reason given" % g)
        for fn in e.get('sites', []):
            if (g, fn) not in used:
                bad.append("ALLOW %s: site %s writes no such value now: a stale entry" % (g, fn))
        if not e.get('sites'):
            bad.append("ALLOW %s: names no site" % g)
        if 'readers' in e:
            have = set()
            for n, ds in ix['fns'].items():
                for d in ds:
                    if g not in locals_of(d) and reading_mask(d['body'], g):
                        have.add(n)
            for fn in sorted(have - set(e['readers'])):
                bad.append("ALLOW %s: %s reads it and is not a listed reader: prove the read follows a "
                           "write in the same call, then list it" % (g, fn))
            for fn in sorted(set(e['readers']) - have):
                bad.append("ALLOW %s: reader %s reads it no more: a stale entry" % (g, fn))
        if e.get('through'):
            for fn in reaches_public(ix, list(e['sites']) + list(e.get('readers', [])), e['through']):
                bad.append("ALLOW %s: a public fn reaches %s without passing %s" % (g, fn, "/".join(e['through'])))
    if not quiet:
        print("=== heap globals: %d module vars, %d written at run time, %d writes; %d fns reach alloc(); "
              "%d allowlisted ===" % (len(ix['globs']), len(writes), nw, len(ix['reach']), len(allow)))
    return bad

LIBS = sorted(glob.glob("lib/*.cyr"))

if mode == "--selftest":
    # Each planted global names its verdict: *_bad must be flagged, *_ok must not be.
    plant = r'''
var ok_flag = 0;
var ok_count = 0;
var ok_static: i64[4];
var ok_ptr_static = 0;
var ok_cleared = 0;
var ok_comment = 0;
var ok_string = 0;
var ok_shadow = 0;
var ok_double = 0;
var ok_nested_cleared = 0;
var a_bad = 0;
public var b_bad = 0;
var c_x = 1; var c_bad = 0;
var d_bad = 0;
var e_bad = 0;
var f_bad = 0;
var g_bad = 0;
var h_bad = 0;
var i_bad = 0;
var j_bad = alloc(8);
public var k_bad = vec_new();
var l_bad = 0;
l_bad = alloc(8);
var m_bad: i64[4];
var n_bad: u8[32];
var o_bad = 0;
var p_bad = 0;
var q_bad = 0;
var r_bad = 0;
var s_bad = 0;
var t_bad = 0;
var u_bad = 0;
var v_bad = 0;
var w_bad = 0;
var w2_bad = 0;
var aa_bad = 0;
var ab_bad = 0;
var ac_bad = 0;
var ad_bad = 0;
var ae_bad: i64[2];
var bd_bad = 0;
var be_bad = 0;
var af_bad: i64[2];
var x_bad = 0;
var y_bad = 0;
var z_bad = 0;
fn _mk() { return alloc(8); }
fn _mk2() { return _mk(); }
fn _id(x) { return x; }
fn _store_it(p) { store64(p, alloc(8)); return 0; }
fn _numbers(x) {
    ok_flag = 1;
    ok_count = ok_count + 1;
    ok_ptr_static = &ok_static;
    ok_double = f64_add(f64_from(1), load64(_mk()));
    # ok_comment = alloc(8);
    var s = "ok_string = alloc(8);";
    return 0;
}
fn _shadow() {
    var ok_shadow = alloc(8);
    return ok_shadow;
}
fn _clears(n) {
    ok_cleared = alloc(8);
    _numbers(0);
    ok_cleared = 0;
    return 0;
}
fn _clears_deep(n) {
    if (n > 0) {
        ok_nested_cleared = alloc(8);
        ok_nested_cleared = 0;
    }
    return 0;
}
fn _direct() { a_bad = alloc(8); return 0; }
fn _pub() { b_bad = arena_new(64); return 0; }
fn _sameline() { c_bad = vec_new(); return 0; }
fn _deep(n) {
    if (n > 0) {
        while (n > 0) {
            if (n == 2) { d_bad = alloc(8); }
            n = n - 1;
        }
    }
    return 0;
}
fn _local() {
    var p = alloc(8);
    var q = p;
    var r = q + 8;
    e_bad = r;
    return 0;
}
fn _helper() { f_bad = _mk2(); return 0; }
fn _arena(a) { g_bad = arena_alloc(a, 8); return 0; }
fn _return_between(n) {
    h_bad = alloc(8);
    if (n > 0) { return 1; }
    h_bad = 0;
    return 0;
}
fn _param(s) { i_bad = s; return 0; }
fn _stack() {
    var b: u8[16];
    o_bad = &b;
    return 0;
}
fn _array_store(k) { store64(&m_bad + k * 8, alloc(8)); return 0; }
fn _alias_store() {
    var d = &n_bad;
    store64(d + 8, vec_new());
    return 0;
}
fn _out_param() { _store_it(&p_bad); return 0; }
fn _indirect(fp, x) { q_bad = fncall1(fp, x); return 0; }
fn _relay(fp, x) {
    var p = fncall1(fp, x);
    if (p == 0) { return 0; }
    return p;
}
fn _relay2(fp, x) { return _relay(fp, x); }
fn _relayed(fp, x) { bd_bad = _relay(fp, x); return 0; }
fn _relayed2(fp, x) {
    var p = _relay2(fp, x);
    be_bad = p;
    return 0;
}
fn _loaded(p) { r_bad = load64(p); return 0; }
fn _handed() { s_bad = _id(alloc(8)); return 0; }
fn _handed_local() {
    var p = alloc(8);
    t_bad = _id(p);
    return 0;
}
fn _from_flagged() { u_bad = a_bad; return 0; }
fn _via_flagged_local() {
    var k = a_bad;
    v_bad = k;
    return 0;
}
fn _tuple(p) {
    w_bad, w2_bad = _pair();
    return 0;
}
fn _pair() { return alloc(8); }
fn _sum_right(p) { x_bad = 8 + p; return 0; }
fn _tagged() {
    var p = alloc(8);
    aa_bad = p | 1;
    return 0;
}
fn _parens() {
    var p = alloc(8);
    ab_bad = (p + 8) - 8;
    return 0;
}
fn _else_clear(n) {
    if (n > 0) {
        ac_bad = alloc(8);
    } else {
        ac_bad = 0;
    }
    return 0;
}
fn _later_clear(n) {
    ad_bad = alloc(8);
    if (n > 0) { ad_bad = 0; }
    return 0;
}
fn _array_out() { _store_it(&ae_bad); return 0; }
fn _alias_out() {
    var d = &af_bad;
    _store_it(d);
    return 0;
}
#must_use
public fn _attr_public() { y_bad = alloc(8); return 0; }
#inline fn _inline_fn() { z_bad = alloc(8); return 0; }
'''
    d = tempfile.mkdtemp(prefix="hisab-heapglobals.")
    p = os.path.join(d, "plant.cyr")
    fails = []
    try:
        with open(p, "w") as fh:
            fh.write(plant)
        ix = analyse([p], LIBS)
        writes = census(ix)
        got = sorted(g for g, ws in writes.items() if any(w[5] and not w[6] for w in ws))
        names = re.findall(r'(?:^|[ ;])var ([a-z][a-z0-9]*_bad|ok_[a-z_]+)', plant, re.M)
        want = sorted(n for n in set(names) if n.endswith("_bad"))
        print("  selftest: planted %d globals, %d must be flagged" % (len(set(names)), len(want)))
        print("  selftest: scanner flagged %d" % len(got))
        for n in want:
            if n not in got:
                fails.append("MISSED: %s" % n)
        for n in got:
            if n not in want:
                fails.append("FALSE ALARM: %s" % n)
        # einsum's own defect, verbatim: the shape this gate was written for.
        p2 = os.path.join(d, "old.cyr")
        with open(p2, "w") as fh:
            fh.write("var _CAP = 8192;\nvar _einsum_arena = 0;\nvar _einsum_label_vals = 0;\n"
                     "fn _begin() {\n"
                     "    if (_einsum_arena == 0) { _einsum_arena = arena_new(_CAP); }\n"
                     "    if (_einsum_arena == 0) { return 1; }\n"
                     "    arena_reset(_einsum_arena);\n"
                     "    _einsum_label_vals = arena_alloc(_einsum_arena, 26 * 8);\n"
                     "    return 0;\n}\n")
        old = sorted(g for g, ws in census(analyse([p2], LIBS)).items() if any(w[5] and not w[6] for w in ws))
        if old != ["_einsum_arena", "_einsum_label_vals"]:
            fails.append("the 3.4.0 einsum shape: flagged %s" % old)
        # The allowlist's own rules, on a small tree.
        p3 = os.path.join(d, "al.cyr")
        with open(p3, "w") as fh:
            fh.write("var G = 0;\nvar H = 0;\n"
                     "fn _set() { G = alloc(8); return 0; }\n"
                     "fn _get() { return load64(G); }\n"
                     "fn _guard(f, x) { return fncall1(f, x); }\n"
                     "public fn p_via(x) { return _guard(&_set, x); }\n"
                     "public fn p_direct() { return _get(); }\n")
        ix3 = analyse([p3], LIBS)
        good = {"G": {"why": "a reason long enough to pass", "sites": ["_set"], "readers": ["_get"]}}
        r = check(ix3, good, floors=False, quiet=True)
        if r:
            fails.append("a complete entry should pass: %s" % r)
        for what, allow, needle in (
                ("no entry fails", {}, "ALLOW does not name"),
                ("a site not named fails", {"G": {"why": "a reason long enough to pass", "sites": ["_get"]}},
                 "ALLOW does not name"),
                ("an empty reason fails", {"G": {"why": "", "sites": ["_set"]}}, "no reason given"),
                ("a new reader fails", {"G": {"why": "a reason long enough to pass", "sites": ["_set"], "readers": []}},
                 "is not a listed reader"),
                ("a reader gone fails", {"G": {"why": "a reason long enough to pass", "sites": ["_set"],
                                               "readers": ["_get", "_nobody"]}}, "reads it no more"),
                ("a stale site fails", {"G": {"why": "a reason long enough to pass", "sites": ["_set", "_nobody"]}},
                 "a stale entry"),
                ("an entry for a clean global fails", dict(good, H={"why": "a reason long enough to pass",
                                                                    "sites": ["_set"]}), "a stale entry"),
                ("a public path round the guard fails",
                 {"G": {"why": "a reason long enough to pass", "sites": ["_set"], "readers": ["_get"],
                        "through": ["_guard"]}}, "without passing")):
            r = check(ix3, allow, floors=False, quiet=True)
            if not any(needle in x for x in r):
                fails.append("allowlist: %s: got %s" % (what, r))
        # `through`: a site reached only by a `&fn` handed to the guard passes.
        r = check(ix3, {"G": {"why": "a reason long enough to pass", "sites": ["_set"], "through": ["_guard"]}},
                  floors=False, quiet=True)
        if r:
            fails.append("a &fn handed to the guard is the guard's: %s" % r)
        # The floor: a scan that reaches nothing must not pass.
        empty = os.path.join(d, "empty.cyr")
        with open(empty, "w") as fh:
            fh.write("# nothing\n")
        r = check(analyse([empty], LIBS), {}, floors=True, quiet=True)
        if not any("floor" in x for x in r):
            fails.append("an empty scan passed the floors")
        # The real tree plus the plant: its own allowlist still holds, and the plant is seen.
        src = sorted(f for f in glob.glob("src/*.cyr") if os.path.basename(f) != "main.cyr")
        r = check(analyse(src + [p], LIBS), ALLOW, floors=True, quiet=True)
        for n in want:
            if not any(re.search(r'  ' + n + r'( =|\b)', x) for x in r):
                fails.append("the real tree plus the plant: %s not reported" % n)
    finally:
        for f in os.listdir(d):
            os.remove(os.path.join(d, f))
        os.rmdir(d)
    for x in fails:
        print("    %s" % x)
    print("  selftest: %s" % ("FAIL" if fails else "PASS"))
    sys.exit(1 if fails else 0)

src = sorted(f for f in glob.glob("src/*.cyr") if os.path.basename(f) != "main.cyr")
ix = analyse(src, LIBS)
if mode == "--census":
    writes = census(ix, ALLOW)
    for g in sorted(writes):
        ws = writes[g]
        ptr = [w for w in ws if w[5] and not w[6]]
        verdict = "number" if not ptr else ("allowlisted" if g in ALLOW else "UNEXPLAINED")
        if any(w[5] and w[6] for w in ws) and not ptr:
            verdict = "cleared in the same fn"
        print("%-22s %-22s %s" % (g, verdict, ", ".join(sorted(set(w[1] for w in ws)))[:100]))
    sys.exit(0)
bad = check(ix, ALLOW)
if bad:
    print()
    print("!! %d problem(s)" % len(bad))
    for x in bad:
        print("  " + x)
    print("A pointer into the bump heap kept across calls is overwritten by a caller's alloc_reset().")
    print("Use static storage or the call's frame, clear it before returning, or prove it safe in ALLOW.")
    sys.exit(1)
print("No module global keeps an unexplained heap pointer across calls.")
PY
