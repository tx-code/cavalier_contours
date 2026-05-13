# Phase 37: C++ C-API Pline Remove-Sequence Range-Equivalence Parity

This report records C-API parity closure for old `remove_range` semantics via
equivalent ordered `cavc_pline_remove` calls on current API.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- `pline_remove_sequence_equivalent_to_cpp_remove_range_parity`

Coverage dimensions:

- first removal equivalent to old `remove_range(0, 1)`
- second stage equivalent to old `remove_range(1, 2)`
- final stage equivalent to old `remove_range(0, 1)` on single-vertex remainder
- vertex-level intermediate assertions + final empty-count closure

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| Remove-range scenario parity via current API | parity | Ordered remove operations reproduce source-backed range-removal behavior step-by-step. |
| Intermediate vertex-state invariants | parity | Vertex values after each stage match source-backed expectations. |
| Final empty-state closure | parity | Final removal leaves zero vertices as expected in old suite scenario. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
