# Phase 34: C++ C-API Function-Surface Parallel-Offset Full Matrix Parity

This report records C-API function-surface full-matrix parallel-offset and
collapsed offset parity closure through source-backed generated circle and
half-circle cases.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust-core parity anchor:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- Offset output extraction helper:
  - `run_parallel_offset_vertexes`
- Closed/open vertex matching helpers:
  - `closed_vertexes_match_with_rotation`
  - `open_vertexes_match_exact`
  - `assert_single_offset_vertex_match`
- Full-matrix parity tests:
  - `pline_function_surface_circle_parallel_offset_cpp_matrix_parity`
  - `pline_function_surface_circle_collapsed_offset_cpp_matrix_parity`
  - `pline_function_surface_half_circle_parallel_offset_cpp_matrix_parity`
  - `pline_function_surface_half_circle_collapsed_offset_cpp_matrix_parity`

Coverage dimensions:

- circle matrix: x/y/diagonal alignment, direction, center, reverse variants
- half-circle matrix: open/closed x/y alignment, direction, center variants
- outward/inward deltas + collapsed deltas from source formulas

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API function-surface full-matrix parallel-offset parity | parity | Circle and half-circle generated matrices match source-backed outward/inward offset expectations at vertex level. |
| C-API collapsed offset parity | parity | Source-backed collapsed deltas return empty offset results across matrix variants. |
| Closed/open vertex matching invariants | parity | Closed outputs accept rotational equivalence; open outputs require exact order, matching old C++ comparison semantics. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
