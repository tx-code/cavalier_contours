# Phase 11-01: C++ Closest-Point Parity Report

## Scope

This report records closest-point parity expansion sourced from old C++
`TEST_cavc_pline_function.cpp` into Rust
`test_cpp_pline_function_parity.rs`.

## Mapping

| C++ source | Rust parity | Notes |
|-----------|-------------|-------|
| `addClosestPointTestPt` circle center `y+0.1` | `cpp_circle_closest_point_parity` case 1 | Expected closest point `(1, 6)`, distance `4.9`. |
| `addClosestPointTestPt` circle center `y-0.1` | `cpp_circle_closest_point_parity` case 2 | Expected closest point `(1, -4)`, distance `4.9`. |

## Index Tie-Break Policy

- `index-check: skip` for these imported cases.
- Reason: old C++ source uses default `indexResult = max_u32` for these
  `addClosestPointTestPt` entries, which means index comparison is explicitly
  not required for these points.
- For future imports that provide explicit index expectations:
  - `index-check: strict`

## Evidence

`cargo test -p cavalier_contours --test test_cpp_pline_function_parity -- --nocapture` - pass.

