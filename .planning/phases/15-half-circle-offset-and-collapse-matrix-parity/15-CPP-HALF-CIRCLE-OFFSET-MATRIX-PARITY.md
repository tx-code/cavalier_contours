# Phase 15: Half-Circle Offset/Collapse Matrix Parity Report

## Scope

This report captures generated half-circle matrix parity imported from old C++
`TEST_cavc_pline_function.cpp::addHalfCircleCases` for:

- `parallel_offset` (outward + inward deltas)
- `collapsedOffsetDeltas` expectations

## Imported Matrix Coverage

Generated combinations executed in
`test_cpp_pline_function_parity.rs`:

- center: `(1,1)`, `(-1,1)`, `(-1,-1)`, `(1,-1)`
- closure: `open`, `closed`
- alignment: `x_aligned`, `y_aligned`
- direction: `ccw`, `cw`

Total generated cases: **32** (with offset + collapse checks per case).

Validated outputs:

- offset count expectation (`1` for outward/inward offsets)
- offset curve properties (area/path/extents/vertex count via property set)
- offset vertex-level parity:
  - open cases: exact order
  - closed cases: closed-curve start rotation tolerance
- collapsed offsets are empty for:
  - closed: `direction * 0.5r`, `direction * 1.5r`, `direction * 2r`
  - open: `direction * r`, `direction * 1.5r`, `direction * 2r`

## Classification

- `bug`: none confirmed in this imported matrix.
- `intentional-divergence`: none observed.
- `not-comparable`: broader C++ function suites outside `pline_function` remain
  out of this phase boundary.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_pline_function_parity -- --nocapture` - pass.
