# Phase 18: C++ Logic Alignment Map

This map defines next deep parity targets after collapsed-filter parity-path
closure.

## Alignment Completed in Phase 18

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| `coincident_case1_intersect` empty expectation | explicit filtered parity path using `collapsed_area_eps` | `cavalier_contours/tests/test_cpp_combine_parity.rs` | complete |
| default-vs-filtered behavior classification | explicit divergence/parity split with evidence | `18-CPP-COINCIDENT-INTERSECT-COLLAPSED-FILTER-PARITY.md` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Default collapsed-area threshold adoption study | `cavalier_contours/src/polyline/pline_types.rs`, `cavalier_contours/src/polyline/internal/pline_boolean.rs`, `cavalier_contours/tests/test_pline_boolean.rs` | Only adopt non-`None` default if wide boolean matrix remains green and no regressions appear in thin-geometry cases. |
| P1 | Sliver suppression without default threshold (structural pruning route) | `cavalier_contours/src/polyline/internal/pline_boolean.rs` | Evaluate whether zero-area two-vertex slivers can be removed without affecting valid 2-vertex arc loops. |
| P2 | Additional C++ C-API pline parity mapping | `cavalier_contours_ffi/tests/test_pline.rs` and parity companions | Extend deep parity across C-API surface once boolean sliver decision is stabilized. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_combine_parity.rs`
- Rust geometry core:
  - `cavalier_contours/src/polyline/internal/pline_boolean.rs`
  - `cavalier_contours/src/polyline/pline_types.rs`

