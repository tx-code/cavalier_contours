# Roadmap: Cavalier Contours Absorption Roadmap

## Overview

This roadmap turns the existing Rust `cavalier_contours` crate into the single
mainline target for a longer absorption effort. The first milestone is not a
porting sprint: it builds the audit, fixture, benchmark, and oracle evidence
needed to safely mine old C++ CavalierContours and Clipper2, then uses that
evidence to prioritize robustness fixes and selected capability absorption.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Absorption Contract Audit** - Define source boundaries, comparability rules, and API migration baseline. (completed 2026-05-12)
- [x] **Phase 2: Fixture Schema and Property Harness** - Create durable geometry fixture and comparison infrastructure. (completed 2026-05-12)
- [x] **Phase 3: Historical C++ Evidence Mining** - Translate high-value old C++ behavior into Rust regression evidence. (completed 2026-05-12)
- [x] **Phase 4: Benchmark Baseline** - Establish performance baselines and benchmark provenance rules. (completed 2026-05-12)
- [x] **Phase 5: Clipper2 Oracle Boundary** - Add polygon-only oracle evidence without changing production behavior. (completed 2026-05-12)
- [x] **Phase 6: Robustness Gap Closure** - Rank and fix the highest-value current Rust robustness issues. (completed 2026-05-12)
- [x] **Phase 7: Capability Absorption Pipeline** - Select and absorb compatible capabilities with tests and visible validation when needed. (completed 2026-05-12)
- [x] **Phase 8: API, FFI, and Migration Readiness** - Harden external surfaces and migration notes for release-quality use. (completed 2026-05-12)
- [x] **Phase 9: C++ Parity Deep Comparison (No Clipper)** - Deeply compare old C++ logic against Rust on boolean/offset/intersection behavior and classify true bugs versus intentional divergences. (completed 2026-05-12)
- [x] **Phase 10: C++ Function-Level Parity Deepening (No Clipper)** - Extend parity from operation-level cases into function-level C++ `pline_function` behavior and classify any newly surfaced logic gaps. (completed 2026-05-13)
- [x] **Phase 11: Closest-Point and Generated Matrix Parity Expansion (No Clipper)** - Expand C++ `pline_function` parity coverage into closest-point and broader generated function-case matrices, with explicit tie-break and comparability classification. (completed 2026-05-13)
- [x] **Phase 12: Strict-Index and Full Half-Circle Matrix Parity (No Clipper)** - Execute full generated half-circle matrix parity with strict closest-point index checks and resolve confirmed tie-break gaps. (completed 2026-05-13)
- [x] **Phase 13: Full Circle Generated Matrix Parity (No Clipper)** - Execute full generated circle matrix parity across centers, alignments, reverse variants, and closest-point expectations. (completed 2026-05-13)
- [x] **Phase 14: Circle Offset and Collapse Matrix Parity (No Clipper)** - Execute full generated circle offset and collapsed-offset matrix parity with vertex-level output checks. (completed 2026-05-13)
- [x] **Phase 15: Half-Circle Offset and Collapse Matrix Parity (No Clipper)** - Execute full generated half-circle offset and collapsed-offset matrix parity with vertex-level output checks and updated next-target alignment map. (completed 2026-05-13)
- [x] **Phase 16: C++ Offset Matrix Expansion and Reversed Parity (No Clipper)** - Execute broader old C++ `parallel_offset` simple/specific case matrices with reversed-input parity and input-immutability checks. (completed 2026-05-13)
- [x] **Phase 17: C++ Coincident Combine Matrix Parity Expansion (No Clipper)** - Import and execute old C++ coincident combine case matrices, classifying confirmed parity and explicit divergence behavior. (completed 2026-05-13)
- [x] **Phase 18: Coincident Intersect Collapsed-Filter Parity Path (No Clipper)** - Validate and lock a source-traceable `collapsed_area_eps` parity path for coincident intersect sliver suppression. (completed 2026-05-13)
- [x] **Phase 19: Coincident Intersect Default-Path Line-Loop Parity Closure (No Clipper)** - Remove degenerate line-only 2-vertex loops during boolean stitching so default coincident intersect behavior matches old C++ empty-output parity. (completed 2026-05-13)
- [x] **Phase 20: C-API Coincident Intersect Parity Bridge (No Clipper)** - Extend parity closure to the FFI boundary with an executable `cavc_pline_boolean` coincident intersect case matching old C++ empty-output expectation. (completed 2026-05-13)
- [x] **Phase 21: C-API Combine Matrix Expansion (No Clipper)** - Expand FFI parity coverage with executable `cavc_pline_boolean` matrix cases for `circle_rectangle` and `coincident_case2` sourced from old C++ combine fixtures. (completed 2026-05-13)
- [x] **Phase 22: C-API Combine Self-Invariants Parity Bridge (No Clipper)** - Bridge old C++ combine-with-self invariants through `cavc_pline_boolean`, including reversed-orientation and mixed-orientation empty-result invariants. (completed 2026-05-13)
- [x] **Phase 23: C-API Parallel-Offset Matrix Parity Bridge (No Clipper)** - Bridge old C++ `parallel_offset` simple/specific/reversed/no-modify parity through `cavc_pline_parallel_offset` at the FFI boundary. (completed 2026-05-13)
- [x] **Phase 24: C-API Combine No-Modify Parity Bridge (No Clipper)** - Bridge old C++ combine input-immutability expectations through `cavc_pline_boolean` operation matrix checks at the FFI boundary. (completed 2026-05-13)

## Phase Details

### Phase 1: Absorption Contract Audit
**Goal**: Establish what can be absorbed from each repository, how behavior will be compared, and which API surfaces must be protected.
**Depends on**: Nothing (first phase)
**Requirements**: [AUD-01, AUD-02, AUD-03, AUD-04]
**Success Criteria** (what must be TRUE):
  1. The three codebases have a recorded capability inventory by operation, geometry model, tests, benchmarks, and public API.
  2. License and provenance rules explain what can be copied, translated, referenced, or used only as oracle output.
  3. Every future reference case can be classified as exact parity, approximate parity, intentional divergence, or not comparable.
  4. Rust API, Rust FFI, old C++ C API, and Clipper2 operations have an initial compatibility and migration comparison.
**Plans**: 4 plans

Plans:
- [x] 01-01: Build the three-codebase capability inventory.
- [x] 01-02: Record license, provenance, and acceptable-use boundaries.
- [x] 01-03: Define the behavior taxonomy and comparison policy.
- [x] 01-04: Compare API, FFI, and migration surfaces.

### Phase 2: Fixture Schema and Property Harness
**Goal**: Create the reusable evidence format and Rust comparison helpers needed before importing more cases.
**Depends on**: Phase 1
**Requirements**: [FIX-01, FIX-02]
**Success Criteria** (what must be TRUE):
  1. A fixture schema records source, geometry model, tolerance policy, comparison mode, and expected properties.
  2. Rust tests can compare geometry properties without relying on brittle vertex ordering.
  3. Existing test utilities have a clear extension path for imported and oracle-derived cases.
**Plans**: 3 plans

Plans:
- [x] 02-01: Define fixture schema and manifest conventions.
- [x] 02-02: Extend property comparison helpers for absorbed cases.
- [x] 02-03: Add seed fixtures proving the schema and harness work.

### Phase 3: Historical C++ Evidence Mining
**Goal**: Turn old C++ CavalierContours tests and examples into classified Rust regression evidence.
**Depends on**: Phase 2
**Requirements**: [FIX-03]
**Success Criteria** (what must be TRUE):
  1. High-value old C++ offset, combine/boolean, C API, and spatial-index cases are prioritized.
  2. Selected cases are translated or represented as Rust fixtures with provenance and tolerance notes.
  3. Imported cases run through the Phase 2 comparison harness.
**Plans**: 3 plans

Plans:
- [x] 03-01: Inventory old C++ tests, examples, and benchmark profiles for fixture value.
- [x] 03-02: Translate prioritized behavior cases into Rust fixtures.
- [x] 03-03: Validate imported fixtures through the property harness.

### Phase 4: Benchmark Baseline
**Goal**: Establish repeatable measurement coverage for current Rust behavior and historical benchmark families.
**Depends on**: Phase 3
**Requirements**: [BEN-01, BEN-02, BEN-03]
**Success Criteria** (what must be TRUE):
  1. Current Rust benchmark or measurement cases cover offsets, booleans, intersections, and spatial-index-heavy inputs.
  2. Old C++ benchmark profile families are mapped to Rust measurement cases.
  3. Benchmark docs state whether arc approximation, conversion, and oracle costs are included or excluded.
**Plans**: 3 plans

Plans:
- [x] 04-01: Add or document current Rust measurement baseline.
- [x] 04-02: Map historical C++ benchmark families to Rust cases.
- [x] 04-03: Document benchmark provenance and cost-accounting rules.

### Phase 5: Clipper2 Oracle Boundary
**Goal**: Use Clipper2 as a dev-only polygon reference without redefining arc-aware Rust behavior.
**Depends on**: Phase 2
**Requirements**: [FIX-04, ORC-01, ORC-02, ORC-03]
**Success Criteria** (what must be TRUE):
  1. Eligible Clipper2 polygon-only cases are represented as classified Rust fixtures.
  2. A dev-only comparison path can produce oracle evidence for eligible boolean and offset cases.
  3. Any arc-to-polygon comparison records approximation tolerance and does not change native arc semantics.
  4. Oracle output is reported as gap-ranking evidence, not production behavior.
**Plans**: 4 plans

Plans:
- [x] 05-01: Classify Clipper2 tests and examples for polygon-only eligibility.
- [x] 05-02: Add Clipper2-derived fixture representations.
- [x] 05-03: Implement or script the dev-only oracle comparison path.
- [x] 05-04: Emit oracle evidence reports for gap ranking.

### Phase 6: Robustness Gap Closure
**Goal**: Rank and fix the highest-value current Rust robustness issues using the evidence built in earlier phases.
**Depends on**: Phase 3, Phase 4, Phase 5
**Requirements**: [ROB-01, ROB-02, ROB-03, ROB-04]
**Success Criteria** (what must be TRUE):
  1. Robustness gaps are ranked across offsets, booleans, intersections, tolerances, degenerates, repeats, tangencies, overlaps, and open/closed behavior.
  2. Top-ranked gaps have focused regression tests before or with fixes.
  3. Current Rust fixes land without broad public API churn.
  4. Workspace verification passes for every changed surface.
**Plans**: 4 plans

Plans:
- [x] 06-01: Build the ranked robustness backlog from fixtures, benchmarks, and oracle evidence.
- [x] 06-02: Add focused regressions for top-ranked failures.
- [x] 06-03: Implement the first robustness fixes.
- [x] 06-04: Run and document the required verification gates.

### Phase 7: Capability Absorption Pipeline
**Goal**: Select and absorb compatible capabilities from the reference repositories after evidence and robustness gates exist.
**Depends on**: Phase 6
**Requirements**: [CAP-01, CAP-02, CAP-03, DEM-01]
**Success Criteria** (what must be TRUE):
  1. Candidate capabilities are selected from audited gaps, not from port-first intuition.
  2. Each absorbed capability preserves the arc-aware model or is explicitly marked polygon-only.
  3. Tests, examples or docs, and FFI impact notes exist for externally visible changes.
  4. The demo UI changes only when a new capability needs visual validation.
**Plans**: 4 plans

Plans:
- [x] 07-01: Select candidate capabilities from audit and gap evidence.
- [x] 07-02: Design capability-specific behavior and API boundaries.
- [x] 07-03: Implement the first absorbed capability slice.
- [x] 07-04: Update examples, docs, FFI notes, or demo UI only as needed.

### Phase 8: API, FFI, and Migration Readiness
**Goal**: Make the absorbed behavior usable and explainable through Rust APIs, C FFI, and migration guidance.
**Depends on**: Phase 7
**Requirements**: [API-01, API-02, API-03]
**Success Criteria** (what must be TRUE):
  1. Public Rust API and FFI changes have compatibility notes.
  2. Any FFI surface change updates ABI tests and regenerates `cavalier_contours_ffi.h`.
  3. Migration notes explain how old C++ CavalierContours users should approach the Rust crate and FFI.
  4. The milestone is ready for a follow-up audit before the next absorption milestone is planned.
**Plans**: 3 plans

Plans:
- [x] 08-01: Audit public API and FFI compatibility after absorbed changes.
- [x] 08-02: Update ABI tests, generated header, and external docs if needed.
- [x] 08-03: Write old C++ migration notes and milestone readiness summary.

### Phase 9: C++ Parity Deep Comparison (No Clipper)
**Goal**: Establish logic-level parity status between old C++ CavalierContours and Rust `cavalier_contours` without involving Clipper-derived evidence.
**Depends on**: Phase 8
**Requirements**: [PAR-01, PAR-02, PAR-03]
**Success Criteria** (what must be TRUE):
  1. A file-level parity map lists C++ modules/tests and mapped Rust modules/tests for boolean, offset, intersection, and base geometry logic.
  2. High-value C++ behavior cases run as executable Rust parity tests where possible; remaining cases are explicitly classified as `bug`, `intentional-divergence`, or `not-comparable`.
  3. Every confirmed mismatch has evidence and a fix/defer decision with verification notes.
**Plans**: 3 plans

Plans:
- [x] 09-01: Build boolean/combined-operation parity map and executable case set.
- [x] 09-02: Build offset/intersection parity map and executable case set.
- [x] 09-03: Classify mismatches, implement selected fixes, and close verification gates.

### Phase 10: C++ Function-Level Parity Deepening (No Clipper)
**Goal**: Deepen old C++ vs Rust parity by executing C++ `TEST_cavc_pline_function.cpp`-style function-level checks (area/path/extents/winding/boolean-self invariants) and classifying any new gaps.
**Depends on**: Phase 9
**Requirements**: [PAR-04, PAR-05, PAR-06]
**Success Criteria** (what must be TRUE):
  1. A function-level map connects C++ `pline_function` expectations to Rust APIs and tests.
  2. Executable Rust parity tests cover selected high-value C++ function-level assertions with evidence.
  3. Newly surfaced mismatches are classified as `bug`, `intentional-divergence`, or `not-comparable`, with fix/defer notes.
**Plans**: 3 plans

Plans:
- [x] 10-01: Add executable function-level parity tests from C++ `pline_function` behavior.
- [x] 10-02: Write function-level parity classification report and gap decisions.
- [x] 10-03: Close verification gates and sync roadmap/requirements/state.

### Phase 11: Closest-Point and Generated Matrix Parity Expansion (No Clipper)
**Goal**: Expand function-level parity from selected C++ `pline_function` checks to closest-point and broader generated case matrices while preserving explicit comparability boundaries.
**Depends on**: Phase 10
**Requirements**: [PAR-07, PAR-08, PAR-09]
**Success Criteria** (what must be TRUE):
  1. C++ closest-point expectations are mapped into executable Rust parity tests with clear index tie-break handling.
  2. A bounded generated case subset from old C++ function matrices is executed or explicitly marked not-comparable with reasons.
  3. Any new mismatches are classified as `bug`, `intentional-divergence`, or `not-comparable` with fix/defer decisions.
**Plans**: 3 plans

Plans:
- [x] 11-01: Add closest-point parity expansion from C++ `pline_function` references.
- [x] 11-02: Add bounded generated function-case parity subset and classification notes.
- [x] 11-03: Close verification gates and sync planning state.

### Phase 12: Strict-Index and Full Half-Circle Matrix Parity (No Clipper)
**Goal**: Deepen C++ function-level parity by executing full generated half-circle matrices with strict closest-point index checks and fixing confirmed tie-break mismatches.
**Depends on**: Phase 11
**Requirements**: [PAR-10, PAR-11, PAR-12]
**Success Criteria** (what must be TRUE):
  1. Full generated half-circle matrix coverage (open/closed, x/y-aligned, cw/ccw, multi-center) runs as executable Rust parity tests.
  2. Closest-point index expectations that are explicit in old C++ are validated in strict mode or fixed with code-level parity adjustments.
  3. Resulting mismatches are classified with evidence and next alignment targets are mapped by file/module.
**Plans**: 3 plans

Plans:
- [x] 12-01: Expand to full half-circle generated matrix parity and strict index checks.
- [x] 12-02: Classify outcomes and publish file/module alignment map for next deep parity slice.
- [x] 12-03: Close verification gates and sync planning state.

### Phase 13: Full Circle Generated Matrix Parity (No Clipper)
**Goal**: Complete C++ function-level generated circle matrix parity execution (all centers, alignments, reverse variants) with explicit closest-point index policy.
**Depends on**: Phase 12
**Requirements**: [PAR-13, PAR-14, PAR-15]
**Success Criteria** (what must be TRUE):
  1. Full generated circle matrix coverage from old C++ `addCircleCases` runs as executable Rust parity tests.
  2. Closest-point explicit index expectations are validated where old C++ provides strict expected indices.
  3. Resulting classification and next deep-alignment map explicitly identify the offset/collapse matrix follow-up.
**Plans**: 3 plans

Plans:
- [x] 13-01: Expand to full generated circle matrix parity tests.
- [x] 13-02: Classify outcomes and publish next-step alignment map.
- [x] 13-03: Close verification gates and sync planning state.

### Phase 14: Circle Offset and Collapse Matrix Parity (No Clipper)
**Goal**: Deepen generated C++ function-level parity by executing `addCircleCases` offset and collapsed-offset matrices across all generated variants.
**Depends on**: Phase 13
**Requirements**: [PAR-16, PAR-17, PAR-18]
**Success Criteria** (what must be TRUE):
  1. Full generated circle matrix offset cases run as executable parity tests for outward and inward deltas.
  2. Collapsed-offset generated deltas from old C++ are executable and parity-green.
  3. Offset outputs are validated at both property level and vertex level (with closed-curve start rotation tolerance), and next scope is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 14-01: Add generated circle offset and collapse matrix parity tests.
- [x] 14-02: Classify outcomes and publish next-step alignment map.
- [x] 14-03: Close verification gates and sync planning state.

### Phase 15: Half-Circle Offset and Collapse Matrix Parity (No Clipper)
**Goal**: Deepen generated C++ function-level parity by executing `addHalfCircleCases` offset and collapsed-offset matrices across all generated variants.
**Depends on**: Phase 14
**Requirements**: [PAR-19, PAR-20, PAR-21]
**Success Criteria** (what must be TRUE):
  1. Full generated half-circle matrix offset cases run as executable parity tests for outward and inward deltas.
  2. Collapsed-offset generated deltas from old C++ are executable and parity-green.
  3. Offset outputs are validated at both property level and vertex level (open exact-order and closed-curve start rotation tolerance), and next scope is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 15-01: Add generated half-circle offset and collapse matrix parity tests.
- [x] 15-02: Classify outcomes and publish next-step alignment map.
- [x] 15-03: Close verification gates and sync planning state.

### Phase 16: C++ Offset Matrix Expansion and Reversed Parity (No Clipper)
**Goal**: Expand C++ deep parity beyond generated circle/half-circle matrices by importing broader `TEST_cavc_parallel_offset.cpp` simple/specific cases and reversed-input invariants.
**Depends on**: Phase 15
**Requirements**: [PAR-22, PAR-23, PAR-24]
**Success Criteria** (what must be TRUE):
  1. Old C++ `parallel_offset` simple and specific case matrices execute as Rust parity tests with source-traceable expected property sets.
  2. Reversed-input parity behavior (reverse + negate delta) is executable with sign-adjusted area expectations and matching geometry properties.
  3. Collapsed behavior and input-immutability checks are executable and next deep-alignment scope is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 16-01: Import and execute expanded C++ offset case matrices.
- [x] 16-02: Classify outcomes and publish next-step alignment map.
- [x] 16-03: Close verification gates and sync planning state.

### Phase 17: C++ Coincident Combine Matrix Parity Expansion (No Clipper)
**Goal**: Deepen C++ boolean/combine parity by importing coincident-heavy case matrices from `TEST_cavc_combine_plines.cpp` and classifying observed geometry alignment versus divergences.
**Depends on**: Phase 16
**Requirements**: [PAR-25, PAR-26, PAR-27]
**Success Criteria** (what must be TRUE):
  1. Old C++ coincident combine case matrices are executable in Rust parity tests for `Or`, `Not`, `And`, and `Xor`.
  2. Geometry parity outcomes are source-traceable with explicit handling for any confirmed divergence case.
  3. Follow-up alignment scope is explicitly mapped and phase closes with full verification gates.
**Plans**: 3 plans

Plans:
- [x] 17-01: Import coincident combine case matrices into executable parity tests.
- [x] 17-02: Classify outcomes and publish next-step alignment map.
- [x] 17-03: Close verification gates and sync planning state.

### Phase 18: Coincident Intersect Collapsed-Filter Parity Path (No Clipper)
**Goal**: Resolve the coincident intersect sliver parity gap by validating an explicit collapsed-area filtered parity path without forcing a broad boolean default change.
**Depends on**: Phase 17
**Requirements**: [PAR-28, PAR-29, PAR-30]
**Success Criteria** (what must be TRUE):
  1. A source-traceable parity test proves `coincident_case1_intersect` matches old C++ empty output when `collapsed_area_eps` is enabled.
  2. Default-path behavior and collapsed-filter-path behavior are both explicitly classified and documented.
  3. Follow-up alignment scope and default-change decision boundary are explicitly mapped, and phase closes with full verification gates.
**Plans**: 3 plans

Plans:
- [x] 18-01: Add collapsed-filter parity test for coincident intersect case.
- [x] 18-02: Classify outcomes and publish next-step alignment map.
- [x] 18-03: Close verification gates and sync planning state.

### Phase 19: Coincident Intersect Default-Path Line-Loop Parity Closure (No Clipper)
**Goal**: Close the remaining default-path coincident intersect gap by removing only degenerate line-only two-vertex loops during boolean slice stitching.
**Depends on**: Phase 18
**Requirements**: [PAR-31, PAR-32, PAR-33]
**Success Criteria** (what must be TRUE):
  1. Boolean stitching skips only 2-vertex closed loops whose two segments are both line segments, without removing valid 2-vertex arc loops.
  2. `coincident_case1_intersect` default behavior matches old C++ empty-output parity in executable Rust tests.
  3. Workspace verification gates are green and next deep no-Clipper alignment scope is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 19-01: Add line-only two-vertex loop guard in boolean stitching and validate local regressions.
- [x] 19-02: Reclassify coincident default-path parity and publish next alignment map.
- [x] 19-03: Close verification gates and sync planning state.

### Phase 20: C-API Coincident Intersect Parity Bridge (No Clipper)
**Goal**: Prove the coincident intersect parity closure through the Rust FFI boundary with direct C-API boolean execution.
**Depends on**: Phase 19
**Requirements**: [PAR-34, PAR-35, PAR-36]
**Success Criteria** (what must be TRUE):
  1. `cavc_pline_boolean` has an executable coincident-case intersect test based on old C++ source data and operation semantics.
  2. FFI default-path result for `coincident_case1_intersect` is empty (parity with old C++ expectation).
  3. Full workspace and planning health gates are green and next C-API parity expansion scope is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 20-01: Add FFI coincident intersect parity test case and run FFI-focused verification.
- [x] 20-02: Publish C-API parity bridge report and next-scope map.
- [x] 20-03: Close verification gates and sync planning state.

### Phase 21: C-API Combine Matrix Expansion (No Clipper)
**Goal**: Expand C-API parity evidence by executing source-traceable combine matrices from old C++ through `cavc_pline_boolean`.
**Depends on**: Phase 20
**Requirements**: [PAR-37, PAR-38, PAR-39]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute full `circle_rectangle` boolean matrix (`Or`, `Not`, `And`, `Xor`) with source-traceable expected remaining/subtracted geometry properties.
  2. FFI tests execute full `coincident_case2` boolean matrix including both exclusion directions, with property-set parity against old C++ evidence.
  3. Workspace and planning health gates are green, and next C-API parity expansion targets are explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 21-01: Add C-API circle/rectangle and coincident_case2 matrix parity tests.
- [x] 21-02: Publish matrix-expansion report and next-scope map.
- [x] 21-03: Close verification gates and sync planning state.

### Phase 22: C-API Combine Self-Invariants Parity Bridge (No Clipper)
**Goal**: Validate that combine-with-self invariants from old C++ hold through Rust FFI boolean APIs for same and reversed orientation inputs.
**Depends on**: Phase 21
**Requirements**: [PAR-40, PAR-41, PAR-42]
**Success Criteria** (what must be TRUE):
  1. FFI tests prove self-invariants for union/intersect returning self and not/xor returning empty through `cavc_pline_boolean`.
  2. Reversed self and mixed-orientation invariants for empty-result modes are explicitly exercised and green.
  3. Full workspace and planning health gates are green and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 22-01: Add C-API combine-with-self invariants parity test coverage.
- [x] 22-02: Publish invariants bridge report and next-scope map.
- [x] 22-03: Close verification gates and sync planning state.

### Phase 23: C-API Parallel-Offset Matrix Parity Bridge (No Clipper)
**Goal**: Prove old C++ `parallel_offset` matrix behavior through Rust FFI `cavc_pline_parallel_offset` with source-traceable simple/specific/reversed/no-modify coverage.
**Depends on**: Phase 22
**Requirements**: [PAR-43, PAR-44, PAR-45]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute old C++ `parallel_offset` simple and specific case matrices with source-traceable expected property sets.
  2. Reversed-input parity (`invert_direction` + negated delta with sign-adjusted area expectations) and input-immutability behavior are explicitly executed and green at C-API boundary.
  3. Full workspace and planning health gates are green and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 23-01: Add C-API parallel-offset matrix, reversed, and no-modify parity tests.
- [x] 23-02: Publish parallel-offset bridge report and next-scope map.
- [x] 23-03: Close verification gates and sync planning state.

### Phase 24: C-API Combine No-Modify Parity Bridge (No Clipper)
**Goal**: Validate old C++ combine no-modify input invariants through Rust FFI boolean operation matrices.
**Depends on**: Phase 23
**Requirements**: [PAR-46, PAR-47, PAR-48]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly validate that combine operations do not mutate input polyline vertex buffers across the representative C++ operation matrix.
  2. Input-immutability checks run through `cavc_pline_boolean` with source-traceable case geometry and operation mapping.
  3. Full workspace and planning health gates are green and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 24-01: Add C-API combine no-modify matrix parity test.
- [x] 24-02: Publish no-modify bridge report and next-scope map.
- [x] 24-03: Close verification gates and sync planning state.

### Phase 25: C-API Function-Surface Matrix Parity (No Clipper)
**Goal**: Execute old C++ function-surface matrix parity at the Rust C-API boundary for area/path/extents/winding, and classify closest-point surface gap.
**Depends on**: Phase 24
**Requirements**: [PAR-49, PAR-50, PAR-51]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute source-traceable circle and half-circle generated matrices through `cavc_pline_eval_area`, `cavc_pline_eval_path_length`, `cavc_pline_eval_extents`, and `cavc_pline_eval_wn`.
  2. Closest-point parity scope from old C++ `TEST_cavc_pline_function.cpp` is explicitly classified as not-comparable for C-API because no closest-point API exists in `cavalier_contours_ffi`.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 25-01: Add C-API circle/half-circle function-surface matrix parity tests.
- [x] 25-02: Publish function-surface parity report and next-scope map.
- [x] 25-03: Close verification gates and sync planning state.

### Phase 26: C-API Options-Path Parity Bridge (No Clipper)
**Goal**: Validate C-API options-path behavior parity against stabilized default-path behavior on source-backed boolean and offset matrix cases.
**Depends on**: Phase 25
**Requirements**: [PAR-52, PAR-53, PAR-54]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute old C++ `circle_rectangle` boolean matrix via C-API options-path (`cavc_pline_boolean_o`) and show property parity with default-path outputs.
  2. FFI tests execute old C++ imported offset matrices via C-API options-path (`cavc_pline_parallel_offset_o`) and show property parity with default-path outputs.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 26-01: Add C-API options-path parity tests for boolean and offset matrices.
- [x] 26-02: Publish options-path parity report and next-scope map.
- [x] 26-03: Close verification gates and sync planning state.

### Phase 27: C-API Coincident No-Modify Matrix Expansion (No Clipper)
**Goal**: Expand C-API boolean no-modify invariants from simple cases to source-backed coincident combine matrices.
**Depends on**: Phase 26
**Requirements**: [PAR-55, PAR-56, PAR-57]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute coincident case1 and case2 boolean operation matrices and validate subject/clip vertex buffers remain unchanged.
  2. Exclusion direction variants (`A-B` and `B-A`) are explicitly covered in no-modify parity checks for coincident cases.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 27-01: Add C-API coincident no-modify matrix parity tests.
- [x] 27-02: Publish coincident no-modify expansion report and next-scope map.
- [x] 27-03: Close verification gates and sync planning state.

### Phase 28: C-API Optioned Coincident Edge Parity (No Clipper)
**Goal**: Validate optioned coincident edge behavior through C-API collapsed-area filter and options-path no-modify invariants.
**Depends on**: Phase 27
**Requirements**: [PAR-58, PAR-59, PAR-60]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute coincident case1 intersect with `collapsed_area_eps` through `cavc_pline_boolean_o` and return source-backed empty-result parity.
  2. FFI tests execute coincident case1/case2 operation matrices through options-path and validate subject/clip no-modify invariants.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 28-01: Add C-API optioned coincident collapsed-edge and no-modify parity tests.
- [x] 28-02: Publish optioned coincident parity report and next-scope map.
- [x] 28-03: Close verification gates and sync planning state.

### Phase 29: C-API Optioned Coincident Output Parity (No Clipper)
**Goal**: Validate output parity between default-path and options-path for source-backed coincident boolean matrices.
**Depends on**: Phase 28
**Requirements**: [PAR-61, PAR-62, PAR-63]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute coincident case1/case2 operation matrices through default-path and options-path and assert property-set parity for outputs.
  2. Exclusion direction variants (`A-B`, `B-A`) are explicitly covered in options output parity checks.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 29-01: Add C-API optioned coincident output parity matrix test.
- [x] 29-02: Publish options-output parity report and next-scope map.
- [x] 29-03: Close verification gates and sync planning state.

### Phase 30: C-API Closest-Point Parity Bridge (No Clipper)
**Goal**: Introduce closest-point C-API surface and bridge source-backed closest-point parity checks through FFI.
**Depends on**: Phase 29
**Requirements**: [PAR-64, PAR-65, PAR-66]
**Success Criteria** (what must be TRUE):
  1. `cavc_pline_eval_closest_point` is exposed in Rust FFI and root header, with explicit empty/null behavior.
  2. FFI tests execute source-backed circle closest-point matrix parity (vertex anchors + axis/45-degree probes) through C-API calls.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 30-01: Add closest-point C-API surface and parity tests.
- [x] 30-02: Publish closest-point bridge report and next-scope map.
- [x] 30-03: Close verification gates and sync planning state.

### Phase 31: C-API Half-Circle Closest-Point Strict Index Parity (No Clipper)
**Goal**: Close half-circle closest-point strict index parity at C-API boundary using source-backed generated matrix expectations.
**Depends on**: Phase 30
**Requirements**: [PAR-67, PAR-68, PAR-69]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute half-circle generated matrix closest-point probes through `cavc_pline_eval_closest_point` and validate strict segment index expectations.
  2. Open/closed, x/y aligned, direction, and center variants from source-backed matrix are covered for point/distance/index parity.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 31-01: Add C-API half-circle closest-point strict index matrix parity test.
- [x] 31-02: Publish half-circle closest-point parity report and next-scope map.
- [x] 31-03: Close verification gates and sync planning state.

### Phase 32: C-API Function-Surface Combine-Self Matrix Parity (No Clipper)
**Goal**: Close remaining function-surface self-boolean invariants across source-backed circle and closed half-circle matrices at C-API boundary.
**Depends on**: Phase 31
**Requirements**: [PAR-70, PAR-71, PAR-72]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute source-backed circle and closed half-circle matrix cases through self-boolean operations (`union`, `intersect`, `exclude`, `xor`).
  2. Output vertex and no-modify invariants match source-backed expectations for function-surface self-combine behavior.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 32-01: Add function-surface combine-with-self matrix parity test.
- [x] 32-02: Publish combine-self parity report and next-scope map.
- [x] 32-03: Close verification gates and sync planning state.

### Phase 33: C-API Closest-Point Epsilon/Tie-Break Parity (No Clipper)
**Goal**: Close source-backed closest-point epsilon/tie-break sensitivity parity for explicit index expectations at C-API boundary.
**Depends on**: Phase 32
**Requirements**: [PAR-73, PAR-74, PAR-75]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute source-backed explicit closest-point index probes across an epsilon matrix through `cavc_pline_eval_closest_point`.
  2. Circle shared-vertex and half-circle explicit-index closest-point behaviors remain stable for index/point/distance under epsilon variation.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 33-01: Add closest-point epsilon/tie-break matrix parity tests.
- [x] 33-02: Publish closest-point epsilon/tie-break parity report and next-scope map.
- [x] 33-03: Close verification gates and sync planning state.

### Phase 34: C-API Function-Surface Parallel-Offset Full Matrix Parity (No Clipper)
**Goal**: Close source-backed function-surface full-matrix parallel-offset and collapsed offset parity for circle and half-circle cases at C-API boundary.
**Depends on**: Phase 33
**Requirements**: [PAR-76, PAR-77, PAR-78]
**Success Criteria** (what must be TRUE):
  1. FFI tests execute source-backed circle and half-circle full matrix outward/inward offset probes through `cavc_pline_parallel_offset`.
  2. Vertex-level parity follows old C++ semantics (closed rotational match, open exact order), and collapsed deltas return empty.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 34-01: Add function-surface full-matrix parallel-offset parity tests.
- [x] 34-02: Publish function-surface full-matrix offset parity report and next-scope map.
- [x] 34-03: Close verification gates and sync planning state.

### Phase 35: C-API Combine-Self Vertex-Exact Reversed Parity (No Clipper)
**Goal**: Strengthen source-backed combine-with-self invariants to vertex-exact parity for nontrivial sample polyline, including reversed combinations.
**Depends on**: Phase 34
**Requirements**: [PAR-79, PAR-80, PAR-81]
**Success Criteria** (what must be TRUE):
  1. FFI tests verify union/intersect self-combine outputs at vertex-exact level for forward and reversed sample polylines.
  2. Exclude/xor emptiness invariants are explicitly checked across forward/reversed and reversed-forward combinations.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 35-01: Add combine-self vertex-exact reversed parity test.
- [x] 35-02: Publish combine-self vertex-exact parity report and next-scope map.
- [x] 35-03: Close verification gates and sync planning state.

### Phase 36: C-API Pline-Suite Buffer/Reserve Parity (No Clipper)
**Goal**: Close remaining source-backed pline-suite edge semantics for empty-buffer read safety and reserve non-modification behavior at C-API boundary.
**Depends on**: Phase 35
**Requirements**: [PAR-82, PAR-83, PAR-84]
**Success Criteria** (what must be TRUE):
  1. FFI tests assert empty `cavc_pline_get_vertex_data` calls do not modify caller buffers.
  2. FFI tests assert `cavc_pline_reserve` does not modify existing populated vertex data.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 36-01: Add pline-suite buffer/reserve parity tests.
- [x] 36-02: Publish pline-suite buffer/reserve parity report and next-scope map.
- [x] 36-03: Close verification gates and sync planning state.

### Phase 37: C-API Pline Remove-Sequence Range-Equivalence Parity (No Clipper)
**Goal**: Close source-backed remove-range scenario parity using equivalent ordered remove sequence on current C-API surface.
**Depends on**: Phase 36
**Requirements**: [PAR-85, PAR-86, PAR-87]
**Success Criteria** (what must be TRUE):
  1. FFI tests reproduce old remove-range scenario with ordered `cavc_pline_remove` calls and vertex-level intermediate assertions.
  2. Final polyline empty-state closure matches source-backed expectation.
  3. Full workspace and planning health gates are green, and next C-API parity target is explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 37-01: Add remove-sequence range-equivalence parity test.
- [x] 37-02: Publish remove-sequence range-equivalence parity report and next-scope map.
- [x] 37-03: Close verification gates and sync planning state.

### Phase 38: C-API Cross-Suite Coverage Audit (No Clipper)
**Goal**: Build explicit coverage mapping from old C++ C-API suite blocks to current FFI evidence and identify residual source-explicit gaps.
**Depends on**: Phase 37
**Requirements**: [PAR-88, PAR-89, PAR-90]
**Success Criteria** (what must be TRUE):
  1. A concrete cross-suite checklist maps each old suite block (`pline`, `pline_function`, `parallel_offset`, `combine_plines`) to current FFI evidence.
  2. Residual uncovered source-explicit gaps are explicitly classified.
  3. Full workspace and planning health gates are green, and post-audit next targets are explicitly mapped.
**Plans**: 3 plans

Plans:
- [x] 38-01: Build cross-suite coverage checklist.
- [x] 38-02: Publish post-audit alignment map.
- [x] 38-03: Close verification gates and sync planning state.

### Phase 39: C-API Equivalence-Zone Regression Hardening (No Clipper)
**Goal**: Harden source-backed API-evolution equivalence zones with tighter executable regressions for `reserve` and remove-sequence semantics.
**Depends on**: Phase 38
**Requirements**: [PAR-91, PAR-92, PAR-93]
**Success Criteria** (what must be TRUE):
  1. Reserve-equivalence zone has a regression test that preserves prefix data through shrink-noop/grow calls and append operations.
  2. Remove-sequence range-equivalence scenario validates final empty-buffer no-write behavior in the same executable flow.
  3. Full workspace and planning health gates are green, with explicit next-step map for post-hardening continuity.
**Plans**: 3 plans

Plans:
- [x] 39-01: Add reserve/remove equivalence-zone regression hardening tests.
- [x] 39-02: Publish post-hardening alignment map and follow-up boundary.
- [x] 39-03: Close verification gates and sync planning state.

### Phase 40: C-API Old-Suite Drift-Detection Hook (No Clipper)
**Goal**: Add an executable drift-detection hook that guards cross-suite alignment by checking canonical old C++ suite files for hash or test-block changes.
**Depends on**: Phase 39
**Requirements**: [PAR-94, PAR-95, PAR-96]
**Success Criteria** (what must be TRUE):
  1. A baseline artifact records source-root path, file hashes, and canonical test-block names for old C++ suite files used by parity alignment.
  2. A repeatable hook command fails on file/test-block drift and passes on no-drift state, with explicit output.
  3. Full workspace and planning health gates are green, and next-step map is updated after drift-hook closure.
**Plans**: 3 plans

Plans:
- [x] 40-01: Add cpp-suite drift baseline artifact and executable check script.
- [x] 40-02: Publish drift-hook operation notes and post-hook alignment map.
- [x] 40-03: Close verification gates and sync planning state.

### Phase 41: C-API Options-Path No-Modify Hardening (No Clipper)
**Goal**: Strengthen C-API options-path invariants by extending source-backed no-modify behavior checks to boolean circle/rectangle and parallel-offset matrix paths.
**Depends on**: Phase 40
**Requirements**: [PAR-97, PAR-98, PAR-99]
**Success Criteria** (what must be TRUE):
  1. Options-path parallel-offset execution keeps input polyline vertex data unchanged across source-backed simple/specific case matrices.
  2. Options-path boolean circle/rectangle operation matrix keeps subject and clip inputs unchanged across union/exclude/intersect/xor.
  3. Full workspace and planning health gates are green, with updated post-phase alignment map.
**Plans**: 3 plans

Plans:
- [x] 41-01: Add options-path no-modify hardening tests for offset and boolean matrix flows.
- [x] 41-02: Publish post-hardening alignment map for options-path follow-up.
- [x] 41-03: Close verification gates and sync planning state.

### Phase 42: C-API Options-Path Vertex-Output Deepening (No Clipper)
**Goal**: Deepen options-path parity by validating vertex-level output equivalence between default and options-path executions on source-backed boolean and offset matrices.
**Depends on**: Phase 41
**Requirements**: [PAR-100, PAR-101, PAR-102]
**Success Criteria** (what must be TRUE):
  1. Options-path boolean circle/rectangle operation matrix output matches default-path output at vertex level (unordered polyline set with closed rotation tolerance).
  2. Options-path parallel-offset simple/specific matrix output matches default-path output at vertex level (unordered output set with closed/open matching rules).
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 42-01: Add options-path vertex-output deepening tests for boolean and offset matrix flows.
- [x] 42-02: Publish post-deepening alignment map for options-path follow-up.
- [x] 42-03: Close verification gates and sync planning state.

### Phase 43: C-API Drift-Failure Triage Template (No Clipper)
**Goal**: Establish a deterministic triage template for old-suite drift-hook failures so alignment response remains consistent and source-explicit.
**Depends on**: Phase 42
**Requirements**: [PAR-103, PAR-104, PAR-105]
**Success Criteria** (what must be TRUE):
  1. A reusable triage template exists with required sections for drift evidence, coverage mapping, classification, and action selection.
  2. Drift-failure handling command flow is documented to connect drift detection with triage execution and re-audit boundaries.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 43-01: Add deterministic drift-failure triage template artifact.
- [x] 43-02: Publish triage flow notes and post-phase alignment map.
- [x] 43-03: Close verification gates and sync planning state.

### Phase 44: C-API Options-Path Coincident Vertex-Output Deepening (No Clipper)
**Goal**: Deepen coincident options-path parity by validating vertex-level output equivalence between default and options-path boolean execution on source-backed coincident matrices.
**Depends on**: Phase 43
**Requirements**: [PAR-106, PAR-107, PAR-108]
**Success Criteria** (what must be TRUE):
  1. Options-path coincident case1/case2 output matches default-path output at vertex level for union/exclude/intersect/xor operations (unordered closed polyline set with rotation tolerance).
  2. Vertex-level comparison covers both remaining and subtracted output sets for coincident options-path matrix cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 44-01: Add coincident options-path vertex-output deepening test.
- [x] 44-02: Publish post-deepening alignment map for coincident options-path follow-up.
- [x] 44-03: Close verification gates and sync planning state.

### Phase 45: C-API Options-Path Tolerance-Matrix Deepening (No Clipper)
**Goal**: Deepen options-path parity by validating tolerance/epsilon matrix stability against default-path outputs on source-backed boolean and offset matrix surfaces.
**Depends on**: Phase 44
**Requirements**: [PAR-109, PAR-110, PAR-111]
**Success Criteria** (what must be TRUE):
  1. Options-path boolean circle/rectangle matrix remains equivalent to default-path output across a bounded `pos_equal_eps` scale matrix.
  2. Options-path parallel-offset simple/specific matrices remain equivalent to default-path output across bounded tolerance scale matrix (`pos_equal_eps`, `slice_join_eps`, `offset_dist_eps`).
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 45-01: Add options-path tolerance-matrix deepening tests for boolean and offset matrix flows.
- [x] 45-02: Publish post-deepening alignment map for tolerance-matrix follow-up.
- [x] 45-03: Close verification gates and sync planning state.

### Phase 46: C-API Options-Path Self-Intersects Mode Matrix (No Clipper)
**Goal**: Validate options-path stability across self-intersects include modes on source-backed non-self-intersecting offset matrices.
**Depends on**: Phase 45
**Requirements**: [PAR-112, PAR-113, PAR-114]
**Success Criteria** (what must be TRUE):
  1. Options-path `parallel_offset` output remains equivalent to default-path output across `handle_self_intersects` mode matrix (`ALL`, `LOCAL`, `GLOBAL`) for source-backed simple cases.
  2. Mode-matrix checks validate both property-level and vertex-level output equivalence.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 46-01: Add self-intersects mode matrix parity test for options-path offset flow.
- [x] 46-02: Publish post-deepening alignment map for self-intersects mode follow-up.
- [x] 46-03: Close verification gates and sync planning state.

### Phase 47: C-API Self-Intersects Mode No-Modify Matrix (No Clipper)
**Goal**: Extend self-intersects mode coverage with input no-modify invariants across source-backed simple/specific offset matrices.
**Depends on**: Phase 46
**Requirements**: [PAR-115, PAR-116, PAR-117]
**Success Criteria** (what must be TRUE):
  1. Options-path self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) preserves input vertices for source-backed simple and specific offset cases.
  2. No-modify checks run across the same matrix with explicit mode attribution in assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 47-01: Add self-intersects mode no-modify matrix parity test for options-path offset flow.
- [x] 47-02: Publish post-deepening alignment map for mode no-modify follow-up.
- [x] 47-03: Close verification gates and sync planning state.

### Phase 48: C-API Options-Path Self-Intersects Stress Matrix (No Clipper)
**Goal**: Deepen options-path self-intersects parity with a bounded stress matrix across source-backed simple/specific offset cases (`mode x tolerance scale`).
**Depends on**: Phase 47
**Requirements**: [PAR-118, PAR-119, PAR-120]
**Success Criteria** (what must be TRUE):
  1. Options-path `parallel_offset` output remains equivalent to default-path output across self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`) for source-backed simple and specific cases.
  2. Stress-matrix checks validate both property-level and vertex-level output equivalence with explicit mode/scale attribution in assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 48-01: Add self-intersects stress-matrix parity test for options-path offset flow.
- [x] 48-02: Publish post-deepening alignment map for stress-matrix follow-up.
- [x] 48-03: Close verification gates and sync planning state.

### Phase 49: C-API Options-Path Reversed Self-Intersects Stress Matrix (No Clipper)
**Goal**: Deepen reversed-input options-path parity with a bounded self-intersects stress matrix across source-backed simple/specific offset cases (`invert_direction + negated delta`, `mode x tolerance scale`).
**Depends on**: Phase 48
**Requirements**: [PAR-121, PAR-122, PAR-123]
**Success Criteria** (what must be TRUE):
  1. Reversed-input options-path `parallel_offset` output remains equivalent to reversed-input default-path output across self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`) for source-backed simple and specific cases.
  2. Reversed stress-matrix checks validate both property-level and vertex-level output equivalence with explicit mode/scale attribution in assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 49-01: Add reversed self-intersects stress-matrix parity test for options-path offset flow.
- [x] 49-02: Publish post-deepening alignment map for reversed stress-matrix follow-up.
- [x] 49-03: Close verification gates and sync planning state.

### Phase 50: C-API Options-Path Reversed Self-Intersects No-Modify Stress Matrix (No Clipper)
**Goal**: Deepen reversed-input options-path invariants with a bounded self-intersects no-modify stress matrix across source-backed simple/specific offset cases (`invert_direction + negated delta`, `mode x tolerance scale`).
**Depends on**: Phase 49
**Requirements**: [PAR-124, PAR-125, PAR-126]
**Success Criteria** (what must be TRUE):
  1. Reversed-input options-path `parallel_offset` execution preserves input vertex data across self-intersects include mode matrix (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`) for source-backed simple and specific cases.
  2. Reversed no-modify stress-matrix checks validate input stability with explicit mode/scale attribution in assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 50-01: Add reversed self-intersects no-modify stress-matrix test for options-path offset flow.
- [x] 50-02: Publish post-deepening alignment map for reversed no-modify stress-matrix follow-up.
- [x] 50-03: Close verification gates and sync planning state.

### Phase 51: C-API FFI Parity Helper Extraction (No Clipper)
**Goal**: Reduce FFI parity test duplication by extracting shared mode/scale constants and options-init helper while preserving all existing parity/no-modify test behavior.
**Depends on**: Phase 50
**Requirements**: [PAR-127, PAR-128, PAR-129]
**Success Criteria** (what must be TRUE):
  1. Shared helper constructs for mode/scale matrix and options initialization are extracted into `test_pline.rs` and reused by relevant options-path parity tests.
  2. Refactor does not change parity/no-modify test semantics; existing assertions remain behaviorally equivalent.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 51-01: Extract and apply shared helper constructs in FFI parity tests.
- [x] 51-02: Publish post-extraction alignment map for next bounded targets.
- [x] 51-03: Close verification gates and sync planning state.

### Phase 52: C-API Reversed Output/No-Modify Merge Matrix (No Clipper)
**Goal**: Deepen reversed-input options-path coverage by merging output parity and no-modify checks into a single bounded stress matrix across source-backed simple/specific offset cases.
**Depends on**: Phase 51
**Requirements**: [PAR-130, PAR-131, PAR-132]
**Success Criteria** (what must be TRUE):
  1. Reversed-input options-path stress matrix validates output parity and input no-modify invariants together across self-intersects include modes (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`).
  2. Merged checks keep explicit mode/scale-attributed failure diagnostics for both output and no-modify assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 52-01: Add reversed output/no-modify merged stress-matrix test for options-path offset flow.
- [x] 52-02: Publish post-deepening alignment map for next bounded targets.
- [x] 52-03: Close verification gates and sync planning state.

### Phase 53: C-API Reversed Specific-Edge Attribution Matrix (No Clipper)
**Goal**: Deepen reversed-input options-path confidence by adding source-backed specific-edge attributions to merged parity/no-modify stress checks.
**Depends on**: Phase 52
**Requirements**: [PAR-133, PAR-134, PAR-135]
**Success Criteria** (what must be TRUE):
  1. Reversed-input specific-case matrix validates output parity and input no-modify invariants across self-intersects include modes (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`).
  2. Each source-backed specific case has explicit legacy provenance attribution in failure diagnostics.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 53-01: Add reversed specific-edge attribution matrix test for options-path offset flow.
- [x] 53-02: Publish post-deepening alignment map for next bounded targets.
- [x] 53-03: Close verification gates and sync planning state.

### Phase 54: C-API Default Output/No-Modify Merge Matrix (No Clipper)
**Goal**: Deepen default-input options-path coverage by merging output parity and no-modify checks into a single bounded stress matrix across source-backed simple/specific offset cases.
**Depends on**: Phase 53
**Requirements**: [PAR-136, PAR-137, PAR-138]
**Success Criteria** (what must be TRUE):
  1. Default-input options-path stress matrix validates output parity and input no-modify invariants together across self-intersects include modes (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`).
  2. Merged checks keep explicit mode/scale-attributed failure diagnostics for both output and no-modify assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 54-01: Add default-input output/no-modify merged stress-matrix test for options-path offset flow.
- [x] 54-02: Publish post-deepening alignment map for next bounded targets.
- [x] 54-03: Close verification gates and sync planning state.

### Phase 55: C-API Default Specific-Edge Attribution Matrix (No Clipper)
**Goal**: Deepen default-input options-path confidence by adding source-backed specific-edge attributions to merged parity/no-modify stress checks.
**Depends on**: Phase 54
**Requirements**: [PAR-139, PAR-140, PAR-141]
**Success Criteria** (what must be TRUE):
  1. Default-input source-backed specific-case matrix validates output parity and input no-modify invariants across self-intersects include modes (`ALL`, `LOCAL`, `GLOBAL`) and bounded tolerance scales (`0.5x`, `1.0x`, `2.0x`).
  2. Each source-backed specific case has explicit legacy provenance attribution in failure diagnostics.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 55-01: Add default specific-edge attribution matrix test for options-path offset flow.
- [x] 55-02: Publish post-deepening alignment map for next bounded targets.
- [x] 55-03: Close verification gates and sync planning state.

### Phase 56: C-API Specific-Edge Runner Helper Extraction (No Clipper)
**Goal**: Reduce FFI parity test duplication by extracting shared specific-edge attribution and matrix-runner helpers while preserving reversed/default options-path parity/no-modify behavior.
**Depends on**: Phase 55
**Requirements**: [PAR-142, PAR-143, PAR-144]
**Success Criteria** (what must be TRUE):
  1. Specific-edge attribution text is centralized in a shared helper so source-backed provenance remains consistent across tests.
  2. Reversed/default specific-edge matrix tests reuse one shared runner and preserve existing mode/scale/case diagnostics plus no-modify assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 56-01: Extract and apply shared specific-edge attribution/runner helpers in FFI parity tests.
- [x] 56-02: Publish post-extraction alignment map for next bounded targets.
- [x] 56-03: Close verification gates and sync planning state.

### Phase 57: C-API Specific-Edge Matrix Coverage Expansion (No Clipper)
**Goal**: Expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ edge cases while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 56
**Requirements**: [PAR-145, PAR-146, PAR-147]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes at least one additional source-backed old C++ edge case beyond the original specific-case trio.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for each covered edge case.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 57-01: Add source-backed specific-edge matrix coverage expansion in FFI parity tests.
- [x] 57-02: Publish post-expansion alignment map for next bounded targets.
- [x] 57-03: Close verification gates and sync planning state.

### Phase 58: C-API Specific-Edge Matrix Open-Path Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ open-path case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 57
**Requirements**: [PAR-148, PAR-149, PAR-150]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ open-path case beyond Phase 57 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 58-01: Add source-backed open-path specific-edge matrix expansion in FFI parity tests.
- [x] 58-02: Publish post-expansion alignment map for next bounded targets.
- [x] 58-03: Close verification gates and sync planning state.

### Phase 59: C-API Specific-Edge Matrix Diamond Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ diamond case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 58
**Requirements**: [PAR-151, PAR-152, PAR-153]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ diamond case beyond Phase 58 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 59-01: Add source-backed diamond specific-edge matrix expansion in FFI parity tests.
- [x] 59-02: Publish post-expansion alignment map for next bounded targets.
- [x] 59-03: Close verification gates and sync planning state.

### Phase 60: C-API Specific-Edge Matrix Open-Diamond Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ open-diamond case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 59
**Requirements**: [PAR-154, PAR-155, PAR-156]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ open-diamond case beyond Phase 59 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 60-01: Add source-backed open-diamond specific-edge matrix expansion in FFI parity tests.
- [x] 60-02: Publish post-expansion alignment map for next bounded targets.
- [x] 60-03: Close verification gates and sync planning state.

### Phase 61: C-API Specific-Edge Matrix Open-Diamond-Outward Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ open-diamond-outward case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 60
**Requirements**: [PAR-157, PAR-158, PAR-159]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ open-diamond-outward case beyond Phase 60 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 61-01: Add source-backed open-diamond-outward specific-edge matrix expansion in FFI parity tests.
- [x] 61-02: Publish post-expansion alignment map for next bounded targets.
- [x] 61-03: Close verification gates and sync planning state.

### Phase 62: C-API Specific-Edge Matrix Closed-Diamond-Inward Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ closed-diamond-inward case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 61
**Requirements**: [PAR-160, PAR-161, PAR-162]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ closed-diamond-inward case beyond Phase 61 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 62-01: Add source-backed closed-diamond-inward specific-edge matrix expansion in FFI parity tests.
- [x] 62-02: Publish post-expansion alignment map for next bounded targets.
- [x] 62-03: Close verification gates and sync planning state.

### Phase 63: C-API Specific-Edge Matrix Closed-Rectangle-Outward Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ closed-rectangle-outward case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 62
**Requirements**: [PAR-163, PAR-164, PAR-165]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ closed-rectangle-outward case beyond Phase 62 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 63-01: Add source-backed closed-rectangle-outward specific-edge matrix expansion in FFI parity tests.
- [x] 63-02: Publish post-expansion alignment map for next bounded targets.
- [x] 63-03: Close verification gates and sync planning state.

### Phase 64: C-API Specific-Edge Matrix Closed-Rectangle-Inward Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ closed-rectangle-inward case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 63
**Requirements**: [PAR-166, PAR-167, PAR-168]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ closed-rectangle-inward case beyond Phase 63 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 64-01: Add source-backed closed-rectangle-inward specific-edge matrix expansion in FFI parity tests.
- [x] 64-02: Publish post-expansion alignment map for next bounded targets.
- [x] 64-03: Close verification gates and sync planning state.

### Phase 65: C-API Specific-Edge Matrix Open-Rectangle-Inward Expansion (No Clipper)
**Goal**: Further expand helper-driven specific-edge options-path matrix coverage with additional source-backed old C++ open-rectangle-inward case inputs while preserving diagnostics and no-modify invariants.
**Depends on**: Phase 64
**Requirements**: [PAR-169, PAR-170, PAR-171]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix execution includes an additional old C++ open-rectangle-inward case beyond Phase 64 coverage.
  2. Shared helper path keeps reversed/default matrix parity and no-modify diagnostics stable with explicit provenance labels for all covered edge cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 65-01: Add source-backed open-rectangle-inward specific-edge matrix expansion in FFI parity tests.
- [x] 65-02: Publish post-expansion alignment map for next bounded targets.
- [x] 65-03: Close verification gates and sync planning state.

### Phase 66: C-API Specific-Edge Matrix Source-Coverage Guard (No Clipper)
**Goal**: Harden helper-driven specific-edge options-path alignment by enforcing source-backed simple-case coverage in matrix construction with explicit omitted-case diagnostics.
**Depends on**: Phase 65
**Requirements**: [PAR-172, PAR-173, PAR-174]
**Success Criteria** (what must be TRUE):
  1. Specific-edge matrix construction fails if a source-backed old C++ simple case is omitted from selection.
  2. Omitted-case diagnostics remain explicit while existing reversed/default parity and no-modify behavior remains unchanged.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 66-01: Add source-coverage guard assertion to specific-edge matrix construction.
- [x] 66-02: Publish post-guard alignment map for next bounded targets.
- [x] 66-03: Close verification gates and sync planning state.

### Phase 67: C-API Coincident Exclude Name Canonicalization (No Clipper)
**Goal**: Align C-API coincident exclude case metadata names with old C++ canonical identifiers while preserving behavior and diagnostics.
**Depends on**: Phase 66
**Requirements**: [PAR-175, PAR-176, PAR-177]
**Success Criteria** (what must be TRUE):
  1. Coincident case1/case2 exclude labels use old C++ canonical names (`excludeAFromB`, `excludeBFromA`) across default/options/no-modify matrix suites.
  2. Naming canonicalization does not alter operation mapping, expected outputs, or no-modify assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 67-01: Canonicalize coincident exclude case labels in FFI matrix metadata.
- [x] 67-02: Publish post-canonicalization alignment map for next bounded targets.
- [x] 67-03: Close verification gates and sync planning state.

### Phase 68: C-API Coincident Matrix Helper Extraction (No Clipper)
**Goal**: Reduce C-API coincident matrix drift risk by reusing one shared source-backed case helper across matrix suites while preserving behavior.
**Depends on**: Phase 67
**Requirements**: [PAR-178, PAR-179, PAR-180]
**Success Criteria** (what must be TRUE):
  1. Default/options/no-modify/output coincident matrix suites consume a shared source-backed case helper.
  2. Helper extraction preserves canonical names, operation mapping, and existing behavior/assertion outcomes.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 68-01: Extract and apply shared coincident matrix case helper in FFI tests.
- [x] 68-02: Publish post-extraction alignment map for next bounded targets.
- [x] 68-03: Close verification gates and sync planning state.

### Phase 69: C-API Coincident Matrix Source-Coverage Guard (No Clipper)
**Goal**: Harden shared coincident matrix helper alignment with explicit source-backed case-coverage and operation-map guardrails while preserving behavior.
**Depends on**: Phase 68
**Requirements**: [PAR-181, PAR-182, PAR-183]
**Success Criteria** (what must be TRUE):
  1. Shared coincident matrix helper fails fast when canonical source-backed case coverage or helper case count drifts.
  2. Shared coincident matrix helper fails fast when source-backed operation mapping drifts from canonical old C++ mapping.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 69-01: Add source-coverage and operation-map guards to shared coincident matrix helper.
- [x] 69-02: Publish post-guard alignment map for next bounded targets.
- [x] 69-03: Close verification gates and sync planning state.

### Phase 70: C-API Coincident Case1 Matrix Parity Expansion (No Clipper)
**Goal**: Expand explicit source-backed default-path parity by adding full old C++ coincident_case1 matrix expectations.
**Depends on**: Phase 69
**Requirements**: [PAR-184, PAR-185, PAR-186]
**Success Criteria** (what must be TRUE):
  1. Default-path `cavc_pline_boolean` parity explicitly covers old C++ coincident_case1 `union/excludeAFromB/excludeBFromA/intersect/xor` expected properties.
  2. Case1 exclude direction variants preserve source-backed operation mapping and expected output properties.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 70-01: Add explicit coincident_case1 default-path matrix parity test from old C++ expectations.
- [x] 70-02: Publish post-expansion alignment map for next bounded targets.
- [x] 70-03: Close verification gates and sync planning state.

### Phase 71: C-API Coincident Default Matrix Source-Map Guard (No Clipper)
**Goal**: Harden explicit coincident default-path matrices by enforcing source-backed `name+operation` mapping guards for case1/case2 and unifying mapping diagnostics.
**Depends on**: Phase 70
**Requirements**: [PAR-187, PAR-188, PAR-189]
**Success Criteria** (what must be TRUE):
  1. Explicit default-path `coincident_case1` and `coincident_case2` parity matrices fail fast when canonical source-backed case coverage or operation mapping drifts.
  2. Shared mapping guard helper is reused by helper-level coincident matrix guard and explicit default-path matrix parity tests.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 71-01: Add shared source-mapping guard helper coverage for explicit case1/case2 default-path matrices.
- [x] 71-02: Publish post-guard alignment map for next bounded targets.
- [x] 71-03: Close verification gates and sync planning state.

### Phase 72: C-API Circle-Rectangle Source Matrix Guard Reuse (No Clipper)
**Goal**: Harden circle-rectangle source-backed boolean parity by guarding default matrix name/operation mapping and reusing one canonical operation sequence across default/options/no-modify suites.
**Depends on**: Phase 71
**Requirements**: [PAR-190, PAR-191, PAR-192]
**Success Criteria** (what must be TRUE):
  1. Explicit default-path `circle_rectangle` parity matrix fails fast when canonical source-backed case coverage or operation mapping drifts.
  2. Circle-rectangle operation matrix order is reused via one shared canonical source-backed operation constant across default/options/no-modify parity surfaces.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 72-01: Add circle-rectangle default matrix source-mapping guard and shared operation-sequence constant reuse.
- [x] 72-02: Publish post-guard alignment map for next bounded targets.
- [x] 72-03: Close verification gates and sync planning state.

### Phase 73: C-API Pline Core Suite Source-Coverage Parity (No Clipper)
**Goal**: Harden C-API pline-core parity by explicitly covering old `TEST_cavc_pline.cpp` core cases (`new/set_capacity/set_vertex_data/add/remove_range/clear`) with source-backed coverage guards.
**Depends on**: Phase 72
**Requirements**: [PAR-193, PAR-194, PAR-195]
**Success Criteria** (what must be TRUE):
  1. Pline core parity suite explicitly executes source-backed old C++ cases for `new`, `set_capacity`-equivalent reserve behavior, `set_vertex_data`, `add_vertex`, `remove_range`-equivalent remove sequence, and `clear`.
  2. Source-case coverage guard fails fast on missing/duplicate pline core source-case entries.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 73-01: Add source-backed pline core suite parity test with explicit coverage guard.
- [x] 73-02: Publish post-suite alignment map for next bounded targets.
- [x] 73-03: Close verification gates and sync planning state.

### Phase 74: C-API AABBIndex Extents Source Parity (No Clipper)
**Goal**: Harden C-API aabbindex parity by adding source-backed extents checks aligned to old `TEST_staticspatialindex.cpp` extents assertions and covering exact/approx/extents-edge behavior.
**Depends on**: Phase 73
**Requirements**: [PAR-196, PAR-197, PAR-198]
**Success Criteria** (what must be TRUE):
  1. C-API `cavc_pline_create_approx_aabbindex` and `cavc_pline_create_aabbindex` extents agree with source-backed extents parity cases derived from old `StaticSpatialIndexTests.index` and `skip_sorting_small_index`.
  2. C-API aabbindex extents flow is hardened with null-path and empty-index NaN checks, while keeping source-case coverage guard diagnostics explicit.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 74-01: Add source-backed aabbindex extents parity tests and coverage guard.
- [x] 74-02: Publish post-extents alignment map for next bounded targets.
- [x] 74-03: Close verification gates and sync planning state.

### Phase 75: C-API Option Lifecycle & CW Userdata Coverage (No Clipper)
**Goal**: Close FFI surface coverage gaps by asserting create/init/free lifecycle behavior for option objects and validating CW userdata set/get semantics on shapes.
**Depends on**: Phase 74
**Requirements**: [PAR-199, PAR-200, PAR-201]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly cover create/init/free lifecycle behavior for previously untested option-surface exports in parity scope.
  2. FFI tests explicitly cover `cavc_shape_set_cw_pline_userdata_values` behavior for success, bounds, null-shape error, and clear-path semantics.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 75-01: Add option lifecycle and CW userdata parity tests for uncovered FFI exports.
- [x] 75-02: Publish post-coverage alignment map for next bounded targets.
- [x] 75-03: Close verification gates and sync planning state.

### Phase 76: C-API CCW Userdata Setter Symmetry Coverage (No Clipper)
**Goal**: Harden shape userdata API symmetry by adding direct behavior coverage for `cavc_shape_set_ccw_pline_userdata_values` equivalent to CW setter semantics.
**Depends on**: Phase 75
**Requirements**: [PAR-202, PAR-203, PAR-204]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly cover `cavc_shape_set_ccw_pline_userdata_values` success path, null-shape error path, out-of-bounds error path, and clear-path semantics.
  2. CCW userdata setter behavior is proven consistent with the already-covered CW setter contract for count/value roundtrip and clear behavior.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 76-01: Add direct CCW userdata setter symmetry parity test.
- [x] 76-02: Publish post-symmetry alignment map for next bounded targets.
- [x] 76-03: Close verification gates and sync planning state.

### Phase 77: C-API Userdata Getter Bounds Contract Hardening (No Clipper)
**Goal**: Harden C-API userdata getter correctness by enforcing explicit out-of-bounds error semantics for CCW/CW userdata getters and aligning docs/tests with implemented contract.
**Depends on**: Phase 76
**Requirements**: [PAR-205, PAR-206, PAR-207]
**Success Criteria** (what must be TRUE):
  1. `cavc_shape_get_ccw_pline_userdata_values` and `cavc_shape_get_cw_pline_userdata_values` return explicit bounds error codes (`2`) for out-of-range `polyline_index`.
  2. FFI header/runtime docs and tests are aligned on null-shape (`1`) and out-of-bounds (`2`) semantics for both userdata getter functions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 77-01: Add bounds checks and parity assertions for CCW/CW userdata getters.
- [x] 77-02: Publish post-hardening alignment map for next bounded targets.
- [x] 77-03: Close verification gates and sync planning state.

### Phase 78: C-API Boolean/Self-Intersect Error Contract Coverage (No Clipper)
**Goal**: Harden C-API error-contract reliability by adding direct invalid-input error-code coverage for boolean operation dispatch and self-intersect options validation.
**Depends on**: Phase 77
**Requirements**: [PAR-208, PAR-209, PAR-210]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_pline_boolean` returns `2` for unrecognized operation values and `1` for null pline inputs.
  2. FFI tests explicitly verify `cavc_pline_scan_for_self_intersect` returns `2` for invalid options and `1` for null pline inputs, with docs aligned on parameter naming.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 78-01: Add direct invalid-operation and invalid-options error contract parity tests.
- [x] 78-02: Publish post-contract alignment map for next bounded targets.
- [x] 78-03: Close verification gates and sync planning state.

### Phase 79: C-API Contains/Extents Invalid-Input Contract Coverage (No Clipper)
**Goal**: Deepen C-API invalid-input contract reliability by adding direct error-code and output-state assertions for contains and extents surfaces.
**Depends on**: Phase 78
**Requirements**: [PAR-211, PAR-212, PAR-213]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_pline_contains` returns `1` for null pline inputs and writes `CAVC_CONTAINS_RESULT_INVALID_INPUT` when `result` is non-null.
  2. FFI tests explicitly verify `cavc_pline_eval_extents` returns `2` for degenerate (<2 vertex) input and preserves output sentinel values on failure.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 79-01: Add direct contains/extents invalid-input contract parity tests.
- [x] 79-02: Publish post-contract alignment map for next bounded targets.
- [x] 79-03: Close verification gates and sync planning state.

### Phase 80: C-API Shape Polyline Accessor Invalid-Input Contract Coverage (No Clipper)
**Goal**: Deepen shape-surface C-API contract reliability by adding direct invalid-input and failure-path output-stability assertions for ccw/cw polyline accessor functions.
**Depends on**: Phase 79
**Requirements**: [PAR-214, PAR-215, PAR-216]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify ccw/cw shape polyline accessor functions return `1` for null shape inputs and `2` for out-of-bounds `polyline_index`.
  2. FFI tests explicitly verify failure-path output sentinel stability for count/is_closed/vertex buffer outputs under null/OOB contracts.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 80-01: Add direct shape polyline accessor invalid-input contract parity tests.
- [x] 80-02: Publish post-contract alignment map for next bounded targets.
- [x] 80-03: Close verification gates and sync planning state.

### Phase 81: C-API Shape-Root Invalid-Input Contract Coverage (No Clipper)
**Goal**: Deepen shape-root C-API contract reliability by adding direct null-input and failure-path output-stability assertions for shape creation/offset/count surfaces.
**Depends on**: Phase 80
**Requirements**: [PAR-217, PAR-218, PAR-219]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_shape_create`, `cavc_shape_parallel_offset`, and `cavc_shape_get_{ccw,cw}_count` return error code `1` for null shape/plinelist inputs.
  2. FFI tests explicitly verify failure-path output sentinel stability for shape pointer and count outputs under null-input contracts.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 81-01: Add direct shape-root invalid-input contract parity tests and doc alignment.
- [x] 81-02: Publish post-contract alignment map for next bounded targets.
- [x] 81-03: Close verification gates and sync planning state.

### Phase 82: C-API Plinelist Failure-Path Output Stability Coverage (No Clipper)
**Goal**: Deepen plinelist C-API contract reliability by adding direct null/OOB/empty failure-path assertions and out-parameter stability checks.
**Depends on**: Phase 81
**Requirements**: [PAR-220, PAR-221, PAR-222]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_plinelist_get_count`, `cavc_plinelist_get_pline`, `cavc_plinelist_pop`, and `cavc_plinelist_take` null/OOB/empty contracts via direct return-code assertions.
  2. FFI tests explicitly verify failure-path output sentinel stability for count and pline out parameters under null/OOB/empty paths.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 82-01: Add direct plinelist failure-path output stability parity tests.
- [x] 82-02: Publish post-contract alignment map for next bounded targets.
- [x] 82-03: Close verification gates and sync planning state.

### Phase 83: C-API AABBIndex Null-Path Output Stability Coverage (No Clipper)
**Goal**: Deepen aabbindex C-API contract reliability by adding direct null-path assertions and out-parameter stability checks for index creation/extents surfaces.
**Depends on**: Phase 82
**Requirements**: [PAR-223, PAR-224, PAR-225]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_pline_create_approx_aabbindex`, `cavc_pline_create_aabbindex`, and `cavc_aabbindex_get_extents` return `1` on null-input contracts.
  2. FFI tests explicitly verify null-path output sentinel stability for aabbindex pointer outputs and extents scalar outputs.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 83-01: Add direct aabbindex null-path output stability parity tests.
- [x] 83-02: Publish post-contract alignment map for next bounded targets.
- [x] 83-03: Close verification gates and sync planning state.

### Phase 84: C-API Pline-Eval Failure-Path Output Stability Coverage (No Clipper)
**Goal**: Deepen pline-eval C-API contract reliability by adding direct null/empty-path assertions and out-parameter stability checks across scalar/point/index eval surfaces.
**Depends on**: Phase 83
**Requirements**: [PAR-226, PAR-227, PAR-228]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify pline-eval null-input contracts for path length, area, winding number, extents, and closest-point surfaces via direct return-code assertions.
  2. FFI tests explicitly verify null/empty failure-path output sentinel stability for scalar, point, and index outputs on covered eval APIs.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 84-01: Add direct pline-eval failure-path output stability parity tests.
- [x] 84-02: Publish post-contract alignment map for next bounded targets.
- [x] 84-03: Close verification gates and sync planning state.

### Phase 85: C-API Pline Core Accessor Output Stability Coverage (No Clipper)
**Goal**: Deepen pline core accessor C-API contract reliability by adding direct null/OOB assertions and out-parameter stability checks for core read/clone surfaces.
**Depends on**: Phase 84
**Requirements**: [PAR-229, PAR-230, PAR-231]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify pline core accessor null/OOB contracts across clone/is_closed/vertex_count/vertex_data/vertex/userdata_count/userdata_values surfaces.
  2. FFI tests explicitly verify failure-path output sentinel stability for pointer, scalar, and vertex outputs on covered pline core accessor APIs.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 85-01: Add direct pline core accessor failure-path output stability parity tests.
- [x] 85-02: Publish post-contract alignment map for next bounded targets.
- [x] 85-03: Close verification gates and sync planning state.

### Phase 86: C-API Shape Userdata Getter Output Stability Coverage (No Clipper)
**Goal**: Deepen shape userdata getter C-API contract reliability by adding direct null/OOB assertions and out-parameter stability checks for ccw/cw userdata count/value getters.
**Depends on**: Phase 85
**Requirements**: [PAR-232, PAR-233, PAR-234]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify shape userdata getter null/OOB contracts across `cavc_shape_get_ccw_pline_userdata_count`, `cavc_shape_get_ccw_pline_userdata_values`, `cavc_shape_get_cw_pline_userdata_count`, and `cavc_shape_get_cw_pline_userdata_values`.
  2. FFI tests explicitly verify failure-path output sentinel stability for scalar and userdata-buffer outputs on covered shape userdata getter APIs.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 86-01: Add direct shape userdata getter failure-path output stability parity tests.
- [x] 86-02: Publish post-contract alignment map for next bounded targets.
- [x] 86-03: Close verification gates and sync planning state.

### Phase 87: C-API Boolean/Self-Intersect Output Stability Coverage (No Clipper)
**Goal**: Deepen boolean and self-intersect C-API contract reliability by adding direct invalid-input assertions and output stability checks for error-path list/flag outputs.
**Depends on**: Phase 86
**Requirements**: [PAR-235, PAR-236, PAR-237]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify invalid-operation/null contracts for `cavc_pline_boolean` and invalid-options/null contracts for `cavc_pline_scan_for_self_intersect`.
  2. FFI tests explicitly verify failure-path output sentinel stability for boolean plinelist outputs and self-intersect result-flag outputs on covered APIs.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 87-01: Add direct boolean/self-intersect failure-path output stability parity tests.
- [x] 87-02: Publish post-contract alignment map for next bounded targets.
- [x] 87-03: Close verification gates and sync planning state.

### Phase 88: C-API Parallel-Offset Null-Path Output Stability Coverage (No Clipper)
**Goal**: Deepen parallel-offset C-API contract reliability by adding direct null-path assertions and output stability checks for result list outputs.
**Depends on**: Phase 87
**Requirements**: [PAR-238, PAR-239, PAR-240]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify null-input contracts for `cavc_pline_parallel_offset` across default-options and explicit-options calls via direct return-code assertions.
  2. FFI tests explicitly verify failure-path output sentinel stability for `result` plinelist outputs under covered null-input contracts.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 88-01: Add direct parallel-offset null-path output stability parity tests.
- [x] 88-02: Publish post-contract alignment map for next bounded targets.
- [x] 88-03: Close verification gates and sync planning state.

### Phase 89: C-API Shape-Offset Null-Path Output Stability Coverage (No Clipper)
**Goal**: Deepen shape-offset C-API contract reliability by adding direct null-path assertions and output stability checks for result shape outputs.
**Depends on**: Phase 88
**Requirements**: [PAR-241, PAR-242, PAR-243]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify null-input contracts for `cavc_shape_parallel_offset` across default-options and explicit-options calls via direct return-code assertions.
  2. FFI tests explicitly verify failure-path output sentinel stability for `result` shape outputs under covered null-input contracts.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 89-01: Add direct shape-offset null-path output stability parity tests.
- [x] 89-02: Publish post-contract alignment map for next bounded targets.
- [x] 89-03: Close verification gates and sync planning state.

### Phase 90: C-API Options-Path Invalid-Input Contract Invariance (No Clipper)
**Goal**: Deepen invalid-input contract reliability by proving explicit-options paths preserve default-path error and output behavior on boolean/contains boundaries.
**Depends on**: Phase 89
**Requirements**: [PAR-244, PAR-245, PAR-246]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_pline_boolean` null-input contracts on explicit-options path with direct return-code assertions.
  2. FFI tests explicitly verify `cavc_pline_contains` invalid-input result behavior on explicit-options path, including deterministic `CAVC_CONTAINS_RESULT_INVALID_INPUT` writes.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 90-01: Add options-path invalid-input contract invariance parity tests.
- [x] 90-02: Publish post-contract alignment map for next bounded targets.
- [x] 90-03: Close verification gates and sync planning state.

### Phase 91: C-API Boolean Invalid-Operation Options-Path Output Stability Coverage (No Clipper)
**Goal**: Deepen boolean/contains options-path invalid-input reliability by hardening invalid-operation output stability and null-result-pointer contract checks.
**Depends on**: Phase 90
**Requirements**: [PAR-247, PAR-248, PAR-249]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_pline_boolean` invalid-operation contracts on explicit-options path with direct return-code assertions and unchanged output sentinel pointers.
  2. FFI tests explicitly verify `cavc_pline_contains` null-result-pointer invalid-input behavior on explicit-options path via direct return-code assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 91-01: Add boolean invalid-operation options-path output-stability parity tests.
- [x] 91-02: Publish post-contract alignment map for next bounded targets.
- [x] 91-03: Close verification gates and sync planning state.

### Phase 92: C-API Self-Intersect/Contains Null-Result Contract Symmetry (No Clipper)
**Goal**: Deepen self-intersect and contains invalid-input reliability by asserting default/options null-result symmetry and output-stability behavior.
**Depends on**: Phase 91
**Requirements**: [PAR-250, PAR-251, PAR-252]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify `cavc_pline_scan_for_self_intersect` null-input contracts on both explicit-options and default-options paths with output sentinel stability.
  2. FFI tests explicitly verify `cavc_pline_contains` null-result-pointer invalid-input behavior symmetry for both null-`pline1` and null-`pline2` on explicit-options path.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 92-01: Add self-intersect/contains null-result contract symmetry parity tests.
- [x] 92-02: Publish post-contract alignment map for next bounded targets.
- [x] 92-03: Close verification gates and sync planning state.

### Phase 93: C-API Pline Mutator Invalid-Input Contract Coverage (No Clipper)
**Goal**: Deepen pline mutator contract reliability by adding direct null/OOB invalid-input assertions for core mutator surfaces.
**Depends on**: Phase 92
**Requirements**: [PAR-253, PAR-254, PAR-255]
**Success Criteria** (what must be TRUE):
  1. FFI tests explicitly verify null-input contracts for `cavc_pline_set_vertex_data`, `cavc_pline_set_is_closed`, `cavc_pline_clear`, `cavc_pline_set_vertex`, and `cavc_pline_remove`.
  2. FFI tests explicitly verify OOB contracts for `cavc_pline_set_vertex` and `cavc_pline_remove` via direct return-code assertions.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 93-01: Add pline mutator invalid-input contract parity tests.
- [x] 93-02: Publish post-contract alignment map for next bounded targets.
- [x] 93-03: Close verification gates and sync planning state.

### Phase 94: C++ Circle-Rectangle Intersection Expected-Table Parity (No Clipper)
**Goal**: Replace count-only intersection snapshot parity with executable point/index expected-table assertions for the historical C++ circle/rectangle geometry.
**Depends on**: Phase 93
**Requirements**: [PAR-256, PAR-257, PAR-258]
**Success Criteria** (what must be TRUE):
  1. Rust parity tests assert all expected basic intersects for the C++ circle/rectangle geometry with segment-index attribution and coordinate checks.
  2. Rust parity tests assert cardinality remains exact and overlapping intersections remain empty for the same geometry.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 94-01: Add circle/rectangle intersection expected-table parity assertions.
- [x] 94-02: Publish post-intersection-table alignment map for next bounded targets.
- [x] 94-03: Close verification gates and sync planning state.

### Phase 95: C++ Circle-Rectangle Intersection Variant Matrix Parity (No Clipper)
**Goal**: Deepen circle/rectangle intersection parity by asserting operand-order and direction-variant matrix invariants for the same historical C++ geometry.
**Depends on**: Phase 94
**Requirements**: [PAR-259, PAR-260, PAR-261]
**Success Criteria** (what must be TRUE):
  1. Rust parity tests explicitly validate swapped-operand circle/rectangle intersection expected-table assertions with index-pair and coordinate checks.
  2. Rust parity tests explicitly validate a bounded operand-order and direction-variant matrix preserves expected intersection point set, exact cardinality, and empty overlapping output.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 95-01: Add swapped-operand and variant-matrix circle/rectangle intersection parity assertions.
- [x] 95-02: Publish post-variant-matrix alignment map for next bounded targets.
- [x] 95-03: Close verification gates and sync planning state.

### Phase 96: C++ Line-Line Primitive Branch Matrix Parity (No Clipper)
**Goal**: Deepen standalone primitive parity by asserting source-traceable line-line branch matrix expectations against old C++ `intrLineSeg2LineSeg2` behavior.
**Depends on**: Phase 95
**Requirements**: [PAR-262, PAR-263, PAR-264]
**Success Criteria** (what must be TRUE):
  1. Rust parity tests explicitly validate a bounded expected-table matrix covering `True`, `False`, `None`, and `Coincident` line-line branch families plus degenerate point paths.
  2. Parity assertions explicitly check expected parametric outputs (`seg1_t`, `seg2_t`, `seg2_t0`, `seg2_t1`) for covered cases.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 96-01: Add line-line primitive branch matrix parity expected-table tests.
- [x] 96-02: Publish post-line-line parity alignment map for next bounded targets.
- [x] 96-03: Close verification gates and sync planning state.

### Phase 97: C++ Line-Circle Primitive Branch Matrix Parity (No Clipper)
**Goal**: Deepen standalone primitive parity by asserting source-traceable line-circle branch matrix expectations against old C++ `intrLineSeg2Circle2` behavior.
**Depends on**: Phase 96
**Requirements**: [PAR-265, PAR-266, PAR-267]
**Success Criteria** (what must be TRUE):
  1. Rust parity tests explicitly validate a bounded expected-table matrix covering line-circle branch families: degenerate-point paths, tangent, no-intersect, and two-intersect cases.
  2. Parity assertions explicitly check expected parametric outputs (`t0`, `t1`) including outside-segment solutions where applicable.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 97-01: Add line-circle primitive branch matrix parity expected-table tests.
- [x] 97-02: Publish post-line-circle parity alignment map for next bounded targets.
- [x] 97-03: Close verification gates and sync planning state.

### Phase 98: C++ Circle-Circle Primitive Branch Matrix Parity (No Clipper)
**Goal**: Deepen standalone primitive parity by asserting source-traceable circle-circle branch matrix expectations against old C++ `intrCircle2Circle2` behavior.
**Depends on**: Phase 97
**Requirements**: [PAR-268, PAR-269, PAR-270]
**Success Criteria** (what must be TRUE):
  1. Rust parity tests explicitly validate a bounded expected-table matrix covering circle-circle branch families: coincident, no-intersect (outside/inside), tangent, and two-intersects.
  2. Parity assertions explicitly validate expected intersect point outputs for covered branch-matrix cases, including near-tangent midpoint behavior.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 98-01: Add circle-circle primitive branch matrix parity expected-table tests.
- [x] 98-02: Publish post-circle-circle parity alignment map for next bounded targets.
- [x] 98-03: Close verification gates and sync planning state.

### Phase 99: C++ Pline Segment Overlap-Order and Endpoint-Stickiness Parity (No Clipper)
**Goal**: Deepen old C++ segment-level parity by asserting source-traceable `intrPlineSegs` overlap-order and endpoint-stickiness behavior across line-line, line-arc, and arc-line branches.
**Depends on**: Phase 98
**Requirements**: [PAR-271, PAR-272, PAR-273]
**Success Criteria** (what must be TRUE):
  1. Rust parity tests explicitly validate bounded overlap-order branch cases for line-line overlap and two-intersect line-arc/arc-line outputs according to second-segment direction.
  2. Rust parity tests explicitly validate endpoint-stickiness behavior in line-arc and arc-line branches where line-circle solutions and arc sweep filtering interact.
  3. Full workspace and planning health gates are green, with post-phase alignment map updated.
**Plans**: 3 plans

Plans:
- [x] 99-01: Add pline segment overlap-order and endpoint-stickiness parity tests.
- [x] 99-02: Publish post-pline-segment parity alignment map for next bounded targets.
- [x] 99-03: Close verification gates and sync planning state.

## Progress

**Execution Order:**
Phases execute in numeric order unless an inserted decimal phase is added for urgent work.

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Absorption Contract Audit | 4/4 | Complete    | 2026-05-12 |
| 2. Fixture Schema and Property Harness | 3/3 | Complete    | 2026-05-12 |
| 3. Historical C++ Evidence Mining | 3/3 | Complete    | 2026-05-12 |
| 4. Benchmark Baseline | 3/3 | Complete    | 2026-05-12 |
| 5. Clipper2 Oracle Boundary | 4/4 | Complete    | 2026-05-12 |
| 6. Robustness Gap Closure | 4/4 | Complete | 2026-05-12 |
| 7. Capability Absorption Pipeline | 4/4 | Complete    | 2026-05-12 |
| 8. API, FFI, and Migration Readiness | 3/3 | Complete    | 2026-05-12 |
| 9. C++ Parity Deep Comparison (No Clipper) | 3/3 | Complete | 2026-05-12 |
| 10. C++ Function-Level Parity Deepening (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 11. Closest-Point and Generated Matrix Parity Expansion (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 12. Strict-Index and Full Half-Circle Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 13. Full Circle Generated Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 14. Circle Offset and Collapse Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 15. Half-Circle Offset and Collapse Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 16. C++ Offset Matrix Expansion and Reversed Parity (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 17. C++ Coincident Combine Matrix Parity Expansion (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 18. Coincident Intersect Collapsed-Filter Parity Path (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 19. Coincident Intersect Default-Path Line-Loop Parity Closure (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 20. C-API Coincident Intersect Parity Bridge (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 21. C-API Combine Matrix Expansion (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 22. C-API Combine Self-Invariants Parity Bridge (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 23. C-API Parallel-Offset Matrix Parity Bridge (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 24. C-API Combine No-Modify Parity Bridge (No Clipper) | 3/3 | Complete   | 2026-05-13 |
| 25. C-API Function-Surface Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 26. C-API Options-Path Parity Bridge (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 27. C-API Coincident No-Modify Matrix Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 28. C-API Optioned Coincident Edge Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 29. C-API Optioned Coincident Output Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 30. C-API Closest-Point Parity Bridge (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 31. C-API Half-Circle Closest-Point Strict Index Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 32. C-API Function-Surface Combine-Self Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 33. C-API Closest-Point Epsilon/Tie-Break Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 34. C-API Function-Surface Parallel-Offset Full Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 35. C-API Combine-Self Vertex-Exact Reversed Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 36. C-API Pline-Suite Buffer/Reserve Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 37. C-API Pline Remove-Sequence Range-Equivalence Parity (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 38. C-API Cross-Suite Coverage Audit (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 39. C-API Equivalence-Zone Regression Hardening (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 40. C-API Old-Suite Drift-Detection Hook (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 41. C-API Options-Path No-Modify Hardening (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 42. C-API Options-Path Vertex-Output Deepening (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 43. C-API Drift-Failure Triage Template (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 44. C-API Options-Path Coincident Vertex-Output Deepening (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 45. C-API Options-Path Tolerance-Matrix Deepening (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 46. C-API Options-Path Self-Intersects Mode Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 47. C-API Self-Intersects Mode No-Modify Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 48. C-API Options-Path Self-Intersects Stress Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 49. C-API Options-Path Reversed Self-Intersects Stress Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 50. C-API Options-Path Reversed Self-Intersects No-Modify Stress Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 51. C-API FFI Parity Helper Extraction (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 52. C-API Reversed Output/No-Modify Merge Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 53. C-API Reversed Specific-Edge Attribution Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 54. C-API Default Output/No-Modify Merge Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 55. C-API Default Specific-Edge Attribution Matrix (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 56. C-API Specific-Edge Runner Helper Extraction (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 57. C-API Specific-Edge Matrix Coverage Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 58. C-API Specific-Edge Matrix Open-Path Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 59. C-API Specific-Edge Matrix Diamond Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 60. C-API Specific-Edge Matrix Open-Diamond Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 61. C-API Specific-Edge Matrix Open-Diamond-Outward Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 62. C-API Specific-Edge Matrix Closed-Diamond-Inward Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 63. C-API Specific-Edge Matrix Closed-Rectangle-Outward Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 64. C-API Specific-Edge Matrix Closed-Rectangle-Inward Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 65. C-API Specific-Edge Matrix Open-Rectangle-Inward Expansion (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 66. C-API Specific-Edge Matrix Source-Coverage Guard (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 67. C-API Coincident Exclude Name Canonicalization (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 68. C-API Coincident Matrix Helper Extraction (No Clipper) | 3/3 | Complete   | 2026-05-14 |
| 69. C-API Coincident Matrix Source-Coverage Guard (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 70. C-API Coincident Case1 Matrix Parity Expansion (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 71. C-API Coincident Default Matrix Source-Map Guard (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 72. C-API Circle-Rectangle Source Matrix Guard Reuse (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 73. C-API Pline Core Suite Source-Coverage Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 74. C-API AABBIndex Extents Source Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 75. C-API Option Lifecycle & CW Userdata Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 76. C-API CCW Userdata Setter Symmetry Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 77. C-API Userdata Getter Bounds Contract Hardening (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 78. C-API Boolean/Self-Intersect Error Contract Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 79. C-API Contains/Extents Invalid-Input Contract Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 80. C-API Shape Polyline Accessor Invalid-Input Contract Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 81. C-API Shape-Root Invalid-Input Contract Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 82. C-API Plinelist Failure-Path Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 83. C-API AABBIndex Null-Path Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 84. C-API Pline-Eval Failure-Path Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 85. C-API Pline Core Accessor Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 86. C-API Shape Userdata Getter Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 87. C-API Boolean/Self-Intersect Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 88. C-API Parallel-Offset Null-Path Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 89. C-API Shape-Offset Null-Path Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 90. C-API Options-Path Invalid-Input Contract Invariance (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 91. C-API Boolean Invalid-Operation Options-Path Output Stability Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 92. C-API Self-Intersect/Contains Null-Result Contract Symmetry (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 93. C-API Pline Mutator Invalid-Input Contract Coverage (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 94. C++ Circle-Rectangle Intersection Expected-Table Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 95. C++ Circle-Rectangle Intersection Variant Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 96. C++ Line-Line Primitive Branch Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 97. C++ Line-Circle Primitive Branch Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 98. C++ Circle-Circle Primitive Branch Matrix Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
| 99. C++ Pline Segment Overlap-Order and Endpoint-Stickiness Parity (No Clipper) | 3/3 | Complete   | 2026-05-15 |
