# Phase 22: C++ C-API Combine Self-Invariants Parity

This report records C-API parity closure for old C++ combine-with-self
invariants through `cavc_pline_boolean`.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_boolean_combine_with_self_invariants_cpp_parity`
  - base polyline vs self:
    - Or => self
    - And => self
    - Not => empty
    - Xor => empty
  - reversed polyline vs self:
    - same invariants
  - mixed-orientation pairs:
    - Not and Xor => empty

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API combine-with-self invariants | parity | Source-traceable invariants from old C++ hold through `cavc_pline_boolean`. |
| Reversed and mixed-orientation empty-result invariants | parity | Explicitly exercised and green in FFI tests. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
