#!/usr/bin/env bash
# check-test-names.sh -- a top-level name in a test harness means one thing: no fn declared twice,
# no fn sharing its name with a value, no global repeated where repeats merge, no fn left reading
# a variable that later code has redeclared, and no library declaration replaced or shadowed
# unless ALLOW says why.
#
# WHY IT EXISTS (3.5.0, roadmap "A duplicate fn in tests/hisab.bcyr, or a repeated top-level var
# in a suite, builds with no diagnostic"; review MAT4, INS4)
# CI's duplicate gate reads `cyrius check --with-deps dist/hisab.cyr`, the bundle; nothing read
# tests/. What a repeated top-level name does depends on WHERE it is (cyrius guide, "Declaring a
# global twice"), and the selftest compiles each rule below under the manifest pin:
#   - A fn is global wherever it is defined: a second definition replaces the first for EVERY
#     call, those written before it included, with only a compiler warning no CI step fails on.
#   - A fn and a value may share a name, silently: `name()` calls the fn, a bare `name` reads the
#     value, and `&name` is the VALUE's address, so a callback written `&name` jumps into data.
#   - In the DECLARATION ZONE (the top-level declarations before the program's first top-level
#     statement; every harness's own `alloc_init();` or the bench's `sys_exit_group(r);` ends it,
#     and no file a harness includes holds a statement) a name declared twice is ONE global, the
#     last definition wins, and a constant redeclaration is its value from program start: a read
#     written before it sees the later value. tests/hisab.bcyr declares nearly all of its globals
#     there. A zone declaration with a library's name is the library's global.
#   - AFTER the first statement a `var` starts a NEW variable for the code after it; the code
#     before it, fns defined earlier included, keeps the earlier one (`alloc_init(); var k = 5;
#     fn rd() { return k; } var k = 7;` gives rd() 5 and k 7). That is lexical shadowing, the
#     language's rule, and the suites reuse scratch names that way on purpose (Result tuple names
#     such as _r1t / _r1v): it is not a defect by itself. It becomes one when a fn bound to an
#     earlier declaration is reached from code after a later one -- named there, directly or through
#     a fn that calls it: that code writes and reads the new variable while the fn reads the old.
#     A `var` there with a library's name shadows it for the code after it: a write never reaches
#     the library, whose fns keep their own.
# [measured: lane L8 probes p1-p15 under cycc 6.7.18, scratch, 2026-11-14; and this script's --selftest]
#
# WHAT IT CHECKS, over tests/*.tcyr, tests/*.bcyr and tests/*.fcyr, each read as cycc reads it:
# comments and strings out, then token by token, so a declaration after a statement on the same
# line (`_fb_exhaust_after(0); var _fb_x1t, _fb_x1 = ...;`) is read like one on its own line.
#   1. A fn declared twice in one file.
#   2. A fn and a top-level value (var, const or enum member) sharing a name.
#   3. A value declared twice in the declaration zone.
#   4. A fn bound to a declaration of a name that is declared again later, reached after that
#      later declaration.
#   5. A top-level fn or value with the name of a declaration in a file the harness compiles: its
#      `include` lines, followed recursively, and the manifest's stdlib leaves. ALLOW below names
#      each deliberate replacement with its reason.
# Floors: every harness in tests/ is read, each has a first statement, and at least
# TOPLEVEL_FLOOR top-level names and FN_FLOOR fns are read in all.
#
# WHAT IT CANNOT SEE: two harnesses are separate programs, so one name in two files is no
# collision; a name declared inside a function, or inside a top-level block, is a local.
# Conditional includes are all followed (a collision on any platform's stdlib is reported).
#
# Usage:
#   scripts/check-test-names.sh                    check tests/ against ALLOW
#   scripts/check-test-names.sh --selftest         plant every shape and check each verdict, and
#                                                  compile the language rules above (needs cyrius)
#   scripts/check-test-names.sh --root DIR         check DIR's tests/ (no floors)
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SELF="$REPO_ROOT/scripts/check-test-names.sh"
MODE="check"
ROOT="$REPO_ROOT"
usage() { echo "usage: check-test-names.sh [--selftest | --root DIR]" >&2; exit 2; }
case "${1:-}" in
    "") : ;;
    --selftest) MODE="selftest" ;;
    --root) [ $# -ge 2 ] && [ -d "$2" ] || usage; MODE="root"; ROOT="$(cd "$2" && pwd)" ;;
    -h|--help) sed -n '2,55p' "$0"; exit 0 ;;
    *) echo "check-test-names: unknown argument $1" >&2; usage ;;
esac
cd "$ROOT"

exec python3 -I - "$MODE" "$SELF" "$REPO_ROOT" <<'PYEOF'
import collections, glob, os, re, shutil, subprocess, sys, tempfile

mode, SELF, REPO_ROOT = sys.argv[1], sys.argv[2], sys.argv[3]
# Library declarations a harness replaces on purpose: (harness, name) -> why.
ALLOW = {
    ('tests/abuse.tcyr', '_alloc_lock_acquire'):
        "the 3.4.1 INS5 refusal sweep interposes on lib/alloc.cyr's lock hook to refuse the k-th "
        "allocation; every alloc() call reaches the last definition (cyrius guide, 6.6.20)",
    ('tests/abuse.tcyr', '_alloc_lock_release'):
        "its pair: restores the heap end and the grains after the refused allocation",
}
# Read on the 3.5.0 tree: 12,537 top-level names and 1,745 fns; the floors a little under.
# [measured: scripts/check-test-names.sh on the 3.5.0 L8 tree, 2026-11-14]
TOPLEVEL_FLOOR = 12000
FN_FLOOR = 1600
HARNESS_FLOOR = 10      # the eight suites (modules_d since 3.5.0), the benchmark harness and the fuzz harness

IDENT = re.compile(r'[A-Za-z_][A-Za-z0-9_]*$')
TOKEN = re.compile(r'[A-Za-z_][A-Za-z0-9_]*|[0-9][0-9A-Za-z_.]*|\S')
INCLUDE = re.compile(r'^\s*include\s+"([^"]+)"', re.M)

def strip(text):
    """The file with comments and string / character literals blanked, newlines kept."""
    out, i, n, q = [], 0, len(text), None
    while i < n:
        c = text[i]
        if q:
            if c == '\\' and i + 1 < n:
                out.append('  ' if text[i + 1] != '\n' else ' \n'); i += 2; continue
            if c == q: q = None
            out.append('\n' if c == '\n' else ' ')
        elif c == '"' or c == "'":
            q = c; out.append(' ')
        elif c == '#':
            j = text.find('\n', i)
            j = n if j < 0 else j
            out.append(' ' * (j - i)); i = j; continue
        else:
            out.append(c)
        i += 1
    return ''.join(out)

def tokens(code):
    return [(t, no) for no, line in enumerate(code.split('\n'), 1) for t in TOKEN.findall(line)]

def close_of(T, i):
    """Index of the bracket closing T[i]."""
    o = T[i][0]; c = {'(': ')', '[': ']', '{': '}'}[o]
    d = 0
    for j in range(i, len(T)):
        if T[j][0] == o: d += 1
        elif T[j][0] == c:
            d -= 1
            if d == 0: return j
    return len(T) - 1

def stmt_end(T, i):
    """Index of the last token of the top-level statement or declaration starting at T[i]."""
    p = b = 0
    for j in range(i, len(T)):
        t = T[j][0]
        if t in '([': p += 1
        elif t in ')]': p -= 1
        elif t == '{': b += 1
        elif t == '}':
            b -= 1
            if b == 0 and p == 0 and (j + 1 >= len(T) or T[j + 1][0] not in ('else', 'elif', ';', ')', ',', '.')):
                return j
        elif t == ';' and p == 0 and b == 0:
            return j
    return len(T) - 1

class Parsed:
    pass

def parse(path):
    """A file's top level: decls [(kind, name, line, index, in_zone)], fns [(name, line, index,
    names used, locals and parameters)], the line of its first top-level statement, its tokens."""
    raw = open(path, encoding='utf-8', errors='replace').read()
    T = tokens(strip(raw))
    P = Parsed()
    P.decls, P.fns, P.first, P.T = [], [], None, T
    P.includes = INCLUDE.findall('\n'.join(l for l in raw.split('\n') if not l.lstrip().startswith('#')))
    i, n = 0, len(T)
    while i < n:
        if T[i][0] == ';':
            i += 1; continue
        j = i
        while j < n and T[j][0] in ('public', 'private'):
            j += 1
        if j >= n:
            break
        k, zone = T[j][0], P.first is None
        if k == 'include':
            line = T[j][1]
            while j < n and T[j][1] == line: j += 1
            i = j; continue
        if k == 'fn' and j + 1 < n:
            name, m = T[j + 1][0], j + 2
            while m < n and T[m][0] not in ('(', '{', ';'): m += 1
            params = set()
            if m < n and T[m][0] == '(':
                c = close_of(T, m)
                params = {x for x, _ in T[m + 1:c] if IDENT.match(x)}
                m = c + 1
            while m < n and T[m][0] not in ('{', ';'): m += 1
            if m < n and T[m][0] == '{':
                c = close_of(T, m)
                body = T[m + 1:c]
                local = set(params)
                for q in range(len(body)):
                    if body[q][0] == 'var':
                        r = q + 1
                        while r < len(body) and IDENT.match(body[r][0]):
                            local.add(body[r][0])
                            if r + 1 < len(body) and body[r + 1][0] == ',': r += 2
                            else: break
                P.fns.append((name, T[j][1], j, {x for x, _ in body if IDENT.match(x)}, local))
                P.decls.append(('fn', name, T[j][1], j, zone))
                i = c + 1; continue
            i = m + 1; continue
        if k in ('var', 'const'):
            r = j + 1
            while r < n and IDENT.match(T[r][0]):
                P.decls.append((k, T[r][0], T[r][1], r, zone))
                if r + 1 < n and T[r + 1][0] == ',': r += 2
                else: break
            i = stmt_end(T, j) + 1; continue
        if k == 'enum':
            m = j + 1
            while m < n and T[m][0] not in ('{', ';'): m += 1
            if m < n and T[m][0] == '{':
                c, expect, p = close_of(T, m), True, 0
                for q in range(m + 1, c):
                    x = T[q][0]
                    if x in '([{': p += 1
                    elif x in ')]}': p -= 1
                    elif p == 0 and x in (';', ','):
                        expect = True; continue
                    if expect and p == 0 and IDENT.match(x):
                        P.decls.append(('enum', x, T[q][1], q, zone))
                    expect = False
                i = c + 1; continue
            i = m + 1; continue
        if k in ('struct', 'impl', 'union', 'trait'):
            m = j + 1
            while m < n and T[m][0] not in ('{', ';'): m += 1
            i = (close_of(T, m) if m < n and T[m][0] == '{' else m) + 1; continue
        if P.first is None:
            P.first = T[i][1]
        i = stmt_end(T, i) + 1
    return P

def stdlib_leaves():
    try:
        t = open('cyrius.cyml', encoding='utf-8').read()
    except OSError:
        return []
    m = re.search(r'^\[deps\]\s*\nstdlib\s*=\s*\[(.*?)\]', t, re.S | re.M)
    return ['lib/%s.cyr' % x for x in re.findall(r'"([^"]+)"', m.group(1))] if m else []

_lib_cache = {}
def library(P):
    """({name: (kind, 'file:line')} declared at top level in every file the harness compiles,
    [files among them holding a top-level statement])."""
    seen, todo, names, stmts = set(), list(P.includes) + stdlib_leaves(), {}, []
    while todo:
        f = todo.pop()
        if f in seen or not os.path.isfile(f):
            continue
        seen.add(f)
        if f not in _lib_cache:
            _lib_cache[f] = parse(f)
        L = _lib_cache[f]
        if L.first is not None:
            stmts.append('%s:%d' % (f, L.first))
        for kind, nm, no, _, _ in L.decls:
            names.setdefault(nm, (kind, '%s:%d' % (f, no)))
        todo.extend(L.includes)
    return names, stmts

def harnesses():
    return sorted(glob.glob('tests/*.tcyr') + glob.glob('tests/*.bcyr') + glob.glob('tests/*.fcyr'))

def scan():
    errors, stats = [], collections.Counter()
    for h in harnesses():
        P = parse(h)
        lib, lib_stmts = library(P)
        # A statement in an included file would end the zone before this file's own declarations.
        if lib_stmts:
            P.decls = [(k, nm, no, ix, False) for k, nm, no, ix, _ in P.decls]
        stats['names'] += len(P.decls)
        stats['fns'] += len(P.fns)
        stats['zone'] += sum(1 for d in P.decls if d[4])
        if P.first is None:
            errors.append('%s: no top-level statement found: the program never starts, or the scanner lost '
                          'its entry, and the declaration zone cannot be placed' % h)
        fns = collections.defaultdict(list)
        vals = collections.defaultdict(list)
        for d in P.decls:
            (fns if d[0] == 'fn' else vals)[d[1]].append(d)
        # 1. a fn twice
        for nm, ds in sorted(fns.items()):
            if len(ds) > 1:
                errors.append('%s: fn %s declared %d times (lines %s): fns are global, so the last body replaces '
                              'the others for every call, those written before it included'
                              % (h, nm, len(ds), ', '.join(str(d[2]) for d in ds)))
        # 2. a fn and a value
        for nm in sorted(set(fns) & set(vals)):
            errors.append('%s: %s is a fn (line %d) and a %s (line %d): `%s()` calls the fn and `&%s` is the '
                          'value\'s address, so a callback written `&%s` jumps into data'
                          % (h, nm, fns[nm][0][2], vals[nm][0][0], vals[nm][0][2], nm, nm, nm))
        # 3. a value twice in the declaration zone
        for nm, ds in sorted(vals.items()):
            z = [d for d in ds if d[4]]
            if len(z) > 1:
                errors.append('%s: %s declared %d times in the declaration zone (lines %s): there it is ONE global, '
                              'the last definition wins, and a constant redeclaration is its value from program '
                              'start, so a read written before it sees the later value'
                              % (h, nm, len(z), ', '.join(str(d[2]) for d in z)))
        # 4. a fn bound to an earlier declaration, reached after a later one
        callers = collections.defaultdict(set)
        fnames = set(fns)
        for g, _, _, used, _ in P.fns:
            for x in used & fnames:
                callers[x].add(g)
        for nm, ds in sorted(vals.items()):
            if len(ds) < 2:
                continue
            stats['shadowed'] += 1 if any(not d[4] for d in ds) else 0
            for f, fl, fq, used, local in P.fns:
                if nm not in used or nm in local:
                    continue
                bound = [d for d in ds if d[3] < fq]
                later = [d for d in ds if not d[4] and d[3] > fq]
                if not bound or not later:
                    continue
                reach, todo = {f}, [f]
                while todo:
                    for c in callers[todo.pop()]:
                        if c not in reach:
                            reach.add(c); todo.append(c)
                hits = [(t, ln) for t, ln in P.T[later[0][3] + 1:] if t in reach]
                if hits:
                    errors.append('%s:%d: fn %s reads %s declared at line %d, and %s is declared again at line %d; '
                                  'the code after that, line %d (%s), reaches %s, which still reads the earlier '
                                  'variable (a var after the first statement starts a new one for the code after '
                                  'it only)' % (h, fl, f, nm, bound[-1][2], nm, later[0][2], hits[0][1],
                                                hits[0][0], f))
        # 5. a library declaration replaced or shadowed
        for kind, nm, no, _, zone in P.decls:
            if nm not in lib or (h, nm) in ALLOW:
                continue
            lk, where = lib[nm]
            if (kind == 'fn') != (lk == 'fn'):
                why = ('shares its name with the library\'s %s at %s: a call reaches the fn and `&%s` is the '
                       'value\'s address' % (lk, where, nm))
            elif kind == 'fn':
                why = ('replaces the library\'s fn at %s for every call in the program, the library\'s own '
                       'included' % where)
            elif zone:
                why = ('is the library\'s global at %s: in the declaration zone one name is one global, so the '
                       'library reads this file\'s value' % where)
            else:
                why = ('shadows the library\'s %s at %s for the code after it, while the library\'s fns keep '
                       'their own: a write to it there never reaches the library' % (lk, where))
            errors.append('%s:%d: top-level %s %s %s' % (h, no, kind, nm, why))
        for (ah, nm), why in ALLOW.items():
            if ah == h and not (any(d[1] == nm for d in P.decls) and nm in lib):
                errors.append('%s: ALLOW names %s, which no longer replaces a library declaration: delete its '
                              'entry' % (h, nm))
    return errors, stats

if mode == 'selftest':
    bad = 0
    # Each plant is a small tree: tests/t.tcyr (and, where it matters, a src/ module it includes).
    # (name, files, want_rc): want_rc 1 = the gate must fail, 0 = it must pass.
    S = 'alloc_init();\n'      # a first statement
    PL = [
        ('clean harness', {'tests/t.tcyr': 'var _t_a = 1;\nfn _t_f() { return _t_a; }\n' + S}, 0),
        ('no first statement', {'tests/t.tcyr': 'var _t_a = 1;\n'}, 1),
        ('fn twice', {'tests/t.tcyr': 'fn _t_f() { return 1; }\nfn _t_f() { return 2; }\n' + S}, 1),
        ('fn twice, after the statement', {'tests/t.tcyr': S + 'fn _t_f() { return 1; }\nfn _t_f() { return 2; }\n'}, 1),
        ('fn twice in a .bcyr', {'tests/t.bcyr': 'fn bench_x() { return 1; }\nfn bench_x() { return 2; }\n' + S}, 1),
        ('fn twice in a .fcyr', {'tests/t.fcyr': S + 'fn _t_f() { return 1; }\npublic fn _t_f() { return 2; }\n'}, 1),
        ('fn twice on one line', {'tests/t.tcyr': S + '_t_x(); fn _t_f() { return 1; } fn _t_f() { return 2; }\n'}, 1),
        ('fn and var', {'tests/t.tcyr': 'var _t_g = 1;\nfn _t_g() { return 2; }\n' + S}, 1),
        ('fn and var, after the statement', {'tests/t.tcyr': S + 'var _t_g = 1;\nfn _t_g() { return 2; }\n'}, 1),
        ('enum member and var', {'tests/t.tcyr': 'enum _TE {\n    _T_E1 = 1;\n}\nvar _T_E1 = 2;\n' + S}, 1),
        # the declaration zone
        ('zone: var twice', {'tests/t.tcyr': 'var _t_a = 1;\nvar _t_b = _t_a;\nvar _t_a = 2;\n' + S}, 1),
        ('zone: a tuple name repeated', {'tests/t.tcyr': 'var _t_r, _t_v = f(1);\nvar _t_q, _t_v = f(2);\n' + S}, 1),
        ('zone: an array, then a scalar', {'tests/t.tcyr': 'var _t_n: i64[9];\nvar _t_n = 0;\n' + S}, 1),
        ('zone: twice on one line', {'tests/t.tcyr': 'var _t_a = 1; var _t_a = 2;\n' + S}, 1),
        ('zone: a const twice', {'tests/t.tcyr': 'const _T_C = 1;\nconst _T_C = 2;\n' + S}, 1),
        # after the first statement: shadowing, a defect only where an earlier fn is reached
        ('after: var twice, no fn (shadowing)', {'tests/t.tcyr': S + 'var _t_a = 1;\nvar _t_a = 2;\n'}, 0),
        ('after: a tuple name reused (shadowing)', {'tests/t.tcyr': S + 'var _t_r, _t_v = f(1);\nvar _t_r, _t_v = f(2);\n'}, 0),
        ('after: an earlier fn reached after the redeclaration',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nfn _t_rd() { return _t_a; }\nvar _t_a = 7;\nvar _t_x = _t_rd();\n'}, 1),
        ('after: reached through a caller',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nfn _t_rd() { return _t_a; }\nfn _t_up() { return _t_rd() + 1; }\n'
                              'var _t_a = 7;\n_t_up();\n'}, 1),
        ('after: reached by a fn defined after the redeclaration',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nfn _t_rd() { return _t_a; }\nvar _t_a = 7;\n'
                              'fn _t_late() { return _t_rd(); }\n'}, 1),
        ('after: written through &fn after the redeclaration',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nfn _t_wr(v) { _t_a = v; return 0; }\nvar _t_a = 7;\nfncall1(&_t_wr, 3);\n'}, 1),
        ('after: the first declaration in the zone',
         {'tests/t.tcyr': 'var _t_a = 5;\nfn _t_rd() { return _t_a; }\n' + S + 'var _t_a = 7;\n_t_rd();\n'}, 1),
        ('after: all on one line',
         {'tests/t.tcyr': S + '_t_x(); var _t_a = 5; fn _t_rd() { return _t_a; } var _t_a = 7; _t_rd();\n'}, 1),
        ('after: the earlier fn called only before the redeclaration',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nfn _t_rd() { return _t_a; }\n_t_rd();\nvar _t_a = 7;\n'}, 0),
        ('after: the fn defined after both declarations',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nvar _t_a = 7;\nfn _t_rd() { return _t_a; }\n_t_rd();\n'}, 0),
        ('after: the fn\'s own local of that name',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nfn _t_rd() { var _t_a = 1; return _t_a; }\nvar _t_a = 7;\n_t_rd();\n'}, 0),
        ('after: the fn\'s parameter of that name',
         {'tests/t.tcyr': S + 'var _t_a = 5;\nfn _t_rd(_t_a) { return _t_a; }\nvar _t_a = 7;\n_t_rd(1);\n'}, 0),
        # libraries
        ('replaces an included fn', {'src/m.cyr': 'public fn m_f() { return 1; }\n',
                                     'tests/t.tcyr': 'include "src/m.cyr"\nfn m_f() { return 2; }\n' + S}, 1,
         'replaces the library\'s fn'),
        ('an included global, in the zone', {'src/m.cyr': 'public var M_K = 1;\n',
                                             'tests/t.tcyr': 'include "src/m.cyr"\nvar M_K = 2;\n' + S}, 1,
         'is the library\'s global'),
        ('an included global, after the statement', {'src/m.cyr': 'public var M_K = 1;\n',
                                                     'tests/t.tcyr': 'include "src/m.cyr"\n' + S + 'var M_K = 2;\n'}, 1,
         'shadows the library\'s var'),
        ('an included global, after a statement on its line',
         {'src/m.cyr': 'public var M_K = 1;\n', 'tests/t.tcyr': 'include "src/m.cyr"\n' + S + '_t_x(); var M_K = 2;\n'}, 1),
        ('a var named like an included fn', {'src/m.cyr': 'public fn m_f() { return 1; }\n',
                                             'tests/t.tcyr': 'include "src/m.cyr"\n' + S + 'var m_f = 2;\n'}, 1,
         'shares its name with the library\'s fn'),
        ('replaces a transitively included fn', {'src/m.cyr': 'include "src/n.cyr"\n',
                                                 'src/n.cyr': 'fn n_f() { return 1; }\n',
                                                 'tests/t.tcyr': 'include "src/m.cyr"\nfn n_f() { return 2; }\n' + S}, 1),
        ('replaces a stdlib leaf fn', {'cyrius.cyml': '[deps]\nstdlib = [\n    "alloc",\n]\n',
                                       'lib/alloc.cyr': 'fn alloc(n) { return 0; }\n',
                                       'tests/t.tcyr': 'fn alloc(n) { return 8; }\n' + S}, 1),
        ('an included enum member', {'src/m.cyr': 'enum M { M_A = 1; M_B = 2; }\n',
                                     'tests/t.tcyr': 'include "src/m.cyr"\n' + S + 'var M_B = 2;\n'}, 1),
        # not collisions
        ('a local of the same name',
         {'tests/t.tcyr': 'var _t_a = 1;\nfn _t_f() {\n    var _t_a = 2;\n    return _t_a;\n}\n' + S}, 0),
        ('a top-level block\'s var', {'tests/t.tcyr': 'var _t_a = 1;\n' + S + 'if (_t_a == 1) {\n    var _t_a = 2;\n}\n'}, 0),
        ('loop headers',
         {'tests/t.tcyr': S + 'for (var i = 0; i < 2; i = i + 1) { }\nfor (var i = 0; i < 2; i = i + 1) { }\n'}, 0),
        ('in a comment', {'tests/t.tcyr': 'var _t_a = 1;\n# var _t_a = 2;\n# fn _t_a() {}\n' + S}, 0),
        ('in a string', {'tests/t.tcyr': 'var _t_a = 1;\nvar _t_s = "\nvar _t_a = 2;\n";\n' + S}, 0),
        ('two harnesses', {'tests/t.tcyr': 'var _t_a = 1;\n' + S, 'tests/u.tcyr': 'var _t_a = 1;\n' + S}, 0),
        ('a module not included', {'src/m.cyr': 'public fn m_f() { return 1; }\n',
                                   'tests/t.tcyr': 'fn m_f() { return 2; }\n' + S}, 0),
        ('a brace in a string', {'tests/t.tcyr': 'fn _t_f() {\n    var s = "{";\n    return s;\n}\nvar _t_g = 1;\n'
                                                 'var _t_g = 2;\n' + S}, 1),
    ]
    for name, files, want, *text in PL:
        with tempfile.TemporaryDirectory() as d:
            for p, body in files.items():
                os.makedirs(os.path.join(d, os.path.dirname(p)), exist_ok=True)
                open(os.path.join(d, p), 'w').write(body)
            os.makedirs(os.path.join(d, 'tests'), exist_ok=True)
            r = subprocess.run(['bash', SELF, '--root', d], capture_output=True, text=True)
        if (r.returncode != 0) != bool(want) or (text and text[0] not in r.stdout):
            print('  selftest: %s: rc %d, want %s%s' % (name, r.returncode, 'a failure' if want else 'a pass',
                                                     ' saying %r' % text[0] if text else ''))
            print('    ' + r.stdout.strip().replace('\n', '\n    '))
            bad += 1
    # ALLOW and the floors hold on the real tree only, so they are tested on a copy of it.
    def tree_rc(edit):
        with tempfile.TemporaryDirectory() as d:
            for sub in ('src', 'tests', 'lib'):
                shutil.copytree(os.path.join(REPO_ROOT, sub), os.path.join(d, sub))
            os.makedirs(os.path.join(d, 'scripts'))
            shutil.copy(SELF, os.path.join(d, 'scripts/check-test-names.sh'))
            shutil.copy(os.path.join(REPO_ROOT, 'cyrius.cyml'), d)
            edit(d)
            return subprocess.run(['bash', os.path.join(d, 'scripts/check-test-names.sh')],
                                  capture_output=True, text=True).returncode
    def append(f, text):
        def e(d):
            with open(os.path.join(d, f), 'a') as fh: fh.write(text)
        return e
    def insert_before(f, anchor, text):
        def e(d):
            p = os.path.join(d, f)
            t = open(p).read()
            if t.count(anchor) != 1: raise SystemExit('  selftest: anchor %r gone from %s; update this arm' % (anchor, f))
            open(p, 'w').write(t.replace(anchor, text + anchor, 1))
        return e
    def drop_allow_site(d):
        p = os.path.join(d, 'tests/abuse.tcyr')
        t = open(p).read()
        t2 = t.replace('fn _alloc_lock_release(): i64 {', 'fn _al_lock_release_gone(): i64 {', 1)
        if t2 == t: raise SystemExit('  selftest: the ALLOW site to drop is gone; update this arm')
        open(p, 'w').write(t2)
    TREE = [
        ('the tree as it stands', lambda d: None, 0),
        # The roadmap's plant: a second `var _c41_s0` appended to modules_c. After the first
        # statement it is a new variable for the code after it, and no earlier fn reads the name,
        # so it changes nothing (review: the earlier read kept 2204 passing) and passes.
        ('the planted _c41_s0 alone (shadowing)', append('tests/modules_c.tcyr', '\nvar _c41_s0 = 12345;\n'), 0),
        ('the planted _c41_s0 behind an earlier fn',
         append('tests/modules_c.tcyr', '\nfn _l8_rd() { return _c41_s0; }\nvar _c41_s0 = 12345;\n'
                                        'var _l8_u = _l8_rd();\n'), 1),
        ('a same-line declaration behind an earlier fn (foundation.tcyr:3593)',
         append('tests/foundation.tcyr', '\nfn _l8_rd() { return _fb_x1t; }\n_fb_exhaust_after(0); var _fb_x1t = 0;\n'
                                         'var _l8_u = _l8_rd();\n'), 1),
        ('a new duplicate fn in hisab.bcyr', append('tests/hisab.bcyr', '\nfn bench_vec3_add() { return 0; }\n'), 1),
        ('a zone repeat in hisab.bcyr', insert_before('tests/hisab.bcyr', 'var r = main();\n', 'var _b341_t = 7;\n'), 1),
        ('a stdlib global in the zone', insert_before('tests/hisab.tcyr', 'alloc_init();\n', 'var ALLOC_MAX = 0;\n'), 1),
        ('a stdlib global after a statement on its line',
         append('tests/hisab.tcyr', '\nalloc_init(); var ALLOC_MAX = 0;\n'), 1),
        ('a src fn replaced', append('tests/foundation.tcyr', '\nfn hvec3_add(a, b) { return a; }\n'), 1),
        ('a fn and a var', append('tests/modules.tcyr', '\nvar _l8_fv = 1;\nfn _l8_fv() { return 2; }\n'), 1),
        ('an ALLOW entry stale', drop_allow_site, 1),
        ('a harness lost (the floor)', lambda d: os.remove(os.path.join(d, 'tests/hisab.fcyr')), 1),
    ]
    for name, edit, want in TREE:
        rc = tree_rc(edit)
        if (rc != 0) != bool(want):
            print('  selftest: tree copy, %s: rc %d, want %s' % (name, rc, 'a failure' if want else 'a pass'))
            bad += 1
    # The language rules the checks rest on, compiled under the manifest pin from the repo root:
    # if a toolchain bump changes one, this fails and the model above is re-derived.
    # (rule, program, expected exit status)
    LANG = [
        ('a zone constant redeclaration is the value from program start',
         'var _p_k = 5;\nvar _p_s = _p_k;\nvar _p_k = 7;\nalloc_init();\nsys_exit_group(_p_s);\n', 7),
        ('after the first statement an earlier fn keeps the earlier var',
         'alloc_init();\nvar _p_k = 5;\nfn _p_rd() { return _p_k; }\nvar _p_k = 7;\nsys_exit_group(_p_rd() * 10 + _p_k);\n', 57),
        ('the same, all on one line',
         'alloc_init(); var _p_k = 5; fn _p_rd() { return _p_k; } var _p_k = 7;\nsys_exit_group(_p_rd() * 10 + _p_k);\n', 57),
        ('a duplicate fn replaces the first for calls written before it',
         'fn _p_f() { return 1; }\nalloc_init();\nvar _p_a = _p_f();\nfn _p_f() { return 2; }\nsys_exit_group(_p_a);\n', 2),
        ('a fn and a var share a name: the call reaches the fn, & the var',
         'var _p_g = 1;\nfn _p_g() { return 2; }\nalloc_init();\nsys_exit_group(_p_g() * 10 + load64(&_p_g));\n', 21),
        ('a zone var with a library name is the library\'s global',
         'var ALLOC_MAX = 0;\nalloc_init();\nsys_exit_group(alloc(64) != 0);\n', 0),
        ('a var with a library name after the first statement shadows it',
         'alloc_init();\nvar ALLOC_MAX = 0;\nsys_exit_group((alloc(64) != 0) * 10 + (ALLOC_MAX == 0));\n', 11),
        ('a test fn with a library fn\'s name is the one the library calls',
         'var _p_n = 0;\nfn _alloc_lock_acquire(): i64 { _p_n = _p_n + 1; return 0; }\nalloc_init();\n'
         'alloc(8);\nalloc(8);\nsys_exit_group(_p_n);\n', 2),
    ]
    if shutil.which('cyrius') is None:
        print('  selftest: language rules: FAIL, no `cyrius` on PATH to compile them with')
        bad += 1
    else:
        with tempfile.TemporaryDirectory() as d:
            for k, (rule, prog, want) in enumerate(LANG):
                src, out = os.path.join(d, 'r%d.cyr' % k), os.path.join(d, 'r%d' % k)
                open(src, 'w').write(prog)
                b = subprocess.run(['cyrius', 'build', src, out], cwd=REPO_ROOT, capture_output=True, text=True)
                if b.returncode != 0 or not os.path.exists(out):
                    print('  selftest: language rule "%s": does not build: %s' % (rule, (b.stdout + b.stderr)[-300:]))
                    bad += 1
                    continue
                rc = subprocess.run([out], capture_output=True).returncode
                if rc != want:
                    print('  selftest: language rule "%s": exit %d, the model says %d' % (rule, rc, want))
                    bad += 1
    n = len(PL) + len(TREE) + len(LANG)
    print('  selftest: %d verdicts (%d planted trees, %d tree copies, %d language rules), %d wrong: %s'
          % (n, len(PL), len(TREE), len(LANG), bad, 'PASS' if bad == 0 else 'FAIL'))
    sys.exit(1 if bad else 0)

errors, stats = scan()
nh = len(harnesses())
print('=== test names: %d harnesses, %d top-level names (%d in declaration zones), %d fns, %d names redeclared '
      'after a first statement (shadowing), %d deliberate library replacements ===' %
      (nh, stats['names'], stats['zone'], stats['fns'], stats['shadowed'], len(ALLOW)))
if mode == 'check':
    if nh < HARNESS_FLOOR:
        errors.append('only %d harnesses in tests/, floor %d: the glob lost some' % (nh, HARNESS_FLOOR))
    if stats['names'] < TOPLEVEL_FLOOR:
        errors.append('only %d top-level names read, floor %d: the scanner lost a shape' % (stats['names'], TOPLEVEL_FLOOR))
    if stats['fns'] < FN_FLOOR:
        errors.append('only %d fns read, floor %d: the scanner lost a shape' % (stats['fns'], FN_FLOOR))
if errors:
    print('\n!! %d PROBLEM(S)\n' % len(errors))
    for e in errors:
        print('  ' + e)
    print('\nRename the later declaration, or give the fn the name it means. scripts/check-test-names.sh says')
    print('what a repeated name does where it is (the declaration zone merges, later code shadows).')
    sys.exit(1)
print('Every top-level name in tests/ means one thing where it is read, or is a listed replacement.')
PYEOF
