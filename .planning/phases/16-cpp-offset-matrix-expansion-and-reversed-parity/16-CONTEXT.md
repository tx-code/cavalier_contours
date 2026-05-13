# Phase 16: cpp-offset-matrix-expansion-and-reversed-parity - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 16 deepens old C++ vs Rust parity (no Clipper) by importing broader
`TEST_cavc_parallel_offset.cpp` case matrices into executable Rust parity
tests, including reversed-input invariants and input-immutability checks.

## Decisions

- **D-01:** Keep imported expectations source-traceable to old C++ simple and
  specific case data tables.
- **D-02:** Preserve C++ reversed parity rule (`invert_direction` + negate
  offset delta, expected area sign flip).
- **D-03:** Keep scope at parity test depth; avoid core algorithm edits unless
  executable mismatches are confirmed.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours/tests/test_cpp_offset_parity.rs`

