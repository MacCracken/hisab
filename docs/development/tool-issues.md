# Tool Issues

Catalog of bugs and quirks in the tools hisab depends on.

**Two tracks:**
- **Language-specific items** (cc5 codegen, cyrius parser, cyrlint, cbt manifest parser) get a dedicated deep-dive in `docs/development/issues/<DATE>-<short-slug>.md` with full investigation, self-contained reproducer, and verification steps. The maintainer agent picks them up when slot opens. Catalog entries here just link to the issue file.
- **Non-language items** (`owl`, `cyim`, anything else) are catalogued inline below.

Items leave this file (and the `issues/` dir) once they're fixed upstream **and** hisab's toolchain pin has moved past the fix.

---

## Language-specific (Cyrius / cc5 / cbt) — see `issues/`

### cc5 codegen

- **[2026-04-26 — 18-arg fn call scrambles register/stack params](issues/2026-04-26-cc5-18-arg-fn-scrambles-params.md)** — silent miscompile, no warning. Args 1, 2, 7-12 read garbage. Hisab worked around in `lib/calc.cyr` `_perm_init` by splitting the 18-arg helper into a 10-arg form. Pre-fix: bench segfaulted at exit 139 inside `perlin_2d`.

### Cyrius parser

- **[2026-04-26 — `for (init; cond; step)` requires all three clauses non-empty](issues/2026-04-26-cyrius-for-empty-clauses.md)** — empty init or empty step makes the parser report `unexpected ';'` / `expected '=' got '{'` at a *later* line, often inside an unrelated function. Hisab fixed in `lib/collision_core.cyr` + `lib/collision_mesh.cyr` by converting to `while`.

### cyrius CLI

- **[2026-04-26 — Unknown flag silently treated as positional, can clobber source files](issues/2026-04-26-cyrius-cli-arg-clobbers-source.md)** — `cyrius -v build src/main.cyr build/foo` truncates `src/main.cyr` to 0 bytes because `-v` is unrecognized and shifts the positional args. Hisab uses `CYRIUS_VERBOSE=1` env var instead.

### cyrius lint

- **[2026-04-26 — `cyrius lint` returns warning count as exit code](issues/2026-04-26-cyrius-lint-rc-as-warning-count.md)** — interacts badly with GHA `set -eo pipefail` in per-file loops. Captured-but-unprinted output masks which file tripped the gate. Hisab CI uses `set +e` + per-file rc capture.

### cbt manifest parser

- **[2026-04-26 — `modules` substring inside `[build]` comments grabs `[lib]` array](issues/2026-04-26-cbt-modules-substring-false-positive.md)** — a comment like `# pull via [deps.foo] modules = [...]` makes `cbt` parse the next `[lib] modules = [...]` as `[build].modules`, auto-prepending every `[lib]` file to every build. Hisab cyrius.cyml has a banner warning + uses `m`+`odules` split-token in surrounding comments. Same false-positive class already guarded for `stdlib` (~L170).

---

## Non-language tools

### `cyim --grep` is literal substring (regex coming via `--find` / `--regex=`)

**Component:** `cyim --grep <pattern> <file>`
**Symptom.** Anchors and metacharacters in `<pattern>` are matched literally. `cyim --grep "^struct "` returns no hits even when the file contains `struct Foo { ... }` at column 0.
**Reproducer.**

```bash
cyim --grep "^struct " /path/to/file.cyr   # zero hits
cyim --grep "struct "  /path/to/file.cyr   # finds "struct " anywhere
```

**Workaround (today).** Use literal patterns and post-filter. If you need anchored results:

```bash
cyim --grep "struct " /path/to/file.cyr | grep -E ":[0-9]+:struct "
```

**Status.** **Intentional + planned upstream work.** `--grep` is the literal matcher by design (deterministic, no regex DSL surprises). A `--find` flag is planned for proper regex matching, with `--regex=<flags>` to control regex flags (case sensitivity, multiline, etc). Once that lands, anchored / pattern-based searches use `cyim --find` (or `cyim --find --regex=i` etc) and `--grep` keeps its literal-substring contract for the use cases that want it.

When `cyim --find` ships:
- Update hisab CI / scripts to prefer `--find` over `--grep | grep` post-filter chains.
- Drop this entry from the catalog (the literal-vs-regex split is then documented in `cyim --help`, not here).

---

## Fixed upstream (kept briefly for searchability)

### cyrius 5.7.7 → 5.7.8 — `lib/syscalls_x86_64_linux.cyr:358: syscall arity mismatch`

**Symptom.** Every `cyrius build` of every project that included syscalls printed two of these warnings, regardless of whether the offending arch-conditional path was even reachable on the build target.

**Status.** Fixed in cc5 5.7.8 (`SYS_SETSID` arity table fix + cross-arch openat sentinel suppression in `_SC_ARITY`). Hisab now on 5.7.10; entry can be removed in the next CHANGELOG sweep.

---

## Process notes

- New language-specific items: `cp issues/2026-04-26-cc5-18-arg-fn-scrambles-params.md issues/<TODAY>-<short-slug>.md` and rewrite. Cross-link from the catalog above.
- New non-language items: append to the section above with the same fields (Component / Symptom / Reproducer / Workaround / Status).
- When a fix lands upstream and we bump the toolchain pin past it, do the cleanup the issue file's "Hisab follow-up after upstream fix" section describes, then `git rm` the issue file and the catalog entry. The CHANGELOG records the fix; the issue dir stays current.
