# Phase 01 Provenance and Usage Contract

**Date:** 2026-05-12
**Requirement:** AUD-02

## Repository Snapshots

| Repo | Local path | Branch | Commit | Working tree | License |
|------|------------|--------|--------|--------------|---------|
| Rust `cavalier_contours` | `E:/Coding/cavalier_contours` | `master` | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | Clean at audit start | `MIT OR Apache-2.0` |
| Old C++ CavalierContours | `E:/Coding/CavalierContours` | `master` | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | Clean at audit start | `MIT` |
| Clipper2 | `E:/Coding/Clipper2` | `main` | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | Clean at audit start | `Boost Software License 1.0` |

## Usage Labels

| Label | Meaning |
|-------|---------|
| `fork-owned/changeable` | Current Rust source controlled by this fork; may change with explicit rationale and verification. |
| `migration-sensitive` | Surface affects users migrating from old C++ or C ABI usage; changes need impact notes. |
| `reference-only` | External implementation code may be read to understand behavior but is not copied by default. |
| `external-oracle` | External output may be used as development evidence for eligible cases, not as production behavior. |
| `translated-fixture-candidate` | Test or example can be manually translated later with source path and tolerance/comparison policy. |
| `benchmark-candidate` | Benchmark profile can be mapped later with cost-accounting notes. |
| `deferred/not-comparable` | Out of scope or model mismatch for this roadmap phase. |

## Acceptable-Use Rules

- Rust `cavalier_contours` is the mainline target.
- Old C++ CavalierContours is same-lineage historical evidence. Use it for
  behavior expectations, fixture ideas, benchmark profile names, and migration
  notes; treat implementations as reference-only unless a later phase records a
  narrower decision.
- Clipper2 is a polygon-only oracle/reference source. Use it for eligible
  polygon path cases, especially offsets and clipping; do not use it as a
  production backend in this roadmap.
- Any translated fixture, benchmark, example, or oracle output added later must
  cite repo, commit, license, source path, usage label, and comparison mode.
- Clipper2 triangulation is deferred and must not be imported without a new
  scope decision.

## Evidence Ledger

| Repo | Commit | License | Path | Usage intent | Notes |
|------|--------|---------|------|--------------|-------|
| Rust | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | `MIT OR Apache-2.0` | `cavalier_contours/src/lib.rs` | `fork-owned/changeable` | Public Rust exports and safe core boundary. |
| Rust | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | `MIT OR Apache-2.0` | `cavalier_contours/src/polyline/internal/pline_offset.rs` | `fork-owned/changeable` | Mainline offset implementation. |
| Rust | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | `MIT OR Apache-2.0` | `cavalier_contours/src/polyline/internal/pline_boolean.rs` | `fork-owned/changeable` | Mainline boolean implementation. |
| Rust | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | `MIT OR Apache-2.0` | `cavalier_contours/tests/test_*.rs` | `fork-owned/changeable` | Existing regression baseline and helper patterns. |
| Rust | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | `MIT OR Apache-2.0` | `cavalier_contours_ffi/src/lib.rs`; `cavalier_contours_ffi.h` | `fork-owned/changeable`, `migration-sensitive` | C ABI and generated header drift surface. |
| Rust | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | `MIT OR Apache-2.0` | `examples/*.rs`; `cavalier_contours_ui/src/scenes/*` | `fork-owned/changeable` | Integration examples and visual demo evidence. |
| Old C++ | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | `include/cavc/*.hpp` | `reference-only`, `migration-sensitive` | Historical header API and behavior concepts. |
| Old C++ | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | `c_api_include/cavaliercontours.h`; `src/cavaliercontours.cpp` | `reference-only`, `migration-sensitive` | Historical C API for future migration notes. |
| Old C++ | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | `tests/tests/TEST_cavc_parallel_offset.cpp` | `translated-fixture-candidate` | Offset regression source for Phase 3. |
| Old C++ | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | `tests/tests/TEST_cavc_combine_plines.cpp` | `translated-fixture-candidate` | Boolean/combine regression source for Phase 3. |
| Old C++ | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | `tests/tests/TEST_staticspatialindex.cpp` | `translated-fixture-candidate`, `benchmark-candidate` | Broad-phase behavior and performance-sensitive evidence. |
| Old C++ | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | `tests/benchmarks/benchmarkprofiles.h`; `tests/benchmarks/*.cpp` | `benchmark-candidate` | Historical benchmark profile source for Phase 4. |
| Old C++ | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | `README.md`; `examples/*.cpp` | `reference-only`, `translated-fixture-candidate` | Algorithm notes and example behavior. |
| Clipper2 | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | `Boost Software License 1.0` | `CPP/Clipper2Lib/include/clipper2/clipper.h` | `external-oracle` | Public polygon clipping operations. |
| Clipper2 | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | `Boost Software License 1.0` | `CPP/Clipper2Lib/include/clipper2/clipper.offset.h`; `CPP/Clipper2Lib/src/clipper.offset.cpp` | `external-oracle`, `reference-only` | Polygon offset oracle/reference. |
| Clipper2 | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | `Boost Software License 1.0` | `CPP/Tests/TestOffsets.cpp`; `CPP/Tests/TestPolygons.cpp`; `CPP/Tests/TestLines.cpp` | `translated-fixture-candidate`, `external-oracle` | Eligible polygon-only tests after Phase 5 classification. |
| Clipper2 | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | `Boost Software License 1.0` | `CPP/BenchMark/*.cpp`; `CPP/Examples/*` | `benchmark-candidate`, `external-oracle` | Benchmark and example source after eligibility review. |
| Clipper2 | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | `Boost Software License 1.0` | `CPP/Clipper2Lib/include/clipper2/clipper.triangulation.h`; `CPP/Clipper2Lib/src/clipper.triangulation.cpp` | `deferred/not-comparable` | Triangulation explicitly out of v1 scope. |

## Comparison Evidence Requirements

Later phases must record:

- Source repo, commit, license, and path.
- Usage label from this document.
- Comparison class from `01-AUDIT.md`.
- Tolerance or approximation policy for approximate parity.
- Intentional-divergence decision link when behavior deliberately differs.

## API and FFI Impact Notes

Future public Rust API or C FFI changes must include an impact note naming the
changed surface, why the change is worthwhile, and affected tests, examples,
headers, FFI behavior, and docs. Regenerate `cavalier_contours_ffi.h` only when
the FFI surface changes.

## Requirement Coverage

| Requirement | Evidence in this artifact |
|-------------|---------------------------|
| AUD-02 | Repository snapshots, usage labels, acceptable-use rules, evidence ledger, and comparison evidence requirements. |
