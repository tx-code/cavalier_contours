# Phase 27: C++ C-API Coincident No-Modify Matrix Parity

This report records C-API no-modify parity expansion from simple cases to
source-backed coincident combine matrices.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_boolean_coincident_matrices_do_not_modify_input_cpp_parity`

The test executes no-modify checks for:

- coincident case1: union, exclude (`A-B`), exclude (`B-A`), intersect, xor
- coincident case2: union, exclude (`A-B`), exclude (`B-A`), intersect, xor

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API coincident case1 no-modify matrix | parity | Subject and clip vertex buffers stay unchanged across full operation matrix. |
| C-API coincident case2 no-modify matrix | parity | Subject and clip vertex buffers stay unchanged across full operation matrix. |
| Exclusion direction coverage | parity | Both `A-B` and `B-A` no-modify invariants are explicitly validated. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
