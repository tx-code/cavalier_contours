# Phase 26: C++ C-API Options-Path Parity

This report records C-API options-path parity closure against default-path
behavior for source-backed boolean and offset matrix cases.

## Scope

- C++ source references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_boolean_options_path_circle_rectangle_cpp_parity`
- `pline_parallel_offset_options_path_cpp_matrix_parity`

Options-path parity compares property sets from:

- default path: null options
- options path: initialized options + AABB index pointer path

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API boolean options-path parity | parity | Options-path outputs match default-path outputs across full circle/rectangle operation matrix (`Or`, `Not`, `And`, `Xor`). |
| C-API offset options-path parity | parity | Options-path outputs match default-path outputs across imported simple and specific offset matrices. |
| New optioned behavior divergence | bug: none new | No options-path-only divergence surfaced in this phase. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
