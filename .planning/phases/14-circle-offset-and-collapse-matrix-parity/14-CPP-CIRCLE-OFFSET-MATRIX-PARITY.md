# Phase 14: Circle Offset/Collapse Matrix Parity Report

## Scope

This report captures generated circle matrix parity imported from old C++
`TEST_cavc_pline_function.cpp::addCircleCases` for:

- `parallel_offset` (outward + inward deltas)
- `collapsedOffsetDeltas` expectations

## Imported Matrix Coverage

Generated combinations executed in
`test_cpp_pline_function_parity.rs`:

- center: `(1,1)`, `(-1,1)`, `(-1,-1)`, `(1,-1)`
- reverse: `false`, `true`
- alignment: `x_aligned`, `y_aligned`, `not_axis_aligned`
- direction: `ccw`, `cw`

Total generated cases: **48** (with offset + collapse checks per case).

Validated outputs:

- offset count expectation (`1` for outward/inward circle offsets)
- offset curve properties (area/path/extents/vertex count via property set)
- offset vertex-level parity (position+bulge, allowing closed-curve start rotation)
- collapsed offsets are empty for:
  - `direction * radius`
  - `direction * 1.5 * radius`
  - `direction * 2.0 * radius`

## Classification

- `bug`: none confirmed in this imported matrix.
- `intentional-divergence`: none observed.
- `not-comparable`: half-circle generated offset vertex matrices remain out of
  this phase boundary.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_pline_function_parity -- --nocapture` - pass.
