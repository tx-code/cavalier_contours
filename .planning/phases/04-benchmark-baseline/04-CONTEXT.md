# Phase 04: benchmark-baseline - Context

**Gathered:** 2026-05-12
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase establishes repeatable benchmark coverage and baseline documentation
for current Rust `cavalier_contours` behavior. It maps old C++
CavalierContours benchmark profile families into Rust measurement cases and
records cost-accounting rules. It does not fix algorithms, set performance
budgets, run Clipper2 oracle comparisons, change public APIs, touch FFI/header
surfaces, or redesign the UI.

</domain>

<decisions>
## Implementation Decisions

### Benchmark Harness Shape

- **D-01:** Use a stable Rust benchmark harness, preferably Criterion, rather
  than nightly `#[bench]` or ad hoc timing loops.
- **D-02:** Put repo-owned benchmark definitions under the core crate benchmark
  surface, such as `cavalier_contours/benches/`, with reusable profile builders
  separated from individual operation benchmarks when that keeps duplication low.
- **D-03:** Do not commit generated benchmark output directories such as
  `target/criterion`; commit only source benchmark definitions and a concise
  baseline/provenance document.

### Coverage and Mapping

- **D-04:** Current Rust baseline coverage must include offsets, booleans,
  intersections, and spatial-index-heavy inputs.
- **D-05:** Old C++ benchmark profile families should be mapped by shape and
  operation, not copied as C++ implementation code. Required profile families
  include square, diamond, circle, rounded rectangle, profile1, profile2, and
  pathologicalProfile1.
- **D-06:** Historical benchmark source paths to map include
  `tests/benchmarks/benchmarkprofiles.h`, `offsetbenchmarks.cpp`,
  `combinebenchmarks.cpp`, `spatialindexbenchmarks.cpp`,
  `areabenchmarks.cpp`, `extentsbenchmarks.cpp`, `pathlengthbenchmarks.cpp`,
  and `windingnumberbenchmarks.cpp`.

### Cost Accounting

- **D-07:** Native arc-aware Rust benchmarks and arc-to-line converted variants
  must be separate cases. Conversion cost is excluded unless a benchmark is
  explicitly named as measuring conversion.
- **D-08:** Clipper2 runtime/oracle costs remain out of Phase 4. Clipper2 cost
  policy belongs to Phase 5 because comparability and approximation rules are
  not yet established.
- **D-09:** Baseline documentation must state environment, command, harness
  mode, whether dependency optimization/profile settings were used, and whether
  conversion/oracle/setup costs are included or excluded.

### Verification and Scope

- **D-10:** Verification should include normal workspace gates plus a benchmark
  compile/smoke command that is practical for CI-like local validation. Full
  benchmark measurement can be documented as a manual/local baseline command.
- **D-11:** The benchmark phase may add dev-only dependencies and benchmark
  source files, but it must not change production geometry behavior.

### the agent's Discretion

The agent may choose exact benchmark file names, grouping, Criterion benchmark
IDs, and sample/smoke command details as long as coverage, provenance, and cost
accounting decisions above are preserved.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Planning

- `.planning/PROJECT.md` - core value, source roles, and no port-first rule.
- `.planning/REQUIREMENTS.md` - Phase 4 requirements `BEN-01`, `BEN-02`, and
  `BEN-03`.
- `.planning/ROADMAP.md` - Phase 4 goal, success criteria, and plan skeleton.
- `.planning/STATE.md` - current workflow position.

### Prior Phase Evidence

- `.planning/phases/01-absorption-contract-audit/01-PROVENANCE.md` - old C++
  repo snapshot, license, and usage labels.
- `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md` -
  benchmark-candidate old C++ source inventory and Phase 4 deferrals.
- `.planning/phases/03-historical-c-evidence-mining/03-VERIFICATION.md` -
  proof that Phase 3 evidence is complete and ready for benchmark baseline work.

### Codebase Maps

- `.planning/codebase/TESTING.md` - standard workspace verification commands and
  integration-test patterns.
- `.planning/codebase/STACK.md` - Rust edition, MSRV, workspace members, and
  dependency policy context.
- `.planning/codebase/STRUCTURE.md` - crate layout and expected locations for
  core crate additions.

### Historical Benchmark Sources

- `E:/Coding/CavalierContours/tests/benchmarks/benchmarkprofiles.h` - old C++
  profile builders and macro-generated shape families.
- `E:/Coding/CavalierContours/tests/benchmarks/offsetbenchmarks.cpp` - old C++
  offset measurement loop.
- `E:/Coding/CavalierContours/tests/benchmarks/combinebenchmarks.cpp` - old C++
  shifted/coincident boolean measurement loops.
- `E:/Coding/CavalierContours/tests/benchmarks/spatialindexbenchmarks.cpp` -
  old C++ spatial index creation/query benchmark behavior.
- `E:/Coding/CavalierContours/tests/benchmarks/CMakeLists.txt` - old benchmark
  executable grouping and Google Benchmark dependency boundary.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `cavalier_contours/tests/test_utils/pline_test_properties.rs` provides
  property helpers that can inform baseline validation, but benchmark code
  should avoid test-only assertions inside timed loops.
- `cavalier_contours/tests/test_historical_cavalier_contours.rs` already
  contains old C++ provenance and selected profile-like shapes that can guide
  benchmark naming.
- `cavalier_contours/src/polyline/traits.rs` exposes the operations to measure:
  offset, boolean, area, extents, path length, winding, containment, and
  intersection-adjacent traversal.
- `static_aabb2d_index` is already a core dependency; spatial-index-heavy
  benchmark cases can exercise current Rust index creation/query paths without
  adding production dependencies.

### Established Patterns

- Rust workspace verification uses `cargo test --workspace`, `cargo fmt
  --all --check`, `cargo clippy --all-targets -- -D warnings`, and
  `git diff --check`.
- Existing tests prefer property parity and small reusable geometry builders;
  benchmark profiles should follow that style instead of copying broad C++
  tables.
- Phase 3 kept old C++ C API and static spatial index evidence metadata-only;
  Phase 4 may use them for benchmark mapping, not API changes.

### Integration Points

- `cavalier_contours/Cargo.toml` currently has no benchmark dev-dependency;
  adding Criterion or equivalent should stay dev-only.
- Root `Cargo.toml` workspace profiles optimize dependencies in dev builds;
  baseline docs should record which profile/command was used.
- No existing `cavalier_contours/benches/` directory was found, so Phase 4 may
  create the benchmark surface from scratch.

</code_context>

<specifics>
## Specific Ideas

- Start with one benchmark target for geometry baseline if that keeps execution
  simple, then split only if the file gets hard to navigate.
- Use old C++ profile names in Rust benchmark IDs where practical:
  `square`, `diamond`, `circle`, `rounded_rectangle`, `profile1`, `profile2`,
  and `pathological_profile1`.
- Include a short smoke command for development and a full baseline command for
  local measurement.

</specifics>

<deferred>
## Deferred Ideas

- Clipper2 oracle/runtime comparison remains Phase 5.
- Performance budgets, regression thresholds, and CI gating remain later work
  after a baseline exists.
- Algorithm optimization or robustness fixes remain Phase 6 or later.
- FFI benchmark coverage is not part of Phase 4 unless a later plan explicitly
  scopes it.

</deferred>

---

*Phase: 04-benchmark-baseline*
*Context gathered: 2026-05-12*

