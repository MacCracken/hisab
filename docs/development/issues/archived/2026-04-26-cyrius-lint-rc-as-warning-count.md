# 2026-04-26 — `cyrius lint` returns warning count as exit code (interacts badly with GHA `set -eo pipefail`)

**Component:** `cyrius lint` / `cyrlint`
**Toolchain seen:** cyrius 5.7.7 → 5.7.10
**Severity:** CI ergonomics — masks which file tripped the gate
**Hisab impact:** First saw it as `Error: Process completed with exit code 6.` in CI with no file context. The captured-but-unprinted lint output contained 6 "multiple consecutive blank lines" warnings on `tests/modules.tcyr`.
**Hisab workaround:** CI lint loop uses `set +e` and per-file rc capture, then emits `::error file=$f::cyrius lint failed (rc=$rc)` annotations so the GHA UI shows which file is broken. See `.github/workflows/ci.yml` `Lint` step.
**Status:** ✅ **RESOLVED on cyrius 6.2.11** (re-verified 2026-06-15 during the 6.0.14→6.2.11 bump). `cyrius lint` on a file with 2 "multiple consecutive blank lines" warnings now exits **0** while still printing the per-warning `  warn line N: …` lines and a `2 warnings` summary — rc is no longer the warning count. The CI `Lint` step's dual check (`rc != 0` **OR** `grep -qE '^\s*warn '` on stdout) still gates correctly: the stdout grep is now the load-bearing condition. The stale "returns the warning count as its exit code" comment in `.github/workflows/ci.yml` should be refreshed opportunistically. Archived.

## Symptom

```bash
cyrius lint path/to/file_with_6_warnings.cyr; echo "rc=$?"
# emits the 6 warn lines
# rc=6
```

A successful lint run with N warnings exits with code N. Under GHA's default `bash --noprofile --norc -eo pipefail` for `run:` blocks, the per-file pattern

```bash
out=$(cyrius lint "$f" 2>&1)   # if rc != 0, set -e aborts the script here
echo "$out"
```

aborts on the first file with warnings, *before* the captured output gets echoed. The user sees `Error: Process completed with exit code N` with zero context.

The output IS captured into `$out`, but never printed because `set -e` killed the line.

## Self-contained reproducer

```bash
cat > /tmp/lint_repro.cyr << 'EOF'
fn main() {
    return 0;
}



# Three blank lines above this comment trip "multiple consecutive blank lines"
var r = main();
EOF

set -eo pipefail
out=$(cyrius lint /tmp/lint_repro.cyr 2>&1)   # script aborts here with rc=N
echo "  --- captured output (you will not see this line) ---"
echo "$out"
```

## Where to look in cc5

`programs/cyrlint.cyr` final exit path. Today: `syscall(SYS_EXIT, warning_count)`. Suggested: keep that behavior under an opt-in flag (`--exit-with-count`), and default to `syscall(SYS_EXIT, 0)` on success / `1` on parse error / `2` if `--strict` and warnings present. Or just `--strict` mode = "exit 1 if any warnings".

## Hisab CI workaround in detail

```yaml
- name: Lint
  run: |
    set +e   # don't abort the loop on per-file non-zero rc
    fail=0
    for f in src/*.cyr examples/*.cyr tests/*.tcyr tests/*.bcyr tests/*.fcyr; do
      [ -f "$f" ] || continue
      echo "=== cyrlint: $f ==="
      out=$(cyrius lint "$f" 2>&1)
      rc=$?
      echo "$out"
      if [ "$rc" -ne 0 ] || echo "$out" | grep -qE '^\s*warn '; then
        echo "::error file=$f::cyrius lint failed (rc=$rc)"
        fail=1
      fi
    done
    [ $fail -eq 0 ] || { echo "lint: warnings present"; exit 1; }
```

This works but is more boilerplate than `set -e` would be if the lint exit code matched conventions.

## Verification once a fix lands

```bash
cyrius lint --strict path/with/warnings.cyr; echo "rc=$?"
# Expected: prints warns, exits 1 (or some sentinel != warning_count)
cyrius lint path/with/warnings.cyr; echo "rc=$?"
# Expected: prints warns, exits 0 (or 1 — but consistent and not a count)
```

## Hisab follow-up after upstream fix

If upstream defaults to `exit 0 on success / 1 on warnings present`, simplify the CI loop to drop the `set +e` dance. Otherwise leave the workaround in place — it's robust either way.

This file gets removed when the CI loop simplifies.
