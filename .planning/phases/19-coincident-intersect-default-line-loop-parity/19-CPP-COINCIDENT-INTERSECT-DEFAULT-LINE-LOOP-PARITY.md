# Phase 19: C++ Coincident Intersect Default-Path Line-Loop Parity

This report captures closure of the remaining default-path coincident intersect
gap after Phase 18.

## Scope

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust implementation:
  - `cavalier_contours/src/polyline/internal/pline_boolean.rs`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_combine_parity.rs`
  - `cavalier_contours/tests/test_pline_boolean.rs`

## Classification Update

### Before Phase 19

- `coincident_case1_intersect` default path:
  - classification: `intentional-divergence`
  - behavior: tiny line-only loop could remain

### After Phase 19

- `coincident_case1_intersect` default path:
  - classification: `parity`
  - behavior: degenerate line-only two-vertex loop is pruned during stitch

### Guard Scope

- Guard condition applies only when:
  - stitched result has exactly 2 vertices, and
  - both vertex bulges are zero (line-line loop)
- Valid two-vertex arc loops are preserved and covered by boolean regression
  tests.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_combine_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours --test test_pline_boolean -- --nocapture` - pass.
