#!/usr/bin/env bash
# Run hisab's Cyrius benchmark suite, append results to CSV history,
# and regenerate benchmarks.md with a 3-point trend table
# (baseline → middle → current).
#
# Usage:
#   ./scripts/bench-history.sh                      # default CSV: bench-history.csv
#   ./scripts/bench-history.sh results.csv         # custom output
#   ./scripts/bench-history.sh "" tests/foo.bcyr   # custom suite
set -euo pipefail

HISTORY_FILE="${1:-bench-history.csv}"
SUITE="${2:-tests/hisab.bcyr}"
BENCHMARKS_MD="benchmarks.md"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

# Schema note (v2.9.2). The first five columns are unchanged and old rows still
# parse positionally into them. `stat` names WHICH statistic `estimate_ns` holds:
# rows written before 2.9.2 have no value there and are `max` — see the parser
# comment below for why that was wrong. New rows are `avg` and also carry the
# min/max/iters they came from, so no information is discarded and the two
# populations are never silently averaged together.
if [ ! -f "$HISTORY_FILE" ]; then
    echo "timestamp,commit,branch,benchmark,estimate_ns,stat,avg_ns,min_ns,max_ns,iters" > "$HISTORY_FILE"
elif ! head -1 "$HISTORY_FILE" | grep -q ',stat,'; then
    # Widen the header in place; existing 5-field rows keep their meaning and
    # read back as stat="" (i.e. max).
    sed -i '1s/.*/timestamp,commit,branch,benchmark,estimate_ns,stat,avg_ns,min_ns,max_ns,iters/' "$HISTORY_FILE"
    echo "note: bench-history header widened (estimate_ns is 'avg' from this run on; earlier rows were 'max')"
fi

echo "╔══════════════════════════════════════════╗"
echo "║         hisab benchmark suite            ║"
echo "╠══════════════════════════════════════════╣"
echo "║  commit: $COMMIT"
echo "║  branch: $BRANCH"
echo "║  date:   $TIMESTAMP"
echo "║  suite:  $SUITE"
echo "╚══════════════════════════════════════════╝"
echo ""

# `cyrius bench` (lib/bench.cyr) emits one line per benchmark in the form:
#   <name>: <avg><unit> avg (min=<min><unit> max=<max><unit>) [<n> iters]
#
# ⚠ THE COMMENT THAT USED TO BE HERE DESCRIBED A FORMAT THAT DOES NOT EXIST
# ("<name> ... <median> (per iter)"), and the parser under it took "the last
# <num><unit> in the line" — which in the REAL format is the `max=` field. So
# every row this script has ever written recorded the single worst iteration,
# under a column named `estimate_ns`, and `benchmarks.md` presented that as the
# trend. Measured 2026-08-09, same binary, three back-to-back runs: the `avg`
# field is stable to a median of 3.5% (worst 6.9%, 0 of 55 benchmarks over 20%),
# while consecutive max-based CSV rows swung −93% to +742%. CLAUDE.md designates
# this CSV as the proof for every performance claim, so the proof was being
# recorded on the noisiest statistic the harness reports. Now parsed by ANCHORING
# on the named fields rather than by scavenging tokens.
BENCH_OUTPUT=$(cyrius bench "$SUITE" 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$BENCH_OUTPUT"
echo ""

normalize_to_ns() {
    local val="$1"
    local unit="$2"
    case "$unit" in
        ps)        awk -v v="$val" 'BEGIN{printf "%.4f", v / 1000}' ;;
        ns)        echo "$val" ;;
        us|µs)     awk -v v="$val" 'BEGIN{printf "%.4f", v * 1000}' ;;
        ms)        awk -v v="$val" 'BEGIN{printf "%.4f", v * 1000000}' ;;
        s)         awk -v v="$val" 'BEGIN{printf "%.4f", v * 1000000000}' ;;
        *)         echo "$val" ;;
    esac
}

declare -a BENCH_NAMES=()
declare -a BENCH_NS=()

SKIPPED=0
while IFS= read -r line; do
    # Anchored on the harness's own field names. A line that does not match this
    # shape is COUNTED and reported rather than silently dropped — a format
    # change upstream must be loud, not quietly halve the recorded benchmarks.
    if [[ "$line" =~ ^[[:space:]]*([A-Za-z_][A-Za-z_0-9]*):[[:space:]]+([0-9]+(\.[0-9]+)?)(ps|ns|µs|us|ms|s)[[:space:]]+avg[[:space:]]+\(min=([0-9]+(\.[0-9]+)?)(ps|ns|µs|us|ms|s)[[:space:]]+max=([0-9]+(\.[0-9]+)?)(ps|ns|µs|us|ms|s)\)([[:space:]]+\[([0-9]+)[[:space:]]+iters\])? ]]; then
        BENCH_NAME="${BASH_REMATCH[1]}"
        AVG_NS=$(normalize_to_ns "${BASH_REMATCH[2]}" "${BASH_REMATCH[4]}")
        MIN_NS=$(normalize_to_ns "${BASH_REMATCH[5]}" "${BASH_REMATCH[7]}")
        MAX_NS=$(normalize_to_ns "${BASH_REMATCH[8]}" "${BASH_REMATCH[10]}")
        ITERS="${BASH_REMATCH[12]:-}"
        echo "${TIMESTAMP},${COMMIT},${BRANCH},${BENCH_NAME},${AVG_NS},avg,${AVG_NS},${MIN_NS},${MAX_NS},${ITERS}" \
            >> "$HISTORY_FILE"
        BENCH_NAMES+=("$BENCH_NAME")
        BENCH_NS+=("$AVG_NS")
    elif echo "$line" | grep -qE '^[[:space:]]+[A-Za-z_][A-Za-z_0-9]*:.*(ps|ns|µs|us|ms|s)'; then
        SKIPPED=$((SKIPPED + 1))
        echo "::warning:: unparsed benchmark line (format drift?): $line"
    fi
done <<< "$BENCH_OUTPUT"

COUNT=${#BENCH_NAMES[@]}
if [ "$SKIPPED" -gt 0 ]; then
    echo "ERROR: $SKIPPED benchmark line(s) did not match the expected format;" >&2
    echo "       lib/bench.cyr's output shape has changed — fix the parser." >&2
    exit 1
fi
if [ "$COUNT" -eq 0 ]; then
    echo "ERROR: no benchmarks parsed — refusing to write an empty history row." >&2
    exit 1
fi

# Trend table (3-point). Skip if Python missing — CSV is the primary record.
if command -v python3 >/dev/null 2>&1; then
python3 - "$HISTORY_FILE" "$BENCHMARKS_MD" <<'PYEOF'
import csv, sys
from collections import OrderedDict

history_file = sys.argv[1]
md_file = sys.argv[2]

rows = list(csv.DictReader(open(history_file)))
if not rows:
    sys.exit(0)

# `estimate_ns` means different things either side of 2.9.2: rows written before
# it hold the MAX (a single worst iteration), rows after hold the AVG. Comparing
# across that boundary produces deltas of hundreds of percent that are pure
# artefact, so the trend is built ONLY from rows whose `stat` matches the newest
# run's. Old rows predate the column and read back as None -> "max".
def stat_of(r):
    return (r.get("stat") or "max").strip() or "max"

current_stat = stat_of(rows[-1])
rows = [r for r in rows if stat_of(r) == current_stat]
if not rows:
    sys.exit(0)

timestamps = list(OrderedDict.fromkeys(r["timestamp"] for r in rows))
if len(timestamps) >= 3:
    pick = [timestamps[0], timestamps[len(timestamps)//2], timestamps[-1]]
elif len(timestamps) == 2:
    pick = [timestamps[0], timestamps[-1]]
else:
    pick = [timestamps[0]]

seen = set()
pick = [t for t in pick if not (t in seen or seen.add(t))]

data = {}
commits = {}
for r in rows:
    ts = r["timestamp"]
    if ts in pick:
        try:
            data.setdefault(r["benchmark"], {})[ts] = float(r["estimate_ns"])
        except ValueError:
            continue
        commits[ts] = r["commit"]

labels = []
for i, ts in enumerate(pick):
    if i == 0 and len(pick) > 1:
        labels.append(f"Baseline (`{commits[ts]}`)")
    elif i == len(pick) - 1:
        labels.append(f"Current (`{commits[ts]}`)")
    else:
        labels.append(f"Mid (`{commits[ts]}`)")

def fmt_ns(ns):
    if ns >= 1_000_000:
        return f"{ns/1000:.1f} µs"
    elif ns >= 100:
        return f"{ns:.1f} ns"
    else:
        return f"{ns:.2f} ns"

# The threshold was +-3%, which sits BELOW this harness's measured run-to-run
# noise floor: three back-to-back runs of one binary (2026-08-09) span a median
# of 3.5% and up to 6.9% per benchmark. At +-3% roughly half the table lights up
# on a rebuild that changed nothing, which is how a reader learns to ignore it.
# 10% is the first round number clear of the measured worst case.
NOISE_FLOOR_PCT = 10

def delta(old, new):
    if old == 0:
        return ""
    pct = ((new - old) / old) * 100
    if pct < -NOISE_FLOOR_PCT:
        return f" **{pct:+.0f}%**"
    elif pct > NOISE_FLOOR_PCT:
        return f" {pct:+.0f}%"
    return ""

with open(md_file, "w") as f:
    f.write("# Benchmarks\n\n")
    # Dated measurement-method changes. A row's number is only comparable back to
    # the most recent entry here that affects it — the CSV cannot express that on
    # its own, and a silent step change reads as a win.
    f.write("> **Measurement changes** — read before comparing across a date.\n"
            "> * **2026-08-10**: 17 sub-microsecond benchmarks moved from `bench()` to\n"
            ">   `bench_batch()`. `bench_run` wraps a `clock_gettime` PAIR around every call\n"
            ">   (~240 ns, documented in `lib/bench.cyr`), so those rows were **~95% clock\n"
            ">   overhead**: `ray_sphere` read 1,466 ns and is 79 ns; `vec3_add` read 1,457 ns\n"
            ">   and is 36 ns. It also FLATTENED them — triangle/sphere measured 1.19x when the\n"
            ">   true ratio is 3.6x — so a real regression could hide inside the overhead.\n"
            ">   Their drop at this date is an artefact of the fix, not a speedup.\n"
            "> * **2026-08-09**: `estimate_ns` changed from each benchmark's MAX to its AVG.\n"
            ">   The `stat` column records which; rows with no value there are `max`.\n\n")
    ts_last = pick[-1]
    f.write(f"Latest: **{ts_last}** — commit `{commits[ts_last]}`\n\n")
    if len(pick) >= 3:
        f.write(f"Tracking: `{commits[pick[0]]}` (baseline) → `{commits[pick[1]]}` (mid) → `{commits[pick[-1]]}` (current)\n\n")

    groups = OrderedDict()
    for bench in data:
        group = bench.split("/")[0] if "/" in bench else bench
        groups.setdefault(group, []).append(bench)

    for group, benches in groups.items():
        f.write(f"## {group}\n\n")
        cols = " | ".join(labels)
        f.write(f"| Benchmark | {cols} |\n")
        f.write(f"|-----------|{'|'.join(['------'] * len(labels))}|\n")

        for bench in benches:
            name = bench.split("/", 1)[1] if "/" in bench else bench
            vals = data[bench]
            cells = []
            for ts in pick:
                ns = vals.get(ts)
                if ns is None:
                    cells.append("—")
                else:
                    cell = fmt_ns(ns)
                    if ts != pick[0] and pick[0] in vals:
                        cell += delta(vals[pick[0]], ns)
                    cells.append(cell)
            f.write(f"| `{name}` | {' | '.join(cells)} |\n")
        f.write("\n")

    f.write("---\n\n")
    f.write("Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.\n")

print(f"  Generated {md_file} with {len(pick)}-point trend across {len(data)} benchmarks")
PYEOF
fi

echo "════════════════════════════════════════════"
echo "  ${COUNT} benchmarks recorded"
echo "  CSV:      ${HISTORY_FILE}"
[ -f "$BENCHMARKS_MD" ] && echo "  Markdown: ${BENCHMARKS_MD}"
echo "════════════════════════════════════════════"
