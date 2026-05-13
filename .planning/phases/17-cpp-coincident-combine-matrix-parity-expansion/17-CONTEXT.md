# Phase 17: cpp-coincident-combine-matrix-parity-expansion - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 17 deepens old C++ vs Rust parity (no Clipper) by importing coincident
combine case matrices from `TEST_cavc_combine_plines.cpp` into executable Rust
parity tests.

## Decisions

- **D-01:** Import source-traceable `coincident_case1` and `coincident_case2`
  expected property sets for `Or`, `Not`, `And`, `Xor`.
- **D-02:** Preserve explicit divergence classification when Rust behavior
  consistently differs from C++ expected emptiness in a bounded case.
- **D-03:** Keep scope at parity-test and classification depth; avoid kernel
  rewrites in this phase.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours/tests/test_cpp_combine_parity.rs`

