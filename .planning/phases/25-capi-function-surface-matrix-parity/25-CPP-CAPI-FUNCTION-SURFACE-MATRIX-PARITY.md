# Phase 25: C++ C-API Function-Surface Matrix Parity

This report records C-API parity closure for old C++ function-surface generated
matrices through Rust FFI `cavc_pline_eval_*` calls.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust-core parity anchor:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_function_surface_circle_metrics_winding_cpp_matrix_parity`
- `pline_function_surface_half_circle_metrics_winding_cpp_matrix_parity`

Coverage executes matrix cases directly through:

- `cavc_pline_eval_area`
- `cavc_pline_eval_path_length`
- `cavc_pline_eval_extents`
- `cavc_pline_eval_wn`

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API circle function-surface matrix | parity | Area/path/extents/winding match source-traceable old C++ expectations across center/alignment/direction/reverse variants. |
| C-API half-circle function-surface matrix | parity | Area/path/extents/winding match source-traceable old C++ expectations across open/closed, x/y alignment, direction, and center variants. |
| C-API closest-point function matrix | not-comparable | `cavalier_contours_ffi/src/lib.rs` has no closest-point C API surface; parity stays at Rust-core test layer until API exists. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
