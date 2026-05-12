# Phase 01 Audit: Absorption Contract

**Date:** 2026-05-12
**Requirements:** AUD-01, AUD-03, AUD-04

## Source Snapshots

| Source | Path | Commit | Role |
|--------|------|--------|------|
| Rust `cavalier_contours` | `E:/Coding/cavalier_contours` | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | Fork-owned mainline |
| Old C++ CavalierContours | `E:/Coding/CavalierContours` | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | Same-lineage historical reference |
| Clipper2 | `E:/Coding/Clipper2` | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | Polygon-only oracle/reference |

## Audit Rules

- External implementations are reference-only unless a later phase records a
  narrower usage decision.
- Tests, fixture ideas, benchmark profiles, examples, and behavior expectations
  may be translated with provenance.
- Every main matrix entry below includes evidence paths or an explicit status.
- Clipper2 triangulation is `deferred` and is not a Phase 1 absorption candidate.

## Main Cross-Codebase Matrix

| Area | Rust `cavalier_contours` evidence | Old C++ evidence | Clipper2 evidence | Phase 1 classification |
|------|-----------------------------------|------------------|-------------------|------------------------|
| Geometry model | `cavalier_contours/src/polyline/pline.rs`; `cavalier_contours/src/polyline/pline_vertex.rs`; `README.md` | `include/cavc/polyline.hpp`; `include/cavc/plinesegment.hpp`; `README.md` | `CPP/Clipper2Lib/include/clipper2/clipper.core.h`; `README.md` | Rust and old C++ are line-plus-bulge-arc comparable; Clipper2 is polygon/path comparable only. |
| Construction and editing | `cavalier_contours/src/polyline/mod.rs`; `cavalier_contours/src/polyline/pline_view.rs` | `include/cavc/polyline.hpp`; `tests/tests/TEST_cavc_pline.cpp` | `CPP/Clipper2Lib/include/clipper2/clipper.h` | Exact or approximate parity candidates for Rust/old C++; Clipper2 reference-only. |
| Segment math and intersections | `cavalier_contours/src/polyline/pline_seg.rs`; `cavalier_contours/src/polyline/pline_seg_intersect.rs`; `cavalier_contours/tests/test_line_circle_intersect.rs` | `include/cavc/plinesegment.hpp`; `include/cavc/segmentintersects.hpp`; `tests/tests/TEST_cavc_pline_intersects.cpp` | `CPP/Tests/TestLines.cpp`; `CPP/Clipper2Lib/include/clipper2/clipper.core.h` | Arc-aware comparisons use Rust/old C++; Clipper2 line/path cases are external-oracle only. |
| Parallel offsets | `cavalier_contours/src/polyline/internal/pline_offset.rs`; `cavalier_contours/tests/test_pline_parallel_offset.rs`; `examples/parallel_offsets.rs` | `include/cavc/polylineoffset.hpp`; `tests/tests/TEST_cavc_parallel_offset.cpp`; `README.md` | `CPP/Clipper2Lib/include/clipper2/clipper.offset.h`; `CPP/Clipper2Lib/src/clipper.offset.cpp`; `CPP/Tests/TestOffsets.cpp` | Core absorption evidence. Clipper2 only for eligible polygon-only offset oracle cases. |
| Boolean/combine | `cavalier_contours/src/polyline/internal/pline_boolean.rs`; `cavalier_contours/tests/test_pline_boolean.rs`; `examples/boolean_ops.rs` | `include/cavc/polylinecombine.hpp`; `tests/tests/TEST_cavc_combine_plines.cpp` | `CPP/Clipper2Lib/include/clipper2/clipper.engine.h`; `CPP/Tests/TestPolygons.cpp`; `CPP/Tests/TestPolytree.cpp` | Rust scope is two closed non-self-intersecting polylines; broader Clipper2 cases are not comparable until scoped. |
| Containment and winding | `cavalier_contours/src/polyline/internal/pline_contains.rs`; `cavalier_contours/tests/test_pline_contains.rs`; `cavalier_contours/tests/test_pline_properties.rs` | `include/cavc/polyline.hpp`; `tests/tests/TEST_cavc_pline_function.cpp` | `CPP/Tests/TestOrientation.cpp`; `CPP/BenchMark/PointInPolygonBench.cpp` | Property parity candidate where geometry model matches. |
| Shape and multi-polyline operations | `cavalier_contours/src/shape_algorithms/mod.rs`; `cavalier_contours/tests/test_shape_algorithms.rs`; `cavalier_contours_ui/src/scenes/shape_offset.rs` | `include/cavc/polylineoffsetislands.hpp`; `examples/offsetislands.cpp` | `CPP/Clipper2Lib/include/clipper2/clipper.h`; `CPP/Tests/TestPolytreeHoles.cpp` | Rust shape behavior is closed-area scoped; Clipper2 hole/polytree cases need eligibility review. |
| Spatial indexing | `cavalier_contours/src/lib.rs` re-export of `static_aabb2d_index`; algorithm usage in `polyline/internal/*` | `include/cavc/staticspatialindex.hpp`; `tests/tests/TEST_staticspatialindex.cpp` | `not applicable`: Clipper2 internals are not the target interface for this roadmap | Rust/old C++ evidence candidate for performance and broad-phase behavior. |
| Cleanup and degenerates | `cavalier_contours/src/polyline/internal/pline_repeats.rs`; `cavalier_contours/tests/test_pline_remove_repeat_pos.rs`; `README.md` limitations | `include/cavc/polyline.hpp`; `tests/tests/TEST_cavc_pline.cpp` | `CPP/Tests/TestSimplifyPath.cpp`; `CPP/Tests/TestTrimCollinear.cpp` | Candidate evidence for later robustness ranking. |
| Tests | `cavalier_contours/tests/test_*.rs`; `cavalier_contours/tests/test_utils/` | `tests/tests/TEST_cavc_*.cpp`; `tests/tests/testhelpers.hpp` | `CPP/Tests/Test*.cpp` | Later fixture import/translation source; no import in Phase 1. |
| Benchmarks | `not found`: no current Rust benchmark baseline in repo | `tests/benchmarks/*.cpp`; `tests/benchmarks/benchmarkprofiles.h`; `README.md` benchmark notes | `CPP/BenchMark/*.cpp`; `CPP/Examples/Benchmarks/` | Phase 4 source inventory. Rust gap is benchmark baseline, not implementation behavior. |
| Examples and demos | `examples/*.rs`; `cavalier_contours_ui/src/scenes/*`; `README.md` | `examples/*.cpp`; `README.md` | `CPP/Examples/*`; `README.md` | Integration evidence and later smoke-test guidance. |
| Public API and ABI | `cavalier_contours/src/lib.rs`; `cavalier_contours_ffi/src/lib.rs`; `cavalier_contours_ffi.h`; `buildFFI.sh` | `include/cavc/*.hpp`; `c_api_include/cavaliercontours.h`; `src/cavaliercontours.cpp` | `CPP/Clipper2Lib/include/clipper2/clipper.h`; `clipper.offset.h`; `clipper.export.h` | See public surface comparison below. |
| Documented limitations | `README.md`; `.planning/codebase/CONCERNS.md` | `README.md` notes that C++ is no longer active; offset algorithm notes | `README.md` polygon library scope and triangulation warning | Limitations must be carried into fixture and oracle classification. |
| Triangulation | `not applicable`: out of v1 scope | `not found` | `CPP/Clipper2Lib/include/clipper2/clipper.triangulation.h`; `CPP/Clipper2Lib/src/clipper.triangulation.cpp`; `CPP/Examples/Triangulation/`; `README.md` warning | `deferred`; excluded from Phase 1 candidates. |

## Source Appendices

### Rust Mainline Paths

- Public exports: `cavalier_contours/src/lib.rs`.
- Core math and traits: `cavalier_contours/src/core/math/*`,
  `cavalier_contours/src/core/traits/*`.
- Polyline algorithms: `cavalier_contours/src/polyline/*`,
  `cavalier_contours/src/polyline/internal/pline_offset.rs`,
  `pline_boolean.rs`, `pline_intersects.rs`, `pline_contains.rs`,
  `pline_repeats.rs`.
- Shape algorithms: `cavalier_contours/src/shape_algorithms/mod.rs`.
- Tests: `cavalier_contours/tests/test_*.rs`,
  `cavalier_contours/tests/test_utils/*`.
- FFI: `cavalier_contours_ffi/src/lib.rs`, `cavalier_contours_ffi.h`.
- Examples/UI: `examples/*.rs`, `cavalier_contours_ui/src/scenes/*`.

### Old C++ Reference Paths

- Public headers: `include/cavc/*.hpp`.
- Offset/combine/intersection headers: `include/cavc/polylineoffset.hpp`,
  `polylinecombine.hpp`, `polylineintersects.hpp`, `plinesegment.hpp`.
- C API: `c_api_include/cavaliercontours.h`, `src/cavaliercontours.cpp`.
- Tests: `tests/tests/TEST_cavc_*.cpp`, `tests/tests/TEST_staticspatialindex.cpp`.
- Benchmarks: `tests/benchmarks/*.cpp`, `tests/benchmarks/benchmarkprofiles.h`.
- Examples and algorithm notes: `examples/*.cpp`, `README.md`.

### Clipper2 Reference Paths

- Public operations: `CPP/Clipper2Lib/include/clipper2/clipper.h`,
  `clipper.core.h`, `clipper.engine.h`, `clipper.offset.h`,
  `clipper.rectclip.h`, `clipper.minkowski.h`, `clipper.export.h`.
- Implementations for reference-only review: `CPP/Clipper2Lib/src/clipper.engine.cpp`,
  `CPP/Clipper2Lib/src/clipper.offset.cpp`.
- Tests: `CPP/Tests/Test*.cpp`.
- Benchmarks/examples: `CPP/BenchMark/*.cpp`, `CPP/Examples/*`.
- Deferred: `clipper.triangulation.h`, `clipper.triangulation.cpp`,
  `CPP/Examples/Triangulation/`.

## Behavior Taxonomy

| Classification | Meaning | Required evidence |
|----------------|---------|-------------------|
| `exact parity` | Strict property parity, not literal vertex sequence parity. Examples: result count, open/closed state, orientation, area, path length, extents, containment, and unexpected repeat vertices. | Source path, expected properties, and comparison property list. |
| `approximate parity` | Comparison depends on arc approximation or numeric tolerance. | Source path plus recorded tolerance or approximation policy. |
| `intentional divergence` | Rust behavior intentionally differs. | Decision-backed rationale from PROJECT, ROADMAP, CONTEXT, or phase artifacts. |
| `not comparable` | Model or scope mismatch, such as Clipper2 polygon paths vs native bulge arcs. | Source path or explicit `not applicable` plus reason. |
| `gap` | Missing or different behavior inside Rust target scope. | Rust evidence, reference evidence, and suggested follow-up phase. |

## Candidate Registry

| Candidate | Source path | Domain | Value | Risk | Follow-up |
|-----------|-------------|--------|-------|------|-----------|
| Historical offset regression cases | `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp` | Offsets | Same-lineage edge cases for open, closed, and self-intersecting offsets. | Arc and tolerance expectations may need property translation. | Phase 3 |
| Historical boolean/combine cases | `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp` | Booleans | Direct reference for OR/AND/NOT/XOR behavior. | Rust boolean scope may be narrower than old cases. | Phase 3 |
| Historical polyline function cases | `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp` | Properties | Good source for area, length, winding, containment, and distance expectations. | Exact vertex sequence should not be asserted. | Phase 3 |
| Static spatial index cases | `E:/Coding/CavalierContours/tests/tests/TEST_staticspatialindex.cpp` | Spatial index | Broad-phase behavior and performance-sensitive regression source. | Rust uses external `static_aabb2d_index`; compare behavior, not internals. | Phase 3/4 |
| Historical benchmark profiles | `E:/Coding/CavalierContours/tests/benchmarks/benchmarkprofiles.h` | Benchmarks | Named profile families such as square, circle, rounded rectangle, profiles, and pathological profiles. | Benchmark cost accounting and arc approximation must be documented. | Phase 4 |
| C++ offset algorithm notes | `E:/Coding/CavalierContours/README.md` | Algorithm behavior | Documents raw offsets, clipping, slicing, filtering, and stitching strategy. | Reference-only; no porting in Phase 1. | Phase 6/7 |
| Clipper2 offset tests | `E:/Coding/Clipper2/CPP/Tests/TestOffsets.cpp` | Polygon oracle | Polygon-only offset reference for eligible fixtures. | Not arc-aware; requires eligibility and tolerance policy. | Phase 5 |
| Clipper2 polygon tests | `E:/Coding/Clipper2/CPP/Tests/TestPolygons.cpp` | Polygon oracle | Boolean clipping cases for closed polygon behavior. | Broader clipping scope may be not comparable to Rust boolean scope. | Phase 5 |
| Clipper2 polytree/hole tests | `E:/Coding/Clipper2/CPP/Tests/TestPolytree.cpp`; `TestPolytreeHoles.cpp` | Shape/oracle | Hole and nesting behavior reference. | Rust shape offset scope is closed-area oriented; classify before import. | Phase 5 |
| Clipper2 simplify/collinear tests | `E:/Coding/Clipper2/CPP/Tests/TestSimplifyPath.cpp`; `TestTrimCollinear.cpp` | Cleanup | Degenerate polygon cleanup evidence. | Polygon-only; may be not comparable to bulge-arc cleanup. | Phase 5/6 |
| Current Rust tests | `E:/Coding/cavalier_contours/cavalier_contours/tests/test_*.rs` | Baseline | Existing expected behavior and helper patterns. | Existing tests may miss imported edge cases. | Phase 2/6 |

## Public Surface Comparison

| Surface | Evidence | Label | Migration note |
|---------|----------|-------|----------------|
| Rust public API | `cavalier_contours/src/lib.rs`; `cavalier_contours/src/polyline/mod.rs`; `cavalier_contours/src/shape_algorithms/mod.rs` | `fork-owned/changeable` | Changeable when it serves absorption goals; document externally visible changes. |
| Rust C FFI | `cavalier_contours_ffi/src/lib.rs`; `cavalier_contours_ffi.h`; `buildFFI.sh` | `fork-owned/changeable`, `migration-sensitive` | ABI/header changes require tests and regenerated header when the FFI surface changes. |
| Old C++ header API | `include/cavc/*.hpp` | `reference-only`, `migration-sensitive` | Compare concepts and migration expectations, not literal API preservation. |
| Old C++ C API | `c_api_include/cavaliercontours.h`; `src/cavaliercontours.cpp` | `reference-only`, `migration-sensitive` | Useful for migration notes and FFI parity discussion. |
| Clipper2 public operations | `CPP/Clipper2Lib/include/clipper2/clipper.h`; `clipper.offset.h`; `clipper.rectclip.h`; `clipper.minkowski.h` | `external-oracle` | Polygon-only reference for eligible cases; not a backend replacement. |
| Clipper2 triangulation | `CPP/Clipper2Lib/include/clipper2/clipper.triangulation.h` | `deferred`, `not comparable` | Excluded from this roadmap unless explicitly rescoped later. |

## Future API/FFI Impact Note Rule

Any later API or FFI change must name the changed surface, explain why the
change is worthwhile, and list affected tests, examples, FFI/header outputs, and
docs. Existing Rust API and FFI are not protected by default because this is a
fork, but impact notes prevent untracked surface drift.

## Requirement Coverage

| Requirement | Evidence in this artifact |
|-------------|---------------------------|
| AUD-01 | Source snapshots, main matrix, and source appendices. |
| AUD-03 | Behavior taxonomy and candidate registry. |
| AUD-04 | Public surface comparison and impact-note rule. |

## Deferred Follow-Ups

- Fixture schema and property harness: Phase 2.
- Old C++ fixture translation: Phase 3.
- Benchmark baseline and profile mapping: Phase 4.
- Clipper2 polygon-only oracle tooling: Phase 5.
- Robustness fixes and capability absorption: Phases 6 and 7.
- Triangulation: out of v1 scope.
