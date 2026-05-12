# Phase 03: historical-c-evidence-mining - Research

**Researched:** 2026-05-12
**Domain:** Historical geometry fixture mining and Rust integration tests
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- Translate executable historical fixtures only from high-value old C++ offset,
  combine/boolean, and polyline property cases.
- Include old C API evidence as migration-sensitive inventory or metadata; do
  not change current Rust FFI code or regenerate `cavalier_contours_ffi.h`.
- Include old static spatial index evidence as inventory and behavior notes;
  defer performance-sensitive benchmark treatment to Phase 4.
- Use manually curated typed Rust fixtures. Do not add parsers, generators, or
  external fixture formats.
- Keep executable imported fixtures green. Mismatches become metadata-only gap,
  not-comparable, or intentional-divergence records.
- Every translated or represented case must carry repo, commit, license, source
  path, usage label, comparison mode, and tolerance or non-comparability notes.

### the agent's Discretion

- Choose the exact curated case count and fixture IDs.
- Add a dedicated Phase 3 inventory document if that keeps the fixture test
  file focused.
- Add test-only metadata classifications when needed for C API and spatial
  index records.

### Deferred Ideas (OUT OF SCOPE)

- Broad C++ parameter parsing or generated fixture import.
- FFI execution tests, ABI changes, or header regeneration.
- Spatial-index benchmarks and historical benchmark profile measurement.
- Algorithm fixes for old C++ parity gaps.
- Clipper2 oracle output and polygon-only eligibility work.
</user_constraints>

<architectural_responsibility_map>
## Architectural Responsibility Map

Single-tier Rust test and planning artifact work. Production crate code,
runtime FFI code, and UI code are outside the phase boundary.

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|--------------|----------------|-----------|
| Historical source inventory | `.planning/phases/03-historical-c-evidence-mining` docs | Old C++ local repo | Inventory is planning evidence, not production behavior. |
| Executable old C++ fixtures | `cavalier_contours/tests/` integration tests | `tests/test_utils/` fixture harness | Phase 2 made the fixture harness test-only. |
| Metadata-only C API evidence | Phase 3 inventory and/or test-only fixture metadata | `fixture_schema.rs` usage labels | Current FFI must not change in Phase 3. |
| Static spatial index evidence | Phase 3 inventory and optional metadata-only test record | `static_aabb2d_index` re-export | Current Rust uses the external crate; compare behavior notes, not internals. |
</architectural_responsibility_map>

<research_summary>
## Summary

Phase 3 should be planned as a three-step evidence lane: first inventory and
select, then translate a small executable fixture set, then validate that the
imported set is visible through the Phase 2 harness and metadata collector. The
old C++ tests are usable as expected-property sources because they already
compare result count, vertex count, area, path length, and extents with
`TEST_EPSILON() == 1e-5`.

The existing Rust harness executes offset, boolean, and contains/properties.
Pure old C++ property cases from `TEST_cavc_pline_function.cpp` do not map
perfectly to `ContainsProperties`, so the planner should allow a small
test-only `Properties` fixture operation rather than contorting property cases
into containment pairs. This remains inside `cavalier_contours/tests/test_utils`
and does not touch public APIs.

**Primary recommendation:** create `03-INVENTORY.md`, extend only test-only
fixture helpers as needed, add `test_historical_cavalier_contours.rs`, and use
metadata-only records for C API, static spatial index, and any old C++ mismatch.
</research_summary>

<standard_stack>
## Standard Stack

No new libraries are needed.

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust integration tests | repo standard | Run historical fixtures | Existing test layout and CI already use this. |
| Phase 2 fixture harness | local | Typed fixture execution and metadata | Already verified in Phase 2. |
| `static_aabb2d_index` | 2.0.0 | Existing Rust spatial index dependency | Already re-exported and used by current algorithms. |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Old C++ CavalierContours | commit `31a012947aa2e7e9474e2ec90502825afe8b99a4` | Historical expected behavior | Fixture provenance and inventory only. |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Handwritten typed fixtures | C++ parameter parser | More automation but adds tooling scope and fragile parsing. |
| Test-only metadata records | FFI execution tests | Higher confidence but expands Phase 3 into ABI/build work. |
| Property operation in harness | Force property cases into contains fixtures | Avoids schema update but misrepresents source behavior. |
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Evidence Flow

Old C++ source file -> Phase 3 inventory classification -> selected case ->
typed Rust `FixtureCase` -> `run_fixture(&FixtureCase)` -> targeted integration
test -> workspace verification.

Metadata-only records follow the same provenance path but stop before behavior
assertion.

### Recommended Project Structure

```text
.planning/phases/03-historical-c-evidence-mining/
  03-INVENTORY.md        # old C++ source inventory and selected cases
  03-RESEARCH.md         # this research
  03-VALIDATION.md       # validation strategy
cavalier_contours/tests/
  test_historical_cavalier_contours.rs
cavalier_contours/tests/test_utils/
  fixture_schema.rs      # test-only enum/expected-data additions if needed
  fixture_harness.rs     # runner branch for any new test-only operation
```

### Pattern 1: Property-Based Fixture Assertions

**What:** Compare result count and `PlineProperties` instead of literal vertex
order.  
**When to use:** Offset and boolean outputs where equivalent geometry may be
ordered or started differently.  
**Implementation anchor:** Phase 2 `ExpectedFixtureData::{Offset, Boolean}` and
`property_sets_match_with_options`.

### Pattern 2: Metadata-Only Evidence

**What:** Represent a source case with provenance and classification but no
assertion.  
**When to use:** C API migration-sensitive evidence, spatial-index internals,
known mismatches, or not-comparable behavior.  
**Implementation anchor:** Phase 2 `ExpectedFixtureData::MetadataOnly` and
`ComparisonMode::{Gap, NotComparable, IntentionalDivergence}`.

### Pattern 3: Inventory-Then-Translate

**What:** Decide selected fixture IDs in a markdown inventory before adding test
cases.  
**When to use:** Historical source files contain many parameterized cases and
some cases belong to later phases.

### Anti-Patterns to Avoid

- **Broad test import:** translating every old parameterized case will turn
  Phase 3 into gap debugging.
- **Red tests for known mismatches:** violates the Phase 3 green-gate decision.
- **FFI surface drift:** C API evidence is inventory-only unless a later phase
  explicitly opens ABI work.
- **Spatial index internals as Rust contract:** current Rust uses a separate
  crate; behavior notes should focus on query/visitor semantics.
</architecture_patterns>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Old C++ Combine Constructor Trap

**What goes wrong:** One `CombinePlinesTestCase` constructor initializes
`plineB` from `plineVertexesA[0]` while using B's length.  
**Why it happens:** The tests also use a second constructor with explicit
`cavc_pline*`, so not every visible table path has the same reliability.  
**How to avoid:** Prefer explicitly constructed C++ cases or verify current
Rust output before making a case executable. Treat suspect cases as inventory
until confirmed.  
**Warning signs:** A translated boolean case fails unexpectedly while the source
case used the vector constructor.

### Pitfall 2: Signed Area Direction Mismatch

**What goes wrong:** Old tests sometimes ignore area sign, reverse input, or
expect negated area after reversed offsets.  
**Why it happens:** Equivalent geometry can reverse winding or starting vertex.  
**How to avoid:** Use `PropertyExpectationOptions::compare_abs_area` for cases
where old C++ used `EqIgnoreSignOfArea`; use signed area only where direction is
the behavior under test.

### Pitfall 3: Property Cases Do Not Fit Boolean/Contains

**What goes wrong:** Old `cavc_get_area`, `cavc_get_path_length`, and extents
cases are forced into unrelated operation fixtures.  
**Why it happens:** Phase 2 only proved contains/properties, not pure property
execution.  
**How to avoid:** Add a narrow test-only `Properties` fixture operation if the
selected source cases need it.

### Pitfall 4: Inventory Claims Without Harness Proof

**What goes wrong:** Phase 3 produces a useful inventory but no executable
historical fixtures.  
**Why it happens:** C API and spatial index evidence is easier to document than
to run.  
**How to avoid:** Require at least one executable offset fixture, one executable
boolean fixture, and one executable property fixture.
</common_pitfalls>

<validation_architecture>
## Validation Architecture

### Automated Checks

- `cargo test -p cavalier_contours --test test_historical_cavalier_contours`
  must pass after executable fixture import.
- `cargo test -p cavalier_contours --test test_fixture_harness` must still pass
  after any test-only schema/harness changes.
- `cargo test --workspace` must pass before Phase 3 completion.
- `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, and
  `git diff --check` must pass.
- `gsd-sdk query check.decision-coverage-plan .planning/phases/03-historical-c-evidence-mining .planning/phases/03-historical-c-evidence-mining/03-CONTEXT.md`
  should pass after plans exist.

### Minimum Evidence Sampling

| Evidence class | Minimum sample | Executable? |
|----------------|----------------|-------------|
| Offset | 1 simple case plus 1 edge/collapse or reversed-direction case | yes when green |
| Boolean/combine | At least 1 case across old combine modes, preferably circle/rectangle or self invariant | yes when green |
| Pure properties | At least 1 area/path/extents case from `TEST_cavc_pline_function.cpp` | yes when green |
| C API | Header/source surface inventory and migration-sensitive record | no |
| Spatial index | Query/visit behavior inventory and optional metadata record | no by default |

### Failure Policy

If a candidate old C++ case fails under current Rust behavior, keep the test
suite green by converting that candidate to metadata-only gap evidence and
recording the observed mismatch in `03-INVENTORY.md`.
</validation_architecture>

<open_questions>
## Open Questions

1. **Exact curated count**
   - What we know: the context permits agent discretion.
   - What's unclear: whether the final executable count should be three cases
     or a slightly wider batch.
   - Recommendation: plan for a small floor and let execution add extra green
     cases only if they do not create debugging drag.

2. **Pure property fixture operation**
   - What we know: old property cases do not map cleanly to current operation
     variants.
   - What's unclear: whether execution can find enough useful property coverage
     through contains/properties alone.
   - Recommendation: include permission in the plan for a narrow test-only
     `Properties` operation, then use it only if needed.
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)

- `.planning/phases/03-historical-c-evidence-mining/03-CONTEXT.md` - locked
  Phase 3 decisions.
- `.planning/phases/01-absorption-contract-audit/01-AUDIT.md` - source matrix,
  behavior taxonomy, and candidate registry.
- `.planning/phases/01-absorption-contract-audit/01-PROVENANCE.md` - old C++
  commit, license, and usage rules.
- `.planning/phases/02-fixture-schema-and-property-harness/02-CONTEXT.md` -
  fixture schema and harness constraints.
- `.planning/phases/02-fixture-schema-and-property-harness/02-VERIFICATION.md`
  - proof that Phase 2 harness is green.
- `cavalier_contours/tests/test_utils/fixture_schema.rs` - current fixture
  schema.
- `cavalier_contours/tests/test_utils/fixture_harness.rs` - current runner.
- `cavalier_contours/tests/test_fixture_harness.rs` - seed fixture pattern.
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp` -
  old offset source.
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp` -
  old combine/boolean source.
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp` -
  old property/function source.
- `E:/Coding/CavalierContours/tests/tests/TEST_staticspatialindex.cpp` -
  old spatial index source.
- `E:/Coding/CavalierContours/c_api_include/cavaliercontours.h` and
  `E:/Coding/CavalierContours/src/cavaliercontours.cpp` - old C API source.
- `AGENTS.md` - repo-specific contribution and GSD guidance.

### Secondary (MEDIUM confidence)

- `C:/Users/Administrator/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/static_aabb2d_index-2.0.0/tests/test.rs`
  - current Rust dependency behavior analogous to old spatial index tests.

### Tertiary (LOW confidence - needs validation)

- None.
</sources>

<metadata>
## Metadata

**Research scope:**
- Core technology: Rust integration tests and historical fixture translation.
- Ecosystem: local old C++ CavalierContours and existing Rust test utilities.
- Patterns: typed fixture schema, metadata-only evidence, property comparison.
- Pitfalls: C++ constructor trap, area sign, mismatches, FFI scope creep.

**Confidence breakdown:**
- Standard stack: HIGH - no new dependencies required.
- Architecture: HIGH - Phase 2 harness already exists and matches most needs.
- Pitfalls: HIGH - based on direct source inspection.
- Code examples: MEDIUM - plans should read source before final case selection.
</metadata>

## RESEARCH COMPLETE
