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
