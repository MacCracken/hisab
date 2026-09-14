# 2026-04-26 — `cyrius` CLI: unknown flag silently treated as positional, can clobber source files

**Component:** `cyrius` CLI (`cbt/cyrius.cyr` arg parser)
**Toolchain seen:** cyrius 5.7.7 → 5.7.10 (latent through the whole 5.7.x line as far as we know)
**Severity:** Data-loss risk — destroyed `src/main.cyr` once during normal interactive use.
**Hisab impact:** Lost `src/main.cyr` to a `cyrius -v build src/main.cyr /tmp/test` invocation. Recovered via `git checkout HEAD --`.
**Hisab workaround:** Use `CYRIUS_VERBOSE=1` (env var) instead of unrecognized CLI flags. Don't pass any unknown flag before a subcommand.
**Status:** 🟢 **CLOSED 2026-09-14 (hisab v3.1.1, cycc 6.6.4) — the data loss is FIXED upstream, and
has been since cyrius 6.0.36.** `cbt/commands.cyr:66` refuses to write build output over any `.cyr`
path (`error: refusing to write build output over a .cyr source file: b.cyr`, exit **1**, source
byte-intact — measured on 6.6.4 with the exact misparsed shape `cyrius -v build b.cyr out`, in a
scratch dir, never in this tree). Landed in commit `8990f376` ("bug batch - tools and bigint"),
first tag **6.0.36** — so hisab carried this as "Open" for the whole 6.x line, the same way it carried
`for-empty-clauses` five releases past its decision. ⚠ What is NOT fixed, and is recorded rather than
implied: the top-level parser still treats an unknown flag before the subcommand as positional
(the probe shows the shift — `b.cyr` lands in the OUTPUT slot), and bare `cyrius -v` still exits 0
with the usage banner. The guard closes the destructive outcome, not the misparse; a non-`.cyr`
target in that slot would still be overwritten. Keep the flag AFTER the subcommand
(`cyrius build -v …`, exit 0, builds).

*Original status line, kept for the record:* Open. cc5 5.7.7's atomic-output fix prevents destruction on compile *failure*, but a misparsed-but-successful invocation still nukes the file.

**Upstream status, checked 2026-08-09 — and it is ambiguous, which is itself the finding.** The
upstream filing exists at
`cyrius/docs/development/issues/archived/2026-04-26-cyrius-cli-arg-clobbers-source.md` — i.e. in
**`archived/`**, which by that repo's own README means resolved. But it carries **no `— RESOLVED`
suffix in its heading, no resolution paragraph naming a fix version, and no row in
`archived/README.md`'s index** — all three of which that README requires on a real close. Its
status line still reads `**Status:** Open.` So it was moved during a sweep rather than fixed, and
the sibling `for-empty-clauses` filing next to it demonstrates what a genuine close looks like
there (a `RESOLUTION` block naming v6.0.36 and the reason). **Do not read its location as
evidence the data-loss bug is fixed.** Re-filing would only create a duplicate; the right move is
to ask the Cyrius side to either restore it to `issues/` or attach a resolution.

**2026-08-09 (v2.9.2, cycc 6.5.16) — deliberately NOT re-tested.** The reproducer is destructive by
construction: it works by getting cyrius to treat a real source path as the compiler's *output*
slot, so running it inside this tree would truncate whichever `src/*.cyr` it names to 0 bytes, and
the loss is silent until a later command reports `cannot read file`. Every other toolchain filing was
re-verified at this bump; this one is recorded as unverified rather than assumed-still-live, which
is the honest state. The only safe observation available — `cyrius -v` with **no path arguments** —
still exits 0 and prints the usage banner rather than rejecting the unknown flag, so the top-level
parser has not gained the strict-flag check that would fix this. That is suggestive, not proof.

## Symptom

`cyrius -v build src/main.cyr build/foo` is parsed as:

- subcommand: `build` (or `-v`? unclear which the parser thinks is the subcommand)
- positional 1: `src/main.cyr`
- positional 2: `build/foo`
- ...effectively `cyrius build SRC=build OUT=src/main.cyr` if `-v` ate the `build` slot

Result: `cyrius` "succeeds" by truncating `src/main.cyr` to 0 bytes (the output of compiling whatever it thought the source was). No error, no warning. The user discovers the empty file later when the next `cyrius lint src/main.cyr` reports `cannot read file`.

## Self-contained reproducer

**WARNING: only run in a throwaway git tree.**

```bash
mkdir -p /tmp/cyrius_arg_repro/src && cd /tmp/cyrius_arg_repro
cat > src/main.cyr << 'EOF'
fn main() { return 0; }
var r = main();
syscall(60, r);
EOF
cat > cyrius.cyml << 'EOF'
[package]
name = "repro"
version = "0.0.1"
language = "cyrius"
cyrius = "5.7.10"

[build]
src = "src/main.cyr"
output = "build/repro"

[deps]
stdlib = ["syscalls"]
EOF
ls -la src/main.cyr   # 41 bytes

cyrius -v build src/main.cyr /tmp/repro_out 2>&1 | head -3

ls -la src/main.cyr   # 0 bytes — clobbered
```

## Where to look in cc5

`cbt/cyrius.cyr` arg-parsing top-level dispatch. The fix is to reject unknown flags at the top level (`error: unknown flag '-v'`) instead of falling through into positional-arg parsing for the subcommand handler. Bonus: subcommand handlers should validate that the *output* path doesn't already exist as a non-output file (e.g. doesn't have `.cyr` extension; isn't tracked in git as a source file) before writing to it.

## Verification once a fix lands

The reproducer above should error with `unknown flag '-v'` (or similar) and exit non-zero **without** touching `src/main.cyr`.

## Hisab follow-up after upstream fix

None — this is purely a CLI hardening item. Hisab uses the `CYRIUS_VERBOSE` env var which is unaffected.

This file gets removed once the CLI rejects unknown flags.

---

## 🟢 CLOSED — ARCHIVED 2026-09-09 (v2.11.3), on cycc 6.6.1

**Fixed upstream, fail-closed, and finally safe to re-test.** This filing was carried for four
months as "deliberately never re-tested" because the reproducer destroys a source file. On 6.6.1
it can be run safely, because the destruction is now *guarded* rather than merely *unlikely*:

```
$ cyrius -v build victim.cyr <output>
error: refusing to write build output over a .cyr source file: victim.cyr
```

Run in a throwaway scratch package, never in this tree. `victim.cyr` was **byte-identical**
before and after (same SHA256, 59 B), across three argument shapes — `build src src`,
`--bogus build src src`, and the original `-v build src <out>` — so the guard keys on the
**output path being a `.cyr` source**, not on one particular misparse.

⚠ **The exit code was checked separately, and the first check was wrong.** `cyrius ... | tail`
reports `rc=0` because `$?` after a pipe is *tail's* status, not cyrius's — the same probe defect
this repo recorded in 2.11.2. Measured without the pipe: the refusal exits **1**, and a normal
build of the same file exits **0**. So it is a real failure, not a warning that scores green.

This closes the ambiguity recorded in the 2026-08-09 note above: the upstream filing sat in
`archived/` with `Status: Open` and no resolution paragraph, so its location could not be read as
evidence. It no longer has to be — the behaviour itself was measured on the current pin.
[measured: scratch harness, not reproducible in-tree — the reproducer is destructive by construction]
