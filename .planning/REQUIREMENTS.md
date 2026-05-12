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
| PAR-01 | Phase 9 | Pending |
| PAR-02 | Phase 9 | Pending |
| PAR-03 | Phase 9 | Pending |

**Coverage:**
- v1 requirements: 25 total
- v1 mapped to phases: 25
- v1 unmapped: 0
- additional tracked post-v1 requirements: 3 (`PAR-01..PAR-03`), mapped to Phase 9

---
*Requirements defined: 2026-05-12*
*Last updated: 2026-05-12 after Phase 6 completion*
