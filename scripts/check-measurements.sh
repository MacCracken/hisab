#!/usr/bin/env bash
# Verify that every measurement-shaped claim in a comment carries an explicit
# statement of where the number came from.
#
# WHY THIS EXISTS
# ---------------
# Twice in two cycles a MEASUREMENT stated in a committed comment turned out not
# to reproduce:
#
#   * an eigen_qr justification claiming a call "returns HSB_ERR_NONE carrying
#     the spectrum of the leading 3x3 block" -- it returns -8 at every budget
#     from 50 to 10,000,000;
#   * cqr_decompose arena figures "15,719,272 B before, 15,687,016 B after,
#     -0.21%" -- the true figures are 32,942,104 / 32,909,848 / -0.098%, wrong
#     by ~2x, and they had propagated into src/, tests/, the audit table AND the
#     CHANGELOG before anyone re-ran them.
#
# Both were caught by an adversarial reader, not by a gate.
#
# WHAT THIS CAN AND CANNOT CHECK
# ------------------------------
# It cannot check the NUMBER. A measurement in prose has no machine-checkable
# relationship to the code; "1.2 ms" is not derivable from the source. Unlike
# scripts/check-constants.sh -- which decodes the bits next to the claim and so
# verifies the claim itself -- this gate can only check PROVENANCE: that the
# author said where the number came from, in a form a later reader can re-run.
#
# That is a weaker guarantee, and it is the one that would have caught both
# incidents above. Neither number was wrong because the author mistyped it;
# both were wrong because nobody could tell, a cycle later, which harness had
# produced them, so nobody re-ran them.
#
# THE MARKER
#   [measured: tests/hisab.tcyr:2433]                    a file:line that re-derives it
#   [measured: bench-history.csv num_dct_1024]           a named benchmark row
#   [measured: scripts/check-constants.sh]               a gate that reproduces it
#   [measured: scratch harness, not reproducible in-tree] an honest dead end
#
# The last form is accepted deliberately. A number whose harness is gone is a
# number a reader should distrust, and saying so is strictly better than
# silence. What is NOT accepted is a measurement with no marker at all.
#
# GRANULARITY
# One marker satisfies its whole comment PARAGRAPH -- a run of adjacent comment
# lines, split at blank `#` lines. A paragraph makes one argument; it cites one
# source. Blocks are not the unit (a 70-line preamble makes six arguments), and
# lines are not the unit (a table of timings would need a marker per row).
#
# Usage:
#   ./scripts/check-measurements.sh                 full scan, report + exit 1 on any unmarked
#   ./scripts/check-measurements.sh --list          full scan, print every flagged line
#   ./scripts/check-measurements.sh --diff [REF]    gate ONLY comment lines added vs REF
#                                                   (default origin/main, else HEAD)
#   ./scripts/check-measurements.sh --ratchet       fail only if a file's unmarked count
#                                                   ROSE above scripts/measurement-baseline.txt
#   ./scripts/check-measurements.sh --update-baseline    rewrite that baseline
#   ./scripts/check-measurements.sh --sample N [SEED]    print N random flagged lines
#                                                        (for hand-auditing the false-positive rate)
#
# ADOPTION -- read this before wiring it into CI.
#
# A full scan of this tree at v2.8.4 reports 417 unmarked claim lines in 247
# paragraphs across 23 files. That is pre-existing debt, not a reason to loosen
# the pattern, and it means the DEFAULT mode cannot be a CI gate today.
#
# --diff is cheaper but is NOT free either. Measured against real history:
#     base        added lines   unmarked lines   markers needed
#     HEAD~1           1,114               99               42
#     HEAD~2           1,168               99               42
#     HEAD~3           1,878              133               64
#     HEAD~5           3,757              157               75
# So a single recent commit of this repo would have had to write ~42 markers.
# That is the honest price of turning --diff on, and it is a price paid by the
# author who is writing the numbers while they still remember where they came
# from -- which is the entire point.
#
# --ratchet is the one mode that is green RIGHT NOW (417 == 417 baseline), so it
# can be enabled today with zero back-annotation. It then fails the first commit
# that adds an unmarked claim, which is the same incidence as --diff without
# needing a base ref. Recommended order: --ratchet now, --diff once the marker
# convention is habitual, default full-scan if the debt ever reaches zero.
#
# Hand-audited false-positive rate at the pattern shipped here: 8 of 46
# classified lines (17.4%) are incidental numbers rather than unbacked
# measurements -- 6 of 46 (13.0%) once judged at the paragraph level, which is
# the unit an author actually has to act on. Sample: `--sample 40 42` plus
# exhaustive inspection of every line matched by the 'measurements' noun and the
# 'behavior' shape. The residual false positives are derived bounds
# ("n = 1e9 asks for 8 GB"), named IEEE-754 values ("4.94e-324, the smallest
# subnormal"), and harness-contract prose ("any failure at all is exit 1").
#
# Exit: 0 = clean for the selected mode; 1 = at least one unmarked claim
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

MODE="scan"
ARG=""
ARG2=""
case "${1:-}" in
    --list)            MODE="list" ;;
    --diff)            MODE="diff"; ARG="${2:-}" ;;
    --ratchet)         MODE="ratchet" ;;
    --update-baseline) MODE="update-baseline" ;;
    --sample)          MODE="sample"; ARG="${2:-30}"; ARG2="${3:-0}" ;;
    "")                : ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
esac

# --diff needs the set of comment lines this branch ADDED. Resolve the base ref
# here (bash) rather than in python so the failure is a plain git error.
ADDED_FILE=""
if [ "$MODE" = "diff" ]; then
    BASE="$ARG"
    if [ -z "$BASE" ]; then
        if git rev-parse --verify -q origin/main >/dev/null; then BASE="origin/main"; else BASE="HEAD"; fi
    fi
    git rev-parse --verify -q "$BASE" >/dev/null || { echo "no such ref: $BASE" >&2; exit 2; }
    ADDED_FILE="$(mktemp)"
    trap 'rm -f "$ADDED_FILE"' EXIT
    # -U0 so every hunk header gives the exact added-line span in the NEW file.
    git diff -U0 "$BASE" -- 'src/*.cyr' 'tests/*.cyr' 'tests/*.tcyr' 'tests/*.bcyr' 'tests/*.fcyr' \
        | awk '
            # A deleted file gets `+++ /dev/null`; without this reset, `f` would
            # still hold the PREVIOUS file and its hunks would be misattributed.
            /^\+\+\+ \/dev\/null/ { f = ""; next }
            /^\+\+\+ b\// { f = substr($0, 7); next }
            /^@@/ {
                if (f == "") next
                # @@ -a,b +c,d @@
                s = $3; sub(/^\+/, "", s)
                n = split(s, p, ",")
                start = p[1] + 0; count = (n > 1 ? p[2] + 0 : 1)
                for (i = 0; i < count; i++) print f ":" (start + i)
            }
        ' > "$ADDED_FILE"
    echo "diff mode: base=$BASE, $(wc -l < "$ADDED_FILE") added line(s) in scanned paths"
fi

python3 - "$MODE" "$ARG" "$ARG2" "$ADDED_FILE" <<'PYEOF'
import glob, os, random, re, sys

mode, arg, arg2, added_file = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]

BASELINE = 'scripts/measurement-baseline.txt'
SCAN = sorted(glob.glob('src/*.cyr')) + sorted(glob.glob('tests/*.tcyr')) \
     + sorted(glob.glob('tests/*.bcyr')) + sorted(glob.glob('tests/*.fcyr'))

# ---------------------------------------------------------------------------
# 1. COMMENT EXTRACTION
#
# Cyrius comments run from `#` to end of line. `#` inside a string literal is
# not a comment, so strings are walked rather than regexed away -- a naive
# `#.*$` would mis-slice any line holding a `"...#..."` and silently change what
# the detector sees.
def comment_of(line):
    i, n, in_str, esc = 0, len(line), False, False
    while i < n:
        c = line[i]
        if in_str:
            if esc: esc = False
            elif c == '\\': esc = True
            elif c == '"': in_str = False
        elif c == '"': in_str = True
        elif c == '#': return line[i + 1:].rstrip('\n')
        i += 1
    return None

# ---------------------------------------------------------------------------
# 2. NEUTRALISE INCIDENTAL NUMBERS
#
# Each of these is a number that is NOT a measurement, and every one of them was
# a false positive in an earlier revision of this detector. They are blanked
# before the measurement patterns run, so e.g. `linalg_precision.cyr:1392` can
# never be read as "1392 of something".
NEUTRAL = [
    (re.compile(r'https?://\S+'), ' URL '),                              # links
    (re.compile(r'\b[\w./-]+\.(?:cyr|tcyr|bcyr|fcyr|cyml|toml|sh|md|csv|ya?ml|txt|py)'
                r'(?::\d+(?:-\d+)?)?(?::\d+)?'), ' PATH '),              # foo.cyr:1392
    (re.compile(r'\b\d{4}-\d{2}-\d{2}\b'), ' DATE '),                    # 2026-08-03
    (re.compile(r'\bv?\d+\.\d+\.\d+\b'), ' VER '),                       # 2.8.4, v6.5.6
    (re.compile(r'0[xX][0-9A-Fa-f_]+'), ' HEX '),                        # bit patterns
    (re.compile(r'\bIEEE-754\b|\bUTF-8\b|\bSHA-?256\b|\bGPL-3\.0\b'), ' STD '),
    (re.compile(r'\bP\(-?\d+\)|\bP[0-4]\b'), ' TIER '),                  # P(-1), P0
    (re.compile(r'\b(?:radix|base)-\d+\b'), ' RADIX '),
    (re.compile(r'\b[A-Za-z_]\w*\s*=\s*[-+]?\d+(?:\.\d+)?\b'), ' ASSIGN '),  # n = 4096, max_iter=100000
    (re.compile(r'\b\d+\s*(?:x|X)\s*\d+(?:\s*(?:x|X)\s*\d+)*\b'), ' DIM '),  # 64x64, 2x2x2
    (re.compile(r'\b[a-zA-Z]\^\d+\b|\b\d+\^-?\d+\b|\bO\([^)]*\)|\bTheta\([^)]*\)'), ' FORMULA '),
    (re.compile(r'\bHSB_[A-Z0-9_]+\b|\b_[A-Z][A-Z0-9_]+\b'), ' CONST '),
    # A parenthetical holding nothing but a size is a unit gloss on a declared
    # cap -- `_TENSOR_MAX_ELEMS = 1048576;  # 1M elements max (8MB)`. It restates
    # the constant on the same line, so it cannot be a measurement.
    (re.compile(r'\(\s*[-+]?\d+(?:\.\d+)?\s*[KMG]i?B\s*\)'), ' SIZEGLOSS '),
]

def neutralise(text):
    for rx, rep in NEUTRAL:
        text = rx.sub(rep, text)
    return text

# ---------------------------------------------------------------------------
# 3. MEASUREMENT SHAPES
#
# Each entry is (kind, regex). A comment line matching any of them asserts a
# number that only an execution could have produced.
NUM = r'(?:\d{1,3}(?:,\d{3})+|\d+(?:\.\d+)?)'
# Grouped, k/M-suffixed, or unit-prefixed magnitudes only -- see 'bytes' below.
BIGNUM = r'(?:\d{1,3}(?:,\d{3})+|\d+(?:\.\d+)?\s*[KMG]i?(?=B))'
# Nouns that turn a percentage into a chosen parameter rather than an observation:
# "a 10% surcharge", "a 0.7% window", "within 5% tolerance".
PCT_PARAM = r'(?!\s*(?:band|window|surcharge|threshold|tolerance|margin|budget|slack|cushion|guard|headroom)\b)'

SHAPES = [
    # An explicit claim of having run something. Highest confidence by far --
    # the author is asserting an observation, whatever the number looks like.
    # Bare "timed" is NOT here: "the timed loop", "per timed call" describe
    # methodology, not a result, and were 2 of the first 40 sampled false hits.
    # Bare "benchmark" is NOT here either: a .bcyr file says it 25 times as a
    # noun ("Collision benchmark data"). Only the past participle asserts an act.
    ('vocab', re.compile(r'\b(?:re-)?measured\b|\bmeasurements?\b|\bbenchmarked\b|\bprofiled\b'
                         r'|\bbatch-timed\b|\bon this machine\b'
                         r'|\brun-to-run\b|\bwall-clock\b', re.I)),
    # Wall time. `s` alone is excluded: too many words start with it and a bare
    # "3 s" does not occur in this corpus, while "3 samples" does.
    ('time',  re.compile(r'\b' + NUM + r'\s*(?:ns|us|µs|ms)\b(?:\s*(?:/|per)\s*\w+)?')),
    # Storage -- but ONLY at a magnitude a human had to group or scale. Plain
    # small byte counts are struct layouts ("{x,y,z,w} = 32 bytes", "base + off
    # + 8 bytes"), which were the single largest false-positive class in the
    # first sample: 5 of 40. A byte figure that came from alloc_used() in this
    # repo is always grouped (32,942,104 B) or scaled (2.07 MB).
    ('bytes', re.compile(r'\b' + BIGNUM + r'\s*(?:[KMG]i?B|B|bytes?)\b')),
    # Percentages, including deltas written -0.098% / +2.6%.
    ('pct',   re.compile(r'[-+−]?' + NUM + r'\s*%' + PCT_PARAM)),
    # Speedups. `\d+x\d+` was already blanked to DIM above, so 64x64 is gone.
    # The lookahead drops algebra: in "240x + 46y" the x is a variable, and an
    # operator or a lone letter follows it; in "581x faster" a word does.
    # Integer 2x and 3x are excluded outright. Every one of the 14 in this tree
    # is either the variable x ("2x is nonzero", "2x at 3") or the English
    # "twice the" ("2x area of the convex hull") -- none is a measured ratio.
    # 4x and up, and any decimal ratio, are always speedups here.
    ('ratio', re.compile(r'\b(?:[4-9]|\d{2,})\d*x\b(?!\s*(?:[\d+\-*/^=]|[a-zA-Z]\b))'
                         r'|\b\d+\.\d+x\b(?!\s*(?:[\d+\-*/^=]|[a-zA-Z]\b))')),
    # Tallies. The word form is unambiguous; the slash form only counts when the
    # two sides are equal AND >= 2 (40/40, 1127/1127) -- 4/3 and 1/2 are
    # fractions and 0/0 is a division-by-zero being described.
    ('tally', re.compile(r'\b\d+\s+(?:of|out of)\s+\d+\b|\b([2-9]\d*|\d{2,})/(?=\1\b)\d+')),
    # Comma-grouped magnitudes (538,608) and k/M suffixes (766k, 1.4M). A number
    # a human bothered to group is a counted quantity, not an algorithm constant.
    # Round scaled caps ("1M elements max") are excluded by the lookahead.
    # The k/M form needs two integer digits (189k, 766k) or a decimal (33.5M).
    # A single digit before k is the summation variable: `-PI/2 + 2k*PI`.
    ('count', re.compile(r'\b\d{1,3}(?:,\d{3})+\b'
                         r'|\b(?:\d{2,}|\d+\.\d+)[kKM]\b'
                         r'(?!\s*(?:elements?|entries|items|max|limit|cap)\b)')),
    # Observed process outcomes.
    ('exit',  re.compile(r'\bexit\s+\d+\b|\bSIGSEGV\b|\bSIGFPE\b|\bSIGABRT\b')),
    # Error figures at 3-7 significant figures: 1.34e-13 and 8.88e-16 are things
    # someone observed. 1e-12 is a tolerance someone chose, and a 10+ digit
    # mantissa (1.220703125e-4 = h^2/8, 7.947285970052083e-09 = 2/(15*2^24)) is
    # an exact closed form being expanded, not a measurement.
    ('error', re.compile(r'\b\d\.\d{2,6}e[-+]?\d+\b(?!\d)')),
]

# Shapes that must see the RAW comment, because the neutralisers above would
# destroy the very thing they key on.
#
# 'behavior' exists for the eigen_qr incident, which the nine numeric shapes all
# miss: "at max_iter = 10000 the same call returns HSB_ERR_NONE carrying the
# spectrum of the leading 3x3 block" has no byte count, no timing and no
# percentage -- it is a claim about a RETURN VALUE, and it was false (the call
# returns -8 at every budget from 50 to 10,000,000).
#
# A bare "returns HSB_ERR_X" cannot be flagged: every API doc comment in the
# repo says that, as a CONTRACT, universally quantified. What distinguishes a
# measurement is that it is conditioned on a CONCRETE numeric setting -- the
# author is reporting one run, not stating a spec. Requiring both halves costs
# 2 matched lines tree-wide and catches the incident text verbatim.
RAW_SHAPES = [
    ('behavior', re.compile(
        r'(?=.*\b(?:returns?|returned|comes? back|answers?)\b)'
        r'(?=.*\b(?:at|with|for|on)\s+[\w_]+\s*=\s*[-+]?\d)')),
]

MARKER = re.compile(r'\[measured:\s*([^\]]+?)\s*\]', re.I)
# A marker written wrong is worse than none -- it reads as provenance and is not
# machine-locatable. Caught separately so the message can say what is malformed.
MARKER_LOOSE = re.compile(r'\[\s*measured\b[^\]]*\]?', re.I)


def shapes_in(comment):
    n = neutralise(comment)
    found = {k for k, rx in SHAPES if rx.search(n)}
    found |= {k for k, rx in RAW_SHAPES if rx.search(comment)}
    return sorted(found)

# ---------------------------------------------------------------------------
# 4. PARAGRAPHS
#
# A paragraph is a run of adjacent comment-bearing lines, split wherever the
# comment is empty (a bare `#`). One marker anywhere in a paragraph satisfies
# every claim in it.
class Para:
    __slots__ = ('path', 'lines', 'markers')
    def __init__(self, path):
        self.path, self.lines, self.markers = path, [], []

def paragraphs(path):
    out, cur, prev_no = [], None, None
    with open(path, encoding='utf-8', errors='replace') as fh:
        for no, line in enumerate(fh, 1):
            c = comment_of(line)
            if c is None or not c.strip():
                # A bare `#` ends the paragraph but a code line does too.
                cur = None
                prev_no = no if (c is not None) else None
                continue
            if cur is None or prev_no != no - 1:
                cur = Para(path); out.append(cur)
            cur.lines.append((no, c))
            m = MARKER.search(c)
            if m: cur.markers.append(m.group(1).strip())
            prev_no = no
    return out

# ---------------------------------------------------------------------------
# 5. MARKER PROVENANCE
#
# The marker names a source; check that the source EXISTS. A marker pointing at
# a deleted test is exactly the state the gate is meant to prevent.
BENCH_CSV = 'bench-history.csv'
_bench_names = None
def bench_names():
    global _bench_names
    if _bench_names is None:
        _bench_names = set()
        if os.path.exists(BENCH_CSV):
            with open(BENCH_CSV, encoding='utf-8', errors='replace') as fh:
                for row in fh:
                    f = row.split(',')
                    if len(f) >= 4: _bench_names.add(f[3].strip().rstrip(':'))
    return _bench_names

NOT_REPRO = re.compile(r'\bnot reproducible in-tree\b|\bno longer reproducible\b', re.I)

def check_marker(src):
    """Return (ok, note). ok=False means the marker names something absent."""
    if NOT_REPRO.search(src):
        return True, 'declared non-reproducible'
    m = re.match(r'^([\w./-]+?\.\w+)(?::(\d+))?', src)
    if m:
        path, ln = m.group(1), m.group(2)
        if not os.path.exists(path):
            return False, f'names a file that does not exist: {path}'
        if ln:
            with open(path, encoding='utf-8', errors='replace') as fh:
                total = sum(1 for _ in fh)
            if int(ln) > total:
                return False, f'{path} has {total} lines, marker cites line {ln}'
        if os.path.basename(path) == BENCH_CSV:
            rest = src[m.end():].strip()
            if rest and rest.split()[0] not in bench_names():
                return False, f'no benchmark named {rest.split()[0]!r} in {BENCH_CSV}'
        return True, f'resolves to {path}'
    return True, 'free-text provenance (not machine-locatable)'

# ---------------------------------------------------------------------------
# 6. SCAN
flagged = []      # (path, lineno, kinds, text)   -- unmarked measurement lines
bad_marker = []   # (path, lineno, src, note)
satisfied = 0     # measurement lines covered by a marker
per_file = {}
unmarked_paras = 0    # paragraphs needing a marker -- the actionable unit
marked_paras = 0
para_of = {}          # (path, lineno) -> paragraph id, so any subset of flagged
                      # lines can be costed in markers rather than in lines

for path in SCAN:
    for p in paragraphs(path):
        hits = [(no, c, shapes_in(c)) for no, c in p.lines]
        hits = [(no, c, k) for no, c, k in hits if k]
        if not hits:
            # No claim here, but a marker on a claimless paragraph is dead
            # weight pointing nowhere -- not an error, just not counted.
            continue
        if p.markers:
            satisfied += len(hits)
            marked_paras += 1
            for src in p.markers:
                ok, note = check_marker(src)
                if not ok:
                    bad_marker.append((p.path, p.lines[0][0], src, note))
        else:
            unmarked_paras += 1
            pid = (path, p.lines[0][0])
            for no, c, k in hits:
                flagged.append((path, no, k, c.strip()))
                para_of[(path, no)] = pid
                per_file[path] = per_file.get(path, 0) + 1
    # A marker typo'd into unrecognisability is reported wherever it appears.
    with open(path, encoding='utf-8', errors='replace') as fh:
        for no, line in enumerate(fh, 1):
            c = comment_of(line)
            if c and MARKER_LOOSE.search(c) and not MARKER.search(c):
                bad_marker.append((path, no, c.strip(), 'malformed marker -- want [measured: SOURCE]'))

total_claims = len(flagged) + satisfied

# ---------------------------------------------------------------------------
# 7. MODES
def show(rows, limit=None):
    for i, (path, no, kinds, text) in enumerate(rows):
        if limit is not None and i >= limit:
            print(f"  ... and {len(rows) - limit} more"); break
        print(f"  {path}:{no}  [{','.join(kinds)}]")
        print(f"      # {text[:150]}")

def report_bad_markers():
    if not bad_marker:
        return
    print(f"\n!! {len(bad_marker)} BROKEN PROVENANCE MARKER(S)\n")
    for path, no, src, note in bad_marker:
        print(f"  {path}:{no}")
        print(f"      marker: {src[:120]}")
        print(f"      note  : {note}")
    print()

if mode == 'sample':
    n = int(arg or 30)
    random.seed(int(arg2 or 0))
    pick = random.sample(flagged, min(n, len(flagged)))
    print(f"=== {len(pick)} of {len(flagged)} flagged lines, seed {arg2 or 0} ===\n")
    show(pick)
    sys.exit(0)

if mode == 'update-baseline':
    with open(BASELINE, 'w', encoding='utf-8') as fh:
        fh.write("# scripts/check-measurements.sh --ratchet baseline.\n")
        fh.write("# Per-file count of measurement-shaped comment claims carrying NO\n")
        fh.write("# [measured: ...] marker. These are pre-existing debt, not approval:\n")
        fh.write("# the ratchet only forbids a file's count from RISING. Lower a number\n")
        fh.write("# whenever you annotate; never raise one.\n")
        fh.write(f"# total unmarked at time of writing: {len(flagged)}\n")
        for path in sorted(per_file):
            fh.write(f"{per_file[path]}\t{path}\n")
    print(f"wrote {BASELINE}: {len(per_file)} file(s), {len(flagged)} unmarked claim(s)")
    sys.exit(0)

if mode == 'ratchet':
    if not os.path.exists(BASELINE):
        print(f"no {BASELINE} -- run --update-baseline first"); sys.exit(2)
    base = {}
    with open(BASELINE, encoding='utf-8') as fh:
        for row in fh:
            if row.startswith('#') or not row.strip(): continue
            cnt, path = row.split('\t', 1)
            base[path.strip()] = int(cnt)
    risen = [(p, base.get(p, 0), per_file[p]) for p in sorted(per_file)
             if per_file[p] > base.get(p, 0)]
    fell = sum(max(0, base.get(p, 0) - per_file.get(p, 0)) for p in base)
    print(f"=== measurement ratchet: {len(flagged)} unmarked, "
          f"{sum(base.values())} at baseline ({fell} paid down) ===")
    report_bad_markers()
    if risen:
        print(f"\n!! {len(risen)} FILE(S) GAINED UNMARKED MEASUREMENT CLAIMS\n")
        for p, b, n in risen:
            print(f"  {p}: {b} -> {n}")
            show([f for f in flagged if f[0] == p], limit=8)
        print("\nAdd [measured: SOURCE] to the new claims, or lower nothing and fix the count.")
        sys.exit(1)
    if bad_marker: sys.exit(1)
    print("No file gained an unmarked measurement claim.")
    sys.exit(0)

if mode == 'diff':
    added = set()
    if added_file and os.path.exists(added_file):
        with open(added_file, encoding='utf-8') as fh:
            for row in fh:
                row = row.strip()
                if ':' in row:
                    p, _, n = row.rpartition(':')
                    added.add((p, int(n)))
    new = [f for f in flagged if (f[0], f[1]) in added]
    new_bad = [b for b in bad_marker if (b[0], b[1]) in added]
    # Report the marker cost, not the line count: one marker fixes a paragraph.
    new_paras = {para_of[(f[0], f[1])] for f in new}
    print(f"=== measurement gate (added lines only): {len(new)} unmarked claim line(s) "
          f"in {len(new_paras)} paragraph(s), {len(flagged)} unmarked tree-wide ===")
    if new_paras:
        print(f"    -> {len(new_paras)} [measured: ...] marker(s) would clear this branch")
    if new_bad:
        print(f"\n!! {len(new_bad)} BROKEN PROVENANCE MARKER(S) ON ADDED LINES\n")
        for path, no, src, note in new_bad:
            print(f"  {path}:{no}\n      marker: {src[:120]}\n      note  : {note}")
    if new:
        print(f"\n!! {len(new)} NEW MEASUREMENT CLAIM(S) WITH NO PROVENANCE\n")
        show(new)
        print("\nA number nobody can re-derive is a number nobody will re-check. Add")
        print("  [measured: tests/hisab.tcyr:NNNN]  or  [measured: bench-history.csv NAME]")
        print("to the comment paragraph, or say [measured: scratch harness, not reproducible in-tree].")
    if new or new_bad: sys.exit(1)
    print("Every measurement claim added on this branch names its source.")
    sys.exit(0)

# default: full scan
kinds = {}
for _, _, ks, _ in flagged:
    for k in ks: kinds[k] = kinds.get(k, 0) + 1
print(f"=== measurement check: {satisfied}/{total_claims} claim lines carry provenance, "
      f"{len(flagged)} unmarked across {len(per_file)} file(s) ===")
print(f"    paragraphs (the unit a marker satisfies): {marked_paras} marked, "
      f"{unmarked_paras} unmarked")
print("    by shape: " + ', '.join(f"{k}={v}" for k, v in sorted(kinds.items(), key=lambda x: -x[1])))
report_bad_markers()
if mode == 'list':
    print()
    show(flagged)
if flagged:
    if mode != 'list':
        print("\nrun with --list to see them, --sample N to audit a random subset")
    print("\nA measurement in a comment is an assertion about an execution. Without a")
    print("marker naming the harness, the next reader cannot re-run it -- which is")
    print("exactly how the eigen_qr and cqr_decompose figures survived being wrong.")
    sys.exit(1)
if bad_marker: sys.exit(1)
print("Every measurement-shaped claim names its source.")
PYEOF
