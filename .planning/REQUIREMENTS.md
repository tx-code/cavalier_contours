# Requirements: Cavalier Contours Absorption Roadmap

**Defined:** 2026-05-12
**Core Value:** Make the Rust crate a robust, well-tested, arc-aware 2D geometry library whose behavior is defensible against historical CavalierContours behavior and polygon-only Clipper2 reference results.

## v1 Requirements

Requirements for the initial multi-milestone absorption roadmap. Each maps to
one roadmap phase.

### Audit

- [x] **AUD-01**: The project records a capability inventory across Rust `cavalier_contours`, old C++ CavalierContours, and Clipper2.
- [x] **AUD-02**: The project records license, provenance, and acceptable-use boundaries for mined tests, fixtures, benchmarks, and reference behavior.
- [x] **AUD-03**: The project defines a behavior taxonomy for exact parity, approximate parity, intentional divergence, and not-comparable cases.
- [x] **AUD-04**: The project compares public Rust APIs, C FFI surface, old C++ C API, and relevant Clipper2 operations for migration and compatibility planning.

### Fixtures

- [x] **FIX-01**: The project defines a durable fixture schema with source, geometry model, tolerance policy, comparison mode, and expected properties.
- [x] **FIX-02**: Tests can compare geometry by properties such as area, extents, path length, orientation, containment, repeat vertices, and result counts.
- [x] **FIX-03**: High-value old C++ tests and benchmark profiles are translated or represented as Rust regression fixtures.
- [x] **FIX-04**: Eligible Clipper2 polygon-only cases are represented as Rust fixtures with explicit comparability classification.

### Benchmarks

- [x] **BEN-01**: The project records a current Rust benchmark baseline for offsets, booleans, intersections, and spatial-index-heavy cases.
- [x] **BEN-02**: Historical old C++ benchmark profile families are mapped to Rust benchmark or measurement cases.
- [x] **BEN-03**: Benchmarks document whether arc approximation, conversion, and oracle execution costs are included or excluded.

### Oracle

- [x] **ORC-01**: A dev-only Clipper2 comparison path exists for eligible polygon-only boolean and offset cases.
- [x] **ORC-02**: Any arc-to-polygon comparison records approximation tolerance and does not redefine native arc behavior.
- [x] **ORC-03**: Oracle results are reported as evidence for gap ranking, not as automatic production behavior.

### Robustness

- [x] **ROB-01**: The project maintains a ranked robustness backlog for offsets, booleans, intersections, tolerances, degenerates, repeat vertices, tangencies, overlaps, and open/closed behavior.
- [x] **ROB-02**: Top-ranked robustness gaps have focused regression tests before or with fixes.
- [x] **ROB-03**: Top-ranked current Rust robustness issues are fixed without broad API churn.
- [x] **ROB-04**: Robustness phases pass the workspace verification gate required for the changed surface.

### Capability Absorption

- [x] **CAP-01**: Candidate capabilities from old C++ and Clipper2 are selected only after audit, fixtures, and gap ranking.
- [x] **CAP-02**: Each absorbed capability preserves the Rust crate's arc-aware model or explicitly documents why it is polygon-only.
- [x] **CAP-03**: Absorbed capabilities include tests, examples or docs, and FFI impact notes when externally visible.

### API and FFI

- [x] **API-01**: Public Rust API and FFI changes include explicit compatibility notes.
- [x] **API-02**: Any FFI surface change updates ABI tests and regenerates `cavalier_contours_ffi.h`.
- [x] **API-03**: The project produces migration notes for users coming from old C++ CavalierContours.

### Demo

- [x] **DEM-01**: The demo UI is updated only when a new or changed geometry capability needs visual validation.

## v2 Requirements

Deferred to a later roadmap. Tracked but not in current v1 scope.

### Geometry Expansion

- **GEO-01**: Add triangulation support after explicit rescoping and independent validation.
- **GEO-02**: Expand boolean operations beyond two closed non-self-intersecting polylines.
- **GEO-03**: Add additional offset join styles beyond rounded joins.
- **GEO-04**: Explore wider support for intersecting or open multi-polyline shape offsets.

### Tooling

- **TLG-01**: Automate large-scale import from old C++ and Clipper2 test corpora if manual translation becomes a bottleneck.
- **TLG-02**: Add generated/randomized differential test cases with shrinking or minimization.

### Product Surface

- **SUR-01**: Redesign the demo UI as a productized geometry workbench.
- **SUR-02**: Provide a production Clipper2 backend or runtime dependency.

### C++ Parity Deep Comparison

- **PAR-01**: The project maps old C++ logic modules/tests to Rust modules/tests for deep parity review without Clipper involvement.
- **PAR-02**: High-value C++ cases in boolean, offset, and intersection paths are executed or explicitly classified with evidence.
- **PAR-03**: Confirmed C++ vs Rust mismatches are classified as bug, intentional divergence, or not-comparable with recorded fix/defer decisions.
- **PAR-04**: The project maps old C++ `TEST_cavc_pline_function.cpp` function-level expectations to Rust function-level APIs and tests.
- **PAR-05**: Selected C++ function-level expectations (area/path/extents/winding/self-boolean invariants) execute in Rust parity tests with evidence.
- **PAR-06**: Newly surfaced function-level mismatches are classified with explicit fix/defer decisions.
- **PAR-07**: Closest-point expectations from old C++ `pline_function` cases are mapped into executable Rust parity checks with explicit index tie-break policy.
- **PAR-08**: A bounded subset of old C++ generated function-case matrices is executed in Rust parity tests or explicitly marked not-comparable.
- **PAR-09**: Closest-point and generated-matrix mismatches are classified with evidence and explicit fix/defer decisions.
- **PAR-10**: Full old C++ generated half-circle case matrices (open/closed, x/y-aligned, cw/ccw, multi-center) execute as Rust parity tests with reusable tolerance helpers.
- **PAR-11**: Closest-point expectations with explicit index results in generated half-circle cases are validated in strict mode and any tie-break mismatches are fixed or explicitly classified.
- **PAR-12**: Deep parity continuation includes a file/module alignment map that names next high-value C++ targets and Rust implementation surfaces.
- **PAR-13**: Full old C++ generated circle case matrices (all centers, alignments, reverse variants, and direction variants) execute as Rust parity tests with source-traceable expectations.
- **PAR-14**: Closest-point expectations from generated circle cases validate explicit index expectations in strict mode and keep non-explicit index cases as point/distance parity checks.
- **PAR-15**: After full circle matrix closure, a file/module alignment map names the next deep-parity targets for offset and collapsed-offset matrices.
- **PAR-16**: Full old C++ generated circle offset matrix expectations execute as Rust parity tests for outward and inward deltas across all generated variants.
- **PAR-17**: Generated circle collapsed-offset deltas from old C++ execute as Rust parity checks and remain empty where expected.
- **PAR-18**: Offset matrix parity validates both geometry properties and vertex-level output (with closed-curve start rotation tolerance) and publishes next-step alignment scope.
- **PAR-19**: Full old C++ generated half-circle offset matrix expectations execute as Rust parity tests for outward and inward deltas across all generated variants.
- **PAR-20**: Generated half-circle collapsed-offset deltas from old C++ execute as Rust parity checks and remain empty where expected.
- **PAR-21**: Half-circle offset parity validates both geometry properties and vertex-level output (open exact-order and closed-curve start rotation tolerance) and publishes next-step alignment scope.
- **PAR-22**: Old C++ `TEST_cavc_parallel_offset.cpp` simple and specific `parallel_offset` matrices execute as Rust parity tests with source-traceable expected property sets.
- **PAR-23**: Reversed-input parity (`invert_direction` + negated delta) executes across imported offset matrices with sign-adjusted area and matching geometric properties.
- **PAR-24**: Imported offset parity includes collapsed-result and input-immutability checks and publishes next deep-alignment scope.
- **PAR-25**: Old C++ `TEST_cavc_combine_plines.cpp` coincident case matrices execute as Rust parity tests across `Or`, `Not`, `And`, and `Xor` combine modes.
- **PAR-26**: Coincident combine outcomes are classified with explicit parity/divergence decisions and source-traceable evidence.
- **PAR-27**: Coincident combine phase closes with explicit next-target alignment map and full verification gate closure.
- **PAR-28**: Coincident intersect sliver behavior is covered by an explicit parity test path using `PlineBooleanOptions.collapsed_area_eps` that matches old C++ empty-output expectation.
- **PAR-29**: Default-path versus collapsed-filter-path behavior is explicitly classified and documented for the coincident intersect case.
- **PAR-30**: The project records the follow-up decision boundary for adopting or deferring a default collapsed-area threshold in boolean operations.
- **PAR-31**: Boolean stitching removes only degenerate line-only two-vertex closed loops while preserving valid two-vertex arc loops.
- **PAR-32**: `coincident_case1_intersect` default-path behavior matches old C++ empty-output parity through executable Rust tests.
- **PAR-33**: The project records the post-fix no-Clipper deep-alignment map and closes with full verification gates.
- **PAR-34**: The FFI surface (`cavc_pline_boolean`) includes an executable coincident intersect parity case sourced from old C++ combine inputs.
- **PAR-35**: FFI default-path `coincident_case1_intersect` behavior returns empty results for `And` operation parity with old C++ expectation.
- **PAR-36**: C-API parity bridge work records next C-API expansion scope and closes with full verification gates.
- **PAR-37**: The FFI surface executes full old C++ `circle_rectangle` combine matrix parity through `cavc_pline_boolean` with source-traceable expected properties.
- **PAR-38**: The FFI surface executes full old C++ `coincident_case2` combine matrix parity, including both exclude directions, with property-set matching.
- **PAR-39**: C-API combine matrix expansion records next C-API parity targets and closes with full verification gates.
- **PAR-40**: The FFI surface executes old C++ combine-with-self invariants for union/intersect/self-empty modes through `cavc_pline_boolean`.
- **PAR-41**: Reversed and mixed-orientation self-invariant empty-result cases are explicitly executed and verified at the C-API boundary.
- **PAR-42**: C-API self-invariants bridge work records next C-API parity scope and closes with full verification gates.
- **PAR-43**: The FFI surface executes old C++ `parallel_offset` simple and specific matrices through `cavc_pline_parallel_offset` with source-traceable expected properties.
- **PAR-44**: C-API `parallel_offset` reversed-input parity and no-modify input invariants are explicitly executed and verified.
- **PAR-45**: C-API parallel-offset bridge work records next C-API parity scope and closes with full verification gates.

## Out of Scope

Explicitly excluded to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Triangulation | User explicitly deferred it, and Clipper2 README warns its triangulation code is buggy. |
| Clipper2 as production backend | Clipper2 is polygon-focused and should be an oracle/reference, not a replacement for the arc-aware Rust kernel. |
| Port-first algorithm work | Absorption must begin with audit, fixtures, benchmarks, and evidence. |
| Broad UI redesign | UI changes are tied only to new functionality that needs demo or validation support. |
| Unsafe code in the core crate | Current crate-level policy forbids unsafe in the core library. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| AUD-01 | Phase 1 | Complete |
| AUD-02 | Phase 1 | Complete |
| AUD-03 | Phase 1 | Complete |
| AUD-04 | Phase 1 | Complete |
| FIX-01 | Phase 2 | Complete |
| FIX-02 | Phase 2 | Complete |
| FIX-03 | Phase 3 | Complete |
| FIX-04 | Phase 5 | Complete |
| BEN-01 | Phase 4 | Complete |
| BEN-02 | Phase 4 | Complete |
| BEN-03 | Phase 4 | Complete |
| ORC-01 | Phase 5 | Complete |
| ORC-02 | Phase 5 | Complete |
| ORC-03 | Phase 5 | Complete |
| ROB-01 | Phase 6 | Complete |
| ROB-02 | Phase 6 | Complete |
| ROB-03 | Phase 6 | Complete |
| ROB-04 | Phase 6 | Complete |
| CAP-01 | Phase 7 | Complete |
| CAP-02 | Phase 7 | Complete |
| CAP-03 | Phase 7 | Complete |
| API-01 | Phase 8 | Complete |
| API-02 | Phase 8 | Complete |
| API-03 | Phase 8 | Complete |
| DEM-01 | Phase 7 | Complete |
| PAR-01 | Phase 9 | Complete |
| PAR-02 | Phase 9 | Complete |
| PAR-03 | Phase 9 | Complete |
| PAR-04 | Phase 10 | Complete |
| PAR-05 | Phase 10 | Complete |
| PAR-06 | Phase 10 | Complete |
| PAR-07 | Phase 11 | Complete |
| PAR-08 | Phase 11 | Complete |
| PAR-09 | Phase 11 | Complete |
| PAR-10 | Phase 12 | Complete |
| PAR-11 | Phase 12 | Complete |
| PAR-12 | Phase 12 | Complete |
| PAR-13 | Phase 13 | Complete |
| PAR-14 | Phase 13 | Complete |
| PAR-15 | Phase 13 | Complete |
| PAR-16 | Phase 14 | Complete |
| PAR-17 | Phase 14 | Complete |
| PAR-18 | Phase 14 | Complete |
| PAR-19 | Phase 15 | Complete |
| PAR-20 | Phase 15 | Complete |
| PAR-21 | Phase 15 | Complete |
| PAR-22 | Phase 16 | Complete |
| PAR-23 | Phase 16 | Complete |
| PAR-24 | Phase 16 | Complete |
| PAR-25 | Phase 17 | Complete |
| PAR-26 | Phase 17 | Complete |
| PAR-27 | Phase 17 | Complete |
| PAR-28 | Phase 18 | Complete |
| PAR-29 | Phase 18 | Complete |
| PAR-30 | Phase 18 | Complete |
| PAR-31 | Phase 19 | Complete |
| PAR-32 | Phase 19 | Complete |
| PAR-33 | Phase 19 | Complete |
| PAR-34 | Phase 20 | Complete |
| PAR-35 | Phase 20 | Complete |
| PAR-36 | Phase 20 | Complete |
| PAR-37 | Phase 21 | Complete |
| PAR-38 | Phase 21 | Complete |
| PAR-39 | Phase 21 | Complete |
| PAR-40 | Phase 22 | Complete |
| PAR-41 | Phase 22 | Complete |
| PAR-42 | Phase 22 | Complete |
| PAR-43 | Phase 23 | Complete |
| PAR-44 | Phase 23 | Complete |
| PAR-45 | Phase 23 | Complete |

**Coverage:**
- v1 requirements: 25 total
- v1 mapped to phases: 25
- v1 unmapped: 0
- additional tracked post-v1 requirements: 45 (`PAR-01..PAR-45`), mapped to Phases 9-23

---
*Requirements defined: 2026-05-12*
*Last updated: 2026-05-13 after Phase 23 completion*
