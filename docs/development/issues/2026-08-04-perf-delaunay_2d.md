<!-- 2.7.0-I performance record. Analysed and adversarially reviewed, both agents running
real code. Each optimisation is CONFIRMED as a real complexity win, but each was also shown
to CHANGE RESULTS or to regress on some input class -- see the CHALLENGE section before
applying. NOT APPLIED. The bicgstab hoist, the only result-safe one, shipped in 2.7.0-I. -->

# delaunay_2d
verdict=CONFIRMED

## MEASURED
Measured with `cyrius bench` on deterministic scattered points (the same PCG-ish mix tests/hisab.bcyr uses for the hull fixture: hx=(i*2654435761)%10007, hy=(i*40503)%10009). Scratch harness: /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/dl_base.bcyr, dl_opt.bcyr, dl_final.bcyr. Numbers are bench_avg_ns; three separate runs agreed to within 3%.

BEFORE (shipped delaunay_2d) — avg per call, and ratio vs the previous size:
  n=  100:    1.125 ms    —
  n=  200:    3.846 ms    3.42x
  n=  400:   14.110 ms    3.67x
  n=  800:   53.859 ms    3.82x
  n= 1600:  217.482 ms    4.04x
  n= 3200:  847.027 ms    3.89x
  n= 6400: 3376     ms    3.99x
GROWTH RATIO PER DOUBLING: 3.4 -> 4.0, converging on 4.0. That is O(n^2), not a
large constant. The finding's "232 ms at n=1,600" reproduces (217 ms here).

AFTER (x-sweep with triangle retirement, same fixture, same machine):
  n=  100:    0.706 ms    —
  n=  200:    1.658 ms    2.35x
  n=  400:    5.155 ms    3.11x
  n=  800:   10.720 ms    2.08x
  n= 1600:   23.339 ms    2.18x
  n= 3200:   58.547 ms    2.51x
  n= 6400:  147.880 ms    2.53x
GROWTH RATIO PER DOUBLING: 2.1 -> 2.5. Between O(n log n) (2.1x) and O(n^1.5)
(2.83x), which is the documented complexity of a Bourke-style x-sweep. The
exponent changed; this is not a constant-factor win.

SPEEDUP: n=800 5.0x, n=1600 9.3x (-89%), n=3200 14.5x (-93%), n=6400 22.8x (-96%).
Proposed CI fixture (n=1200, 5 iters): 122.595 ms -> 16.467 ms avg (-86.6%).

MEMORY: 10 calls at n=1200 (5 before + 5 after) allocated 17.8 MB total from the
bump arena, ~1.8 MB per call. The new code adds three O(n) vecs (order, min_fx,
tri_max_x) plus `done`; all are allocated once per call, none inside a loop.

## ROOT CAUSE
Confirmed, and it is the "find bad triangles" scan, exactly as the finding says.

src/collision_mesh.cyr:113-127 — the insertion loop `for (var i = 0; i < n; ...)`
contains, at line 121, `for (var ti = 0; ti < n_tris; ti = ti + 1)` which calls
`_col_in_circumcircle` (line 125) on EVERY live triangle for EVERY inserted point.
Bowyer-Watson keeps ~2i triangles after i insertions, so the total number of
circumcircle determinants is sum_i 2i ~= n^2. At n=1600 that is ~2.5M evaluations
of a 20-multiply determinant plus 3 vec_get indirections each — 217 ms, and 85 ns
per test, which matches.

Nothing else in the function is superlinear at realistic sizes: the boundary-edge
double loop (lines 133-158) and the descending sort of `bad` (lines 162-170) are
O(n_bad^2) with n_bad averaging ~4, and the final super-triangle filter is O(n).
Removing them would change nothing; the scan is the whole cost.

The finding's suggested remedies do not fit this data structure. A "walk to the
containing triangle" needs triangle adjacency, and Bowyer-Watson still has to find
the WHOLE bad set (a flood fill from that triangle) — this representation is a
flat vec with swap-remove and has no neighbour links, so that is a rewrite that
would also have to maintain adjacency through every swap. Grid bucketing has the
same problem: circumcircle bboxes are unbounded for slivers, so it needs an
overflow list plus bucket fixups on every swap-remove.

The change below gets the same asymptotic result without either. Insert points in
ascending x (there is already an O(n log n) heapsort for exactly this in
collision_core.cyr: `_col_sort_indices_by_xy`, added in 2.6.15 for convex_hull_2d)
and RETIRE a triangle once the sweep front has passed the right edge of its
circumcircle — it can never be bad again, so it leaves the scanned set. The scan
then costs O(front size) = O(sqrt n) instead of O(n).

## FIX
Three edits to src/collision_mesh.cyr. All measured numbers above come from this
exact code (dl_final.bcyr runs it verbatim).

(1) One new constant, next to the delaunay section header. The comment form
    "# 2^-20" is a grammar scripts/check-constants.sh already accepts.

# Relative slack for the sweep bound in _col_circum_max_x -- used both as the
# conditioning gate on the circumcentre solve and as the outward pad on the
# result. One-sided: raising it only makes triangles retire LATER.
var _COL_DL_SWEEP_REL = 0x3EB0_0000_0000_0000;  # 2^-20

(2) New helper, placed just above delaunay_2d:

# Rightmost x of triangle (a, b, c)'s circumcircle, rounded outward.
#
# This is the sweep's only input. Once every point still to be inserted lies to
# the right of this bound, no future point can land inside the circumcircle, so
# the triangle is final and never has to be circumcircle-tested again -- which is
# what turns delaunay_2d's per-point rescan of EVERY triangle into a rescan of
# just the sweep front.
#
# Every exit is one-sided: when the geometry is too ill-conditioned to trust, it
# returns +inf, meaning "never retire", and the triangle keeps going through the
# exact _col_in_circumcircle test exactly as it did before. A wrong answer here
# would silently drop a triangle that still needed splitting, so the gate is
# deliberately paranoid.
fn _col_circum_max_x(points, ai, bi, ci) {
    var ap = vec_get(points, ai);
    var bp = vec_get(points, bi);
    var cp = vec_get(points, ci);

    var ax = HVec2_x(ap);
    var ay = HVec2_y(ap);
    var bx = f64_sub(HVec2_x(bp), ax);
    var by = f64_sub(HVec2_y(bp), ay);
    var cx = f64_sub(HVec2_x(cp), ax);
    var cy = f64_sub(HVec2_y(cp), ay);

    var pr1 = f64_mul(bx, cy);
    var pr2 = f64_mul(by, cx);
    var d = f64_mul(_COL_F64_TWO, f64_sub(pr1, pr2));

    # Conditioning gate, NOT a degeneracy test. |d| is a difference of two
    # products, so its own cancellation error is ~eps * (|pr1| + |pr2|); demanding
    # that |d| dominate that by 2^20 caps the relative error of d, and hence of
    # the centre below, at ~1e-10. Testing the radius-to-longest-edge ratio
    # instead does NOT work: a needle formed by the super-triangle apex and two
    # near-coincident points has r/edge ~ 0.7, looks perfectly well shaped, and
    # still loses every digit of d. That exact case produced the one unsound
    # retirement observed during verification.
    var perm = f64_add(f64_abs(pr1), f64_abs(pr2));
    if (f64_gt(f64_abs(d), f64_mul(_COL_DL_SWEEP_REL, perm)) == 0) {
        return _COL_F64_POS_INF;
    }

    var b_sq = f64_add(f64_mul(bx, bx), f64_mul(by, by));
    var c_sq = f64_add(f64_mul(cx, cx), f64_mul(cy, cy));

    var ux = f64_div(f64_sub(f64_mul(cy, b_sq), f64_mul(by, c_sq)), d);
    var uy = f64_div(f64_sub(f64_mul(bx, c_sq), f64_mul(cx, b_sq)), d);
    var r = f64_sqrt(f64_add(f64_mul(ux, ux), f64_mul(uy, uy)));

    # Pad outward by 2^-20 of the working scale -- ~4000x the residual error the
    # gate above admits -- plus EPSILON_F64 absolute, which also covers the
    # x-equality band _col_sort_indices_by_xy uses. |ax| + |ux| rather than
    # |ax + ux| so the pad still covers a cancellation in that sum.
    var scale = f64_add(f64_add(f64_abs(ax), f64_abs(ux)), r);
    var pad = f64_add(f64_mul(_COL_DL_SWEEP_REL, scale), EPSILON_F64);
    var max_x = f64_add(f64_add(f64_add(ax, ux), r), pad);
    if (f64_lt(f64_abs(max_x), _COL_F64_POS_INF) == 0) { return _COL_F64_POS_INF; }
    return max_x;
}

(3) delaunay_2d. Everything up to and including `var st_c = n + 2;` is unchanged,
    as is the super-triangle filter and the CCW pass at the end. The replaced
    span is from the `var tris = vec_new();` on line 105 to the end of the
    insertion loop on line 190. Full body of the changed region:

    # Insert in ascending x. The Delaunay triangulation of points in general
    # position does not depend on insertion order, but a monotone order is what
    # makes retirement possible below -- and retirement is the whole fix.
    # Reuses the heapsort convex_hull_2d already carries (collision_core.cyr).
    var order = vec_new();
    for (var i = 0; i < n; i = i + 1) {
        vec_push(order, i);
    }
    _col_sort_indices_by_xy(order, points, n);

    # min_fx[k] = the smallest x among the points not yet inserted at step k
    # (k itself included). Derived from the ORDER THAT CAME BACK rather than
    # assumed from the sort: _col_xy_greater compares x within an EPSILON_F64
    # band, which is not transitive, so a chain of near-equal x can come out
    # slightly unordered. Taking the suffix minimum makes retirement sound
    # whatever the comparator did -- a badly ordered input costs speed, never
    # correctness.
    var min_fx = vec_new();
    for (var i = 0; i < n; i = i + 1) {
        vec_push(min_fx, _COL_F64_POS_INF);
    }
    var run_min = _COL_F64_POS_INF;
    for (var i = n - 1; i >= 0; i = i - 1) {
        run_min = f64_min(run_min, HVec2_x(vec_get(all_points, vec_get(order, i))));
        vec_set(min_fx, i, run_min);
    }

    # Triangles stored as flat array of index triples in a vec
    var tris = vec_new();
    vec_push(tris, st_a);
    vec_push(tris, st_b);
    vec_push(tris, st_c);
    var n_tris = 1;

    # Parallel to tris, one sweep bound per triangle. Kept in step through every
    # swap-remove below -- a stale entry here retires the wrong triangle.
    var tri_max_x = vec_new();
    vec_push(tri_max_x, _col_circum_max_x(all_points, st_a, st_b, st_c));

    # Triangles the sweep has retired. Final, so they leave the scanned set
    # entirely; folded back in after the last insertion.
    var done = vec_new();

    # Incrementally insert each point
    for (var k = 0; k < n; k = k + 1) {
        var i = vec_get(order, k);
        var p = vec_get(all_points, i);
        var px = HVec2_x(p);
        var py = HVec2_y(p);
        var front = vec_get(min_fx, k);

        # Find bad triangles, retiring the ones the front has passed as we go.
        #
        # Retiring inside this pass is safe on both counts. A retired triangle
        # can never be bad: retirement means every remaining point, this one
        # included, is right of its circumcircle. And the triangle swapped into
        # the freed slot always comes from index >= si, which this pass has not
        # visited, so no index already recorded in `bad` is invalidated.
        var bad = vec_new();
        var si = 0;
        while (si < n_tris) {
            if (f64_gt(front, vec_get(tri_max_x, si)) == 1) {
                vec_push(done, vec_get(tris, si * 3));
                vec_push(done, vec_get(tris, si * 3 + 1));
                vec_push(done, vec_get(tris, si * 3 + 2));
                var rlast = n_tris - 1;
                if (si != rlast) {
                    vec_set(tris, si * 3,     vec_get(tris, rlast * 3));
                    vec_set(tris, si * 3 + 1, vec_get(tris, rlast * 3 + 1));
                    vec_set(tris, si * 3 + 2, vec_get(tris, rlast * 3 + 2));
                    vec_set(tri_max_x, si, vec_get(tri_max_x, rlast));
                }
                vec_pop(tris);
                vec_pop(tris);
                vec_pop(tris);
                vec_pop(tri_max_x);
                n_tris = n_tris - 1;
            } else {
                var s0 = vec_get(tris, si * 3);
                var s1 = vec_get(tris, si * 3 + 1);
                var s2 = vec_get(tris, si * 3 + 2);
                if (_col_in_circumcircle(all_points, s0, s1, s2, px, py) == 1) {
                    vec_push(bad, si);
                }
                si = si + 1;
            }
        }

        # --- boundary polygon: UNCHANGED from here, lines 128-160 verbatim ---
        var poly_edges = vec_new();  # pairs: [e0, e1, e0, e1, ...]
        var n_bad = vec_len(bad);
        for (var bi = 0; bi < n_bad; bi = bi + 1) {
            var ti = vec_get(bad, bi);
            var t0 = vec_get(tris, ti * 3);
            var t1 = vec_get(tris, ti * 3 + 1);
            var t2 = vec_get(tris, ti * 3 + 2);
            var edges_v = vec_new();
            vec_push(edges_v, t0); vec_push(edges_v, t1);
            vec_push(edges_v, t1); vec_push(edges_v, t2);
            vec_push(edges_v, t2); vec_push(edges_v, t0);

            for (var ei = 0; ei < 3; ei = ei + 1) {
                var e0 = vec_get(edges_v, ei * 2);
                var e1 = vec_get(edges_v, ei * 2 + 1);
                var is_shared = 0;
                for (var bj = 0; bj < n_bad; bj = bj + 1) {
                    if (is_shared == 0) {
                        var otj = vec_get(bad, bj);
                        if (otj != ti) {
                            if (_col_tri_has_edge(tris, otj, e0, e1) == 1) {
                                is_shared = 1;
                            }
                        }
                    }
                }
                if (is_shared == 0) {
                    vec_push(poly_edges, e0);
                    vec_push(poly_edges, e1);
                }
            }
        }

        # Sort bad indices descending (unchanged)
        for (var sa = 0; sa < n_bad; sa = sa + 1) {
            for (var sb = sa + 1; sb < n_bad; sb = sb + 1) {
                if (vec_get(bad, sb) > vec_get(bad, sa)) {
                    var tmp = vec_get(bad, sa);
                    vec_set(bad, sa, vec_get(bad, sb));
                    vec_set(bad, sb, tmp);
                }
            }
        }
        # Swap-remove in descending order -- now mirrored into tri_max_x
        for (var bi = 0; bi < n_bad; bi = bi + 1) {
            var ti = vec_get(bad, bi);
            var last = n_tris - 1;
            if (ti != last) {
                vec_set(tris, ti * 3,     vec_get(tris, last * 3));
                vec_set(tris, ti * 3 + 1, vec_get(tris, last * 3 + 1));
                vec_set(tris, ti * 3 + 2, vec_get(tris, last * 3 + 2));
                vec_set(tri_max_x, ti, vec_get(tri_max_x, last));
            }
            vec_pop(tris);
            vec_pop(tris);
            vec_pop(tris);
            vec_pop(tri_max_x);
            n_tris = n_tris - 1;
        }

        # Create new triangles from polygon edges to point i (NOT k -- i is the
        # point index, k is only its rank in the sweep)
        var n_poly = vec_len(poly_edges) / 2;
        for (var ei = 0; ei < n_poly; ei = ei + 1) {
            var e0 = vec_get(poly_edges, ei * 2);
            var e1 = vec_get(poly_edges, ei * 2 + 1);
            vec_push(tris, e0);
            vec_push(tris, e1);
            vec_push(tris, i);
            vec_push(tri_max_x, _col_circum_max_x(all_points, e0, e1, i));
            n_tris = n_tris + 1;
        }
    }

    # Fold the retired triangles back in -- they are part of the answer, they
    # just stopped being candidates. The filter and CCW passes below are
    # unchanged and handle them like any other.
    var n_done = vec_len(done) / 3;
    for (var di = 0; di < n_done; di = di + 1) {
        vec_push(tris, vec_get(done, di * 3));
        vec_push(tris, vec_get(done, di * 3 + 1));
        vec_push(tris, vec_get(done, di * 3 + 2));
        n_tris = n_tris + 1;
    }

    # ... existing super-triangle filter (line 192) and CCW pass (line 218)
    # follow unchanged ...

## GAIN
Measured, not expected: -86.6% at the proposed CI size (n=1200: 122.595 ms -> 16.467 ms, 7.4x), -89% at n=1600 (217.5 -> 23.3 ms, 9.3x), -93% at n=3200 (847 -> 58.5 ms, 14.5x), -96% at n=6400 (3.376 s -> 148 ms, 22.8x). The gain compounds because the exponent changes: growth per doubling goes from 3.4-4.0x (O(n^2)) to 2.1-2.5x (O(n^1.5)).

## RESULT RISK (author)
Not "none". I checked, and here is exactly what I found.

WHAT I VERIFIED. I ran the shipped and the new function side by side on 13
fixtures, comparing the canonical triangle SET (each triple sorted, then the list
sorted, so output order and vertex rotation are factored out). I also isolated the
two halves of the change by running a sort-only variant with retirement disabled.
And I audited retirement directly: on every retirement, the retired triangle was
re-tested against every point not yet inserted using the exact
_col_in_circumcircle predicate, counting any point that was still inside.

RETIREMENT IS INERT. On all 13 fixtures, sort-only and sort+retire produced
identical sets, and the direct audit found 0 unsound retirements out of 2,700+
retirements. So the sweep bound never dropped a triangle that was still needed.
Getting there took one correction: my first conditioning gate tested circumradius
against the LONGEST edge, and it let through a needle formed by the super-triangle
apex plus two points 1e-13 apart — r/edge was 0.71, so it looked well shaped, but
d was 1.5e-11 against products of magnitude 100, so ux/uy were noise at the 1e-3
level and the computed bound came out at x = -0.0028 when the true rightmost point
of the circle was +0.00085. That retired a triangle a later point was genuinely
inside. The gate in the fix tests |d| against its own cancellation error, which
catches that case (threshold 1.9e-4 vs |d| = 1.5e-11) and drives it to +inf. After
that change the audit is clean, including a fixture with coordinates at 1e-14,
where the sweep now retires nothing at all and falls back to today's behaviour.

WHAT DOES CHANGE.
1. Output ORDER. Triangles come out in sweep order with the retired ones appended
   before the filter pass, so a consumer that indexes triangles positionally or
   relies on the rotation within a triple will see different bytes for the same
   triangulation. The repo's own tests assert only counts and the empty-
   circumcircle property, so they are unaffected — I replayed all 8 delaunay_2d
   assertions from tests/modules.tcyr against the new function and all 8 pass
   (square+interior 12, 3x3 cocircular grid 24, irregular-6, n=3, n<3, collinear).
2. Ties. For points in general position the triangulation is unique, so the set is
   unchanged — confirmed identical on scattered (n=200, 600), 5x5/9x9/12x12
   cocircular grids, duplicate points, all-collinear, all-same-x, negative and
   fractional coordinates with a 1e5 x-gap, and 1e-14 coordinates (n=150). Where
   the set is genuinely non-unique (cocircular quads) a different but equally
   valid diagonal is possible; I did not hit one in these fixtures, but I cannot
   rule it out.
3. Inputs where the shipped function is ALREADY wrong. On a fully cocircular set
   (n points on a circle) delaunay_2d does not produce a valid triangulation
   today: at n=40 it returns 216 triangles where a convex-position set admits
   exactly 38, with 2,842 empty-circumcircle violations and a total triangle area
   4,016% of the hull — i.e. heavily overlapping. The new order returns 228
   triangles / 3,129 violations / 3,507%. At n=200 it is 3,299 vs 5,753 triangles
   (52,798% vs 85,701% of hull area). Both are garbage; the sweep changes which
   garbage, and at n=200 produces more of it. This is a pre-existing robustness
   failure of the non-adaptive in-circle predicate, not something the fix
   introduces — but if a consumer feeds near-cocircular data, its (already
   invalid) output will change. That is a separate defect worth its own finding.
4. Empty / tiny inputs are untouched: n < 3 still returns empty before any of the
   new code runs, and collinear input still returns empty (0 triangles, verified).
NaN or infinite coordinates flow to +inf from _col_circum_max_x (f64_gt is
NaN-false in the safe direction at every branch) so nothing is retired and
behaviour matches today's.

## BENCH
# Add to tests/hisab.bcyr. The include list already has src/collision_mesh.cyr,
# src/collision_core.cyr and src/vec2.cyr, so no include changes are needed.

# --- with the other _b_* globals near the top ---
var _b_dl_pts = 0;
var _b_dl_n = 0;

# --- with the other collision hot-path bench fns (beside bench_convex_hull_2d) ---
# Delaunay was the third O(n^2) collision path. Every point insertion rescanned
# every live triangle, so the circumcircle test ran ~n^2 times: 3.9x per
# doubling, 122 ms at n=1,200 and 3.4 s at n=6,400. The x-sweep retires a
# triangle once the front passes its circumcircle, which drops the scan to the
# sweep front. 5 iterations, not 20: each build allocates ~1.8 MB from the
# never-freeing bump arena.
fn bench_delaunay_2d() {
    var t = delaunay_2d(_b_dl_pts, _b_dl_n);
    return 0;
}

# --- in main(), beside the _b_hull_pts setup ---
    # 1,200 scattered points -> ~2,370 triangles. Two coprime moduli, so no two
    # points coincide and the cloud is not a lattice the sweep could exploit.
    _b_dl_n = 1200;
    _b_dl_pts = vec_new();
    var di = 0;
    while (di < _b_dl_n) {
        var dpx = (di * 2654435761) % 10007;
        var dpy = (di * 40503) % 10009;
        vec_push(_b_dl_pts, hvec2_new(f64_from(dpx), f64_from(dpy)));
        di = di + 1;
    }

# --- with the other collision bench() calls ---
    bench("delaunay_2d_1200",   &bench_delaunay_2d,             5);

# Measured with this exact fixture (dl_final.bcyr, 5 iters):
#   before 122.595 ms avg (min 120.822, max 125.438)
#   after   16.467 ms avg (min  16.229, max  16.692)   -86.6%
# bench-history.csv row: delaunay_2d_1200, 122595000 -> 16467000 ns

## CHALLENGE changes_results=True compiles=True repro=True stands=True
- THE CENTRAL SAFETY CLAIM IS FALSE. 'RETIREMENT IS INERT ... sort-only and sort+retire produced identical sets' does not hold. I built three variants (shipped / patched / patched-with-retirement-disabled by forcing `front = _COL_F64_NEG_INF`) and compared canonical triangle sets. On near-coincident cluster inputs the patched output differs from the sort-only output: 30 clusters x 5 points at 1e-9 separation over a ~77-unit domain -> 95 vs 125 triangles; at 1e-13 -> 101 vs 129; at 1e-16 -> 23 vs 34. MINIMAL REPRODUCER AT n=8: two clusters of four points each, separation 1e-7, centres (0,0) and (415/13, 951/13); the offsets are (0,0),(1e-7,2e-7),(2e-7,4e-7),(3e-7,1e-7). There `old == sortonly` but `new != sortonly`, so retirement alone -- not the sort -- changes the answer. Their own audit method (re-testing each retirement against every remaining point with `_col_in_circumcircle`) flags 23,741 retirements across five cluster fixtures; they reported 0 out of 2,700+. Their fixture set simply contains no near-coincident clusters, which is the single most common degeneracy a physics/mesh consumer feeds a triangulator.
- THE DIRECTION IS ACTUALLY IN THEIR FAVOUR, AND THEY DID NOT KNOW IT. I re-checked all 23,741 flagged retirements in exact rational arithmetic: 0 are geometrically unsound. `_col_circum_max_x` is right in every one; it is the shipped `_col_in_circumcircle` that is wrong. Decoded example from the probe: triangle (0,0),(1e-9,2e-9),(3e-9,1e-9) has circumcentre (1.5e-9, 5e-10), radius 1.5811e-9, true rightmost x 3.08114e-9; `_col_circum_max_x` returns 3.08214e-9 (correctly padded outward). The point at (5.4615, 63.2308) is 63.47 away from that centre, yet the shipped predicate returns det = +2.3045e-10 where the exact value is -2.0140e-14 -- a sign error from catastrophic cancellation, so the shipped code destroys a valid tiny triangle on noise. Retirement suppresses that. Consequence on my cluster fixture at 1e-9 separation: total triangle area / convex-hull area is 7.277 for the shipped code, 2.800 for sort-only, and 0.9997 for the patch. So the patch is much closer to a valid triangulation there. That is an unclaimed, untested behaviour change resting on a predicate the patch's own safety argument assumes is faithful and which demonstrably is not.
- UNMEASURED 1.9x-3.0x SLOWDOWN AND UP TO 2.5x MEMORY BLOWUP ON COCIRCULAR INPUT. They wrote 'the sweep changes which garbage' and stopped there; they never timed it. Points on a circle, shipped -> patched: n=100 191.1 ms -> 359.6 ms (1.88x SLOWER); n=200 4.408 s -> 8.231 s (1.87x); n=300 16.218 s -> 48.998 s (3.02x). Bump-arena bytes per call: n=100 2,476 -> 3,964 KiB; n=200 16,097 -> 25,751 KiB; n=300 35,233 -> 75,805 KiB; n=400 72,913 -> 184,520 KiB (+153%). Triangle counts: n=400 17,325 -> 25,078 (+45%). Under a never-freeing bump allocator 180 MB per call is 180 MB permanently gone. A circle-sampled boundary or regular polygon is an ordinary input for impetus/kiran/joshua, and the regression grows with n. Their proposed CI fixture (scattered, n=1200) would never see it.
- 'I DID NOT HIT [A NON-UNIQUE COCIRCULAR QUAD]' -- I HIT ONE AT n=4, ON THE UNIT SQUARE. Points (0,0),(1,0),(1,1),(0,1): shipped returns diagonal 0-2 (triangles {0,1,2},{0,2,3}); patched returns diagonal 1-3 ({0,1,3},{1,2,3}). Both valid, zero shared triangles. This is the simplest possible cocircular input and every axis-aligned quad and every regular grid cell is one. 'Cannot rule it out' understates it to the point of being misleading.
- DUPLICATE POINTS ARE SILENTLY RE-INDEXED. On 12 of 12 duplicate-heavy integer-lattice seeds (120 points on an 11x11 lattice) the canonical triangle set differs while the geometry is identical (same area, 0 violations for both). Overlap is only 16-52 of ~120 triangles. Any consumer that maps a triangle's vertex indices back to per-index attributes -- weights, UVs, materials, ids -- will read different attributes for the same mesh.
- THE MEMORY FIGURE IS MISREPORTED. '17.8 MB over 10 calls at n=1200, ~1.8 MB per call' averages 5 shipped calls with 5 patched calls, so it hides the delta. Measured separately from `_heap_used` at n=1200 on the scattered fixture: shipped 1,420 KiB per call, patched 2,046 KiB per call -- +44%, permanent under the bump allocator (the new `order`, `min_fx`, `tri_max_x`, `done` vecs plus their doubling-growth intermediates).
- PROCESS GAPS. (a) The fix touches src/collision_mesh.cyr, so `cyrius distlib` must be re-run or CI's distlib gate fails -- I verified the committed dist/hisab.cyr is byte-current against pristine src and differs after the patch; the proposal never mentions it. (b) collision_mesh.cyr now depends on a FUNCTION from collision_core.cyr (`_col_sort_indices_by_xy`) and on `EPSILON_F64` from error.cyr, where before it only borrowed constants. That is fine under the current [lib] order and both current includers (tests/modules.tcyr, tests/hisab.bcyr) pull collision_core, but it is a new coupling worth stating. (c) `cyrius fmt --check <file>` prints a usage error; the working form is `cyrius fmt <file> --check` (returns 0 on the patched file) -- pre-existing CLI quirk, not caused by this change.

### CORRECTED
The code itself needs no change -- I could not break it. The CLAIMS and the TEST GATE need fixing.

WHAT I VERIFIED CLEAN (patch applied to a copy at /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/work):
- Compiles. `cyrius lint src/collision_mesh.cyr` 0 warnings, no line over 120 chars, `cyrius fmt <f> --check` rc=0, `cyrius vet src/main.cyr` OK, `cyrius fuzz` PASS.
- `./scripts/check-constants.sh` 142/142. I traced the regex: the `# 2^-20` comment IS machine-verified (eval_exact turns "2^-20" into 2**-20, 8-ulp tolerance, 0x3EB0_0000_0000_0000 matches exactly). `_COL_DL_SWEEP_REL` starts with `_` so the duplicate-globals gate skips it; no name collision exists.
- Full suites: 1523/1523 before and after, identical (hisab 306, foundation 310, modules 692, edge_cases 215). All eight delaunay_2d assertions still pass.
- The circumcentre formula is correct, `d == 2*cross` bit-for-bit shares the shipped winding test (so cross==0 always forces +inf), tri_max_x is mirrored correctly through both swap-remove sites, `bad` indices survive in-scan retirement (moved element always comes from index >= si), and n_tris/vec_len stay consistent through the done fold-back.
- General position is safe: 40 random seeds (n=66..240) plus scatter-200/600, fractional+negative with a 1e5 gap, 1e-14 coords, 1e12 coords, two far clusters, parabola, near-collinear, mixed-scale, x-jump, 3x3/5x5/9x9/12x12 grids -> byte-identical canonical triangle sets. n=0/1/2 and all-collinear/all-same-x/all-coincident return empty in both.
- Retirement is geometrically sound: 0 of 23,741 exact-rational violations.

MEASUREMENT REPRODUCES, within ~2% of theirs. Shipped -> patched, ms: n=100 1.208->0.711; 200 3.886->1.660; 400 14.556->5.205; 800 54.813->10.580; 1200 122.397->15.956 (-87.0%, they said -86.6%); 1600 216.559->22.895; 3200 856.911->56.327; 6400 3403->142.2. Growth per doubling shipped 3.22/3.75/3.77/3.96/3.97 -> 4.0, so O(n^2) is real. I pushed the patched version to n=25,600 on an independent wider fixture: 29.7/80.6/182.6/447.5/1058 ms, ratios 2.71/2.27/2.45/2.36 -- stable near 2.4, exponent ~1.25. The exponent genuinely changed; this is not a constant factor dressed up. The win is real at modest n too: 1.7x at n=100, 2.8x at n=400, 5.2x at n=800.

REQUIRED CORRECTIONS BEFORE LANDING:
1. Delete the "RETIREMENT IS INERT" claim. Replace with: retirement changes the result whenever the shipped in-circle predicate disagrees with exact geometry, which happens routinely on near-coincident points; the sweep bound is geometrically correct in every case tested (0/23,741 exact-arithmetic violations) and the change there is toward validity, but it is a behaviour change, not a no-op.
2. Delete the "0 unsound retirements" evidence, or re-state it honestly: their audit predicate is `_col_in_circumcircle`, which returns 0 only because their fixtures contain no near-coincident clusters. Re-run their audit on the n=8 reproducer above -- it fires immediately.
3. Add the cocircular cost numbers to the "what does change" list. "Changes which garbage" must become "1.9x-3.0x slower and up to 2.5x more permanently-allocated memory on cocircular input, growing with n."
4. Fix the memory line: 1,420 -> 2,046 KiB per call at n=1200, +44%, not "~1.8 MB per call".
5. Change the n=4 unit-square wording from "I cannot rule it out" to the concrete diagonal flip, and add the duplicate-point re-indexing as a named consumer-visible consequence.
6. The CI bench fixture must not be scattered-only. Add a cocircular guard -- e.g. `bench("delaunay_2d_circle_200", ...)` on 200 points of a circle -- so the 1.9x regression is visible in bench-history.csv rather than discovered by a consumer. Add regression tests for the n=8 cluster reproducer and the n=4 unit square, pinning whatever behaviour is chosen.
7. Run `cyrius distlib` and commit dist/hisab.cyr.
8. File the separate finding the proposal already gestures at, and make it a blocker for anyone feeding near-cocircular data: `_col_in_circumcircle` is a non-adaptive determinant with catastrophic cancellation. Both the shipped and patched functions produce heavily overlapping non-triangulations on cocircular input (n=400 on a circle: 17,325 / 25,078 triangles where 398 is correct). The honest fix for that module is an adaptive or filtered in-circle predicate; this sweep optimisation is orthogonal to it and should not be described as if it were unaffected by it.