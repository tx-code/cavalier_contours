# Phase 29: C++ C-API Optioned Coincident Output Parity

This report records C-API output parity between default-path and options-path
for source-backed coincident boolean matrices.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_boolean_options_coincident_matrices_output_cpp_parity`

Coverage executes coincident case1/case2 operation matrices and compares:

- default path: `cavc_pline_boolean(..., options=null)`
- options path: `cavc_pline_boolean(..., options=&cavc_pline_boolean_o)`

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API coincident options output parity | parity | Options-path remaining/subtracted property sets match default-path outputs across case1/case2 operation matrices. |
| Exclusion direction options output parity (`A-B`,`B-A`) | parity | Both direction variants are explicitly validated for output set parity. |
| New options output divergence | bug: none new | No options-path-only output drift surfaced in this phase. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
