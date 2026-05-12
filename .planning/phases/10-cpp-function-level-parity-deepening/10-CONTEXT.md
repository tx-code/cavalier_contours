# Phase 10: cpp-function-level-parity-deepening - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 10 extends parity depth beyond Phase 9 operation-level checks into
function-level behavior captured in old C++
`tests/tests/TEST_cavc_pline_function.cpp`.

Focus areas:
- `area`, `path_length`, `extents`, `winding_number`
- boolean combine-with-self invariants

Clipper remains explicitly out of scope for this phase.

## Implementation Decisions

- **D-01:** Start from stable, low-ambiguity C++ function-level expectations
  before broad randomized or generated imports.
- **D-02:** Record exact C++ source anchors for each translated parity assertion.
- **D-03:** Classify any new differences as `bug`, `intentional-divergence`, or
  `not-comparable`.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/src/polyline/traits.rs`
- `cavalier_contours/tests/test_cpp_combine_parity.rs`
- `cavalier_contours/tests/test_cpp_offset_parity.rs`

