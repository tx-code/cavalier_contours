# Phase 09-01: C++ Boolean Parity Report

## Scope

This report covers deep parity checks for the old C++ combine case
`TEST_cavc_combine_plines.cpp` (circle vs rectangle across `Or/Not/And/Xor`)
mapped to executable Rust tests in `test_cpp_combine_parity.rs`.

## C++ to Rust Module Map

| C++ source | Rust source | Notes |
|------------|-------------|-------|
| `include/cavc/polylinecombine.hpp` | `cavalier_contours/src/polyline/internal/pline_boolean.rs` | Core boolean/combine algorithm mapping. |
| `tests/tests/TEST_cavc_combine_plines.cpp` | `cavalier_contours/tests/test_cpp_combine_parity.rs` | Direct parity execution target for this plan. |
| `include/cavc/polylineintersects.hpp` | `cavalier_contours/src/polyline/internal/pline_intersects.rs` | Intersection primitives used by boolean stitching. |
| public combine entry points | `cavalier_contours/src/polyline/traits.rs` (`boolean`, `boolean_opt`) | Public API parity surface. |

## Executed Case Outcomes

Evidence command:
`cargo test -p cavalier_contours --test test_cpp_combine_parity -- --nocapture`

| Operation | Geometry parity (area/path/extents) | Topology parity (vertex_count) | Classification |
|-----------|-------------------------------------|----------------------------------|----------------|
| `Or` | pass | C++ `[10]` vs Rust `[8]` | intentional-divergence |
| `Not` | pass | C++ `[3,3]` vs Rust `[2,2]` | intentional-divergence |
| `And` | pass | C++ `[4]` vs Rust `[4]` | parity (no mismatch) |
| `Xor` | pass | C++ `[3,3,4,4]` vs Rust `[2,2,4,4]` | intentional-divergence |

## Mismatch Classification Notes

- `intentional-divergence`: current Rust normalizes boolean output topology with
  fewer vertices while preserving the tested geometry invariants.
- `bug`: none confirmed in 09-01.
- `not-comparable`: none required for these four simple combine operations.

## 09-01 Decision

No boolean kernel rewrite in 09-01. Keep this as an explicit topology
divergence record and continue to 09-02 for offset/intersection parity depth.
