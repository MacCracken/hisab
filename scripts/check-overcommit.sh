#!/usr/bin/env bash
# check-overcommit.sh -- say which overcommit regime CI's Test, Fuzz and Bench steps are about to
# run under, and fail before them where strict overcommit would refuse mappings they make.
#
# WHY IT EXISTS (3.5.0, roadmap "Strict overcommit is unverified")
# lib/alloc.cyr reserves its heap in 256 MB grain chunks mapped MAP_NORESERVE. Under
# vm.overcommit_memory 0 or 1 an untouched chunk costs no commit; under 2 (strict) the kernel
# ignores MAP_NORESERVE and charges every private writable mapping in full against CommitLimit,
# and a mapping past it is refused with ENOMEM. tests/hisab.tcyr's m0157 group reserves two
# chunks on purpose, and its claim that a refused reservation fails loudly was measured under
# `ulimit -v` only, while nothing read CI's overcommit mode: a green run said nothing about it.
#
# WHAT THE STEPS TAKE. Each figure is the peak, over the step's whole process tree, of the sum
# of every private writable mapping (what mode 2 charges), re-read after every mmap, munmap,
# mremap, brk, mprotect and execve, with every mapping granted:
#   Test        2,621,960 KB  `cyrius test tests/modules_b.tcyr`: the suite's 9 chunks, 2,359,596,
#                             while the cyrius CLI holds its own chunk, 262,364 (the compile before
#                             it, cycc 1,851,692 beside the CLI, is 2,114,056); the other suites:
#                             modules 1,000,132, hisab 786,672, edge_cases 524,580, abuse,
#                             modules_c and foundation one chunk each
#   counts      2,623,204 KB  the same run piped to two greps (Published test counts)
#   Fuzz        2,114,492 KB  `cyrius build tests/hisab.fcyr` (the harness itself 262,300)
#   Bench      13,369,756 KB  `cyrius bench tests/hisab.bcyr`: the harness fills 50 grain chunks
#                             and never frees them, 13,107,392 (its resident peak is 12,968,428 KB
#                             in any mode), beside the CLI's 262,364. 3.4.2's harness, 113 rows
#                             shorter, filled 40: 10,748,308 in all
# [measured: lane L8 commitpeak.py ptrace tracer, the CI step commands on this tree, cycc 6.7.18, 2026-11-14]
# The Bench row re-measured on the final 3.5.0 tree:
# [measured: scratch vmpeak.py, VmData + VmStk over the step's process tree every 2 ms, which reads modules_b's 2,621,960 and 3.4.2's Bench 10,748,308 exactly; cycc 6.7.18, 2026-11-14]
# That is what each step takes when every mapping it asks for is granted, the configuration its
# pass was measured in. With less room some are refused: the allocator then falls back to its
# 16 MB grain, and a step may still pass or fail loudly. tests/hisab.tcyr, charged in full
# against a budget the way mode 2 charges (a ptrace emulation refusing each private writable
# mmap past it with ENOMEM), passes from 786,672 KB down to 610,000 KB, and fails below:
# at 605,000 and 600,000 KB the CO4 delaunay group fails 3 assertions and then test code pushes
# onto a refused vec_new() (SIGSEGV, exit 139); at 540,000 KB m0157 fails 8 and test code then
# writes through a refused multivector (exit 139); at 300,000 KB m0157 fails 3 and the suite exits
# 1. `ulimit -v` gives the same verdicts at the same budgets. Loud each time, never a silent pass.
# The other steps were not swept, so this gate does not guess how they fare with less room.
# [measured: lane L8 commitpeak.py BUDGET_KB emulation and ulimit -v runs of the hisab.tcyr binary, 2026-11-14]
#
# WHAT IT DOES. It reads PROC/sys/vm/overcommit_memory (PROC defaults to /proc) and, in mode 2,
# CommitLimit and Committed_AS from PROC/meminfo:
#   mode 0 or 1  passes and says the chunks cost no commit here, so a pass of the steps says
#                nothing about strict overcommit;
#   mode 2       passes when CommitLimit - Committed_AS covers every step's figure above plus
#                MARGIN_KB, and otherwise fails naming each step that would run with refused
#                mappings and its shortfall;
#   anything else, or a file it cannot read, fails.
# MARGIN_KB, 16,384 KB, is what the tracer does not see: the step's own shell (436 KB measured)
# and stack growth between the syscalls it reads at, up to the usual 8 MB limit.
# The kernel's own mode 2 was not run: setting it needs root.
#
# Usage:
#   scripts/check-overcommit.sh [--proc DIR]
#   scripts/check-overcommit.sh --selftest     plant each regime in a fake PROC and check the verdict
set -euo pipefail

SELF="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/check-overcommit.sh"
PROC=/proc
MODE=check
usage() { echo "usage: check-overcommit.sh [--proc DIR] | --selftest" >&2; exit 2; }
case "${1:-}" in
    "") : ;;
    --proc) [ $# -ge 2 ] && [ -n "$2" ] || usage; PROC="$2" ;;
    --selftest) MODE=selftest ;;
    -h|--help) sed -n '2,53p' "$0"; exit 0 ;;
    *) echo "check-overcommit: unknown argument $1" >&2; usage ;;
esac

exec python3 -I - "$MODE" "$PROC" "$SELF" <<'PYEOF'
import os, re, subprocess, sys, tempfile

mode, proc, SELF = sys.argv[1], sys.argv[2], sys.argv[3]
# The peak charge of each CI step after this gate, every mapping granted (see the header).
STEPS = [
    ('Test (cyrius test tests/modules_b.tcyr)', 2621960),
    ('Published test counts (the same run, piped)', 2623204),
    ('Fuzz (cyrius build tests/hisab.fcyr)', 2114492),
    ('Bench (cyrius bench tests/hisab.bcyr)', 13369756),
]
MARGIN_KB = 16384         # the step's shell (436 KB) and stack growth between the tracer's reads

if mode == 'selftest':
    # (name, overcommit_memory text or None, meminfo text or None, want_rc, text the output must hold)
    MEM = 'MemTotal: 16000000 kB\nCommitLimit: %d kB\nCommitted_AS: %d kB\n'
    CASES = [
        ('mode 0', '0\n', None, 0, 'cost no commit'),
        ('mode 1', '1\n', None, 0, 'cost no commit'),
        ('mode 2, room', '2\n', MEM % (20000000, 1000000), 0, 'charged in full'),
        # The needs written out, not read from STEPS, so the constants are pinned:
        # Bench 13,369,756 + 16,384 = 13,386,140; Test 2,623,204 + 16,384 = 2,639,588.
        ('mode 2, exactly the need', '2\n', MEM % (1000000 + 13386140, 1000000), 0, 'charged in full'),
        ('mode 2, one kB short of Bench', '2\n', MEM % (1000000 + 13386139, 1000000), 1, 'Bench (cyrius bench'),
        ('mode 2, room for Test, not Bench', '2\n', MEM % (1000000 + 2639588, 1000000), 1, '10,746,552 kB short'),
        ('mode 2, one kB short of the counts step', '2\n', MEM % (1000000 + 2639587, 1000000), 1,
         'Published test counts (the same run, piped): needs 2,639,588 kB, 1 kB short'),
        # Each step's row pinned at one kB short of its own need: Test 2,638,344, Fuzz 2,130,876.
        ('mode 2, one kB short of Test', '2\n', MEM % (1000000 + 2638343, 1000000), 1,
         'Test (cyrius test tests/modules_b.tcyr): needs 2,638,344 kB, 1 kB short'),
        ('mode 2, one kB short of Fuzz', '2\n', MEM % (1000000 + 2130875, 1000000), 1,
         'Fuzz (cyrius build tests/hisab.fcyr): needs 2,130,876 kB, 1 kB short'),
        ('mode 2, no room', '2\n', MEM % (4000000, 3900000), 1, 'Test (cyrius test'),
        ('mode 2, no meminfo', '2\n', None, 1, 'cannot read'),
        ('mode 2, no CommitLimit line', '2\n', 'MemTotal: 1 kB\nCommitted_AS: 1 kB\n', 1, 'cannot read'),
        ('an unknown mode', '3\n', None, 1, 'unknown'),
        ('no overcommit_memory', None, None, 1, 'cannot read'),
        ('not a number', 'x\n', None, 1, 'cannot read'),
    ]
    bad = 0
    for name, oc, mem, want, text in CASES:
        with tempfile.TemporaryDirectory() as d:
            os.makedirs(d + '/sys/vm')
            if oc is not None: open(d + '/sys/vm/overcommit_memory', 'w').write(oc)
            if mem is not None: open(d + '/meminfo', 'w').write(mem)
            r = subprocess.run(['bash', SELF, '--proc', d], capture_output=True, text=True)
        out = r.stdout + r.stderr
        if r.returncode != want or text not in out:
            print('  selftest: %s: rc %d (want %d), output %r' % (name, r.returncode, want, out.strip()[:300]))
            bad += 1
    # The command line: --proc with no directory is a usage error, not a shell crash.
    for args, want, text in ((['--proc'], 2, 'usage:'), (['--proc', ''], 2, 'usage:'), (['--bogus'], 2, 'usage:')):
        r = subprocess.run(['bash', SELF] + args, capture_output=True, text=True)
        if r.returncode != want or text not in r.stdout + r.stderr or 'unbound' in r.stderr:
            print('  selftest: %s: rc %d (want %d), output %r' % (' '.join(args), r.returncode, want,
                                                                (r.stdout + r.stderr).strip()[:200]))
            bad += 1
    n = len(CASES) + 3
    print('  selftest: %d verdicts, %d wrong: %s' % (n, bad, 'PASS' if bad == 0 else 'FAIL'))
    sys.exit(1 if bad else 0)

def fail(msg):
    print('check-overcommit: FAIL: ' + msg)
    sys.exit(1)

try:
    oc = int(open(proc + '/sys/vm/overcommit_memory').read().strip())
except (OSError, ValueError) as e:
    fail('cannot read %s/sys/vm/overcommit_memory (%s): the regime the steps run under is unknown' % (proc, e))
if oc in (0, 1):
    print('check-overcommit: vm.overcommit_memory = %d: the allocator\'s 256 MB grain chunks are MAP_NORESERVE '
          'and cost no commit here, so a pass of Test, Fuzz and Bench says nothing about strict overcommit '
          '(mode 2).' % oc)
    sys.exit(0)
if oc != 2:
    fail('vm.overcommit_memory = %d is an unknown mode' % oc)
try:
    mi = open(proc + '/meminfo').read()
    lim = int(re.search(r'^CommitLimit:\s*(\d+)\s*kB', mi, re.M).group(1))
    used = int(re.search(r'^Committed_AS:\s*(\d+)\s*kB', mi, re.M).group(1))
except (OSError, AttributeError) as e:
    fail('cannot read CommitLimit / Committed_AS from %s/meminfo (%s)' % (proc, e))
room = lim - used
short = [(name, kb + MARGIN_KB) for name, kb in STEPS if room < kb + MARGIN_KB]
if short:
    fail('vm.overcommit_memory = 2 (strict), CommitLimit %d kB, Committed_AS %d kB: %s kB of room. Every private '
         'writable mapping is charged in full here, MAP_NORESERVE ignored, and these steps would run with mappings '
         'refused, on a path their passes were not measured on:\n%s\nRaise vm.overcommit_ratio, add swap, or run in '
         'mode 0 or 1.' % (lim, used, format(room, ','),
                           '\n'.join('  %s: needs %s kB, %s kB short' % (n, format(k, ','), format(k - room, ','))
                                     for n, k in short)))
print('check-overcommit: vm.overcommit_memory = 2 (strict): every grain chunk is charged in full; %s kB of room '
      'covers every step after this one (the largest, Bench, needs %s kB).'
      % (format(room, ','), format(max(k for _, k in STEPS) + MARGIN_KB, ',')))
PYEOF
