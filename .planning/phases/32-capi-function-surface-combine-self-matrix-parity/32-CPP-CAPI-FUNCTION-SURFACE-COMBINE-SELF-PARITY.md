# Phase 32: C++ C-API Function-Surface Combine-With-Self Matrix Parity

This report records C-API function-surface `combine_with_self_invariants` parity
closure across source-backed circle and closed half-circle matrix cases.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust-core parity anchor:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- Boolean output vertex extraction helper:
  - `plinelist_vertexes`
  - `run_boolean_vertexes`
- Matrix parity test:
  - `pline_function_surface_closed_matrix_combine_with_self_cpp_parity`

Coverage dimensions:

- circle matrix: x/y/diagonal alignment, direction, center, reverse variants
- half-circle matrix: closed x/y aligned + direction + center variants
- operations: union/intersect/exclude/xor self-combine invariants

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API function-surface combine-with-self matrix parity | parity | Union/intersect return one unchanged polyline; exclude/xor return empty across full matrix. |
| Output-vertex invariant parity | parity | Output vertices match source-backed input vertices for self-union and self-intersection cases. |
| Input no-modify behavior under matrix ops | parity | Input polyline vertices remain unchanged after all self boolean operations. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
