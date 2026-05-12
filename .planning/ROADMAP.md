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

- [ ] **Phase 1: Absorption Contract Audit** - Define source boundaries, comparability rules, and API migration baseline.
- [ ] **Phase 2: Fixture Schema and Property Harness** - Create durable geometry fixture and comparison infrastructure.
- [ ] **Phase 3: Historical C++ Evidence Mining** - Translate high-value old C++ behavior into Rust regression evidence.
- [ ] **Phase 4: Benchmark Baseline** - Establish performance baselines and benchmark provenance rules.
- [ ] **Phase 5: Clipper2 Oracle Boundary** - Add polygon-only oracle evidence without changing production behavior.
- [ ] **Phase 6: Robustness Gap Closure** - Rank and fix the highest-value current Rust robustness issues.
- [ ] **Phase 7: Capability Absorption Pipeline** - Select and absorb compatible capabilities with tests and visible validation when needed.
- [ ] **Phase 8: API, FFI, and Migration Readiness** - Harden external surfaces and migration notes for release-quality use.

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
- [ ] 01-01: Build the three-codebase capability inventory.
- [ ] 01-02: Record license, provenance, and acceptable-use boundaries.
- [ ] 01-03: Define the behavior taxonomy and comparison policy.
- [ ] 01-04: Compare API, FFI, and migration surfaces.

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
- [ ] 02-01: Define fixture schema and manifest conventions.
- [ ] 02-02: Extend property comparison helpers for absorbed cases.
- [ ] 02-03: Add seed fixtures proving the schema and harness work.

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
- [ ] 03-01: Inventory old C++ tests, examples, and benchmark profiles for fixture value.
- [ ] 03-02: Translate prioritized behavior cases into Rust fixtures.
- [ ] 03-03: Validate imported fixtures through the property harness.

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
- [ ] 04-01: Add or document current Rust measurement baseline.
- [ ] 04-02: Map historical C++ benchmark families to Rust cases.
- [ ] 04-03: Document benchmark provenance and cost-accounting rules.

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
- [ ] 05-01: Classify Clipper2 tests and examples for polygon-only eligibility.
- [ ] 05-02: Add Clipper2-derived fixture representations.
- [ ] 05-03: Implement or script the dev-only oracle comparison path.
- [ ] 05-04: Emit oracle evidence reports for gap ranking.

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
- [ ] 06-01: Build the ranked robustness backlog from fixtures, benchmarks, and oracle evidence.
- [ ] 06-02: Add focused regressions for top-ranked failures.
- [ ] 06-03: Implement the first robustness fixes.
- [ ] 06-04: Run and document the required verification gates.

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
- [ ] 07-01: Select candidate capabilities from audit and gap evidence.
- [ ] 07-02: Design capability-specific behavior and API boundaries.
- [ ] 07-03: Implement the first absorbed capability slice.
- [ ] 07-04: Update examples, docs, FFI notes, or demo UI only as needed.

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
- [ ] 08-01: Audit public API and FFI compatibility after absorbed changes.
- [ ] 08-02: Update ABI tests, generated header, and external docs if needed.
- [ ] 08-03: Write old C++ migration notes and milestone readiness summary.

## Progress

**Execution Order:**
Phases execute in numeric order unless an inserted decimal phase is added for urgent work.

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Absorption Contract Audit | 0/4 | Not started | - |
| 2. Fixture Schema and Property Harness | 0/3 | Not started | - |
| 3. Historical C++ Evidence Mining | 0/3 | Not started | - |
| 4. Benchmark Baseline | 0/3 | Not started | - |
| 5. Clipper2 Oracle Boundary | 0/4 | Not started | - |
| 6. Robustness Gap Closure | 0/4 | Not started | - |
| 7. Capability Absorption Pipeline | 0/4 | Not started | - |
| 8. API, FFI, and Migration Readiness | 0/3 | Not started | - |
