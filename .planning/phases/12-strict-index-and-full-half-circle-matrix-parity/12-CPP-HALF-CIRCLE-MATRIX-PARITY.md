# Phase 12: Full Half-Circle Matrix Parity Report

## Scope

This report expands old C++ `TEST_cavc_pline_function.cpp::addHalfCircleCases`
coverage from bounded subset to full generated matrix parity in Rust.

## Imported Matrix Coverage

Generated combinations now executed in
`test_cpp_pline_function_parity.rs`:

- center: `(1,1)`, `(-1,1)`, `(-1,-1)`, `(1,-1)`
- closure: `open`, `closed`
- alignment: `x_aligned`, `y_aligned`
- direction: `ccw`, `cw`

Total generated cases: **32**.

Validated properties:

- `area`
- `path_length`
- `extents`
- `winding_number`
- `closest_point` position/distance
- `closest_point.seg_start_index` (strict where C++ expects explicit index)

## Classification

- `bug`:
  - **fixed** in Phase 12:
    `closest_point` strict-index mismatch on closed half-circle endpoint/vertex
    tie cases. Rust previously kept first equal-distance segment; now tie-break
    prefers segment-start index, matching C++ expectations used by
    `addClosestPointOnVertexes`.
- `intentional-divergence`: none observed in imported matrix.
- `not-comparable`: full circle generated matrix and offset matrix families are
  still outside this phase boundary.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_pline_function_parity -- --nocapture` - pass.
