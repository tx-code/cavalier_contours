# Phase 24: C++ C-API Combine No-Modify Parity

This report records C-API parity closure for old C++ combine no-modify input
invariants through `cavc_pline_boolean`.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_boolean_does_not_modify_input_cpp_parity`
  - operation matrix: `Or`, `Not`, `And`, `Xor`
  - validates:
    - subject (`pline_a`) vertex buffer unchanged
    - clip (`pline_b`) vertex buffer unchanged

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API combine input immutability | parity | Source-traceable no-modify behavior holds through boolean operation matrix. |
| Subject + clip unchanged guarantees | parity | Both buffers explicitly compared before/after each operation. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
