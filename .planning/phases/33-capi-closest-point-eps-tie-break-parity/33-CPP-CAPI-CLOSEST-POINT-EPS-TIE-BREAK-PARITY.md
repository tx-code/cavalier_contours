# Phase 33: C++ C-API Closest-Point Epsilon/Tie-Break Parity

This report records C-API closest-point epsilon/tie-break sensitivity parity
closure through source-backed explicit index probes.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust-core parity anchor:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- Epsilon matrix:
  - `CPP_CLOSEST_EPS_MATRIX`
- Circle explicit-index epsilon/tie-break matrix test:
  - `pline_function_surface_circle_closest_point_eps_tie_break_cpp_parity`
- Half-circle explicit-index epsilon/tie-break matrix test:
  - `pline_function_surface_half_circle_closest_point_eps_tie_break_cpp_parity`

Coverage dimensions:

- `pos_equal_eps`: `1e-9`, `1e-7`, `1e-5`, `1e-4`
- circle matrix vertex anchors (explicit index expectations)
- half-circle matrix explicit index probes (open/closed, x/y aligned, direction, centers)

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API closest-point epsilon/tie-break matrix parity | parity | Explicit source-backed index expectations remain stable across epsilon matrix. |
| Circle vertex tie-break behavior | parity | Closed-circle shared-vertex index choices stay deterministic under epsilon variation. |
| Half-circle strict index behavior under epsilon variation | parity | Endpoint and midpoint closest-point index/point/distance expectations stay stable across matrix variants. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
