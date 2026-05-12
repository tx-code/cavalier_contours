# Phase 09 Research: C++ Parity Deep Comparison

## Research Question

What concrete file/module alignment and test promotion is needed to determine
whether C++ vs Rust logic differences are bugs or intentional divergences?

## Inputs Reviewed

- C++: `tests/tests/TEST_cavc_combine_plines.cpp`,
  `include/cavc/polylinecombine.hpp`,
  `include/cavc/polylineoffset.hpp`,
  `include/cavc/polylineintersects.hpp`.
- Rust: `test_pline_boolean.rs`,
  `test_historical_cavalier_contours.rs`,
  `pline_boolean.rs`, `pline_offset.rs`, `pline_intersects.rs`.

## Findings

1. A known metadata-only gap exists for `circle_rectangle_union` in C++ combine
   tests: expected vertex count differs while area/path/extents match.
2. Existing Rust tests already validate geometry properties extensively but do
   not yet run a dedicated C++-named executable parity suite for this gap.
3. First value move is not immediate algorithm rewrite; it is promoting gap
   evidence to executable parity tests and collecting topology deltas.

## Phase 09-01 Focus

- Build explicit C++→Rust module map for combine logic.
- Add executable parity tests for C++ simple circle/rectangle combine modes.
- Record first mismatch classification with evidence.

## RESEARCH COMPLETE
