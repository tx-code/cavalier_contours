# Phase 23: C++ C-API Parallel-Offset Matrix Parity

This report records C-API parity closure for old C++ `parallel_offset` matrices
through `cavc_pline_parallel_offset`.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- Rust-core parity anchor:
  - `cavalier_contours/tests/test_cpp_offset_parity.rs`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_parallel_offset_cpp_simple_matrix_parity`
- `pline_parallel_offset_cpp_specific_matrix_parity`
- `pline_parallel_offset_cpp_reversed_matrix_parity`
- `pline_parallel_offset_does_not_modify_input_cpp_parity`

All tests execute `cavc_pline_parallel_offset` directly and compare
source-traceable property sets.

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API simple offset matrix | parity | Property sets match old C++ expected behavior. |
| C-API specific offset matrix | parity | Property sets match old C++ expected behavior. |
| C-API reversed parity | parity | Negated delta + inverted input produce sign-adjusted parity. |
| C-API no-modify input invariant | parity | Input vertexes remain unchanged after offset call. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
