# Phase 95: cpp-circle-rectangle-intersection-variant-matrix-parity - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 95 deepens historical C++ circle/rectangle intersection parity by adding:

- swapped-operand expected-table assertions (index pairs + coordinates)
- bounded operand-order and direction-variant matrix invariants
- explicit cardinality and empty-overlapping checks across variants

## Decisions

- **D-01:** Keep this phase scoped to Rust parity tests and planning artifacts.
- **D-02:** Reuse the same circle/rectangle geometry traced from old C++ combine
  fixtures.
- **D-03:** Treat this as parity-evidence deepening only, not geometry kernel
  rewrite.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours/tests/test_cpp_offset_parity.rs`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`

