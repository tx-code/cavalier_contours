# Phase 02: fixture-schema-and-property-harness - Context

**Gathered:** 2026-05-12
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase creates a reusable, test-only fixture schema and property comparison
harness for the Rust `cavalier_contours` tests. It proves the schema with a
small set of current-Rust seed fixtures. It does not import old C++ fixtures,
does not generate Clipper2 oracle output, does not add benchmark profiles, and
does not expose new public Rust or FFI APIs.

</domain>

<decisions>
## Implementation Decisions

### Fixture Format and Layout

- **D-01:** Use Rust typed fixtures as the Phase 2 fixture format. Do not add
  JSON, RON, TOML, or file parser dependencies in this phase.
- **D-02:** Place schema and harness code under
  `cavalier_contours/tests/test_utils/`, reusing existing property helpers.
- **D-03:** Add seed proof tests in a separate integration test file,
  `cavalier_contours/tests/test_fixture_harness.rs`.
- **D-04:** Do not create an empty fixture data directory in Phase 2. Future
  file-based fixture import paths may be documented in types or comments.
- **D-05:** Keep all fixture schema and harness code test-only. Do not add it to
  the crate public API, FFI crate, or a new internal crate.

### Fixture Schema Minimum Fields

- **D-06:** Every fixture must include full provenance, including current Rust
  seed fixtures: source repo, source commit, source path, license, and usage
  label.
- **D-07:** Represent geometry model as an enum. It must distinguish native
  bulge-arc polylines from Clipper2 polygon paths.
- **D-08:** Represent operation as an enum with operation-specific input and
  option structures rather than one broad optional-field input type.
- **D-09:** Expected properties are required according to comparison mode. The
  schema must prevent a fixture from passing while asserting nothing.

### Property Comparison Strategy

- **D-10:** Support the full Phase 1 taxonomy in the schema:
  `exact parity`, `approximate parity`, `intentional divergence`,
  `not comparable`, and `gap`.
- **D-11:** The default harness only executes assertions for exact and
  approximate property comparisons. `gap`, `not comparable`, and
  `intentional divergence` cases may be recorded as metadata without creating
  failing tests.
- **D-12:** Add a unified test-only tolerance policy/helper. Defaults should
  preserve existing helper semantics: property comparison epsilon `1e-4`,
  position epsilon `1e-5`, and remove-redundant epsilon `1e-4`.
- **D-13:** Allow per-fixture tolerance overrides through the centralized
  tolerance policy. Do not scatter raw epsilon values through fixture tests.
- **D-14:** Use a layered default property set. Base assertions are result
  count plus per-result vertex count, area, path length, and extents. Opt-in
  properties include orientation, open/closed state, repeat vertices, user data,
  and absolute-area comparison.

### Harness Integration

- **D-15:** Provide one generic test-only runner, conceptually
  `run_fixture(&FixtureCase)`, that dispatches by operation enum and comparison
  mode.
- **D-16:** Phase 2 runner execution should cover offset, boolean, and
  contains/properties seed paths only. Do not implement every audited operation.
- **D-17:** Failure output must include fixture id, source path/provenance,
  operation, comparison mode, tolerance policy, and actual vs expected property
  sets.
- **D-18:** Provide a test-only metadata collector that can expose fixture id,
  source, classification, operation, and status for later phases. Do not
  generate formal markdown or JSON reports in Phase 2.

### Seed Fixture Scope

- **D-19:** Seed fixtures must use current Rust behavior only. Do not mine old
  C++ CavalierContours or Clipper2 cases in Phase 2.
- **D-20:** Add one seed each for offset, boolean, and contains/properties to
  prove runner dispatch and property comparison.
- **D-21:** Add one metadata-only `not comparable` or `gap` seed to prove that
  the schema can record non-executable taxonomy cases without running
  assertions.
- **D-22:** Explicitly avoid external fixture imports, oracle output, and
  benchmark profile work. Those belong to Phases 3, 4, and 5.

### the agent's Discretion

The agent may choose concrete Rust type names, module names, and seed geometry
shapes as long as the decisions above are preserved and existing test utility
style is followed.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Planning

- `.planning/PROJECT.md` - project goal, constraints, and fork/mainline
  decisions.
- `.planning/REQUIREMENTS.md` - Phase 2 requirement IDs `FIX-01` and `FIX-02`.
- `.planning/ROADMAP.md` - Phase 2 goal, success criteria, and plan skeleton.
- `.planning/STATE.md` - current workflow position and continuity notes.

### Phase 1 Contract

- `.planning/phases/01-absorption-contract-audit/01-CONTEXT.md` - locked
  source usage, taxonomy, and API/FFI boundary decisions.
- `.planning/phases/01-absorption-contract-audit/01-AUDIT.md` - capability
  inventory, comparison taxonomy, candidate registry, and public surface
  comparison.
- `.planning/phases/01-absorption-contract-audit/01-PROVENANCE.md` - required
  provenance fields, usage labels, and acceptable-use rules.

### Codebase Maps

- `.planning/codebase/TESTING.md` - current test commands, layout, and property
  comparison patterns.
- `.planning/codebase/CONVENTIONS.md` - Rust style, naming, safety boundary,
  public API, and FFI guidance.
- `.planning/codebase/STRUCTURE.md` - workspace layout, test layout, and
  planning artifact locations.

No external specs or ADRs were referenced during discussion.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `cavalier_contours/tests/test_utils/pline_test_properties.rs`: existing
  `PlineProperties`, fuzzy AABB comparison, property set comparison, and
  current epsilon constants.
- `cavalier_contours/tests/test_utils/pline_modifiers.rs`: existing modified
  polyline visitor utilities for direction inversion and closed-polyline start
  index cycling.
- `cavalier_contours/tests/test_utils/debug.rs`: debug JSON helper that can
  inform structured failure output.

### Established Patterns

- Integration tests declare `mod test_utils;` and reuse helpers from
  `cavalier_contours/tests/test_utils/`.
- Offset, boolean, and shape tests compare property sets rather than literal
  vertex order.
- Existing property comparisons remove redundant vertices before comparing
  vertex count and geometry properties.

### Integration Points

- New helper modules should be exported from
  `cavalier_contours/tests/test_utils/mod.rs`.
- New proof tests should live in
  `cavalier_contours/tests/test_fixture_harness.rs`.
- No changes should be made to `cavalier_contours/src/lib.rs`,
  `cavalier_contours_ffi/src/lib.rs`, or `cavalier_contours_ffi.h` for this
  phase.

</code_context>

<specifics>
## Specific Ideas

- Use a centralized type such as `FixtureTolerance` or `TolerancePolicy` to
  manage default and per-fixture tolerance values.
- Use a central fixture type such as `FixtureCase` and a runner shaped like
  `run_fixture(&FixtureCase)`.
- Include metadata-only taxonomy fixtures in the schema without turning them
  into failing or ignored tests.

</specifics>

<deferred>
## Deferred Ideas

- JSON or other file-based fixture import is deferred to later fixture mining or
  oracle phases.
- Old C++ fixture translation belongs to Phase 3.
- Benchmark profile mapping belongs to Phase 4.
- Clipper2 polygon oracle output and fixture eligibility belong to Phase 5.

</deferred>

---

*Phase: 02-fixture-schema-and-property-harness*
*Context gathered: 2026-05-12*
