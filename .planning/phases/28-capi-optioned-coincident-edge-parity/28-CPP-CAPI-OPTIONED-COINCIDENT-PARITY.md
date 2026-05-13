# Phase 28: C++ C-API Optioned Coincident Edge Parity

This report records C-API optioned coincident edge parity through collapsed-area
edge behavior and options-path no-modify invariants.

## Scope

- C++ source references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust-core parity anchor:
  - `cavalier_contours/tests/test_cpp_combine_parity.rs`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_boolean_options_coincident_case1_intersect_collapsed_filter_cpp_parity`
- `pline_boolean_options_coincident_matrices_do_not_modify_input_cpp_parity`

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API optioned coincident intersect collapsed edge | parity | `collapsed_area_eps=1e-4` returns empty result for case1 intersect, matching source-backed parity route. |
| C-API options-path coincident no-modify matrix | parity | Subject and clip vertex buffers stay unchanged across case1/case2 operation matrices. |
| Exclusion direction options-path no-modify variants | parity | Both `A-B` and `B-A` are explicitly validated in options-path matrix checks. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
