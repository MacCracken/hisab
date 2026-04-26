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

if [ ! -f "$HISTORY_FILE" ]; then
    echo "timestamp,commit,branch,benchmark,estimate_ns" > "$HISTORY_FILE"
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

# `cyrius bench` emits one line per benchmark in the form:
#   <name> ... <median>{ps|ns|us|µs|ms|s} (per iter)
# The exact format is owned by lib/bench.cyr; this parser tolerates
# extra columns by anchoring on "ns/iter", "ms/iter", etc.
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

while IFS= read -r line; do
    # Match lines containing "<num><unit>/iter" or trailing "<num> <unit>"
    if echo "$line" | grep -qE '([0-9.]+)[[:space:]]*(ps|ns|µs|us|ms|s)(/iter)?'; then
        # Pull benchmark name (before "..." or first whitespace burst)
        BENCH_NAME=$(echo "$line" | sed -E 's/[[:space:]]+(\.{2,}|[0-9]).*$//' | xargs)
        [ -z "$BENCH_NAME" ] && continue
        # Last <num><unit> in the line wins
        TOKEN=$(echo "$line" | grep -oE '[0-9]+(\.[0-9]+)?[[:space:]]*(ps|ns|µs|us|ms|s)' | tail -n 1)
        VAL=$(echo "$TOKEN" | grep -oE '[0-9]+(\.[0-9]+)?' | head -n 1)
        UNIT=$(echo "$TOKEN" | grep -oE '(ps|ns|µs|us|ms|s)$')
        NS=$(normalize_to_ns "$VAL" "$UNIT")
        echo "${TIMESTAMP},${COMMIT},${BRANCH},${BENCH_NAME},${NS}" >> "$HISTORY_FILE"
        BENCH_NAMES+=("$BENCH_NAME")
        BENCH_NS+=("$NS")
    fi
done <<< "$BENCH_OUTPUT"

COUNT=${#BENCH_NAMES[@]}

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

def delta(old, new):
    if old == 0:
        return ""
    pct = ((new - old) / old) * 100
    if pct < -3:
        return f" **{pct:+.0f}%**"
    elif pct > 3:
        return f" {pct:+.0f}%"
    return ""

with open(md_file, "w") as f:
    f.write("# Benchmarks\n\n")
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
