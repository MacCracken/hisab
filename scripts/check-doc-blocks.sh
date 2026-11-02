#!/usr/bin/env bash
# check-doc-blocks.sh — list every public function in src/ that has no comment block.
#
# WHY THIS EXISTS
# The 3.4.0 audits counted 258 of 653 public functions with no comment block (3.3.7), and
# several of their findings were exactly "no doc block, so no contract": a caller cannot
# tell a designed fallback from a defect when nothing states the domain or the failure
# channel. That count was a scratch scan, so nothing could re-run it on a later tree. This
# is that scan, checked in (3.4.1), with a selftest and a population floor.
#
# THE RULE
# A public function is documented when a comment block sits DIRECTLY above its
# declaration: attribute lines (#must_use, #inline, #derive(...), ...) may come between.
# The block must say something: a bare `#`, or a block made only of section dividers
# (`# ----`, `# ---- Spatial hash ----`), is not a doc block. A comment above a GROUP of
# functions documents only the first. Whether the block is accurate is a reader's job;
# this checks only that one exists.
# ONE blank line may separate the block from the function when the block does not END in a
# divider: diffgeo.cyr's riemann_tensor, ricci_tensor, killing_residual, wedge_1_1 and
# christoffel_symbols, and linalg_precision.cyr's cqr_decompose, carry their argument and
# Returns contracts that way (3.4.1 review S5). A block that ends in a fence is a section
# header (`# ====` / `# Tone mapping -- returns Vec3` / `# ====`), and two blank lines
# detach any block. The rule reads presence, not content: a history or a file-wide note one
# blank line up counts as well, which is why it is limited to a single blank line.
# On the merged 3.4.1 tree, 14 public fns sit one blank line (and any attributes) below a
# block; the rule takes 8 -- the five above, ricci_scalar, einstein_tensor and
# cqr_decompose, each a contract -- and leaves 6 that end in a fence. Undocumented: 87 of
# 653 without it, 79 with it. (On the INS2 tree, before the other lanes' doc blocks, it
# took 11 of 36: 242 to 231.)
# [measured: scripts/check-doc-blocks.sh with and without the rule, merged 3.4.1 tree (s2), lane INS3]
# (The 3.4.0 audit's scan used the strict rule without the divider clause; on the 3.4.1
# starting tree both count 249 of 653, and no divider-only block sits above a public fn.)
#
# A declaration counts with any attributes leading its own line (`#must_use public fn f(`
# on one line): through the third 3.4.1 review only `#inline` was read there, and four
# such plants raised the total by 3 (review N6).
#
# THE RATCHET (CI, since 3.4.1). The per-file undocumented counts are committed in
# scripts/doc-blocks-baseline.txt, the way check-measurements.sh keeps its ratchet: --ratchet
# fails when any file's count RISES above its baseline (a file absent from it has baseline 0).
# The debt was paid in full in 3.4.1 (lane INS6: the last 8 of 653), so the baseline names no
# file, and --ratchet is now the full gate: any undocumented public fn fails it. It also fails
# on a baseline that names a file again, so --update-baseline cannot buy a new fn its way in.
#
# USAGE
#   scripts/check-doc-blocks.sh                   list the undocumented, per-file counts; exit 1 if any
#   scripts/check-doc-blocks.sh --summary         per-file counts only; same exit status
#   scripts/check-doc-blocks.sh --ratchet         exit 1 if any public fn is undocumented, or the baseline has a debt
#   scripts/check-doc-blocks.sh --update-baseline rewrite scripts/doc-blocks-baseline.txt
#   scripts/check-doc-blocks.sh --selftest        plant every shape in a scratch dir, check each verdict
# Exit: 0 = every public fn documented; 1 = at least one is not, the baseline names a debt, or
# the population floor failed (the scan reached too few declarations to be trusted); 2 = bad
# option, or no baseline for --ratchet.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

python3 - "${1:-}" <<'PY'
import glob, os, re, sys, tempfile

mode = sys.argv[1] if len(sys.argv) > 1 else ""
if mode not in ("", "--summary", "--selftest", "--ratchet", "--update-baseline"):
    print("unknown option: %s" % mode, file=sys.stderr)
    sys.exit(2)

# The attribute tokens (the same list as scripts/mutation-test.sh's blanker). A line of
# attributes alone sits between a doc block and its declaration; a line an attribute merely
# LEADS (`#inline fn _f(x) { ... }`) is code, and ends the walk -- it is not a comment.
ATTR_TOK = (r'#(?:inline|naked|pure|io|alloc|must_use|regalloc|deprecated|assert|pe_import)'
            r'(?![A-Za-z0-9_])|#derive\([^)\n]*\)')
ATTR_ONLY = re.compile(r'^\s*(?:(?:' + ATTR_TOK + r')\s*)+$')
ATTR_LEAD = re.compile(r'^\s*(?:' + ATTR_TOK + r')')
# A declaration at column 0, `public` first or after attributes on the same line.
DECL = re.compile(r'^(?:(?:' + ATTR_TOK + r')\s*)*public\s+fn\s+([A-Za-z_][A-Za-z_0-9]*)\s*\(')
BASELINE = "scripts/doc-blocks-baseline.txt"
# A divider: punctuation only, or a title fenced by runs of punctuation.
DIVIDER = re.compile(r'^#\s*(?:[-=*~#_]{3,}.*[-=*~#_]{3,}|[-=*~#_]*)\s*$')

def scan(paths):
    """(total, [(file, line, name)]) over the given files."""
    total, bad = 0, []
    for f in paths:
        lines = open(f, encoding="utf-8", errors="replace").read().split("\n")
        for i, l in enumerate(lines):
            m = DECL.match(l)
            if not m:
                continue
            total += 1
            j = i - 1
            while j >= 0 and ATTR_ONLY.match(lines[j]):
                j -= 1
            block = []
            while j >= 0 and lines[j].lstrip().startswith("#") and not ATTR_LEAD.match(lines[j]):
                block.append(lines[j].strip())
                j -= 1
            says = [b for b in block if re.search(r"[A-Za-z]{2}", b) and not DIVIDER.match(b)]
            if not block and j >= 1 and lines[j].strip() == "":
                # One blank line, then a block that does not end in a divider (3.4.1).
                gap = []
                k = j - 1
                while k >= 0 and lines[k].lstrip().startswith("#") and not ATTR_LEAD.match(lines[k]):
                    gap.append(lines[k].strip())
                    k -= 1
                if gap and not DIVIDER.match(gap[0]):
                    says = [b for b in gap if re.search(r"[A-Za-z]{2}", b) and not DIVIDER.match(b)]
            if not says:
                bad.append((f, i + 1, m.group(1)))
    return total, bad

def per_file(bad):
    per = {}
    for f, _, _ in bad:
        per[f] = per.get(f, 0) + 1
    return per

def read_baseline(path):
    base = {}
    with open(path, encoding="utf-8") as fh:
        for row in fh:
            if row.startswith("#") or not row.strip():
                continue
            cnt, f = row.split("\t", 1)
            base[f.strip()] = int(cnt)
    return base

def risen(per, base):
    """The files whose undocumented count rose above the baseline: (file, baseline, now)."""
    return [(f, base.get(f, 0), per[f]) for f in sorted(per) if per[f] > base.get(f, 0)]

def debt(base):
    """The files a baseline still excuses; the rule since 3.4.1 is none."""
    return [f for f in sorted(base) if base[f] > 0]

if mode == "--selftest":
    # Each planted function names its expected verdict: *_doc documented, *_nodoc not.
    plant = "\n".join([
        "# Adds one.",
        "public fn a_doc(x) {",
        "    return x + 1;",
        "}",
        "# Doubles.",
        "#must_use",
        "public fn b_doc(x) {",
        "    return x * 2;",
        "}",
        "# An inline accessor with its own block.",
        "#inline public fn c_doc(x) { return x; }",
        "#inline public fn c_nodoc(x) { return x; }",
        "}",
        "public fn d_nodoc(x) {",
        "    return x;",
        "}",
        "# A contract one blank line above its function still documents it.",
        "",
        "public fn e_doc(x) {",
        "    return x;",
        "}",
        "# Two blank lines detach a block from what follows.",
        "",
        "",
        "public fn p_nodoc(x) {",
        "    return x;",
        "}",
        "# ================================================================",
        "# A section title, fenced: a header, not a contract",
        "# ================================================================",
        "",
        "#must_use",
        "public fn o_nodoc(x) {",
        "    return x;",
        "}",
        "# A contract above a blank line and an attribute.",
        "",
        "#must_use",
        "public fn q_doc(x) {",
        "    return x;",
        "}",
        "# ------------------------------------------------------------",
        "public fn f_nodoc(x) {",
        "    return x;",
        "}",
        "# ---- Section title ----",
        "public fn g_nodoc(x) {",
        "    return x;",
        "}",
        "#must_use",
        "public fn j_nodoc(x) {",
        "    return x;",
        "}",
        "#",
        "public fn k_nodoc(x) {",
        "    return x;",
        "}",
        "# ---- Section title ----",
        "# The block under a divider documents the function.",
        "#derive(accessors)",
        "#must_use",
        "public fn l_doc(x) {",
        "    return x;",
        "}",
        "# The private one-liner below is documented; it documents nothing past itself.",
        "#inline fn _priv(x) { return x; }",
        "public fn m_nodoc(x) {",
        "    return x;",
        "}",
        "# public fn commented_out(x) is prose, not a declaration.",
        "fn _private_undocumented(x) {",
        "    return x;",
        "}",
        "    public fn indented_is_not_top_level(x) {",
        "#must_use public fn r_nodoc(x) { return x; }",
        "# A one-line declaration under attributes, documented.",
        "#must_use #inline public fn s_doc(x) { return x; }",
        "",
    ])
    d = tempfile.mkdtemp(prefix="hisab-docblocks.")
    p = os.path.join(d, "plant.cyr")
    try:
        with open(p, "w") as fh:
            fh.write(plant)
        total, bad = scan([p])
        names = re.findall(r"public fn ([a-z]+_(?:doc|nodoc))\(", plant)
        want = sorted(n for n in names if n.endswith("_nodoc"))
        got = sorted(b[2] for b in bad)
        ok = (total == len(names)) and (got == want)
        # The ratchet: equal to the baseline passes; one more than it, or a file the baseline
        # does not name, fails; one fewer passes (debt paid down).
        per = per_file(bad)
        rat = {"equal passes": risen(per, {p: len(want)}) == [],
               "one over fails": risen(per, {p: len(want) - 1}) == [(p, len(want) - 1, len(want))],
               "an unnamed file fails": risen(per, {}) == [(p, 0, len(want))],
               "one under passes": risen(per, {p: len(want) + 1}) == [],
               "debt moved between files fails": risen({"a": 2, "b": 1}, {"a": 1, "b": 2}) == [("a", 1, 2)],
               "a baseline with a debt fails": debt({"a": 1}) == ["a"],
               "an empty baseline passes": debt({}) == [],
               "a zero count is no debt": debt({"a": 0}) == []}
        ok = ok and all(rat.values())
        print("  selftest: planted %d public fns, %d without a block" % (len(names), len(want)))
        print("  selftest: scanner saw %d, flagged %d" % (total, len(got)))
        for what, good in rat.items():
            if not good:
                print("    RATCHET WRONG: %s" % what)
        for n in want:
            if n not in got:
                print("    MISSED: %s" % n)
        for n in got:
            if n not in want:
                print("    FALSE ALARM: %s" % n)
        print("  selftest: %s" % ("PASS" if ok else "FAIL"))
        sys.exit(0 if ok else 1)
    finally:
        os.remove(p)
        os.rmdir(d)

files = sorted(f for f in glob.glob("src/*.cyr") if os.path.basename(f) not in ("main.cyr", "visibility.cyr"))
total, bad = scan(files)
per = per_file(bad)
print("=== doc blocks: %d of %d public fn(s) in %d file(s) have no comment block ==="
      % (len(bad), total, len(per)))
# ⚠ A scan that reaches nothing reports "0 undocumented". The floor sits under the 3.4.1
# starting tree's 653 public fns with room for ordinary churn.
FLOOR = 550
if total < FLOOR:
    print("!! only %d public fn declarations found (floor %d): the scan is not reaching src/" % (total, FLOOR))
    sys.exit(1)
if mode == "--update-baseline":
    with open(BASELINE, "w", encoding="utf-8") as fh:
        fh.write("# scripts/check-doc-blocks.sh --ratchet baseline.\n")
        fh.write("# Per-file count of public fns in src/ with no comment block. The debt was\n")
        fh.write("# paid in full in 3.4.1, so this file names none, and --ratchet fails on any\n")
        fh.write("# entry: document the fn, never add it here.\n")
        fh.write("# total undocumented at time of writing: %d of %d\n" % (len(bad), total))
        for f in sorted(per):
            fh.write("%d\t%s\n" % (per[f], f))
    print("wrote %s: %d file(s), %d undocumented" % (BASELINE, len(per), len(bad)))
    sys.exit(0)
if mode == "--ratchet":
    if not os.path.exists(BASELINE):
        print("no %s -- run --update-baseline first" % BASELINE)
        sys.exit(2)
    base = read_baseline(BASELINE)
    up = risen(per, base)
    fell = sum(max(0, n - per.get(f, 0)) for f, n in base.items())
    print("=== doc-block ratchet: %d undocumented, %d at baseline (%d paid down) ==="
          % (len(bad), sum(base.values()), fell))
    if debt(base):
        print()
        print("!! %s NAMES A DEBT: %d file(s). It was paid in full in 3.4.1: document the fn instead."
              % (BASELINE, len(debt(base))))
        sys.exit(1)
    if up:
        print()
        print("!! %d FILE(S) GAINED AN UNDOCUMENTED PUBLIC FN" % len(up))
        for f, b, n in up:
            print("  %s: %d -> %d" % (f, b, n))
            for bf, line, name in bad:
                if bf == f:
                    print("      %s:%d  %s" % (bf, line, name))
        print("Give each new public fn a comment block that states its contract.")
        sys.exit(1)
    print("No file gained an undocumented public fn.")
    sys.exit(0)
for f in sorted(per, key=lambda k: (-per[k], k)):
    print("  %-28s %d" % (f, per[f]))
if mode != "--summary" and bad:
    print()
    for f, n, name in bad:
        print("  %s:%d  %s" % (f, n, name))
if bad:
    sys.exit(1)
print("Every public function has a comment block.")
PY
