# 2026-04-26 — `cyrius` CLI: unknown flag silently treated as positional, can clobber source files

**Component:** `cyrius` CLI (`cbt/cyrius.cyr` arg parser)
**Toolchain seen:** cyrius 5.7.7 → 5.7.10 (latent through the whole 5.7.x line as far as we know)
**Severity:** Data-loss risk — destroyed `src/main.cyr` once during normal interactive use.
**Hisab impact:** Lost `src/main.cyr` to a `cyrius -v build src/main.cyr /tmp/test` invocation. Recovered via `git checkout HEAD --`.
**Hisab workaround:** Use `CYRIUS_VERBOSE=1` (env var) instead of unrecognized CLI flags. Don't pass any unknown flag before a subcommand.
**Status:** Open. cc5 5.7.7's atomic-output fix prevents destruction on compile *failure*, but a misparsed-but-successful invocation still nukes the file.

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
