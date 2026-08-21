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
#
# Schema note (v2.11.2) — `regime` + `floor_ns`. `stat` says which STATISTIC the
# number is; `regime` says what the INSTRUMENT was doing when it was taken, and
# those are independent. cyrius 6.5.19 taught lib/bench.cyr to calibrate one
# clock read and subtract it from every sample, and taught `bench_run` to size
# its own batches instead of wrapping a clock pair around every iteration. Both
# rewrite the number without touching the code being measured.
#
# ⚠ THIS IS THE THIRD TIME A MEASUREMENT-METHOD CHANGE HAS MOVED EVERY ROW —
# 2.9.2 (max→avg), 2.10.0 (bench()→bench_batch() for 17 rows), and now this —
# and the first two were each answered with a paragraph of prose in the
# generated markdown, which the trend table itself could not read. CLAUDE.md's
# rule is to wait for the third instance before extracting an abstraction. This
# is the third, so the marker is now a COLUMN the trend filter enforces, not a
# note a reader is trusted to have seen.
#
# ⚠ AND IT IS DERIVED, NOT DECLARED. `regime` is read from whether this run's
# harness printed its measured timer floor — the instrument reporting on itself
# — so it cannot go stale the way a hand-maintained constant does. `floor_ns`
# records what that instrument cost on THIS host, this boot; upstream measured a
# 230x spread across the four hosts its own gate runs on, so it is not a
# property of the code and must travel with the row.
#
# ⚠ WHAT IT CANNOT DO, said plainly: it distinguishes "floor subtracted" from
# "floor not subtracted" because that is what the output makes visible. A future
# instrument change that keeps printing the floor line would NOT flip it. This
# narrows the hole, it does not close it.
CSV_HEADER="timestamp,commit,branch,benchmark,estimate_ns,stat,avg_ns,min_ns,max_ns,iters,regime,floor_ns"
if [ ! -f "$HISTORY_FILE" ]; then
    echo "$CSV_HEADER" > "$HISTORY_FILE"
elif ! head -1 "$HISTORY_FILE" | grep -q ',stat,'; then
    # Widen the header in place; existing 5-field rows keep their meaning and
    # read back as stat="" (i.e. max) and regime="" (i.e. raw).
    sed -i "1s/.*/${CSV_HEADER}/" "$HISTORY_FILE"
    echo "note: bench-history header widened (estimate_ns is 'avg' from this run on; earlier rows were 'max')"
elif ! head -1 "$HISTORY_FILE" | grep -q ',regime,'; then
    sed -i "1s/.*/${CSV_HEADER}/" "$HISTORY_FILE"
    echo "note: bench-history header widened with regime/floor_ns; earlier rows read back as regime='raw'"
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

# Measurement regime, read off the harness's own output rather than declared here.
#
# Since cyrius 6.5.19 `bench_report` prints, once per process, the timer floor it
# MEASURED on this host and subtracts it from every sample:
#   [timer floor 1.349us per clock read, measured; subtracted from every sample]
# Its presence is the regime marker. Absent → the pre-6.5.19 instrument, whose
# numbers include the clock and whose `bench_run` paid a clock pair per iteration.
#
# ⚠ The floor line deliberately carries no " avg", so the benchmark parser below
# cannot mistake it for a result row — but that is upstream's guarantee, not
# ours, so the parser is anchored on its own field names independently.
REGIME="raw"
FLOOR_NS=""
if [[ "$BENCH_OUTPUT" =~ \[timer\ floor\ ([0-9]+(\.[0-9]+)?)(ps|ns|µs|us|ms|s)\ per\ clock\ read ]]; then
    REGIME="net"
    FLOOR_NS=$(normalize_to_ns "${BASH_REMATCH[1]}" "${BASH_REMATCH[3]}")
    echo "measurement regime: net (timer floor ${FLOOR_NS} ns, measured on this host, subtracted from every sample)"
else
    echo "measurement regime: raw (harness reported no measured timer floor — pre-6.5.19 instrument)"
fi
echo ""

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
        echo "${TIMESTAMP},${COMMIT},${BRANCH},${BENCH_NAME},${AVG_NS},avg,${AVG_NS},${MIN_NS},${MAX_NS},${ITERS},${REGIME},${FLOOR_NS}" \
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
#
# `regime` (2.11.2) is the SECOND axis and is independent of the first: it says
# what the instrument was doing, not which statistic was taken. Rows on either
# side of it are both "avg" and would otherwise be compared directly. Measured
# on this tree at the 6.5.18 -> 6.5.33 bump, same source, zero code changes:
# ease_in_out 1407ns -> 7ns, cx_mul 1459 -> 37, quat_mul 1473 -> 67,
# perlin_2d 1461 -> 82 — and 44 of 72 rows would have printed as an improvement
# past the 10% noise gate. Filtering on `stat` alone admitted every one of them.
def stat_of(r):
    return (r.get("stat") or "max").strip() or "max"

def regime_of(r):
    return (r.get("regime") or "raw").strip() or "raw"

current_stat = stat_of(rows[-1])
current_regime = regime_of(rows[-1])
kept = [r for r in rows if stat_of(r) == current_stat and regime_of(r) == current_regime]
dropped = len(rows) - len(kept)
rows = kept
if not rows:
    sys.exit(0)
# Never silently. A trend built on a fraction of the history must say so, or the
# reader takes a two-point table for the whole record.
if dropped:
    print(f"  note: {dropped} row(s) excluded from the trend — "
          f"not stat={current_stat} regime={current_regime} (kept {len(rows)})")

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
            "> * **2026-08-21** (hisab 2.11.2, cyrius 6.5.18 → 6.5.33): `lib/bench.cyr` now\n"
            ">   MEASURES one clock read on the host and subtracts it from every sample, and\n"
            ">   `bench_run` sizes its own batches instead of wrapping a clock pair around every\n"
            ">   iteration. **No hisab source changed.** The floor is re-measured every run and\n"
            ">   recorded per row in `floor_ns` (~1,340–1,350 ns on this host; upstream measured a\n"
            ">   230x spread across its four gate hosts, and it moves between reboots, so it is\n"
            ">   not a constant and is not written down as one).\n"
            ">   `ease_in_out` 1,407 ns → 7 ns, `cx_mul` 1,459 → 37, `quat_mul` 1,473 → 67,\n"
            ">   `perlin_2d` 1,461 → 82 — those four were 94–100% instrument. The old numbers\n"
            ">   also FLATTENED them: four operations spanning **149x** in reality were reported\n"
            ">   within **1.26x** of each other, so a real regression had room to hide. 44 of 72\n"
            ">   rows moved more than 10%; **none of it is a speedup**. Rows carry `regime=net`\n"
            ">   from this date and the trend filter refuses to mix them with `raw` ones.\n"
            ">   ⚠ `min_ns`/`max_ns` also changed meaning for anything `bench_run` chose to batch:\n"
            ">   they are now per-CHUNK averages, not per-iteration extremes. That is the honest\n"
            ">   reading — a single sub-floor iteration has no measurable duration — but it means\n"
            ">   the spread narrows for reasons unrelated to the code. `estimate_ns` (the avg) is\n"
            ">   unaffected and remains the column the trend is built on.\n"
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
