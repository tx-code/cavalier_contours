# Phase 30: C++ C-API Closest-Point Parity

This report records C-API closest-point parity bridge closure, replacing the
prior closest-point not-comparable gap at FFI boundary.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust-core parity anchor:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust FFI surface:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`

## Added Coverage

- New C-API function:
  - `cavc_pline_eval_closest_point`
- New FFI tests:
  - `pline_eval_closest_point`
  - `pline_function_surface_circle_closest_point_cpp_matrix_parity`

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API closest-point function surface | parity | C-API exposes closest-point evaluation with index/point/distance outputs and explicit null/empty behavior codes. |
| Circle matrix closest-point probes | parity | Vertex anchors plus axis/45-degree probes match source-backed expectations across generated circle cases. |
| ABI/header sync | parity | Root header regenerated to include `cavc_pline_eval_closest_point`. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
