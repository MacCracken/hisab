#!/usr/bin/env bash
# check-miscompile.sh -- compile the shipped bundle the way a consumer does, under the
# consumers' toolchain pins, and disassemble it looking for the one miscompile class
# hisab's suites have found twice: a frame slot that code reads and nothing writes.
#
# WHY IT EXISTS
# Twice a cycc release has emitted wrong code that hisab's suites caught: 2.11.3's
# f64v_* destination slot read from a stack slot nothing wrote (fixed in cycc 6.6.2), and
# 3.2.0's register picker turning an intrinsic's destination-slot STORE into a register
# move while the inline loop still READ the slot (cycc 6.6.0-6.6.4, fixed in 6.6.5;
# docs/development/issues/archived/2026-09-14-cyrius-simd-dst-slot-regalloc-picker.md).
# 3.2.0 audited every suite binary for it once, by hand with objdump, and the threat
# model then said the binaries "are now audited" (roadmap D119). Consumers compile
# dist/hisab.cyr under their OWN pins -- six of them on 6.6.3, inside that picker's
# window -- so the build that matters is the bundle under those pins, not the suites
# under hisab's.
#
# WHAT IT DOES
# For each pin (default: 6.6.3, the lowest pin that compiles a 3.x bundle and the lowest
# live consumer pin, plus the manifest's own): a scratch project pinned to it, the
# declared stdlib leaves vendored from that toolchain with `cyrius lib sync` (what a
# consumer's build does), and a program that includes the bundle. The build must name
# versions/<pin>/bin/cycc on its `compiler:` line, or the run is refused (CLAUDE.md,
# "Check the probe"). cycc emits every function unless CYRIUS_DCE=1, which is unset here,
# so every bundle function is in the binary whether or not the program calls it. Then,
# over `objdump -d` of the binary, function by function (a function starts at
# `push %rbp; mov %rsp,%rbp`):
#   1. every negative %rbp slot an instruction READS is WRITTEN by some instruction of
#      the same function (a slot inside an address-taken range -- a `lea` at or below
#      it -- could be written through a pointer; those are counted, not failed);
#   2. every packed-SIMD memory operand `(%reg,%rsi,8)` whose base register was loaded
#      from a frame slot has that slot written in the same function -- the 3.2.0 audit,
#      and the picker defect's exact signature.
# Floors, so the gate cannot pass by seeing nothing: the binary must hold at least as
# many functions as the bundle declares non-#inline fns, and at least two packed
# operands per f64v_* call site in the bundle's code.
#
# WHAT IT CANNOT SEE, said plainly. The check is flow-insensitive: a slot written on one
# path and read on another passes, and so does a slot that IS written but by the wrong
# variable (upstream found the 6.6.0-6.6.4 picker defect changes shape when a `var` takes
# the freed slot). Values the picker keeps in registers are not examined, nor is any
# arithmetic.
# x86-64 only (aarch64 has no picker). It audits hisab's bundle, not a consumer's own
# code. It is a detector for one known class, not a proof that the build is right.
#
# --suites also builds the six suites under the manifest pin (in the repo, output to the
# scratch dir) and audits each binary, which is what 3.2.0 did by hand.
#
# --selftest proves the gate can fail: (a) the archived 3.2.0 repro built under 6.6.4
# must show its one unwritten slot (-0xe8) and under 6.6.5 none, each from a pinned dir
# (skipped, and said so, when either toolchain is not installed); (b) whatever the
# toolchain, the repro built under the manifest pin with the writes of one traced
# operand slot deleted from its disassembly must be flagged by both checks.
#
# Usage:
#   scripts/check-miscompile.sh [--pin V]... [--bundle PATH] [--suites] [--allow-missing] [--keep]
#   scripts/check-miscompile.sh --selftest
# A requested pin that is not installed under ~/.cyrius/versions fails the run unless
# --allow-missing. Not wired into CI: CI installs one toolchain. Run it by hand at a
# release and at every pin bump.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"
MANIFEST_PIN="$(sed -n 's/^cyrius = "\([0-9.]*\)".*/\1/p' cyrius.cyml | head -1)"
PINS=()
BUNDLE="dist/hisab.cyr"
SUITES=0
SELFTEST=0
ALLOW_MISSING=0
KEEP=0
while [ $# -gt 0 ]; do
    case "$1" in
        --pin) PINS+=("$2"); shift 2 ;;
        --bundle) BUNDLE="$2"; shift 2 ;;
        --suites) SUITES=1; shift ;;
        --selftest) SELFTEST=1; shift ;;
        --allow-missing) ALLOW_MISSING=1; shift ;;
        --keep) KEEP=1; shift ;;
        -h|--help) sed -n '2,60p' "$0"; exit 0 ;;
        *) echo "check-miscompile: unknown argument $1" >&2; exit 2 ;;
    esac
done
[ ${#PINS[@]} -gt 0 ] || PINS=(6.6.3 "$MANIFEST_PIN")
VERSIONS="${HOME}/.cyrius/versions"
REPRO="docs/development/issues/archived/2026-09-14-simd-dst-slot-regalloc-picker-repro.cyr"

T="$(mktemp -d "${TMPDIR:-/tmp}/hisab-mc.XXXXXX")"
if [ "$KEEP" = 1 ]; then trap 'echo "scratch kept: $T" >&2' EXIT; else trap 'rm -rf "$T"' EXIT; fi

# The declared stdlib leaves, read from the manifest so the pinned projects vendor
# exactly what a consumer of the bundle needs.
STDLIB="$(python3 -I - cyrius.cyml <<'PYEOF'
import re, sys
t = open(sys.argv[1]).read()
m = re.search(r'^\[deps\]\s*\nstdlib\s*=\s*\[(.*?)\]', t, re.S | re.M)
print(', '.join('"%s"' % x for x in re.findall(r'"([^"]+)"', m.group(1))))
PYEOF
)"

cat > "$T/audit.py" <<'PYEOF'
import json, re, subprocess, sys

SLOT = re.compile(r'^(-?0x[0-9a-f]+)?\(%rbp\)$')
BASE = re.compile(r'^(-?0x[0-9a-f]+)?\((%r[a-z0-9]+)(,%r[a-z0-9]+,\d)?\)$')
NOWRITE = re.compile(r'^(cmp|test|ucomis|comis|push|bt|call|jmp)')
RMW = re.compile(r'^(add|sub|and|or|xor|inc|dec|neg|not|sh[lr]|sa[rl]|ro[lr]|adc|sbb|xadd|imul)')
PACKED = re.compile(r'^(mov[ua]p[ds]|addp[ds]|subp[ds]|mulp[ds]|divp[ds]|sqrtp[ds]|maxp[ds]|minp[ds]|'
                    r'andp[ds]|orp[ds]|xorp[ds]|haddp[ds]|dpp[ds]|unpck[lh]p[ds]|vmov|vfmadd)')

def disasm(path):
    out = subprocess.run(['objdump', '-d', '--no-show-raw-insn', '-M', 'att', path],
                         stdout=subprocess.PIPE, check=True).stdout.decode('utf-8', 'replace')
    ins = []
    for line in out.splitlines():
        m = re.match(r'^\s*([0-9a-f]+):\s+(\S+)\s*(.*)$', line)
        if m: ins.append((int(m.group(1), 16), m.group(2), m.group(3).strip()))
    return ins

def split_ops(s):
    ops, depth, cur = [], 0, ''
    for ch in s:
        if ch == '(': depth += 1
        elif ch == ')': depth -= 1
        if ch == ',' and depth == 0: ops.append(cur.strip()); cur = ''
        else: cur += ch
    if cur.strip(): ops.append(cur.strip())
    return ops

def slot(op):
    m = SLOT.match(op)
    if not m: return None
    d = m.group(1) or '0x0'
    return -int(d[1:], 16) if d.startswith('-') else int(d, 16)

def functions(ins):
    starts = [i for i in range(len(ins) - 1) if ins[i][1] == 'push' and ins[i][2] == '%rbp'
              and ins[i + 1][1] == 'mov' and ins[i + 1][2] == '%rsp,%rbp']
    for k, s in enumerate(starts):
        yield ins[s:(starts[k + 1] if k + 1 < len(starts) else len(ins))]

def audit_fn(f):
    writes, reads, taken, last, uses = set(), {}, set(), {}, []
    for addr, mn, opstr in f:
        ops = split_ops(opstr)
        if mn.startswith('lea'):
            for o in ops[:-1]:
                s = slot(o)
                if s is not None: taken.add(s)
            if ops and ops[-1].startswith('%'): last.pop(ops[-1], None)
            continue
        for j, o in enumerate(ops):
            s = slot(o)
            if s is None: continue
            if j == len(ops) - 1 and len(ops) >= 2:
                if not NOWRITE.match(mn): writes.add(s)
                if RMW.match(mn) or NOWRITE.match(mn): reads.setdefault(s, addr)
            elif j == len(ops) - 1:
                if mn.startswith(('pop', 'set')): writes.add(s)
                elif RMW.match(mn): writes.add(s); reads.setdefault(s, addr)
                else: reads.setdefault(s, addr)
            else:
                reads.setdefault(s, addr)
        if mn.startswith('mov') and len(ops) == 2 and ops[1].startswith('%r'):
            s = slot(ops[0])
            if s is not None: last[ops[1]] = (s, addr)
            else: last.pop(ops[1], None)
        elif ops and ops[-1].startswith('%r') and not mn.startswith(('cmp', 'test', 'push')):
            last.pop(ops[-1], None)
        if PACKED.match(mn):
            for o in ops:
                m = BASE.match(o)
                if m and m.group(2) not in ('%rbp', '%rsp') and m.group(2) in last:
                    uses.append((addr, mn, o, last[m.group(2)]))
    unwritten, excluded = [], 0
    for s, a in reads.items():
        if s < 0 and s not in writes:
            if any(t <= s for t in taken): excluded += 1
            else: unwritten.append((a, s))
    bad = [(a, mn, o, s, la) for a, mn, o, (s, la) in uses if s < 0 and s not in writes]
    return dict(reads=len(reads), unwritten=unwritten, excluded=excluded, packed=len(uses), bad=bad)

def audit(ins):
    tot = dict(functions=0, reads=0, unwritten=[], excluded=0, packed=0, bad=[])
    for f in functions(ins):
        r = audit_fn(f)
        tot['functions'] += 1
        for k in ('reads', 'excluded', 'packed'): tot[k] += r[k]
        tot['unwritten'] += r['unwritten']; tot['bad'] += r['bad']
    return tot

def report(tag, r):
    print('  %s: %d functions, %d frame-slot reads, %d unwritten, %d address-taken (not failed), '
          '%d packed operands traced, %d from unwritten slots' % (tag, r['functions'], r['reads'],
          len(r['unwritten']), r['excluded'], r['packed'], len(r['bad'])))
    for a, s in r['unwritten'][:10]: print('    unwritten slot %s, read at 0x%x' % (hex(s), a))
    for a, mn, o, s, la in r['bad'][:10]:
        print('    %s %s at 0x%x: base loaded from %s at 0x%x, which nothing in the function writes' % (mn, o, a, hex(s), la))

def strip_comments(src):
    out = []
    for line in src.split('\n'):
        code, q = '', False
        for i, ch in enumerate(line):
            if ch == '"' and (i == 0 or line[i - 1] != '\\'): q = not q
            if ch == '#' and not q:
                if re.match(r'#(inline|derive|must_use)', line[i:]): code += line[i:]; break
                break
            code += ch
        out.append(code)
    return '\n'.join(out)

def floors(bundle):
    code = strip_comments(open(bundle, encoding='utf-8', errors='replace').read())
    # fns not marked #inline, on their own line or the line before (an #inline fn is
    # replayed at its call sites, so it need not appear as a function of its own)
    fns, prev_inline = 0, False
    for line in code.split('\n'):
        s = line.strip()
        if not s: continue
        if re.match(r'(?:public\s+)?fn\s+\w', s) and not prev_inline: fns += 1
        prev_inline = s == '#inline'
    sites = len(re.findall(r'(?<![A-Za-z0-9_])f64v_[a-z0-9_]+\(', code))
    return fns, 2 * sites, sites

def main():
    cmd = sys.argv[1]
    if cmd == 'bundle':
        path, bundle, tag = sys.argv[2], sys.argv[3], sys.argv[4]
        r = audit(disasm(path))
        fmin, pmin, sites = floors(bundle)
        report(tag, r)
        print('    floors: >= %d functions (the bundle\'s non-#inline fns), >= %d packed operands (%d f64v_* sites)'
              % (fmin, pmin, sites))
        ok = True
        if r['unwritten'] or r['bad']: ok = False
        if r['functions'] < fmin:
            print('    FLOOR: %d functions < the bundle\'s %d non-#inline fns -- was DCE on?' % (r['functions'], fmin)); ok = False
        if r['packed'] < pmin:
            print('    FLOOR: %d packed operands traced < 2 per f64v_* call site (%d sites) -- the trace saw too little'
                  % (r['packed'], sites)); ok = False
        return 0 if ok else 1
    if cmd == 'binary':
        r = audit(disasm(sys.argv[2])); report(sys.argv[3], r)
        return 0 if not (r['unwritten'] or r['bad']) else 1
    if cmd == 'expect':
        # expect BIN TAG N_UNWRITTEN SLOT: the real-toolchain control
        r = audit(disasm(sys.argv[2])); report(sys.argv[3], r)
        want = int(sys.argv[4])
        got = len(r['bad'])
        ok = got == want and len(r['unwritten']) == want
        if want and ok: ok = all(hex(s) == sys.argv[5] for a, mn, o, s, la in r['bad'])
        return 0 if ok else 1
    if cmd == 'plant':
        # plant BIN: delete every write to one traced operand slot of one function; both checks must fire
        ins = disasm(sys.argv[2])
        for f in functions(ins):
            r = audit_fn(f)
            if not r['packed'] or r['bad']: continue
            # the slot the first traced packed operand's base was loaded from
            last = {}
            target = None
            for a, mn, opstr in f:
                ops = split_ops(opstr)
                if mn.startswith('mov') and len(ops) == 2 and ops[1].startswith('%r') and slot(ops[0]) is not None:
                    last[ops[1]] = slot(ops[0])
                if PACKED.match(mn):
                    for o in ops:
                        m = BASE.match(o)
                        if m and m.group(2) in last and last[m.group(2)] < 0: target = last[m.group(2)]; break
                if target is not None: break
            if target is None: continue
            planted = []
            for a, mn, opstr in f:
                ops = split_ops(opstr)
                if ops and len(ops) >= 2 and slot(ops[-1]) == target and not NOWRITE.match(mn):
                    continue   # the deleted store
                planted.append((a, mn, opstr))
            rp = audit_fn(planted)
            print('  planted: deleted the stores to %s in the function at 0x%x -> %d unwritten, %d packed operands from unwritten slots'
                  % (hex(target), f[0][0], len(rp['unwritten']), len(rp['bad'])))
            return 0 if rp['unwritten'] and rp['bad'] else 1
        print('  planted: no function with a traced packed operand to plant in'); return 1
    return 2

sys.exit(main())
PYEOF

FAIL=0

# mkpin PIN DIR -- scratch project pinned to PIN with the manifest's stdlib leaves vendored
mkpin() {
    local pin="$1" dir="$2"
    mkdir -p "$dir"
    printf '[package]\nname = "hisab-mc"\nversion = "0.0.0"\nlanguage = "cyrius"\ncyrius = "%s"\n\n[deps]\nstdlib = [%s]\n' \
        "$pin" "$STDLIB" > "$dir/cyrius.cyml"
    if ! (cd "$dir" && cyrius lib sync > sync.log 2>&1); then
        echo "check-miscompile: cyrius lib sync failed for pin $pin:" >&2
        tail -5 "$dir/sync.log" >&2
        return 1
    fi
}

# build DIR SRC OUT PIN -- build from DIR and refuse a build the pin's compiler did not make
build() {
    local dir="$1" src="$2" out="$3" pin="$4" log
    log="$(cd "$dir" && env -u CYRIUS_DCE cyrius build -v "$src" "$out" 2>&1)" || { echo "$log" | tail -5; return 1; }
    if ! printf '%s\n' "$log" | grep -F 'compiler:' | grep -F -q "/versions/${pin}/bin/cycc"; then
        echo "  REFUSED: the build did not run versions/${pin}/bin/cycc:"
        printf '%s\n' "$log" | grep -F 'compiler:' || echo "  (no compiler: line)"
        return 1
    fi
}

if [ "$SELFTEST" = 1 ]; then
    echo "selftest (a): the archived 3.2.0 repro under 6.6.4 and 6.6.5"
    if [ -d "$VERSIONS/6.6.4" ] && [ -d "$VERSIONS/6.6.5" ]; then
        for p in 6.6.4 6.6.5; do
            mkpin "$p" "$T/st-$p"
            cp "$REPRO" "$T/st-$p/repro.cyr"
            build "$T/st-$p" repro.cyr "$T/repro-$p" "$p" || FAIL=1
        done
        python3 -I "$T/audit.py" expect "$T/repro-6.6.4" "repro, cycc 6.6.4" 1 -0xe8 || { echo "  FAIL: 6.6.4 must show exactly the -0xe8 slot"; FAIL=1; }
        python3 -I "$T/audit.py" expect "$T/repro-6.6.5" "repro, cycc 6.6.5" 0 none || { echo "  FAIL: 6.6.5 must show none"; FAIL=1; }
    else
        echo "  SKIPPED: cycc 6.6.4 and 6.6.5 are not both installed under $VERSIONS"
    fi
    echo "selftest (b): a planted unwritten slot, cycc $MANIFEST_PIN"
    mkpin "$MANIFEST_PIN" "$T/st-m"
    cp "$REPRO" "$T/st-m/repro.cyr"
    build "$T/st-m" repro.cyr "$T/repro-m" "$MANIFEST_PIN" || FAIL=1
    python3 -I "$T/audit.py" binary "$T/repro-m" "repro, cycc $MANIFEST_PIN (unplanted)" || { echo "  FAIL: the unplanted repro must be clean"; FAIL=1; }
    python3 -I "$T/audit.py" plant "$T/repro-m" || { echo "  FAIL: the planted slot was not flagged by both checks"; FAIL=1; }
    [ "$FAIL" = 0 ] && echo "selftest: PASS" || echo "selftest: FAIL"
    exit "$FAIL"
fi

[ -f "$BUNDLE" ] || { echo "check-miscompile: no bundle at $BUNDLE" >&2; exit 2; }
echo "bundle: $BUNDLE ($(sed -n 's/^# Version: *//p' "$BUNDLE" | head -1)); pins: ${PINS[*]}"
for p in "${PINS[@]}"; do
    if [ ! -d "$VERSIONS/$p" ]; then
        echo "  cycc $p: NOT INSTALLED under $VERSIONS"
        [ "$ALLOW_MISSING" = 1 ] || FAIL=1
        continue
    fi
    d="$T/pin-$p"
    mkpin "$p" "$d"
    mkdir -p "$d/dist"
    cp "$BUNDLE" "$d/dist/hisab.cyr"
    printf 'include "dist/hisab.cyr"\nfn main() { return 0; }\nvar exit_code = main();\nsys_exit_group(exit_code);\n' > "$d/consumer.cyr"
    if ! build "$d" consumer.cyr "$T/consumer-$p" "$p"; then
        echo "  cycc $p: the bundle did not build"; FAIL=1; continue
    fi
    python3 -I "$T/audit.py" bundle "$T/consumer-$p" "$BUNDLE" "bundle under cycc $p" || FAIL=1
done

if [ "$SUITES" = 1 ]; then
    echo "suites under the manifest pin (cycc $MANIFEST_PIN)"
    for s in hisab foundation modules modules_b edge_cases abuse; do
        if ! build "$REPO_ROOT" "tests/$s.tcyr" "$T/suite-$s" "$MANIFEST_PIN"; then FAIL=1; continue; fi
        python3 -I "$T/audit.py" binary "$T/suite-$s" "tests/$s.tcyr" || FAIL=1
    done
fi

if [ "$FAIL" = 0 ]; then echo "check-miscompile: PASS"; else echo "check-miscompile: FAIL"; fi
exit "$FAIL"
