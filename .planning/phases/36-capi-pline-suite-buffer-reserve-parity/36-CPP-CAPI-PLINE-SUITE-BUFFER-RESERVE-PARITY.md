# Phase 36: C++ C-API Pline Suite Buffer/Reserve Parity

This report records C-API parity closure for remaining source-backed
`TEST_cavc_pline.cpp` edge semantics: empty-buffer read safety and reserve
non-modification.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_get_vertex_data_empty_does_not_modify_buffer_cpp_parity`
- `pline_reserve_does_not_modify_existing_vertex_data_cpp_parity`

Coverage dimensions:

- empty polyline buffer no-write invariant for `cavc_pline_get_vertex_data`
- reserve non-modification invariant for populated polyline vertices

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| Empty-buffer vertex-data read safety | parity | Sentinel buffer remains unchanged when reading vertex data from empty polyline. |
| Reserve vertex persistence invariant | parity | Reserve calls do not alter existing vertex values on populated polyline. |
| Pline suite parity hardening | parity | Source-backed pline-suite edge semantics now have explicit C-API regression coverage. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
