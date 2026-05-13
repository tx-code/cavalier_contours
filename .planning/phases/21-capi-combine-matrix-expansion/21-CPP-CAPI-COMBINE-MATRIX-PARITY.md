# Phase 21: C++ C-API Combine Matrix Parity

This report records expanded C-API parity coverage for old C++ combine
matrices through `cavc_pline_boolean`.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_boolean_circle_rectangle_cpp_matrix_parity`
  - validates `Or`, `Not`, `And`, `Xor`
- `pline_boolean_coincident_case2_cpp_matrix_parity`
  - validates `Or`, `Not` (A-B), `Not` (B-A), `And`, `Xor`

Both tests:

- execute `cavc_pline_boolean` directly
- compare unordered property sets (`vertex_count`, `|area|`, `path_length`,
  `extents`)
- assert expected `remaining` and `subtracted` result sets

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API `circle_rectangle` matrix | parity | Source-traceable property sets match old C++ expectations. |
| C-API `coincident_case2` matrix | parity | Source-traceable property sets match old C++ expectations. |
| C-API `coincident_case1_intersect` bridge (Phase 20) | parity | Retained and still green under expanded matrix coverage. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
