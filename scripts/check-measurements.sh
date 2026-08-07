#!/usr/bin/env bash
# Verify that every measurement-shaped claim in a comment carries an explicit
# statement of where the number came from.
#
# WHY THIS EXISTS
# ---------------
# Three times now a MEASUREMENT stated in committed text has turned out not to
# reproduce:
#
#   * an eigen_qr justification claiming a call "returns HSB_ERR_NONE carrying
#     the spectrum of the leading 3x3 block" -- it returns -8 at every budget
#     from 50 to 10,000,000;
#   * cqr_decompose arena figures "15,719,272 B before, 15,687,016 B after,
#     -0.21%" -- the true figures are 32,942,104 / 32,909,848 / -0.098%, wrong
#     by ~2x, and they had propagated into src/, tests/, the audit table AND the
#     CHANGELOG before anyone re-ran them.
#   * an abuse-register reproducer recorded as exit 139 --
#     `cmat_get(cmat_new(2, 2), -1, 0)` -- which re-measures as exit 0 returning
#     0+0i (CHANGELOG: "The register's reproducer did not reproduce").
#
# All three were caught by an adversarial reader, not by a gate.
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
# RECALL IS THE HARD HALF
# -----------------------
# The false-positive rate of this detector can be audited from the tree: --sample
# prints flagged lines and a human classifies them. RECALL cannot -- nothing in
# the tree tells you which real claims the pattern walked past. The first version
# of this gate was audited that way, looked healthy on its false-positive rate
# (13% paragraph-level), and then missed 8 of 10 realistically-phrased planted
# claims -- and 12 of the 22 in the corpus below, re-measured here. A gate that
# flags noise and misses claims is worse than no gate, because it is trusted. So
# the planted corpus now lives in the script and `--selftest` re-runs it (see
# SELFTEST below): 21 of 22 claims flagged, 0 of 15 decoys, one known miss
# documented in place.
#
# The shapes added to close that gap, and what each one exists for:
#   time      spelled-out durations -- "0.9 seconds before, 0.4 after", "120 s"
#   bytes     ungrouped magnitudes  -- "998400 bytes to 512000 bytes"
#   ratio     the word forms        -- "three times faster", "twice as fast",
#                                      "an order of magnitude slower"
#   rate      throughput            -- "1200 ops/sec to 1900 ops/sec"
#   residual  an observed VALUE     -- "the weights summed to 2.000492 instead of 2"
#   crash     an observed OUTCOME   -- "passing a null handle here segfaults the
#                                      process", "hung forever", "never returned"
#   arrow     the house before/after style -- "support calls 901 -> 1790"
#   exit      "exits 0" as well as "exit 139" (the third incident above was
#             exactly this form and the old pattern did not see it)
#
# Usage:
#   ./scripts/check-measurements.sh                 full scan, report + exit 1 on any unmarked
#   ./scripts/check-measurements.sh --selftest      re-measure recall on the planted corpus
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
# Measured 2026-08-06 on 2399022 plus the working tree. Tree counts drift with
# every commit; re-run rather than quoting these.
#
#   default full scan  519 unmarked claim lines in 287 paragraphs, 26 files
#   --ratchet          RED. The baseline records 417; the tree holds 519. 61 of
#                      that gap predates the widening (4 files were already above
#                      baseline under the OLD pattern -- ongoing 2.9.x work added
#                      unmarked claims and nobody re-baselined). --ratchet is
#                      therefore NOT enableable today without a deliberate
#                      `--update-baseline`, and that decision forgives those 61
#                      lines, so it is the author's to make, not this script's.
#   --diff             GREEN-ABLE, and cheap. Against real history:
#                          base      added lines   unmarked   markers needed
#                          HEAD~1          199          6            4
#                          HEAD~2          258          6            4
#                          HEAD~3          319          9            6
#                          HEAD~5          681         41           14
#                      A commit-sized change costs 4-6 markers, paid by the
#                      author while they still remember where the number came
#                      from -- which is the entire point. (An older revision of
#                      this header quoted 42 markers for HEAD~1; that was a
#                      1,114-line commit at v2.8.4, not a different pattern.)
#
# RECOMMENDATION: enable --diff. Leave the default full scan and --ratchet out of
# CI until the debt is deliberately dealt with.
#
# FALSE POSITIVES -- hand-audited at the pattern shipped here, `--sample 40 91`,
# all 40 lines classified. (The draw is seeded but the population is the tree, so
# re-running the same seed later draws a different 40.)
#     11 of 40 lines        (27.5%) are not unbacked measurements
#      8 of 38 paragraphs   (21.1%) once judged at the paragraph level, which is
#                                   the unit an author has to act on
# The widening measured on its own -- the 41 lines it added over the previous
# pattern, every one classified -- is 8 false positives (19.5%): 'arrow' 1 of 11,
# 'crash' 2 of 10, 'residual' 5 of 15, and 0 of 6 across exit/ratio/time.
# 'residual' is the loosest shape here and is the one to cut first if this ever
# gets annoying.
#
# The false positives are of five kinds, all previously known: derived bounds
# ("128 MB at 8 B/elem", "+14.3%"), named IEEE-754 values ("4.94e-324, the
# smallest subnormal"), scale PARAMETERS read as ratios (3 of the 40 sampled
# lines say "10x" meaning the super-triangle scale, not a speedup, so the 'ratio'
# shape's claim that "4x and up is always a speedup here" is no longer true),
# closed-form arithmetic near an observation verb
# ("2^3 is 7.999...", "gives 5/24 = 0.208333, off by 8.3e-3"), and cross
# references that point AT the provenance ("see that constant for the measured
# reason").
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
    --selftest)        MODE="selftest" ;;
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
        if git rev-parse --verify -q origin/main >/dev/null; then
            BASE="origin/main"
        else
            # The HEAD fallback gates only UNCOMMITTED comment lines. On a clean
            # checkout that is nothing at all, which is exactly how a CI gate
            # goes green forever while checking nothing -- so say it out loud.
            BASE="HEAD"
            echo "NOTE: no origin/main in this clone; falling back to --diff HEAD, which" >&2
            echo "      gates only UNCOMMITTED lines. In CI, pass the base ref explicitly" >&2
            echo "      and check out with fetch-depth: 0." >&2
        fi
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
# Countable units that make `N unit / time` a THROUGHPUT rather than a stride.
# Deliberately narrow: "2/iter" (a loop step) and "3 entries per point" (a layout)
# are not throughputs, and both occur in this tree.
RATE_UNIT = (r'(?:ops?|iters?|iterations?|calls?|queries|points?|elements?|samples?|rows?'
             r'|nodes?|frames?|inserts?|lookups?|updates?|bytes?|[KMG]i?B)')
TIME_UNIT = r'(?:s|secs?|seconds?|ms|us|µs|ns|mins?|minutes?|hours?)'
# Verbs that report an OBSERVED value. Paired with a high-precision decimal (see
# 'residual') they mark the eigen_qr class of claim: "the call came back X".
# `gives` and `gave` are NOT here, and neither is `actually`. In a math library
# "gives" is the verb of DERIVATION -- "the single panel gives 5/24 = 0.208333",
# "Euler gives 2.0, RK3 8/3 = 2.6666667" -- and a derivation carries its own
# proof; a reader re-derives it instead of re-running it. Including them cost 4
# false positives out of 18 in the hand audit and bought one true positive.
OBSERVED = (r'(?:instead of|rather than|off by|but got|came back|came out|summed? to|totall?ed'
            r'|drifted|overshot|undershot|returned|reported|rendered|printed|observed'
            r'|converged to|stalled at|landed at|,\s*not\b)')

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
    # Wall time. Sub-second units, plus the spelled-out units a slow case gets
    # reported in ("did not terminate in 120 s", "0.9 seconds before, 0.4 after").
    # Bare `s` requires WHITESPACE in front of it: "120 s" is a duration, while
    # "all 1s", "4 f64s" and "150 random symmetric 4x4s" are not, and all three
    # occur here.
    ('time',  re.compile(r'\b' + NUM + r'\s*(?:ns|us|µs|ms)\b(?:\s*(?:/|per)\s*\w+)?'
                         r'|\b' + NUM + r'\s*(?:secs?|seconds?|mins?|minutes?|hours?)\b'
                         r'|\b' + NUM + r'\s+s\b')),
    # Throughput. A rate is a measurement even when its magnitude is unremarkable:
    # "1200 ops/sec to 1900 ops/sec" carries no grouped number, no unit prefix and
    # no percentage, so every other shape here misses it.
    ('rate',  re.compile(r'\b' + NUM + r'\s*' + RATE_UNIT + r'\s*(?:/|\s+per\s+)' + TIME_UNIT + r'\b'
                         r'|\b' + NUM + r'\s+per\s+second\b', re.I)),
    # Storage -- but ONLY at a magnitude a human had to group or scale. Plain
    # small byte counts are struct layouts ("{x,y,z,w} = 32 bytes", "base + off
    # + 8 bytes"), which were the single largest false-positive class in the
    # first sample: 5 of 40. A byte figure that came from alloc_used() in this
    # repo is always grouped (32,942,104 B) or scaled (2.07 MB).
    # ...but the SIZE of the number decides, not the way it is written: an
    # UNGROUPED four-digit-or-longer byte count ("998400 bytes to 512000 bytes")
    # is still an arena figure. Nothing in this tree writes a struct layout that
    # large, so the widening costs zero false positives here.
    ('bytes', re.compile(r'\b' + BIGNUM + r'\s*(?:[KMG]i?B|B|bytes?)\b'
                         r'|\b\d{4,}(?:\.\d+)?\s*(?:B|bytes?)\b')),
    # Percentages, including deltas written -0.098% / +2.6%, and the spelled-out
    # form ("12 percent slower") that the symbol alone misses.
    ('pct',   re.compile(r'[-+−]?' + NUM + r'\s*(?:%|percent\b)' + PCT_PARAM)),
    # Speedups. `\d+x\d+` was already blanked to DIM above, so 64x64 is gone.
    # The lookahead drops algebra: in "240x + 46y" the x is a variable, and an
    # operator or a lone letter follows it; in "581x faster" a word does.
    # Integer 2x and 3x are excluded outright. Every one of the 14 in this tree
    # is either the variable x ("2x is nonzero", "2x at 3") or the English
    # "twice the" ("2x area of the convex hull") -- none is a measured ratio.
    #
    # An earlier revision of this comment claimed "4x and up is always a speedup
    # here". The 2026-08-06 hand audit falsified it: "the 10x super-triangle",
    # "the 10x build returned ONE triangle" name a SCALE PARAMETER, and they were
    # 3 of the 40 lines sampled. Left in anyway -- the same form covers "244x the
    # convex hull" and "~700x clear of the noise", which are measured.
    #
    # The SAME claim written in words -- "three times faster", "twice as fast",
    # "an order of magnitude slower" -- is the same measurement and was invisible
    # to the `Nx` form. It needs the comparative to be present: "rewritten three
    # times after its first use" and "four times the cone" are both in this tree
    # and neither is a speedup, so `N times <comparative>` is required, never
    # `N times <anything>`.
    ('ratio', re.compile(r'\b(?:[4-9]|\d{2,})\d*x\b(?!\s*(?:[\d+\-*/^=]|[a-zA-Z]\b))'
                         r'|\b\d+\.\d+x\b(?!\s*(?:[\d+\-*/^=]|[a-zA-Z]\b))'
                         r'|(?i:\b(?:twice|thrice|two|three|four|five|six|seven|eight|nine|ten'
                         r'|\d+(?:\.\d+)?)\s+times\s+(?:faster|slower|quicker|cheaper|costlier'
                         r'|longer|shorter|worse|better|fewer|more|less)\b)'
                         r'|(?i:\b(?:twice|half|double|triple)\s+as\s+'
                         r'(?:fast|slow|quick|long|many|much|expensive|cheap)\b)'
                         r'|(?i:\b(?:costs?|took|takes?|saves?|spends?|is|was|are|were)\s+'
                         r'(?:[\w-]+[\s,]+){0,3}an order of magnitude\b)'
                         r'|(?i:\borders? of magnitude\s+(?:faster|slower|more|fewer|less'
                         r'|bigger|smaller|worse|better|cheaper)\b)')),
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
    # Observed process outcomes. `exits 0` and `exit code 139` are the same claim
    # as `exit 139` -- and the THIRD non-reproducing figure in this repo's history
    # was exactly one of them (a register reproducer recorded as exit 139 that
    # re-measures as exit 0; CHANGELOG "The register's reproducer did not
    # reproduce"), which the bare `exit \d+` form did not see.
    ('exit',  re.compile(r'\bexits?\s+(?:code\s+|status\s+|with\s+)?\d+\b'
                         r'|\bexit(?:s|ed)?\s+non-?zero\b'
                         r'|\bSIGSEGV\b|\bSIGFPE\b|\bSIGABRT\b|\bSIGBUS\b|\bSIGILL\b')),
    # A crash, a hang or a non-return is a measurement with the number left off:
    # "passing a null handle here segfaults the process" is an assertion about an
    # execution and nothing else in this file sees it. The lookbehinds drop the
    # NOUN uses, which are discussion rather than observation -- "effectively a
    # hang", "a HANG detector", "the other three cases were not crashes".
    ('crash', re.compile(r'\bseg\s?fault(?:s|ed|ing)?\b'
                         r'|(?<!a )(?<!the )(?<!not )(?<!any )(?<!no )'
                         r'\b(?:crashe[sd]|crashing|hangs|hung)\b'
                         r'|\bdid not terminate\b|\bnever returned\b|\bnever terminate[sd]?\b'
                         r'|\bdeadlocks?\b|\bout of memory\b|\bOOM\b|\binfinite loop\b'
                         r'|\bt(?:ook|akes?) the process down\b', re.I)),
    # Before/after pairs written as an arrow. This is the repo's house style for a
    # measured delta ("box miss 1.452 -> 1.698 us", "support calls 901 -> 1790"),
    # but `A -> B` is ALSO how it writes rewrite rules and index maps ("grade 1 ->
    # 4", "sin(a)^2 + cos(a)^2 -> 1"), so magnitude has to carry the distinction:
    # one side a 2+ decimal, or both sides 3+ digits. That keeps every measured
    # pair in the tree and drops every rewrite rule.
    ('arrow', re.compile(r'\b(?:\d+\.\d{2,}|\d{3,})\s*[\w%]*\s*(?:->|→|=>)\s*[-+]?(?:\d+\.\d+|\d{3,})'
                         r'|\b\d+(?:\.\d+)?\s*[\w%]*\s*(?:->|→|=>)\s*[-+]?\d+\.\d{2,}')),
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
#
# 'residual' is the same incident class one step down: not a wrong return CODE
# but a wrong VALUE -- "the weights summed to 2.000492 instead of 2", "ivl_sin
# returned [0.9738, 0.9854]", "concentric unit spheres returned 1.8875". None of
# these is a timing, a size or a percentage.
#
# A long decimal alone cannot be the trigger. This is a math library: 84 comment
# lines hold one, and nearly all are exact closed forms being expanded (44/45 =
# 0.97777..., the DOPRI45 tableau, the sRGB breakpoints). What separates an
# observation from an expansion is that someone SAYS a run produced it -- so the
# shape is `high-precision decimal` AND `observation verb`, never either alone.
# It runs on the raw comment because the ASSIGN neutraliser would eat the very
# thing it keys on ("exact depth = 1.3599, came back ...").
RAW_SHAPES = [
    ('behavior', re.compile(
        r'(?=.*\b(?:returns?|returned|comes? back|answers?)\b)'
        r'(?=.*\b(?:at|with|for|on)\s+[\w_]+\s*=\s*[-+]?\d)')),
    ('residual', re.compile(r'(?=.*\b\d+\.\d{3,}\b)(?=.*' + OBSERVED + r')', re.I)),
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
# 3b. RECALL CORPUS  (`--selftest`)
#
# The false-positive rate of this detector can be audited from the tree itself:
# --sample prints real flagged lines and a human classifies them. RECALL cannot.
# Nothing in the tree tells you which real claims the pattern walked past, and an
# earlier revision of this gate missed 8 of 10 planted claims while looking
# healthy on its own scan -- a gate that flags noise and misses claims is worse
# than no gate, because it is trusted.
#
# So the misses are written down. Each line below is a claim in this repo's own
# comment register, labelled with what the gate must do with it:
#
#   +  a real measurement/observation -- MUST be flagged
#   -  a decoy: a number, an outcome word or a mapping arrow that is NOT a
#      measurement -- must NOT be flagged
#   ?  a known miss, kept visible on purpose. Not a failure, but not forgotten.
#
# Adding a shape means adding its claim here first. `--selftest` exits 1 if any
# `+` goes unflagged or any `-` gets flagged, so widening the pattern cannot
# silently break a family that used to work.
SELFTEST = [
    ('+', "The twin-pairing loop went from 0.9 seconds to 0.4 on the 60k-face mesh."),
    ('+', "halfedge_build on the 200k fixture: 998400 bytes of arena before, 512000 bytes after."),
    ('+', "Sorting the sites first makes hull_2d three times faster on 50k points."),
    ('+', "The insert path sustains 1200 ops/sec today; the freelist rewrite took it to 1900 ops/sec."),
    ('+', "The Gauss-Legendre weights summed to 2.000492 instead of 2, so the rule is not exact."),
    ('+', "Passing a null handle here segfaults the process instead of returning an error code."),
    ('+', "kdtree_radius on the clustered fixture never returned; the run was killed after 90 seconds."),
    ('+', "spatial_hash_query answers in 4 minutes at cell 1e-6, which is not a usable bound."),
    ('+', "quat_slerp is twice as fast as the matrix path once the branch is hoisted."),
    ('+', "The residual came back 3.4471e-9 where the closed form says 0."),
    ('+', "num_dct_1024 costs an order of magnitude more than num_fft_1024 on this machine."),
    ('+', "Support calls on the tangency fixture: 901 -> 1790 for sphere/sphere."),
    ('+', "mat_inverse_4x4 exits 139 when the pivot row is all zeros."),
    ('+', "The bump arena peaked at 24.3 MB on the 20,000-deep chain."),
    ('+', "Verified on this tree: 274 of 400 radius queries missed a hit before the split fix."),
    ('+', "Rebuilding the tree per frame costs 8.2 ms; incremental update costs 0.7 ms."),
    ('+', "ivl_pow returned [0.9738, 1.0002] for an interval that has to contain 1."),
    ('+', "The sweep found 5 sign disagreements in 2,440 evaluations."),
    ('+', "solve_bicgstab crashed the process on a singular system before the tolerance fix."),
    ('+', "The old loop hung forever when any entry of m was NaN."),
    ('+', "On the 4,096-point fixture the second call is 12 percent slower."),
    # KNOWN MISS. A comparative with no quantity anywhere in it. Catching this
    # needs "doubles/halves/triples" as bare verbs, and those are ordinary math
    # prose here ("the map doubles the angle", "halves the interval each step"),
    # so the shape would fire on derivations rather than observations. Left open.
    ('?', "Building the mesh twice in a row doubles the arena high-water mark."),
    ('-', "items_vec: vec of interleaved [point_ptr, index, point_ptr, index, ...]"),
    ('-', 'Returns 0 for an empty input; the caller must treat 0 as "no node".'),
    ('-', "_KD_MAX_DEPTH = 64 caps the recursion; deeper splits cannot separate f64 coords."),
    ('-', "The struct is {x, y, z, w} = 32 bytes, so the stride is 4 f64s."),
    ('-', "Tolerance is 1e-12 here, chosen to sit two orders below the accumulated rounding."),
    ('-', "Simpson is exact for degree <= 3, so a 1/6, 4/6, 1/6 weighting integrates cubics."),
    ('-', "out_plane receives the axis value that separates the two partitions."),
    ('-', "A 5% band around the target keeps the guard from chattering."),
    ('-', "Newton's method is quadratic: the error squares each step, 1e-4 -> 1e-8."),
    ('-', "The 4x4 case runs the same code path as 3x3; only the stride changes."),
    ('-', "Callers pass max_iter = 100000 when they want the tolerance to decide."),
    ('-', "Grade 1 -> 4 under the dual map; no e1 part remains."),
    ('-', "The paper's Table 2 lists 12 of the 16 coefficients; the rest are zero by symmetry."),
    ('-', "alloc() returns 0 on failure, so every caller checks before storing."),
    ('-', "Rewritten three times since the first cut; the assertions below pin the shape."),
]

if mode == 'selftest':
    want_hit = [t for lab, t in SELFTEST if lab == '+']
    known    = [t for lab, t in SELFTEST if lab == '?']
    decoys   = [t for lab, t in SELFTEST if lab == '-']
    missed   = [t for t in want_hit if not shapes_in(' ' + t)]
    still    = [(t, shapes_in(' ' + t)) for t in known if shapes_in(' ' + t)]
    alarms   = [(t, shapes_in(' ' + t)) for t in decoys if shapes_in(' ' + t)]
    caught = len(want_hit) - len(missed)
    denom = len(want_hit) + len(known)
    print(f"=== selftest: recall {caught}/{denom} "
          f"({100.0 * caught / denom:.0f}%) on the planted corpus, "
          f"{len(alarms)}/{len(decoys)} decoy(s) falsely flagged ===")
    print(f"    ({len(known)} claim(s) labelled a KNOWN MISS are in that denominator)")
    for t in missed:
        print(f"  MISSED (must flag): {t}")
    for t, k in alarms:
        print(f"  FALSE ALARM [{','.join(k)}]: {t}")
    for t, k in still:
        print(f"  known miss now CAUGHT [{','.join(k)}] -- promote it to '+': {t}")
    if missed or alarms:
        sys.exit(1)
    print("Every planted claim is flagged and no decoy is.")
    sys.exit(0)

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
