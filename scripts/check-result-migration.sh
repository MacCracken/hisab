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
# `return Err(` -- or, since 3.3.1, when it only FORWARDS one: it tail-returns a
# migrated call (`return f(...)`) or applies `?` (to anything, since 3.4.1: Cyrius has
# no `?:`, so every `?` propagates). That set is computed to a fixed point. For each, every call site outside its own definition must bind
# both halves (`var t, v = f(...)`) or propagate (`f(...)?`) — never sit in an
# argument position.
# ⛔ Through 3.3.0 the forwarding half was missing: `ad_grad_into` returns a Result
# purely through `ad_grad(...)?` and a tail call, so the gate counted 50 functions,
# never saw it, and its three argument-position test sites read only the tag.
#
# ⛔ 3.4.1 (audit F-R1) -- THREE FORMS PASSED THAT COMPILE CLEAN AND READ ONLY THE TAG.
# With `rf(x)` = `Ok(x + 100)`, each built and ran under cycc 6.7.9 and 6.7.18:
#   (1) `two(rf(3), rf(5)?)` = 105 -- the first argument got tag 0. The exemption
#       looked for `)?` ANYWHERE later on the line, so the second call's `?`
#       excused the first. Each call is now matched to ITS OWN closing paren
#       (balanced, across lines), and only a `?` right after that paren propagates.
#   (2) `return rf(3) == 0;` returned 1 -- taken as a tail return because the line
#       read `return rf(`. A tail return now needs the call to BE the returned
#       expression: its own closing paren (or the parens that only group it), then `;`.
#   (3) `fncall1(&rf, 3)` read the tag -- a Result function taken by address.
# The stdlib's Result functions (lib/io.cyr's `file_*_r`, lib/result.cyr's `ok_via` /
# `err_via`) and the Result helpers that tests/ and examples/ define seed the set too.
#
# ⛔ 3.4.1 -- THE CONTRACT IS EVERY SUPPORTED PIN, NOT THE PINNED ONE. hisab's src/ is
# compiled by consumers under their OWN cycc, down to 6.6.3 (CLAUDE.md, Consumers), so a
# form is an offence when it keeps both halves on 6.7.18 but not on some pin >= 6.6.3. The
# same rule holds in tests/ and examples/, so that nothing in the tree models a form src/
# must not use. Each row below was built and run from scratch dirs pinned to 6.6.3 and
# 6.7.18, with rf = Ok(x + 100) and ef = Err(x + 200):
#     form                                    6.6.3                    6.7.18    gate
#     var t, v = f(x);  (one line or split)   both halves              both      ok
#     case 1: var t, v = f(x);                both halves              both      ok
#     for (var t, v = f(x); ..; ..)           both halves              both      ok
#     f(x)?  (also inside an argument)        propagates               same      ok
#     return f(x);  return (f(x));            forwards the pair        same      ok
#     is_ok(f()), is_ok((f())), is_err_result reads the tag, by design same      ok
#     is_tag(f(), x)  (f() the whole arg 1)   reads the tag, by design same      ok
#     x &f(x)?   x &&f(x)?                    value 7, 1               same      ok
#     (f(x))?                                 SIGSEGV                  103       flagged
#     var t, v = (f(x));                      compile error            both      flagged
#     var t: (i64, i64) = f(x);               compile error            both      flagged
#     t, v = f(x);  (also `case 1: t, v = ..`) compile error           both      flagged
#     fncall1(&rf, 4)?                        SIGSEGV                  104       flagged
#     var t, v = callptr(&rf, 4);             compile error            both      flagged
#     var t, v = fncall1(&rf, 4);             both, by accident        both      flagged
# From cyrius 6.7.17 (decision D26) `fncallN(&f, ..)` and `callptr(&f, ..)` with the
# callee written `&f` are the direct call's pair. Before it they hand back rax, and the
# 6.6.3 bind above reads a payload only because rdx survives fncall's epilogue, which no
# version promises; a callee held in a variable or passed into a parameter is one word on
# every pin. So every `&f` of a Result function is an offence. (The first 3.4.1 form of
# this header said "no indirect call carries rdx"; that is false on 6.7.17 and later.)
# [measured: scratch forms probes, dirs pinned to cycc 6.6.3 and 6.7.18, lane INS3, 2026-10-31]
# The case-label bind, the for-init bind and is_tag were built on 6.6.3, 6.6.10, 6.6.20,
# 6.7.16 and 6.7.18 with the Err twin beside each: `0 103 / 1 203` for both binds on every
# pin, and is_tag(rf(3), 0) / (rf(3), 1) / (ef(3), 1) / (ef(3), 0) = 1 0 1 0 on every pin,
# with a call as its second argument too (3.4.1 third review, N1).
# [measured: lane INS4 forms_k probes, dirs pinned to cycc 6.6.3 6.6.10 6.6.20 6.7.16 6.7.18, 2026-10-31]
#
# ⛔ 3.4.1 THIRD REVIEW -- A CLOSURE THAT RETURNS A RESULT IS A TAG READ ON EVERY PIN. A
# closure is called through a variable (`fncallN(g, ..)`, `callptr(g, ..)`), and a callee
# held in a variable returns one word. Built on 6.6.3, 6.6.10, 6.6.20, 6.7.16 and 6.7.18,
# with g = |x| { return rf(x); } and its Err twin h:
#     form                                    every pin (6.6.3 inside a fn)          gate
#     pr(fncall1(g, 3)), pr(fncall1(h, 3))    0, 1: the tag alone                    flagged
#     var t, v = fncall1(h, 3);               1 203, by the same accident as &f      flagged
#     var t, v = callptr(h, 3);               compile error through 6.6.20           flagged
#     fncall1(g, 3)?   (and through p = &ef)  SIGSEGV on every pin                   flagged
#     is_ok(fncall1(g, 3))                    1, 0: the tag, by design               ok
# The same holds for `|x| rf(x)`, for `|x| { return Ok(..); }`, and for a closure that
# applies `?` (`|x| { var r = ef(x)?; return r + 1; }` hands its caller 1, the Err tag, as
# if it were a value). On 6.6.3 a block-bodied closure in a top-level `var` does not run at
# all (the 6.6.6 fix), and on 6.6.3 / 6.6.10 its `return` is booked against the enclosing
# fn. So a closure that returns a Result -- by a tail return of a Result call, `return
# Ok(..)` / `Err(..)`, an expression body that is one, or any `?` -- is an offence where it
# is written (`result-closure`), and is no part of the fn around it. A variable bound to
# one, or to `&f` of a Result fn, or to a fn that returns one, is a Result callable, and
# every indirect call through it is judged by the table above: argument position and a
# single bind as for a direct call, and the bind, the `?` and the tail return, which no
# pin carries, as `indirect-bind`, `indirect-propagation` and `indirect-tail-return`.
# [measured: lane INS4 forms_k probes K1-K17, dirs pinned to cycc 6.6.3 6.6.10 6.6.20 6.7.16 6.7.18, 2026-10-31]
#
# How the scan reads code: the WHOLE file is lexed, so a string may span lines (a raw
# newline or a `\`-newline); string and char literals are blanked ('"' and '#' are char
# literals, not a string opener or a comment); a comment is blanked from its `#` to the end
# of the line; an attribute (`#must_use`, `#inline`, `#derive(..)`, ..) that leads a line
# is blanked and the rest of the line is code; an `include` line and a `#define` line end
# a statement; and a `#define` body is code, so a Result call inside a macro is an offence
# where it is written. A call is judged by the statement it sits in, read back to the last
# `;`, `{` or `}` (or to the start of the closure body it sits in). A fn name defined more
# than once (one per test suite, say) is a Result fn if ANY definition is one, and a name
# with both a Result and a plain definition is reported (`mixed-definition`).
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

python3 - "${1:-}" <<'PY'
import re, sys, glob, os

selftest = (len(sys.argv) > 1 and sys.argv[1] == "--selftest")

ID = r'[A-Za-z_][A-Za-z_0-9]*'
# The attributes that lead a declaration (scripts/check-doc-blocks.sh's list). A run of
# them is blanked only when a declaration, a comment or nothing follows it on the line;
# anything else (`#must_use results ...`) is prose and the line stays a comment. `#assert`
# is a line of its own and stays a comment too.
ATTR = re.compile(r'[ \t]*(?:(?:#(?:inline|naked|pure|io|alloc|must_use|regalloc|deprecated|pe_import)'
                  r'(?![A-Za-z0-9_])(?:\([^)\n]*\))?|#derive\([^)\n]*\))[ \t]*)+'
                  r'(?=$|#|(?:public\s+)?(?:fn|async|extern|struct|enum|var|const|impl|union|trait)(?![A-Za-z0-9_]))')
CHARLIT = re.compile(r"'(?:\\.|[^\\'\n])'")

def lex(src):
    """The whole file, column- and line-preserving, with string and char literal contents,
    comments and leading attributes blanked; an `include` or `#define` line ends with a `;`
    the scan can see. A string stays open across a line end (3.4.1 third review, S2a)."""
    out = []
    in_str, esc = False, False
    for line in src.splitlines(True):
        nl = "\n" if line.endswith("\n") else ""
        body = line[:-1] if nl else line
        n, i, term, res = len(body), 0, "", []
        if not in_str:
            if re.match(r'\s*include\s+"', body):
                out.append(" " * n + ";" + nl)
                continue
            a = ATTR.match(body)
            if a:
                res.append(" " * a.end())
                i = a.end()
            d = re.match(r'(\s*)#define(?![A-Za-z0-9_])', body[i:])
            if d:
                res.append(d.group(1) + " " * 7)
                i += d.end()
                term = ";"
        while i < n:
            c = body[i]
            if in_str:
                if esc:
                    esc = False
                    res.append(" ")
                elif c == "\\":
                    esc = True
                    res.append(" ")
                elif c == '"':
                    in_str = False
                    res.append(c)
                else:
                    res.append(" ")
            elif c == '"':
                in_str = True
                res.append(c)
            elif c == "'" and CHARLIT.match(body, i):
                # A char literal: '"' and '#' would otherwise open a string or a comment.
                w = 4 if body[i + 1] == "\\" else 3
                res.append("'" + " " * (w - 2) + "'")
                i += w
                continue
            elif c == "#":
                # ⚠ Skip comments, trailing ones too. Without this the gate reports every
                # prose mention of a migrated function -- 12 of the first 17 hits were
                # comment lines, and a gate people learn to ignore is not a gate.
                res.append(" " * (n - i))
                break
            else:
                res.append(c)
            i += 1
        if in_str and esc and nl:
            esc = False                       # `\`-newline: the newline was the escaped char
        out.append("".join(res) + term + nl)
    return "".join(out)

def code_of_file(f):
    src = open(f, errors="replace").read()
    return src.splitlines(True), lex(src)

def close_paren(text, i, op="(", cl=")"):
    """Index of the closer matching the opener at text[i], or -1. Literals are blanked."""
    depth = 0
    for j in range(i, len(text)):
        c = text[j]
        if c == op:
            depth += 1
        elif c == cl:
            depth -= 1
            if depth == 0:
                return j
    return -1

CLOSURE_HEAD = re.compile(r'\|\s*(?:' + ID + r'(?:\s*,\s*' + ID + r')*)?\s*\|')

def closures(text):
    """Every closure literal: (start, body_start, body_end, form). A `|` opens one only where
    an operand cannot stand before it -- after `=`, `(`, `,`, `[`, `{`, `;` or `return` --
    so `a | b` and `a || b` are operators. A block body runs from its `{` to the matching
    `}`; an expression body to the first `;` or `,`, or the unmatched closer, at depth 0."""
    res, i = [], 0
    while True:
        i = text.find("|", i)
        if i < 0:
            return res
        k = i - 1
        while k >= 0 and text[k] in " \t\n":
            k -= 1
        prev = text[k] if k >= 0 else ";"
        opens = prev in "(,[{;" \
            or (prev == "=" and (k == 0 or text[k - 1] not in "=!<>")) \
            or bool(re.search(r'(?<![A-Za-z_0-9])return$', text[max(0, k - 6):k + 1]))
        h = CLOSURE_HEAD.match(text, i) if opens else None
        if not h:
            i += 1
            continue
        j = h.end()
        while j < len(text) and text[j] in " \t\n":
            j += 1
        if j < len(text) and text[j] == "{":
            e = close_paren(text, j, "{", "}")
            if e > j:
                res.append((i, j, e, "block"))
        else:
            e, d = j, 0
            while e < len(text):
                c = text[e]
                if c in "([{":
                    d += 1
                elif c in ")]}":
                    if d == 0:
                        break
                    d -= 1
                elif c in ";," and d == 0:
                    break
                e += 1
            res.append((i, j, e, "expr"))
        i = h.end()

def blank(text, spans, lo=0, hi=None):
    """text[lo:hi] with each [a, b] span (whole-text positions) blanked, newlines kept."""
    hi = len(text) if hi is None else hi
    seg = text[lo:hi]
    if not spans:
        return seg
    t = list(seg)
    for a, b in spans:
        for p in range(max(a, lo), min(b + 1, hi)):
            if t[p - lo] != "\n":
                t[p - lo] = " "
    return "".join(t)

SEED_GLOBS = ["lib/*.cyr", "src/*.cyr", "tests/*", "examples/*.cyr"]
DEF = re.compile(r'(?m)^[ \t]*(?:public\s+)?fn\s+(' + ID + r')\s*\(')
OKERR = re.compile(r'return\s*(?:\(\s*)*(Ok|Err)\s*\(')
LEXED = {}      # file -> (raw lines, lexed text, closures)

def lexed(f):
    if f not in LEXED:
        raw, text = code_of_file(f)
        LEXED[f] = (raw, text, closures(text))
    return LEXED[f]

def definitions(extra_bodies=None):
    """Every fn definition: (file, name, line, own body). The own body is the def's braces on
    the lexed text with every closure literal in it blanked, so neither top-level code nor a
    closure's `return` joins it (3.4.1 third review, S1: a closure's `return f(..)` made the
    fn around it a forwarder). extra_bodies (selftest only): name -> body, read as src/."""
    defs = []
    for g in SEED_GLOBS:
        for f in sorted(glob.glob(g)):
            if os.path.isdir(f):
                continue
            _, text, cls = lexed(f)
            for m in DEF.finditer(text):
                ob = text.find("{", m.end())
                cb = close_paren(text, ob, "{", "}") if ob >= 0 else -1
                if cb > ob >= 0:
                    inner = [(a, e) for a, _, e, _ in cls if ob < a < cb]
                    body = blank(text, inner, ob, cb + 1)
                    defs.append((f, m.group(1), text.count("\n", 0, m.start(1)) + 1, body))
    for name, txt in (extra_bodies or {}).items():
        t = lex(txt)
        cls = closures(t)
        defs.append(("<selftest>", name, 1, blank(t, [(a, e) for a, _, e, _ in cls])))
    return defs

def migrated_fns(defs):
    """name -> file for every fn that returns a Result, directly or by forwarding. A name
    defined more than once is in the set when ANY of its definitions is (3.4.1 third
    review, S2c: the last definition read used to win).
    ⛔ 3.4.1 review: through the first 3.4.1 forms only lib/ and src/ seeded this, so
    tests/hisab.tcyr's helpers that `return cqr_decompose(..)` were outside it and 17
    argument-position sites read only the tag; tests/ and examples/ seed it now."""
    res = set(i for i, d in enumerate(defs) if OKERR.search(d[3]))
    # Forwarders, to a fixed point: `return g(` or `return (g(` with g already a Result fn
    # makes the enclosing fn one too, and so does any `?` in its own body -- Cyrius has no
    # `?:`, so every `?` propagates an Err pair out of the fn it sits in (whatever it is
    # applied to; through the third 3.4.1 review only `g(..)?` with g in the set counted).
    # ad_grad_into is the case 3.3.1 found; a grouped `return (g(x));` forwards the pair as
    # well (6.6.3 and 6.7.18 alike), and through INS2 it went unseen (3.4.1 second review, SF1).
    res.update(i for i, d in enumerate(defs) if "?" in d[3])
    while True:
        names = {defs[i][1] for i in res}
        if not names:
            break
        alt = "|".join(sorted(map(re.escape, names), key=len, reverse=True))
        tail = re.compile(r'return\s*(?:\(\s*)*(?:' + alt + r')\s*\(')
        added = [i for i, d in enumerate(defs) if i not in res and tail.search(d[3])]
        if not added:
            break
        res.update(added)
    fns = {}
    for i in sorted(res):
        fns.setdefault(defs[i][1], defs[i][0])
    return fns, res

def mixed(defs, ridx):
    """The plain definitions of a name that also has a Result one: (file, line, name)."""
    names = {defs[i][1] for i in ridx}
    return [(d[0], d[2], d[1]) for i, d in enumerate(defs)
            if d[1] in names and i not in ridx and d[0] != "<selftest>"]

SITES = [0]   # call sites and pointer takes examined, for the population floor
PFX = r'\s*(?:(?:case\s+[^:;]+|default)\s*:\s*|for\s*\(\s*)?'
BIND = re.compile(PFX + r'var\s+' + ID + r'\s*,\s*' + ID + r'\s*=\s*')
PAIR = re.compile(r'\s*(?:(?:case\s+[^:;]+|default)\s*:\s*)?' + ID + r'\s*,\s*' + ID + r'\s*=\s*')
TUPLE = re.compile(r'\s*var\s+' + ID + r'\s*:\s*\(\s*i64\s*,\s*i64\s*\)\s*=\s*')
INDIRECT = re.compile(r'(?<![A-Za-z_0-9])(?:fncall[0-8]|callptr)\s*(\()\s*((?:\(\s*)*)(' + ID + r')\s*')

def judge(text, st, start, op, indirect):
    """The verdict on the call whose name starts at `start` and whose argument list opens at
    `op`, in the statement that starts at `st`: (kind or None, how). `how` names the form an
    accepted call takes ('prop', 'bind', 'tail', 'tag'), and 'whole' when it is all of the
    statement's expression -- an expression closure's body."""
    before = text[st:start]
    j = close_paren(text, op)
    if j < 0:
        return "unbalanced-call", None
    # `f(...)?` propagates both halves -- THIS call's own `?`, on every pin.
    if text[j + 1:].lstrip().startswith("?"):
        return ("indirect-propagation" if indirect else None), "prop"
    if BIND.fullmatch(before):
        return ("indirect-bind" if indirect else None), "bind"
    # Parens that only group the call: opened by a `(` that no name, `)` or `]`
    # precedes (a call's or an index's paren, or `if` / `while` syntax), `return`
    # aside. Count them, strip them from `before`, step over as many `)`.
    b = before.rstrip()
    ngroup = 0
    while b.endswith("("):
        prev = b[:-1].rstrip()
        if prev and (prev[-1].isalnum() or prev[-1] in "_)]") \
                and not re.search(r'(?<![A-Za-z_0-9])return$', prev):
            break
        ngroup += 1
        b = prev
    k = j + 1
    for _ in range(ngroup):
        t = k
        while t < len(text) and text[t].isspace():
            t += 1
        if t < len(text) and text[t] == ")":
            k = t + 1
        else:
            break
    after = text[k:].lstrip()
    # A tail return forwards both halves: `return f(..);`, `return (f(..));`.
    if re.search(r'(?<![A-Za-z_0-9])return$', b):
        if after.startswith(";"):
            return ("indirect-tail-return" if indirect else None), "tail"
        return "tail-return-with-operator", None
    # `is_ok(f())` / `is_err_result(f())`, parens or not, and `is_tag(f(), x)`: these read
    # the TAG, which is what a call's first argument receives (lib/result.cyr and
    # lib/tagged.cyr document them).
    if re.search(r'(?<![A-Za-z_0-9])(?:is_ok|is_err_result)\s*\($', b) and after.startswith(")"):
        return None, "tag"
    if re.search(r'(?<![A-Za-z_0-9])is_tag\s*\($', b) and after.startswith(","):
        return None, "tag"
    if not b.strip() and after[:1] in ("", ";", ",", ")", "]", "}"):
        return "single-bind-or-bare", "whole"
    # The forms that keep both halves on 6.7.x but not on every pin >= 6.6.3.
    if ngroup > 0 and after.startswith("?"):
        return "grouped-propagation", None
    if ngroup > 0 and BIND.fullmatch(b + " "):
        return "grouped-bind", None
    if PAIR.fullmatch(before) or (ngroup > 0 and PAIR.fullmatch(b + " ")):
        return "pair-reassignment", None
    if TUPLE.fullmatch(before):
        return "tuple-capture", None
    if before.rstrip().endswith(("(", ",")):
        return "argument-position", None
    return "single-bind-or-bare", None

def callables(paths, fns, cls_result):
    """Result callables: per file, the variables bound to a Result closure, to `&f` of a
    Result fn, or to another such variable; and, across files, the fns that return one
    (factories), whose results bind a Result callable too. To a fixed point."""
    alt = "|".join(sorted(map(re.escape, fns), key=len, reverse=True))
    per = {f: set() for f in paths}
    factories = set()
    while True:
        grew = False
        for f in paths:
            _, text, cls = lexed(f)
            names = per[f]
            for (a, _, _, _) in cls:
                if (f, a) in cls_result:
                    m = re.search(r'(?<![A-Za-z_0-9])(?:var\s+)?(' + ID + r')\s*=\s*$', text[:a])
                    if m and m.group(1) not in names:
                        names.add(m.group(1))
                        grew = True
            pats = [r'\(?\s*&\s*(?:' + alt + r')(?![A-Za-z_0-9])']
            if names:
                pats.append(r'(?:\(\s*)*(?:' + "|".join(map(re.escape, sorted(names))) + r')\s*(?:\)\s*)*;')
            if factories:
                pats.append(r'(?:' + "|".join(map(re.escape, sorted(factories))) + r')\s*\(')
            for p in pats:
                for m in re.finditer(r'(?<![A-Za-z_0-9])(?:var\s+)?(' + ID + r')\s*=\s*' + p, text):
                    if m.group(1) not in names:
                        names.add(m.group(1))
                        grew = True
            # A fn whose own return hands back a Result callable is a factory.
            for m in DEF.finditer(text):
                if m.group(1) in factories:
                    continue
                ob = text.find("{", m.end())
                cb = close_paren(text, ob, "{", "}") if ob >= 0 else -1
                if cb <= ob:
                    continue
                inner = [(a, e) for a, _, e, _ in cls if ob < a < cb]
                rets = [r for r in re.finditer(r'(?<![A-Za-z_0-9])return\s*', text[ob:cb])
                        if not any(a < ob + r.start() <= e for a, e in inner)]
                hit = False
                for r in rets:
                    at = ob + r.end()
                    if any(a == at and (f, a) in cls_result for a, _, _, _ in cls):
                        hit = True
                    elif names and re.compile(r'(?:\(\s*)*(?:' + "|".join(map(re.escape, sorted(names)))
                                              + r')\s*(?:\)\s*)*;').match(text, at):
                        hit = True
                    elif factories and re.compile(r'(?:' + "|".join(map(re.escape, sorted(factories)))
                                                  + r')\s*\(').match(text, at):
                        hit = True
                if hit:
                    factories.add(m.group(1))
                    grew = True
        if not grew:
            return per, factories

def scan(fns, ridx, defs, paths=None):
    """Return a list of (file, lineno, fnname, kind, text) offences. A name with a Result and a
    plain definition is reported at each plain one: anywhere in the tree on a tree run, in
    `paths` when they are given."""
    bad = []
    mixed_all = paths is None
    if not fns:
        return bad
    alt = "|".join(sorted(map(re.escape, fns), key=len, reverse=True))
    call = re.compile(r'(?<![A-Za-z_0-9])(' + alt + r')\s*\(')
    # A Result fn taken by address: `&f` or `& f`, never `&&` (logical and), and never
    # `& f(` -- a CALL after `&` is a bitwise and of its value, judged as a call.
    addr = re.compile(r'(?<!&)&\s*(' + alt + r')(?![A-Za-z_0-9])(?!\s*\()')
    if paths is None:
        paths = sorted(glob.glob("src/*.cyr") + glob.glob("tests/*") + glob.glob("examples/*.cyr"))
    paths = [f for f in paths if not os.path.isdir(f)]
    cls_result = {}          # (file, closure start) -> (pos, name): why it returns a Result

    def inner_closure(cls, pos):
        best = None
        for c in cls:
            a, j, e, form = c
            if (j < pos < e if form == "block" else j <= pos < e) and (best is None or j > best[1]):
                best = c
        return best

    def stmt_start(text, cls, pos):
        st = max(text.rfind(";", 0, pos), text.rfind("{", 0, pos), text.rfind("}", 0, pos)) + 1
        c = inner_closure(cls, pos)
        if c and c[3] == "expr":
            st = max(st, c[1])
        return st, c

    # Pass 1: the closures that return a Result, each with its first reason: a Result call
    # it tail-returns or applies `?` to (or that is its whole expression body), a `return
    # Ok(` / `return Err(`, or any other `?` (Cyrius has no `?:`, so a `?` is propagation).
    reasons = {}
    for f in paths:
        raw, text, cls = lexed(f)
        for c in cls:
            a, j, e, form = c
            nested = [(x, y) for x, _, y, _ in cls if j < x < e]
            seg = blank(text, nested, j, e + 1 if form == "block" else e)
            m = OKERR.search(seg) if form == "block" else re.match(r'\s*(?:\(\s*)*(Ok|Err)\s*\(', seg)
            if m:
                reasons.setdefault((f, a), []).append((j + m.start(1), m.group(1)))
            q = seg.find("?")
            if q >= 0:
                reasons.setdefault((f, a), []).append((j + q, "?"))
        for m in call.finditer(text):
            st, c = stmt_start(text, cls, m.start())
            if not c or re.search(r'(?<![A-Za-z_0-9])fn\s*$', text[st:m.start()]):
                continue
            kind, how = judge(text, st, m.start(), m.end() - 1, False)
            if (c[3] == "block" and how in ("prop", "tail")) or \
               (c[3] == "expr" and how in ("prop", "whole")):
                reasons.setdefault((f, c[0]), []).append((m.start(), m.group(1)))
    for key, rs in reasons.items():
        pos = min(p for p, _ in rs)
        cls_result[key] = (pos, min(n for p, n in rs if p == pos))
    per, factories = callables(paths, fns, cls_result)

    for f in paths:
        raw, text, cls = lexed(f)
        def lineno(pos):
            return text.count("\n", 0, pos) + 1
        def report(pos, name, kind):
            n = lineno(pos)
            bad.append((f, n, name, kind, raw[n - 1].strip()[:100]))
        for (ff, a), (pos, name) in sorted(cls_result.items()):
            if ff == f:
                report(pos, name, "result-closure")
        for m in addr.finditer(text):
            SITES[0] += 1
            report(m.start(), m.group(1), "function-pointer")
        for m in call.finditer(text):
            name = m.group(1)
            st, c = stmt_start(text, cls, m.start())
            before = text[st:m.start()]
            if re.search(r'(?<![A-Za-z_0-9])fn\s*$', before):
                continue                      # its own definition
            SITES[0] += 1
            if re.match(r'\s*#define(?![A-Za-z0-9_])', raw[lineno(m.start()) - 1]):
                report(m.start(), name, "inside-define")
                continue
            kind, how = judge(text, st, m.start(), m.end() - 1, False)
            if c and ((c[3] == "block" and how in ("prop", "tail")) or
                      (c[3] == "expr" and how in ("prop", "whole"))):
                continue                      # the closure's own report (result-closure)
            if kind:
                report(m.start(), name, kind)
        names = per.get(f, set())
        for m in INDIRECT.finditer(text):
            callee = m.group(3)
            if callee not in names:
                continue
            # The callee must be the whole first argument: its grouping parens closed, then `,` / `)`.
            k = m.end()
            for _ in range(m.group(2).count("(")):
                while k < len(text) and text[k].isspace():
                    k += 1
                if k < len(text) and text[k] == ")":
                    k += 1
            while k < len(text) and text[k].isspace():
                k += 1
            if k >= len(text) or text[k] not in ",)":
                continue
            SITES[0] += 1
            st, c = stmt_start(text, cls, m.start())
            kind, how = judge(text, st, m.start(), m.start(1), True)
            if kind:
                report(m.start(), callee, kind)
    for f, n, name in mixed(defs, ridx):
        if mixed_all or f in paths:
            raw = lexed(f)[0]
            bad.append((f, n, name, "mixed-definition", raw[n - 1].strip()[:100]))
    return bad

if selftest:
    # Plant a synthetic migrated fn, forwarders of it (3.3.1; grouped, 3.4.1), helpers
    # defined in the probe itself and in examples/ (tests/ and examples/ seed the set), and
    # each form the gate must refuse; the correct shapes beside them must pass. The expected
    # set is exact: a missing offence is a blind spot, an extra one a false alarm. Every
    # planted line, with real Result fns in place of the probe names, compiles on the pinned
    # toolchain (6.7.18), so each offence is a form a suite could actually contain (3.4.1
    # second and third reviews).
    os.makedirs("tests", exist_ok=True)
    os.makedirs("examples", exist_ok=True)
    probe = "tests/__result_probe__.tcyr"
    probe_z = "tests/__result_probe_z__.tcyr"     # sorts after the probe: read last
    probe_e = "examples/__result_probe__.cyr"
    P = "__probe_fn__"
    plant = [
        # (line, expected offences on it as (name, kind), or [] for a correct shape)
        ('assert_eq(%s(1), HSB_ERR_NONE, "planted offence");' % P, [(P, "argument-position")]),
        ('var t, v = %s(2);' % P, []),
        ('assert_eq(__probe_fwd__(3), HSB_ERR_NONE, "via a forwarder");', [("__probe_fwd__", "argument-position")]),
        ('var t2, v2 = __probe_fwd__(4);', []),
        # 3.4.1 (1): another call's `)?` later on the line exempted the whole line.
        ('var s = two(%s(3), %s(5)?);' % (P, P), [(P, "argument-position")]),
        # ... and a `?` after the call's own paren, across a line break, propagates.
        ('var s2 = two(4, %s(' % P, []),
        ('    5)?);', []),
        # 3.4.1 (2): `return f(..) <op> ..` was read as a tail return.
        ('    return %s(3) == 0;' % P, [(P, "tail-return-with-operator")]),
        ('    return %s(3);' % P, []),
        ('    return __probe_fwd__(two(1, 2)) ;', []),
        ('    return (%s(3));' % P, []),
        ('    return(%s(3));' % P, []),
        ('    return (%s(3)) == 0;' % P, [(P, "tail-return-with-operator")]),
        # 3.4.1 (3): every `&f` -- the 6.7.17+ pair forms included, which 6.6.3 lacks.
        ('var c = id(fncall1(&%s, 3));' % P, [(P, "function-pointer")]),
        ('var c3, v3c = fncall1(&%s, 3);' % P, [(P, "function-pointer")]),
        ('var p0 = &%s;' % P, [(P, "function-pointer")]),
        ('var p1 = & %s;' % P, [(P, "function-pointer")]),
        ('var c2 = id(fncall1(& %s, 3));' % P, [(P, "function-pointer")]),
        # `&&` and `&` before a CALL are operators on its value.
        ('var d = x &&%s(1)?;' % P, []),
        ('var d2 = x &&%s(1);' % P, [(P, "single-bind-or-bare")]),
        ('var zb = zx &%s(3)?;' % P, []),
        ('var zc = zx & %s(3);' % P, [(P, "single-bind-or-bare")]),
        ('var zq = id(%s(3))?;' % P, [(P, "argument-position")]),
        ('var zr = id((%s(3)) + 1)?;' % P, [(P, "argument-position")]),
        # Pair forms that 6.7.18 keeps whole and 6.6.3 does not (see the header table).
        ('var zv = (%s(3))?;' % P, [(P, "grouped-propagation")]),
        ('var zw = id((%s(3))?);' % P, [(P, "grouped-propagation")]),
        ('var t6, v6 = (%s(3));' % P, [(P, "grouped-bind")]),
        ('t, v = %s(3);' % P, [(P, "pair-reassignment")]),
        ('t, v = ((%s(3)));' % P, [(P, "pair-reassignment")]),
        ('case 1: t, v = %s(3);' % P, [(P, "pair-reassignment")]),
        ('var tt: (i64, i64) = %s(3);' % P, [(P, "tuple-capture")]),
        # A bind split after its `=`, and the tag readers lib/result.cyr documents.
        ('var t3, v3 =', []),
        ('    %s(3);' % P, []),
        ('assert_eq(is_ok(%s(3)), 1, "x");' % P, []),
        ('assert_eq(is_ok((%s(3))), 1, "x");' % P, []),
        ('assert_eq(is_err_result(%s(3)), 0, "x");' % P, []),
        ('assert_eq(is_ok(%s(3) + 1), 1, "x");' % P, [(P, "argument-position")]),
        # The stdlib's Result fns are in the set (lib/result.cyr's ok_via).
        ('assert_eq(ok_via(0, 7), 0, "a stdlib Result fn");', [("ok_via", "argument-position")]),
        # A trailing comment naming a call is prose, not a call site.
        ('var e = 1;  # %s(1) here would read the tag' % P, []),
        # 3.4.1 second review: char literals neither open a string nor start a comment,
        # an `include` line ends a statement, and a `#define` body is code.
        ("var q = '\"';", []),
        ('var t4, v4 = %s(2);' % P, []),
        ("var h = '#';", []),
        ('var t5, v5 = %s(3);' % P, []),
        ("if (zx == '\"') { assert_eq(%s(1), 0, \"x\"); }" % P, [(P, "argument-position")]),
        ("if (zx == '#') { assert_eq(%s(1), 0, \"x\"); }" % P, [(P, "argument-position")]),
        ('include "src/error.cyr"', []),
        ('var t7, v7 = %s(3);' % P, []),
        ('#define RFM(a) %s(a)' % P, [(P, "inside-define")]),
        # A Result helper defined in tests/, and a grouped forwarder (the seed below).
        ('fn __probe_tfn__(y) {', []),
        ('    return Ok(y);', []),
        ('}', []),
        ('assert_eq(__probe_tfn__(1), 0, "a tests/ helper");', [("__probe_tfn__", "argument-position")]),
        ('assert_eq(__probe_gfwd__(1), 0, "a grouped forwarder");', [("__probe_gfwd__", "argument-position")]),
        # 3.4.1 third review (S3): a forwarder by `?` alone, an examples/ helper (probe_e),
        # and a call inside a string, which only string blanking keeps out.
        ('assert_eq(__probe_qfwd__(1), 0, "a ?-only forwarder");', [("__probe_qfwd__", "argument-position")]),
        ('assert_eq(__probe_efn__(1), 0, "an examples/ helper");', [("__probe_efn__", "argument-position")]),
        ('var zs = strlen("%s(1) in a string");' % P, []),
        # S2a: a string spanning lines, by a raw newline and by a `\`-newline.
        ('var ms = "line one', []),
        ('line two"; assert_eq(%s(1), 0, "x");' % P, [(P, "argument-position")]),
        ('var ms2 = "raw newline, then', []),
        ('%s(1) still inside the string";' % P, []),
        ('var ms3 = "one\\', []),
        ('two"; assert_eq(%s(2), 0, "x");' % P, [(P, "argument-position")]),
        ('var ms4 = "first\\', []),
        ('%s(1) in the continued string";' % P, []),
        ('var ms5 = "a\\', []),
        ('"; assert_eq(%s(1), 0, "x");' % P, [(P, "argument-position")]),
        # S2b: an attribute leading the declaration's own line.
        ('#must_use fn __probe_mfn__(y) {', []),
        ('    return Ok(y);', []),
        ('}', []),
        ('assert_eq(__probe_mfn__(1), 0, "x");', [("__probe_mfn__", "argument-position")]),
        ('#must_use public fn __probe_mpfn__(y) {', []),
        ('    return __probe_mfn__(y);', []),
        ('}', []),
        ('assert_eq(__probe_mpfn__(1), 0, "x");', [("__probe_mpfn__", "argument-position")]),
        # S2c: a name defined here and in probe_z, Result in one and plain in the other,
        # each order: every call is a Result call, and the plain definition is reported.
        ('fn __probe_dupa__(y) {', []),
        ('    return Ok(y);', []),
        ('}', []),
        ('assert_eq(__probe_dupa__(1), 0, "x");', [("__probe_dupa__", "argument-position")]),
        ('fn __probe_dupb__(y) {', [("__probe_dupb__", "mixed-definition")]),
        ('    return y;', []),
        ('}', []),
        ('assert_eq(__probe_dupb__(1), 0, "x");', [("__probe_dupb__", "argument-position")]),
        # S1: a closure that returns a Result, and the indirect calls through it.
        ('var __pg = |x| { return %s(x); };' % P, [(P, "result-closure")]),
        ('assert_eq(fncall1(__pg, 1), 0, "x");', [("__pg", "argument-position")]),
        ('var t10, v10 = fncall1(__pg, 1);', [("__pg", "indirect-bind")]),
        ('var t11, v11 = callptr((__pg), 1);', [("__pg", "indirect-bind")]),
        ('assert_eq(is_ok(fncall1(__pg, 1)), 1, "x");', []),
        ('var __ph = |x| %s(x);' % P, [(P, "result-closure")]),
        ('var __pk = |x| { return Ok(x); };', [("Ok", "result-closure")]),
        ('var __pq = |x| { var r = %s(x)?; return r; };' % P, [(P, "result-closure")]),
        ('var __pf = |x| { var tf, vf = %s(x); return vf; };' % P, []),
        ('assert_eq(fncall1(__pf, 1), 0, "x");', []),
        ('var __pi = |x| { var r = fncall1(__pg, x)?; return r; };', [("?", "result-closure"),
                                                                       ("__pg", "indirect-propagation")]),
        ('var __pn = |x| {', []),
        ('    var h2 = |y| { return %s(y); };' % P, [(P, "result-closure")]),
        ('    var h3 = |y| { return Ok(y); };', [("Ok", "result-closure")]),
        ('    return 0;', []),
        ('};', []),
        ('assert_eq(fncall1(__pn, 1), 0, "x");', []),
        ('fn __probe_encl__(y) {', []),
        ('    var g2 = |x| { return %s(x); };' % P, [(P, "result-closure")]),
        ('    assert_eq(fncall1(g2, 1), 0, "x");', [("g2", "argument-position")]),
        ('    return fncall1(g2, y);', [("g2", "indirect-tail-return")]),
        ('}', []),
        ('assert_eq(__probe_encl__(1), 1, "x");', []),
        ('fn __probe_ip__(y) {', []),
        ('    var z2 = fncall1(__pg, y)?;', [("__pg", "indirect-propagation")]),
        ('    return z2;', []),
        ('}', []),
        ('var pp = &%s;' % P, [(P, "function-pointer")]),
        ('assert_eq(fncall1(pp, 1), 0, "x");', [("pp", "argument-position")]),
        ('fn __probe_mk__() {', []),
        ('    return |x| { return %s(x); };' % P, [(P, "result-closure")]),
        ('}', []),
        ('var __pm = __probe_mk__();', []),
        ('assert_eq(fncall1(__pm, 1), 0, "x");', [("__pm", "argument-position")]),
        ('var __pa = __pg;', []),
        ('assert_eq(fncall1(__pa, 1), 0, "x");', [("__pa", "argument-position")]),
        # N1: binds in a case label and a for-init keep both halves on every pin, and
        # is_tag reads the tag as is_ok does; the Result as is_tag's second argument does not.
        ('case 1: var t8, v8 = %s(3);' % P, []),
        ('for (var t9, v9 = %s(3); t9 < 1; t9 = t9 + 1) { zx = v9; }' % P, []),
        ('case 1: var t12, v12 = (%s(3));' % P, [(P, "grouped-bind")]),
        ('assert_eq(is_tag(%s(3), 0), 1, "x");' % P, []),
        ('assert_eq(is_tag(0, %s(3)), 1, "x");' % P, [(P, "argument-position")]),
        ('assert_eq(is_tag(%s(3) + 1, 0), 1, "x");' % P, [(P, "argument-position")]),
    ]
    plant_z = [
        ('fn __probe_dupa__(y) {', [("__probe_dupa__", "mixed-definition")]),
        ('    return y;', []),
        ('}', []),
        ('fn __probe_dupb__(y) {', []),
        ('    return Err(y);', []),
        ('}', []),
    ]
    plant_e = [
        ('fn __probe_efn__(y) {', []),
        ('    return Err(y);', []),
        ('}', []),
    ]
    for path, rows in ((probe, plant), (probe_z, plant_z), (probe_e, plant_e)):
        with open(path, "w") as fh:
            for text, _ in rows:
                fh.write(text + "\n")
    try:
        defs = definitions({
            P: "fn %s(x) {\n    return Ok(x);\n}\n" % P,
            "__probe_fwd__": "fn __probe_fwd__(x) {\n    var r = %s(x)?;\n    return %s(r);\n}\n" % (P, P),
            "__probe_gfwd__": "fn __probe_gfwd__(x) {\n    return (%s(x));\n}\n" % P,
            "__probe_qfwd__": "fn __probe_qfwd__(x) {\n    var r = %s(x)?;\n    return r;\n}\n" % P,
        })
        fake, ridx = migrated_fns(defs)
        found = scan(fake, ridx, defs, [probe, probe_z, probe_e])
        want = sorted((path, i, n, k) for path, rows in ((probe, plant), (probe_z, plant_z), (probe_e, plant_e))
                      for i, (_, exp) in enumerate(rows, 1) for n, k in exp)
        got = sorted((o[0], o[1], o[2], o[3]) for o in found)
        seeded = {"the stdlib's (ok_via, file_read_r)": "ok_via" in fake and "file_read_r" in fake,
                  "a tests/ helper (__probe_tfn__)": "__probe_tfn__" in fake,
                  "an examples/ helper (__probe_efn__)": "__probe_efn__" in fake,
                  "a grouped forwarder (__probe_gfwd__)": "__probe_gfwd__" in fake,
                  "a ?-only forwarder (__probe_qfwd__)": "__probe_qfwd__" in fake,
                  "a one-line #must_use fn (__probe_mfn__, __probe_mpfn__)":
                      "__probe_mfn__" in fake and "__probe_mpfn__" in fake,
                  "both mixed names (__probe_dupa__, __probe_dupb__)":
                      "__probe_dupa__" in fake and "__probe_dupb__" in fake,
                  "a fn with a `?` on an indirect call (__probe_ip__)": "__probe_ip__" in fake}
        kept_out = {"a fn around a Result closure (__probe_encl__)": "__probe_encl__" not in fake,
                    "a closure factory (__probe_mk__)": "__probe_mk__" not in fake}
        ok = (got == want) and all(seeded.values()) and all(kept_out.values())
        print("  selftest: planted %d offence(s) and %d correct shapes"
              % (len(want), sum(1 for _, e in plant + plant_z + plant_e if not e)))
        print("  selftest: detector reported %d offence(s) in the probe" % len(got))
        for w in want:
            if w not in got:
                print("    MISSED: %s:%d  %s  [%s]" % w)
        for g in got:
            if g not in want:
                print("    FALSE ALARM: %s:%d  %s  [%s]" % g)
        for what, there in seeded.items():
            if not there:
                print("    NOT IN THE SET: %s" % what)
        for what, out in kept_out.items():
            if not out:
                print("    WRONGLY IN THE SET: %s" % what)
        print("  selftest: %s" % ("PASS" if ok else "FAIL"))
        sys.exit(0 if ok else 1)
    finally:
        for path in (probe, probe_z, probe_e):
            os.remove(path)

defs = definitions()
fns, ridx = migrated_fns(defs)
if not fns:
    print("=== result migration: 0 functions migrated yet, nothing to check ===")
    print("A function counts as migrated once its body returns Ok(...) or Err(...).")
    sys.exit(0)

bad = scan(fns, ridx, defs)
print("=== result migration: %d migrated function(s), %d call site(s), %d bad ==="
      % (len(fns), SITES[0], len(bad)))
# ⚠ A gate that checks nothing still prints "0 bad". The floor sits under the
# 3.4.1 tree's population (100 functions, 1499 sites) with room for ordinary churn,
# so an edit that empties the set or the scan -- a broken regex, a moved tree --
# fails here instead of reporting a clean nothing.
FLOOR_FNS, FLOOR_SITES = 80, 1000
if len(fns) < FLOOR_FNS or SITES[0] < FLOOR_SITES:
    print("!! population below the floor (%d functions, %d sites): the gate is not reaching"
          % (FLOOR_FNS, FLOOR_SITES))
    print("   the tree. Fix the scan, or lower the floor deliberately with a reason.")
    sys.exit(1)
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
