# Phase 06: robustness-gap-closure - Context

**Gathered:** 2026-05-12
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase turns the audit, fixture, benchmark, and oracle evidence from Phases
3-5 into a ranked robustness backlog, then closes the first high-value current
Rust robustness issue with focused tests and minimal production changes. It does
not broaden public APIs, add Clipper2 as a runtime dependency, import old C++
algorithms wholesale, regenerate FFI headers, or redesign the UI.

</domain>

<decisions>
## Implementation Decisions

### Backlog and Ranking

- **D-01:** Build an explicit ranked robustness backlog before changing
  algorithm code.
- **D-02:** Rank candidates by user-visible correctness risk, evidence quality,
  reproducibility, blast radius, and fit with current Rust semantics.
- **D-03:** Treat metadata-only gaps as evidence. Do not force a fix when the
  observed difference is property-equivalent or intentionally divergent.
- **D-04:** Phase 6 must cover offsets, booleans, intersections, tolerances,
  degenerates, repeat vertices, tangencies, overlaps, and open/closed behavior
  in the backlog even if the first fix touches only one surface.

### First Fix Target

- **D-05:** Prefer a small, high-confidence robustness fix over broad boolean or
  offset rewrites.
- **D-06:** The first fix target is the `Shape::from_plines` / shape offset
  input boundary for repeated-position, redundant, or collapsed polylines.
  Existing polyline offset tests already cover repeat-position sanitization;
  shape offset should receive equivalent focused coverage.
- **D-07:** If the shape input boundary already behaves correctly under focused
  regressions, keep those tests as coverage and promote the next ranked backlog
  item rather than inventing a code change.

### Deferred or Lower-Priority Evidence

- **D-08:** The old C++ circle/rectangle boolean vertex-count mismatch remains
  a gap record unless Phase 6 proves a real property or topology failure.
- **D-09:** Clipper2 broad text fixtures such as `Polygons.txt` case 17 are
  candidates for future robustness evidence, but they should not drive broad
  parser work or a production Clipper2 dependency.
- **D-10:** Performance benchmark data informs prioritization, not optimization
  work, in this phase.

### Verification

- **D-11:** Every code change needs a focused targeted test before or with the
  fix.
- **D-12:** Final verification includes the targeted regression test,
  `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy
  --all-targets -- -D warnings`, `git diff --check`, generated-output checks
  where relevant, and GSD health.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md`
- `.planning/phases/03-historical-c-evidence-mining/03-VERIFICATION.md`
- `.planning/phases/04-benchmark-baseline/04-BENCHMARK-MAP.md`
- `.planning/phases/04-benchmark-baseline/04-BENCHMARKS.md`
- `.planning/phases/05-clipper2-oracle-boundary/05-ORACLE-EVIDENCE.md`
- `.planning/phases/05-clipper2-oracle-boundary/05-CLIPPER2-INVENTORY.md`
- `cavalier_contours/src/shape_algorithms/mod.rs`
- `cavalier_contours/src/polyline/pline_types.rs`
- `cavalier_contours/tests/test_shape_parallel_offset.rs`
- `cavalier_contours/tests/test_pline_parallel_offset.rs`
- `cavalier_contours/tests/test_pline_boolean.rs`
- `cavalier_contours/tests/test_clipper2_oracle_fixtures.rs`
- `cavalier_contours/tests/test_historical_cavalier_contours.rs`

</canonical_refs>

<code_context>
## Existing Code Insights

- `Shape::from_plines` currently filters only `vertex_count() > 1`, then builds
  `IndexedPolyline` values and later expects spatial-index bounds for every
  retained polyline.
- Polyline offset already has repeat-position regressions in
  `test_pline_parallel_offset.rs`; shape offset does not have equivalent
  repeat/degenerate input coverage.
- `Shape::from_plines` is a narrow public boundary where input sanitization can
  happen without changing external API shape.
- The old C++ boolean gap currently records a vertex-count mismatch while
  preserving equivalent area/path/extents; this should be ranked but not assumed
  to require a fix.
- Phase 5 Clipper2 oracle evidence has two passing executable fixtures and
  several deferred/not-comparable records for Phase 6 ranking.

</code_context>

<specifics>
## Specific Plan Direction

- Create `06-ROBUSTNESS-BACKLOG.md` with ranked candidates and scope decisions.
- Add focused shape-offset regressions for repeat-position, redundant, and
  collapsed input polylines.
- Harden `Shape::from_plines` if tests expose that degenerate inputs survive
  into indexing or offset processing.
- Record final verification and requirement coverage in `06-VERIFICATION.md`.

</specifics>

---

*Phase: 06-robustness-gap-closure*
*Context gathered: 2026-05-12*

