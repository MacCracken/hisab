#!/usr/bin/env bash
# mutation-test.sh -- install single-operator mutants of src/ into scratch copies of the
# tree, run the six suites against each, and report which mutants no suite kills.
#
# WHY IT EXISTS
# CLAUDE.md asks for every new assertion to be mutation-proven, and the 2026-08-11
# audit's headline -- 38.4% of value-changing single-operator mutants survive all five
# suites -- came from an ad-hoc run with no instrument checked in, so it could not be
# re-measured (roadmap D101). This is that instrument. It never touches the working
# tree: every mutant is written into a copy, proven installed, run, and the copy restored
# and compared byte for byte before the next mutant uses it.
#
# USAGE
#   scripts/mutation-test.sh --sample N [--seed S] [--jobs J] [--files "src/a.cyr src/b.cyr"]
#   scripts/mutation-test.sh --spec FILE [--jobs J]
#   scripts/mutation-test.sh --gen N [--seed S] [--files ...] > spec.jsonl
#   scripts/mutation-test.sh --spec FILE --dry-run      install, prove, restore; build nothing
# Common options: --out FILE (results, JSON lines; default under the scratch dir, which is
# then KEPT), --all-suites (run every suite even after a kill: a per-suite kill matrix),
# --keep (keep the scratch trees), --work DIR (scratch base instead of mktemp).
# --jobs defaults to 2; keep it at 4 or below on a shared machine -- each job is one
# full compile of a suite at a time.
#
# THE MUTANT SPEC -- one JSON object per line; blank lines and lines starting '#' skipped:
#   {"id": "m0001", "file": "src/vec3.cyr", "line": 503, "expect": "<the original line>",
#    "find": "f64_add(", "nth": 1, "replace": "f64_sub(", "op": "arith"}
# The target line is "line" (1-based) or "pattern" (a Python regex that must match exactly
# one line of the file); "expect", when given, must equal that line exactly, so a spec
# that has drifted from the source is refused rather than applied somewhere else. "find"
# is a literal that must occur in the line; "nth" (default 1) picks the occurrence.
#
# THE SAMPLED OPERATORS (--sample / --gen), over code only -- comments, string contents
# and attribute tokens are blanked by the same scanner check-public-surface.sh uses:
#   arith       f64_add <-> f64_sub, f64_mul <-> f64_div; integer " + " <-> " - ",
#               " * " <-> " / " (binary, space-delimited)
#   relational  f64_lt <-> f64_le, f64_gt <-> f64_ge; " < " <-> " <= ", " > " <-> " >= ",
#               " == " <-> " != "
#   constant    a decimal integer n -> n + 1; a hex literal -> its lowest bit flipped (for
#               an f64 bit pattern, one ulp)
# Every site in src/*.cyr (src/main.cyr and src/visibility.cyr excluded) is a candidate;
# --sample draws N of them uniformly with Python's random.Random(seed).
#
# VERDICTS. Suites run cheapest first, and only the suites that include the mutated file:
#   killed     a suite reported failures, exited non-zero, crashed or timed out
#   survived   every suite that includes the file built and passed with 0 failures
#   stillborn  a suite failed to compile -- the compiler rejected it, no test ran; left
#              out of the survival rate
# The verdict comes from the "N passed, M failed" summary line and the exit status,
# never from the last line printed. Survival = survived / (killed + survived), with a
# Wilson 95% interval. It counts EQUIVALENT mutants as survivors (no tool here can tell
# a mutant that changes nothing from one the suites miss), so it is an upper bound on
# the suites' blind rate; read the survivor list before quoting it.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK=""
KEEP=0
ARGS=()
while [ $# -gt 0 ]; do
    case "$1" in
        --work) WORK="$2"; shift 2 ;;
        --keep) KEEP=1; ARGS+=("$1"); shift ;;
        -h|--help) sed -n '2,52p' "$0"; exit 0 ;;
        *) ARGS+=("$1"); shift ;;
    esac
done
if [ -z "$WORK" ]; then
    WORK="$(mktemp -d "${TMPDIR:-/tmp}/hisab-mut.XXXXXX")"
fi
mkdir -p "$WORK"
cleanup() {
    if [ "$KEEP" = 1 ]; then
        echo "scratch kept: $WORK" >&2
    else
        rm -rf "$WORK/trees" "$WORK/mut.py"
        rmdir "$WORK" 2>/dev/null || echo "results kept in: $WORK" >&2
    fi
}
trap cleanup EXIT

cat > "$WORK/mut.py" <<'PYEOF'
import argparse, json, math, os, queue, random, re, shutil, subprocess, sys, threading, time

SUITES = ['foundation', 'edge_cases', 'hisab', 'abuse', 'modules', 'modules_b']   # cheapest first
SUMMARY = re.compile(r'^(\d+) passed, (\d+) failed', re.M)

# ---- the comment / string blanker (same rules as check-public-surface.sh's scan.py)
ATTR = re.compile(r'#(?:inline|naked|pure|io|alloc|must_use|regalloc|deprecated|assert|pe_import)'
                  r'(?![A-Za-z0-9_])|#derive\([^)\n]*\)')

def strip(src):
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
            m = ATTR.match(src, i)
            if m:
                out.append(' ' * (m.end() - i)); i = m.end(); continue
            while i < n and src[i] != '\n':
                out.append(' '); i += 1
        else:
            out.append(c); i += 1
    return ''.join(out)

SWAPS = [
    ('arith', 'f64_add(', 'f64_sub('), ('arith', 'f64_sub(', 'f64_add('),
    ('arith', 'f64_mul(', 'f64_div('), ('arith', 'f64_div(', 'f64_mul('),
    ('arith', ' + ', ' - '), ('arith', ' - ', ' + '), ('arith', ' * ', ' / '), ('arith', ' / ', ' * '),
    ('relational', 'f64_lt(', 'f64_le('), ('relational', 'f64_le(', 'f64_lt('),
    ('relational', 'f64_gt(', 'f64_ge('), ('relational', 'f64_ge(', 'f64_gt('),
    ('relational', ' < ', ' <= '), ('relational', ' <= ', ' < '),
    ('relational', ' > ', ' >= '), ('relational', ' >= ', ' > '),
    ('relational', ' == ', ' != '), ('relational', ' != ', ' == '),
]
DEC = re.compile(r'(?<![A-Za-z0-9_.])[0-9][0-9_]*(?![A-Za-z0-9_.])')
HEX = re.compile(r'(?<![A-Za-z0-9_.])0[xX][0-9A-Fa-f_]+(?![A-Za-z0-9_.])')

def candidates(repo, files):
    out = []
    for f in files:
        text = open(os.path.join(repo, f), encoding='utf-8', errors='replace').read()
        raw = text.split('\n')
        code = strip(text).split('\n')
        for ln, (r, c) in enumerate(zip(raw, code), 1):
            for op, a, b in SWAPS:
                start = 0
                while True:
                    k = c.find(a, start)
                    if k < 0: break
                    ok = True
                    if a.startswith('f64_') and k > 0 and (c[k - 1].isalnum() or c[k - 1] == '_'):
                        ok = False   # part of a longer name
                    if ok:
                        # the blanker keeps every column, so k indexes the raw line too
                        nth = substr_hits(r, a).index(k) + 1
                        out.append(dict(file=f, line=ln, expect=r, find=a, nth=nth, replace=b, op=op))
                    start = k + 1
            for m in HEX.finditer(c):
                tok = r[m.start():m.end()]
                digits = tok.rstrip('_')
                last = digits[-1]
                flipped = '%X' % (int(last, 16) ^ 1) if last.isupper() or last.isdigit() else '%x' % (int(last, 16) ^ 1)
                new = digits[:-1] + flipped + tok[len(digits):]
                nth = substr_hits(r, tok).index(m.start()) + 1
                out.append(dict(file=f, line=ln, expect=r, find=tok, nth=nth, replace=new, op='constant'))
            for m in DEC.finditer(c):
                tok = r[m.start():m.end()]
                if tok.startswith('0') and len(tok) > 1 and tok[1] in 'xX': continue
                val = int(tok.replace('_', ''))
                new = str(val + 1)
                # occurrence index among whole-token matches in the raw line, as find_nth counts
                hits = [mm.start() for mm in DEC.finditer(r) if r[mm.start():mm.end()] == tok]
                if m.start() not in hits: continue
                out.append(dict(file=f, line=ln, expect=r, find=tok, nth=hits.index(m.start()) + 1,
                                replace=new, op='constant', token=True))
    return out

def substr_hits(line, find):
    hits, k = [], line.find(find)
    while k >= 0:
        hits.append(k); k = line.find(find, k + 1)
    return hits

def find_nth(line, find, nth, token):
    if token:
        hits = [m.start() for m in DEC.finditer(line) if line[m.start():m.end()] == find]
    else:
        hits = substr_hits(line, find)
    return hits[nth - 1] if 0 < nth <= len(hits) else -1

def apply(text, mu):
    lines = text.split('\n')
    if 'line' in mu:
        idx = int(mu['line']) - 1
        if not 0 <= idx < len(lines): raise ValueError('line %s out of range' % mu['line'])
    else:
        rx = re.compile(mu['pattern'])
        hits = [i for i, l in enumerate(lines) if rx.search(l)]
        if len(hits) != 1: raise ValueError('pattern matches %d lines, not 1' % len(hits))
        idx = hits[0]
    old = lines[idx]
    if 'expect' in mu and old != mu['expect']:
        raise ValueError('line %d is not the expected text (spec drifted from the source)' % (idx + 1))
    pos = find_nth(old, mu['find'], int(mu.get('nth', 1)), mu.get('token', False))
    if pos < 0: raise ValueError('"%s" occurrence %s not in line %d' % (mu['find'], mu.get('nth', 1), idx + 1))
    new = old[:pos] + mu['replace'] + old[pos + len(mu['find']):]
    if new == old: raise ValueError('the replacement changes nothing')
    lines[idx] = new
    return '\n'.join(lines), idx, old, new

def wilson(k, n, z=1.96):
    if n == 0: return (float('nan'),) * 2
    p = k / n; d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d; h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return c - h, c + h

def run(cmd, cwd, timeout):
    t0 = time.time()
    try:
        p = subprocess.run(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=timeout)
        return p.returncode, p.stdout.decode('utf-8', 'replace'), time.time() - t0, False
    except subprocess.TimeoutExpired as e:
        return None, (e.stdout or b'').decode('utf-8', 'replace'), time.time() - t0, True

def suite_verdict(tree, s, run_timeout):
    rc, out, dt, to = run([os.path.join(tree, 'bin', s)], tree, run_timeout)
    if to: return 'timeout', None, dt
    m = None
    for m in SUMMARY.finditer(out): pass
    if m is None: return 'crash', rc, dt
    failed = int(m.group(2))
    if failed > 0 or rc != 0: return 'failed', failed, dt
    return 'pass', 0, dt

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--repo'); ap.add_argument('--work')
    ap.add_argument('--sample', type=int); ap.add_argument('--gen', type=int); ap.add_argument('--spec')
    ap.add_argument('--seed', type=int, default=1); ap.add_argument('--jobs', type=int, default=2)
    ap.add_argument('--files'); ap.add_argument('--out'); ap.add_argument('--all-suites', action='store_true')
    ap.add_argument('--dry-run', action='store_true'); ap.add_argument('--keep', action='store_true')
    a = ap.parse_args()
    repo, work = a.repo, a.work
    if a.jobs < 1: sys.exit('--jobs must be >= 1')
    if a.files:
        files = a.files.split()
    else:
        files = sorted('src/' + f for f in os.listdir(os.path.join(repo, 'src'))
                       if f.endswith('.cyr') and f not in ('main.cyr', 'visibility.cyr'))
    if a.spec:
        mutants = []
        for raw in open(a.spec):
            raw = raw.strip()
            if raw and not raw.startswith('#'): mutants.append(json.loads(raw))
    else:
        n = a.sample if a.sample is not None else a.gen
        if n is None: sys.exit('give --sample N, --gen N or --spec FILE')
        cands = candidates(repo, files)
        rng = random.Random(a.seed)
        pick = sorted(rng.sample(range(len(cands)), min(n, len(cands))))
        mutants = []
        for j, ci in enumerate(pick, 1):
            mu = dict(cands[ci]); mu['id'] = 'm%04d' % j; mutants.append(mu)
        print('# %d candidate sites in %d files; sampled %d with seed %d' % (len(cands), len(files), len(mutants), a.seed),
              file=sys.stderr)
        if a.gen is not None:
            print('# mutation-test.sh --gen %d --seed %d (%d candidate sites, %d files)' % (n, a.seed, len(cands), len(files)))
            for mu in mutants: print(json.dumps(mu))
            return 0
    for mu in mutants:
        mu.setdefault('id', 'm%04d' % (mutants.index(mu) + 1))
        mu.setdefault('op', '?')

    # which suites include which file
    inc = {}
    for s in SUITES:
        t = open(os.path.join(repo, 'tests', s + '.tcyr'), encoding='utf-8', errors='replace').read()
        inc[s] = set(re.findall(r'^include "([^"]+)"', t, re.M))
    os.makedirs(os.path.join(work, 'trees'), exist_ok=True)
    out_path = a.out or os.path.join(work, 'results.jsonl')

    # scratch trees: src/ lib/ tests/ and the manifest, lock and VERSION -- nothing else
    trees = []
    for j in range(min(a.jobs, max(1, len(mutants)))):
        t = os.path.join(work, 'trees', 't%d' % j)
        if os.path.exists(t): shutil.rmtree(t)
        os.makedirs(os.path.join(t, 'bin'))
        for d in ('src', 'lib', 'tests'): shutil.copytree(os.path.join(repo, d), os.path.join(t, d))
        for f in ('cyrius.cyml', 'cyrius.lock', 'VERSION'): shutil.copy2(os.path.join(repo, f), t)
        trees.append(t)
    pristine = {f: open(os.path.join(repo, f), 'rb').read() for f in set(m['file'] for m in mutants)}

    # control: every suite builds and passes in every tree, and its run time sizes the timeouts
    base = {}
    if not a.dry_run:
        def control(t):
            res = {}
            for s in SUITES:
                rc, out, dt, to = run(['cyrius', 'build', 'tests/%s.tcyr' % s, 'bin/%s' % s], t, 600)
                if rc != 0: return s, 'control build failed:\n' + out[-2000:]
                v, f, rdt = suite_verdict(t, s, 600)
                if v != 'pass': return s, 'control run: %s (%s)' % (v, f)
                res[s] = rdt
            return None, res
        ths, results = [], [None] * len(trees)
        def go(i): results[i] = control(trees[i])
        for i in range(len(trees)):
            th = threading.Thread(target=go, args=(i,)); th.start(); ths.append(th)
        for th in ths: th.join()
        for r in results:
            if r[0] is not None: sys.exit('mutation-test: the unmutated tree fails suite %s: %s' % r)
            for s, dt in r[1].items(): base[s] = max(base.get(s, 0), dt)
        print('# control: six suites pass in %d scratch trees; slowest runs %s' %
              (len(trees), ', '.join('%s %.1fs' % (s, base[s]) for s in SUITES)), file=sys.stderr)

    free = queue.Queue()
    for t in trees: free.put(t)
    lock = threading.Lock()
    done = [0]
    outf = open(out_path, 'w')

    def one(mu):
        t = free.get()
        rec = dict(mu)
        try:
            path = os.path.join(t, mu['file'])
            orig = pristine[mu['file']].decode('utf-8')
            try:
                text, idx, old, new = apply(orig, mu)
            except Exception as e:
                rec.update(verdict='invalid', reason=str(e)); return rec
            open(path, 'w', encoding='utf-8').write(text)
            # proof of installation: exactly one line differs, and it is the mutant line
            got = open(path, encoding='utf-8').read().split('\n')
            ol = orig.split('\n')
            diff = [i for i in range(max(len(got), len(ol))) if (got[i] if i < len(got) else None) != (ol[i] if i < len(ol) else None)]
            if diff != [idx] or got[idx] != new:
                rec.update(verdict='invalid', reason='installation not proven: %d lines differ' % len(diff)); return rec
            rec.update(line=idx + 1, mutated=new, installed=True)
            if a.dry_run:
                rec['verdict'] = 'dry-run'
            else:
                rec['verdict'] = 'survived'
                rec['suites_run'] = []
                for s in SUITES:
                    if mu['file'] not in inc[s]: continue
                    rc, out, dt, to = run(['cyrius', 'build', 'tests/%s.tcyr' % s, 'bin/%s' % s], t, 600)
                    if rc != 0 or to:
                        errs = [l for l in out.splitlines() if 'error' in l.lower()] or out.strip().splitlines() or ['?']
                        rec.update(verdict='stillborn', by=s, reason=('build timed out' if to else errs[0])[:200]); break
                    v, f, rdt = suite_verdict(t, s, 5 * base.get(s, 10) + 20)
                    rec['suites_run'].append(s)
                    if v != 'pass':
                        if rec['verdict'] == 'survived':
                            rec.update(verdict='killed', by=s, how=v, failures=f)
                        else:
                            rec.setdefault('also', []).append(s)
                        if not a.all_suites: break
                if rec['verdict'] == 'survived' and not rec['suites_run']:
                    rec['verdict'] = 'uncovered'
        finally:
            # restore, and prove the restore before the tree is reused
            p = os.path.join(t, mu['file'])
            open(p, 'wb').write(pristine[mu['file']])
            if open(p, 'rb').read() != pristine[mu['file']]:
                print('FATAL: restore of %s in %s did not verify' % (mu['file'], t), file=sys.stderr)
                os._exit(3)
            rec['restored'] = True
            free.put(t)
        return rec

    from concurrent.futures import ThreadPoolExecutor
    recs = []
    with ThreadPoolExecutor(max_workers=len(trees)) as ex:
        for rec in ex.map(one, mutants):
            with lock:
                done[0] += 1
                recs.append(rec)
                outf.write(json.dumps(rec) + '\n'); outf.flush()
                print('[%d/%d] %s %-9s %s:%s %s %s -> %s%s' % (done[0], len(mutants), rec['id'], rec['verdict'],
                      rec['file'], rec.get('line', '?'), rec['op'], rec['find'].strip(), rec['replace'].strip(),
                      (' by ' + rec['by']) if rec.get('by') else ''), file=sys.stderr)
    outf.close()

    cnt = {}
    for r in recs: cnt[r['verdict']] = cnt.get(r['verdict'], 0) + 1
    k, s_ = cnt.get('killed', 0), cnt.get('survived', 0)
    lo, hi = wilson(s_, k + s_)
    print('mutants %d: %s' % (len(recs), ', '.join('%s %d' % kv for kv in sorted(cnt.items()))))
    if k + s_:
        print('survival %d / %d = %.1f%% (Wilson 95%%: %.1f%% - %.1f%%)' % (s_, k + s_, 100.0 * s_ / (k + s_), 100 * lo, 100 * hi))
        for op in ('arith', 'relational', 'constant'):
            ko = sum(1 for r in recs if r['op'] == op and r['verdict'] == 'killed')
            so = sum(1 for r in recs if r['op'] == op and r['verdict'] == 'survived')
            if ko + so: print('  %-10s %d / %d survive (%.1f%%)' % (op, so, ko + so, 100.0 * so / (ko + so)))
        by = {}
        for r in recs:
            if r['verdict'] == 'killed': by[r['by']] = by.get(r['by'], 0) + 1
        print('  first kill by suite: ' + ', '.join('%s %d' % (x, by.get(x, 0)) for x in SUITES))
    if a.dry_run:
        print('dry run: %d installed and restored, %d invalid' % (cnt.get('dry-run', 0), cnt.get('invalid', 0)))
    surv = [r for r in recs if r['verdict'] == 'survived']
    if surv:
        print('survivors:')
        for r in surv:
            print('  %s %s:%d [%s] %s' % (r['id'], r['file'], r['line'], r['op'], r['mutated'].strip()[:110]))
    for r in recs:
        if r['verdict'] in ('invalid', 'stillborn', 'uncovered'):
            print('  %s %s %s:%s %s' % (r['verdict'], r['id'], r['file'], r.get('line', '?'), r.get('reason', '')[:120]))
    print('results: ' + out_path)
    return 0

sys.exit(main())
PYEOF

python3 -I "$WORK/mut.py" --repo "$REPO_ROOT" --work "$WORK" "${ARGS[@]}"
