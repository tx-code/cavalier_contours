# Phase 03: historical-c-evidence-mining - Context

**Gathered:** 2026-05-12
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase mines the old C++ CavalierContours repository for historical
regression evidence and brings a curated subset into the Rust test harness
created in Phase 2. It prioritizes old offset, combine/boolean, polyline
property, C API, and spatial-index evidence, but it does not perform broad
algorithm porting, public Rust API changes, FFI surface changes, benchmark
baseline work, Clipper2 oracle work, or UI work.

</domain>

<decisions>
## Implementation Decisions

### Evidence Scope and Priorities

- **D-01:** Translate executable historical fixtures only from high-value old
  C++ offset, combine/boolean, and polyline property cases.
- **D-02:** Include old C API evidence in the Phase 3 inventory as
  migration-sensitive metadata. Do not change current Rust FFI code or
  regenerate `cavalier_contours_ffi.h` in this phase.
- **D-03:** Include old static spatial index evidence in the Phase 3 inventory
  with behavior notes. Performance-sensitive benchmark treatment is deferred
  to Phase 4.
- **D-04:** Historical examples and benchmark profiles may be inventoried for
  fixture value, but benchmark profile mapping and measurement design remain
  out of scope.

### Translation Shape

- **D-05:** Use manually curated typed Rust fixtures. Do not add C++ parsers,
  code generators, JSON, RON, TOML, or other external fixture formats in this
  phase.
- **D-06:** Reuse Phase 2 `FixtureCase` schema and `run_fixture(&FixtureCase)`
  harness wherever the operation already maps cleanly to offset, boolean, or
  contains/properties behavior.
- **D-07:** If non-executable C API or spatial-index records need new test-only
  metadata classifications, keep them test-only and metadata-only. Do not make
  them production APIs.

### Mismatch Handling

- **D-08:** Executable imported fixtures must keep `cargo test --workspace`
  green. Import exact or approximate parity cases only when current Rust
  behavior is expected to pass.
- **D-09:** Old C++ cases that expose a current Rust mismatch should be recorded
  as metadata-only `gap`, `not comparable`, or intentional-divergence evidence
  with provenance and tolerance notes instead of red tests.
- **D-10:** Phase 3 may identify future robustness or capability work, but
  algorithm fixes belong to later absorption phases.

### Provenance and Classification

- **D-11:** Every translated or represented case must cite source repo, commit,
  license, source path, usage label, comparison mode, and tolerance or
  non-comparability rationale.
- **D-12:** Old C++ expected results should be compared through geometry
  properties rather than brittle literal vertex ordering unless a case
  explicitly requires ordering evidence.
- **D-13:** Use Phase 2 centralized tolerances by default. Per-case overrides
  must be explicit and justified by the source test or geometry shape.

### the agent's Discretion

The agent may choose the exact curated case count and fixture IDs, as long as
the selected set covers offset, combine/boolean, and polyline property behavior
and keeps the phase small enough to plan and verify cleanly. The agent may add
a dedicated Phase 3 inventory document if that is clearer than overloading the
fixture test file.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Planning

- `.planning/PROJECT.md` - project goal, source roles, and fork/mainline
  decisions.
- `.planning/REQUIREMENTS.md` - Phase 3 requirement `FIX-03`.
- `.planning/ROADMAP.md` - Phase 3 goal, success criteria, and plan skeleton.
- `.planning/STATE.md` - current workflow position and session continuity.

### Prior Phase Contracts

- `.planning/phases/01-absorption-contract-audit/01-CONTEXT.md` - source usage,
  taxonomy, API/FFI boundaries, and deferred scope decisions.
- `.planning/phases/01-absorption-contract-audit/01-AUDIT.md` - capability
  inventory, comparison taxonomy, candidate registry, and public surface
  comparison.
- `.planning/phases/01-absorption-contract-audit/01-PROVENANCE.md` - required
  provenance fields, usage labels, repository snapshots, and acceptable-use
  rules.
- `.planning/phases/02-fixture-schema-and-property-harness/02-CONTEXT.md` -
  locked fixture schema, harness behavior, tolerance policy, and metadata-only
  taxonomy decisions.
- `.planning/phases/02-fixture-schema-and-property-harness/02-VERIFICATION.md`
  - proof that the Phase 2 schema, harness, and metadata-only seeds pass.

### Codebase Maps

- `.planning/codebase/TESTING.md` - integration test layout, commands, and
  property comparison patterns.
- `.planning/codebase/CONVENTIONS.md` - Rust style, naming, safety boundary,
  public API, and FFI guidance.
- `.planning/codebase/STRUCTURE.md` - workspace layout and planning artifact
  locations.

### Old C++ Historical Sources

- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp` -
  primary historical offset regression source.
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp` -
  primary historical combine/boolean regression source.
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp` -
  historical property, containment, winding, and function behavior source.
- `E:/Coding/CavalierContours/tests/tests/testhelpers.hpp` - old C++ property
  helper semantics and epsilon context.
- `E:/Coding/CavalierContours/c_api_include/cavaliercontours.h` and
  `E:/Coding/CavalierContours/src/cavaliercontours.cpp` - C API inventory and
  migration-sensitive behavior notes.
- `E:/Coding/CavalierContours/tests/tests/TEST_staticspatialindex.cpp` -
  spatial-index behavior inventory source.
- `E:/Coding/CavalierContours/examples/*.cpp` and
  `E:/Coding/CavalierContours/README.md` - reference-only examples and
  behavior notes.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `cavalier_contours/tests/test_utils/fixture_schema.rs`: defines
  `FixtureCase`, provenance, geometry model, comparison mode, tolerance, and
  metadata-only expected data.
- `cavalier_contours/tests/test_utils/fixture_harness.rs`: provides
  `run_fixture(&FixtureCase)` and metadata collection for offset, boolean, and
  contains/properties cases.
- `cavalier_contours/tests/test_fixture_harness.rs`: current-Rust seed fixture
  patterns for executable and metadata-only cases.
- `cavalier_contours/tests/test_utils/pline_test_properties.rs`: existing
  property comparison helpers and epsilon behavior used by the harness.

### Established Patterns

- Integration tests live under `cavalier_contours/tests/` and import shared
  helpers via `mod test_utils;`.
- Existing tests compare property sets such as result count, vertex count,
  area, path length, and extents instead of requiring literal vertex order.
- Metadata-only records are allowed for gaps or non-comparable cases and should
  not execute assertions.

### Integration Points

- New historical fixture tests should stay under `cavalier_contours/tests/`.
- Shared fixture construction helpers should stay under
  `cavalier_contours/tests/test_utils/` if they are reused.
- No Phase 3 changes should be made to `cavalier_contours/src/lib.rs`,
  `cavalier_contours_ffi/src/lib.rs`, or `cavalier_contours_ffi.h` unless a
  later explicit scope decision changes the phase boundary.

</code_context>

<specifics>
## Specific Ideas

- Prefer a small, high-signal sample over broad translation: representative
  offset edge cases, boolean/combine cases across union/exclude/intersect/xor,
  and property/containment cases with clear old C++ expected properties.
- Use old C++ commit `31a012947aa2e7e9474e2ec90502825afe8b99a4` and license
  `MIT` for historical provenance unless a fresh audit records a newer source
  snapshot.
- C API and spatial-index evidence should be visible in Phase 3 artifacts even
  when not executable through the current fixture harness.

</specifics>

<deferred>
## Deferred Ideas

- Broad C++ parameter-table parsing or fixture generation is deferred until a
  later phase proves the manual curated set is insufficient.
- FFI execution tests, ABI changes, and header regeneration are deferred to API
  and migration readiness work.
- Spatial-index performance measurement and benchmark profile mapping are
  deferred to Phase 4.
- Algorithm fixes for mismatching historical behavior are deferred to later
  robustness/capability absorption phases.
- Clipper2 oracle output and polygon-only eligibility remain Phase 5 work.

</deferred>

---

*Phase: 03-historical-c-evidence-mining*
*Context gathered: 2026-05-12*
