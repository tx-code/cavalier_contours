# Phase 13: Full Circle Matrix Parity Report

## Scope

This report captures full generated circle matrix parity imported from old C++
`TEST_cavc_pline_function.cpp::addCircleCases`.

## Imported Matrix Coverage

Generated combinations executed in
`test_cpp_pline_function_parity.rs`:

- center: `(1,1)`, `(-1,1)`, `(-1,-1)`, `(1,-1)`
- reverse: `false`, `true`
- alignment: `x_aligned`, `y_aligned`, `not_axis_aligned`
- direction: `ccw`, `cw`

Total generated cases: **48**.

Validated properties:

- `area`
- `path_length`
- `extents`
- `winding_number` (axis and 45-degree probes)
- `closest_point` position/distance
- `closest_point.seg_start_index` for explicit vertex-index expectations

## Classification

- `bug`: none confirmed in this imported matrix.
- `intentional-divergence`: none observed.
- `not-comparable`: matrix-wide `parallel_offset` expected-vertex checks and
  `collapsedOffsetDeltas` remain out of this phase boundary.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_pline_function_parity -- --nocapture` - pass.
